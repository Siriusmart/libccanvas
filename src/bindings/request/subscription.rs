use serde::Serialize;

#[derive(Hash, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Subscription {
    /// subscribes to all key press events
    #[serde(rename = "all key presses")]
    AllKeyPresses,
    /// subscribe to all messages from other components
    #[serde(rename = "all messages")]
    AllMessages,
}
