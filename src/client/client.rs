use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    sync::Arc,
};

use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot, Mutex, OnceCell,
    },
    task::JoinHandle,
};

use crate::bindings::{
    Colour, CursorStyle, Discriminator, Event, RenderRequest, Request, RequestContent, Response,
    ResponseContent, Subscription,
};

use super::ClientConfig;

pub struct Client {
    /// task handle to the listener loop
    listener_handle: JoinHandle<()>,
    /// task handle for sender loop
    request_handle: JoinHandle<()>,
    /// incoming events
    inbound_recv: Arc<Mutex<UnboundedReceiver<Event>>>,
    /// request to ccanvas
    outbound_send: UnboundedSender<Request>,

    /// confirmation handles for requests
    req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>>,
}

static mut REQID: OnceCell<u32> = OnceCell::const_new_with(0);

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self, Box<dyn Error>> {
        // creates the listener socket
        let listener = UnixListener::bind(&config.listener_socket)?;

        // connects and set the listener
        UnixStream::connect(&config.request_socket)?.write_all(
            serde_json::to_vec(&Request::new(
                Discriminator::default(),
                RequestContent::SetSocket {
                    path: config.listener_socket,
                },
            ))
            .unwrap()
            .as_slice(),
        )?;

        let (inbound_send, inbound_recv) = mpsc::unbounded_channel();
        let (outbound_send, mut outbound_recv) = mpsc::unbounded_channel();
        let req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>> =
            Arc::new(Mutex::new(HashMap::default()));

        let listener_handle = {
            let outbound_send = outbound_send.clone();
            let req_confirms = req_confirms.clone();
            tokio::task::spawn_blocking(move || {
                let outbound_send = outbound_send.clone();
                for stream in listener.incoming() {
                    let mut stream = match stream {
                        Ok(stream) => stream,
                        Err(_) => continue,
                    };

                    let mut msg = String::new();
                    match stream.read_to_string(&mut msg) {
                        Ok(_) => {}
                        Err(_) => continue,
                    }

                    let res: Response = match serde_json::from_str(&msg) {
                        Ok(res) => res,
                        Err(_) => continue,
                    };

                    match res.content {
                        // events have to be confirmed
                        ResponseContent::Event { content } => {
                            inbound_send
                                .send(Event::new(content, outbound_send.clone(), res.id))
                                .unwrap();
                        }
                        // these are responses from canvas
                        // and dont have to be confirmed
                        // but their wait locks have to be released
                        // so the callers can know the task is done
                        ResponseContent::Error { .. }
                        | ResponseContent::Success { .. }
                        | ResponseContent::Undelivered => {
                            if let Some(entry) = tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(req_confirms.lock())
                                .remove(&res.request.unwrap())
                            {
                                entry.send(res.content).unwrap();
                            }
                        }
                    }
                }
            })
        };

        // simply sends Request to canvas
        let request_handle = tokio::task::spawn(async move {
            while let Some(req) = outbound_recv.recv().await {
                let request_socket = config.request_socket.clone();
                tokio::task::spawn_blocking(move || {
                    UnixStream::connect(request_socket)
                        .unwrap()
                        .write_all(serde_json::to_vec(&req).unwrap().as_slice())
                        .unwrap();
                });
            }
        });

        Ok(Self {
            listener_handle,
            request_handle,
            inbound_recv: Arc::new(Mutex::new(inbound_recv)),
            outbound_send,
            req_confirms,
        })
    }

    /// get a unique request id
    pub fn reqid() -> u32 {
        let id = unsafe { REQID.get_mut() }.unwrap();
        *id += 1;
        *id
    }

    /// there should only be one recv() per program
    /// more than one recv() at a time results in almost randomised behaviour
    pub async fn recv(&self) -> Option<Event> {
        self.inbound_recv.lock().await.recv().await
    }

    /// send a request
    /// private method as the convenience functions should be used instead
    async fn send(&self, req: Request) -> ResponseContent {
        let (tx, rx) = oneshot::channel();
        self.req_confirms.lock().await.insert(req.id(), tx);
        self.outbound_send.send(req).unwrap();
        rx.await.unwrap()
    }
}

/// convenience functions
impl Client {
    pub async fn subscribe(&self, channel: Subscription) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Subscribe {
                channel,
                priority: None,
                component: None,
            },
        );
        self.send(req).await
    }

    pub async fn exit(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Drop {
                discrim: Some(Discriminator::new(vec![1])),
            },
        );
        self.send(req).await
    }

    pub async fn setchar(&self, x: u32, y: u32, c: char) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetChar { x, y, c },
                flush: true,
            },
        );
        self.send(req).await
    }

    pub async fn setchar_noflush(&self, x: u32, y: u32, c: char) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetChar { x, y, c },
                flush: false,
            },
        );
        self.send(req).await
    }

    pub async fn setcharcoloured(
        &self,
        x: u32,
        y: u32,
        c: char,
        fg: Colour,
        bg: Colour,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetCharColoured { x, y, c, fg, bg },
                flush: true,
            },
        );
        self.send(req).await
    }

    pub async fn setcharcoloured_noflush(
        &self,
        x: u32,
        y: u32,
        c: char,
        fg: Colour,
        bg: Colour,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetCharColoured { x, y, c, fg, bg },
                flush: false,
            },
        );
        self.send(req).await
    }

    pub async fn setcursorstyle(&self, style: CursorStyle) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetCursorStyle { style },
                flush: true,
            },
        );
        self.send(req).await
    }

    pub async fn setcursorstyle_noflush(&self, style: CursorStyle) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::SetCursorStyle { style },
                flush: false,
            },
        );
        self.send(req).await
    }

    pub async fn showcursor(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::ShowCursor,
                flush: true,
            },
        );
        self.send(req).await
    }

    pub async fn showcursor_noflush(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::ShowCursor,
                flush: false,
            },
        );
        self.send(req).await
    }

    pub async fn hidecursor(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::HideCursor,
                flush: true,
            },
        );
        self.send(req).await
    }

    pub async fn hidecursor_noflush(&self) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                content: RenderRequest::HideCursor,
                flush: false,
            },
        );
        self.send(req).await
    }

    pub async fn spawn_at(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
        parent: Discriminator,
    ) -> ResponseContent {
        let req = Request::new(
            parent,
            RequestContent::Spawn {
                command,
                args,
                label,
            },
        );
        self.send(req).await
    }

    pub async fn spawn(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
    ) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Spawn {
                command,
                args,
                label,
            },
        );
        self.send(req).await
    }

    pub async fn message(&self, target: Discriminator, content: String) -> ResponseContent {
        let req = Request::new(
            target.clone(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target,
            },
        );
        self.send(req).await
    }

    pub async fn broadcast(&self, content: String) -> ResponseContent {
        let req = Request::new(
            Discriminator::master(),
            RequestContent::Message {
                content,
                sender: Discriminator::default(),
                target: Discriminator::master(),
            },
        );
        self.send(req).await
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.listener_handle.abort();
        self.request_handle.abort();
    }
}
