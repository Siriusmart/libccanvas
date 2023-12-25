use std::path::PathBuf;

use serde::Serialize;

use crate::{bindings::Discriminator, client::Client};

use super::Subscription;

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    /// reciever
    target: Discriminator,
    /// the content of the request
    pub content: RequestContent,
    /// confirmation identifier
    id: u32,
}

impl Request {
    /// construct new self with a unique id
    pub fn new(target: Discriminator, content: RequestContent) -> Self {
        Self {
            target,
            content,
            id: Client::reqid(),
        }
    }

    /// returns id of self
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum RequestContent {
    #[serde(rename = "confirm recieve")]
    /// confirm that an event has been recieved
    ConfirmRecieve {
        /// event id
        id: u32,
        /// true = does not capture event
        pass: bool,
    },

    #[serde(rename = "subscribe")]
    /// add subscription to a channel with priority
    Subscribe {
        channel: Subscription,
        priority: Option<u32>,
        component: Option<Discriminator>,
    },

    #[serde(rename = "set socket")]
    /// sent responses to this socket
    SetSocket { path: PathBuf },

    #[serde(rename = "drop")]
    /// drop (remove) a component
    Drop { discrim: Option<Discriminator> },

    #[serde(rename = "render")]
    /// render something on canvas
    Render { content: RenderRequest },
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub enum RenderRequest {
    #[serde(rename = "set char")]
    SetChar { x: u32, y: u32, c: char },
}
