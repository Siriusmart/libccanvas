use tokio::sync::mpsc::UnboundedSender;

use crate::bindings::{Discriminator, Request, RequestContent};

use super::EventVariant;

/// an event binding struct, sends a confirm request when dropped
pub struct Event {
    /// real content of the event
    content: EventVariant,
    /// confirmation handle to release the event
    confirm: Option<(u32, UnboundedSender<Request>)>,
}

impl Event {
    pub fn new(content: EventVariant, sender: UnboundedSender<Request>, confirm: u32) -> Self {
        Self {
            content,
            confirm: Some((confirm, sender)),
        }
    }

    /// mark the event as done and release the event
    /// pass = false will capture the event and no lower components can recieve it
    pub fn done(&mut self, pass: bool) {
        if let Some((id, sender)) = std::mem::take(&mut self.confirm) {
            sender
                .send(Request::new(
                    Discriminator::default(),
                    RequestContent::ConfirmRecieve { id, pass },
                ))
                .unwrap();
        }
    }

    /// returns a reference to the content
    pub fn get(&self) -> &EventVariant {
        &self.content
    }

    /// returns a mutable reference to the content
    pub fn get_mut(&mut self) -> &mut EventVariant {
        &mut self.content
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        self.done(true)
    }
}
