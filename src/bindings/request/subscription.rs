use serde::Serialize;

#[derive(Hash, PartialEq, Eq, Serialize, Debug, Clone)]
pub enum Subscription {
    /// subscribes to all key press events
    #[serde(rename = "all key presses")]
    AllKeyPresses,
}
