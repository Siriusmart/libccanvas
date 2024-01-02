use std::{
    collections::HashMap,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
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
    ResponseContent, ResponseSuccess, Subscription,
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

    /// path to request socket
    request_socket: PathBuf,
    /// unflushed render requests
    render_requests: Vec<RenderRequest>,
    /// confirmation handles for requests
    req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(ClientConfig::default())
    }
}

static mut REQID: OnceCell<u32> = OnceCell::const_new_with(0);

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        // creates the listener socket
        let listener = UnixListener::bind(&config.listener_socket).unwrap();

        let (inbound_send, inbound_recv) = mpsc::unbounded_channel();
        let (outbound_send, mut outbound_recv) = mpsc::unbounded_channel();
        let req_confirms: Arc<Mutex<HashMap<u32, oneshot::Sender<ResponseContent>>>> =
            Arc::new(Mutex::new(HashMap::default()));

        // connects and set the listener
        let set_socket = Request::new(
            Discriminator::default(),
            RequestContent::SetSocket {
                path: config.listener_socket,
            },
        );
        UnixStream::connect(&config.request_socket)
            .unwrap()
            .write_all(serde_json::to_vec(&set_socket).unwrap().as_slice())
            .unwrap();

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

        let request_handle = {
            let request_socket = config.request_socket.clone();
            // simply sends Request to canvas
            tokio::task::spawn(async move {
                while let Some(req) = outbound_recv.recv().await {
                    let request_socket = request_socket.clone();
                    tokio::task::spawn_blocking(move || {
                        UnixStream::connect(request_socket)
                            .unwrap()
                            .write_all(serde_json::to_vec(&req).unwrap().as_slice())
                            .unwrap();
                    });
                }
            })
        };

        Self {
            listener_handle,
            request_handle,
            inbound_recv: Arc::new(Mutex::new(inbound_recv)),
            outbound_send,
            request_socket: config.request_socket,
            render_requests: Vec::new(),
            req_confirms,
        }
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
    pub async fn subscribe<T: Into<(Subscription, Option<u32>)>>(&self, channel: T) -> ResponseContent {
        let (channel, priority) = channel.into();
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Subscribe {
                channel,
                priority,
                component: None,
            },
        );
        self.send(req).await
    }

    pub async fn subscribe_multiple<T: Into<(Subscription, Option<u32>)>>(&self, channels: Vec<T>) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Subscribe {
                channel: Subscription::Multiple { subs: channels.into_iter().map(|item| item.into()).collect() },
                priority: None,
                component: None,
            },
        );
        self.send(req).await
    }

    pub async fn unsubscribe(&self, channel: Subscription) -> ResponseContent {
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Unsubscribe {
                channel,
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

    pub fn setchar(&mut self, x: u32, y: u32, c: char) {
        self.render_requests.push(RenderRequest::setchar(x, y, c))
    }

    pub fn setcharcoloured(&mut self, x: u32, y: u32, c: char, fg: Colour, bg: Colour) {
        self.render_requests
            .push(RenderRequest::setchar_coloured(x, y, c, fg, bg))
    }

    pub fn setcursorstyle(&mut self, style: CursorStyle) {
        self.render_requests.push(RenderRequest::setcursor(style))
    }

    pub fn showcursor(&mut self) {
        self.render_requests.push(RenderRequest::ShowCursor)
    }

    pub fn hidecursor(&mut self) {
        self.render_requests.push(RenderRequest::HideCursor)
    }

    pub async fn renderall(&mut self) -> ResponseContent {
        if self.render_requests.is_empty() {
            return ResponseContent::Success {
                content: ResponseSuccess::Rendered,
            };
        }

        let req = Request::new(
            Discriminator::default(),
            RequestContent::Render {
                flush: true,
                content: RenderRequest::RenderMultiple {
                    tasks: std::mem::take(&mut self.render_requests),
                },
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
        let req = Request::new(
            Discriminator::default(),
            RequestContent::Drop { discrim: None },
        );
        UnixStream::connect(self.request_socket.clone())
            .unwrap()
            .write_all(serde_json::to_vec(&req).unwrap().as_slice())
            .unwrap();
    }
}
