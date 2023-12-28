use serde::Deserialize;

use crate::bindings::Discriminator;

use super::EventVariant;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Response {
    /// the content of the response
    pub content: ResponseContent,

    /// send a confirmation to the server using this id
    /// to confirm recieved
    pub id: u32,

    /// request id for confirmation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<u32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseContent {
    #[serde(rename = "undelivered")]
    Undelivered,

    #[serde(rename = "event")]
    Event { content: EventVariant },

    #[serde(rename = "error")]
    Error { content: ResponseError },

    #[serde(rename = "success")]
    Success { content: ResponseSuccess },
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseError {
    #[serde(rename = "component not found")]
    ComponentNotFound,
    #[serde(rename = "spawn failed")]
    SpawnFailed,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseSuccess {
    #[serde(rename = "subscribe added")]
    SubscribeAdded,

    #[serde(rename = "listener set")]
    ListenerSet,

    #[serde(rename = "dropped")]
    Dropped,

    #[serde(rename = "rendered")]
    Rendered,

    #[serde(rename = "spawned")]
    Spawned { discrim: Discriminator },

    #[serde(rename = "message delivered")]
    MessageDelivered,
}
