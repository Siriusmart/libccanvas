use serde::Serialize;

use crate::bindings::{Discriminator, KeyCode, KeyEvent, KeyModifier, MouseType};

#[derive(Hash, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Subscription {
    /// subscribes to all key press events
    #[serde(rename = "all key presses")]
    AllKeyPresses,
    /// all mouse click and drag events
    #[serde(rename = "all mouse events")]
    AllMouseEvents,
    /// subscribe to all messages from other components
    #[serde(rename = "all messages")]
    AllMessages,
    /// a specific key event
    #[serde(rename = "specific key press")]
    SpecificKeyPress { key: KeyEvent },
    /// all key events with that key modifier
    #[serde(rename = "specific key modifier")]
    SpecificKeyModifier { modifier: KeyModifier },
    /// all key events with that key modifier
    #[serde(rename = "specific key code")]
    SpecificKeyCode { code: KeyCode },
    /// a specific mouse event
    #[serde(rename = "specific mouse event")]
    SpecificMouseEvent { mouse: MouseType },
    /// a specific message from someone
    #[serde(rename = "specific message")]
    SpecificMessage { source: Discriminator },
    /// screen resize events
    #[serde(rename = "screen resize")]
    ScreenResize,
    #[serde(rename = "focused")]
    /// current space focused
    Focused,
    #[serde(rename = "unfocused")]
    /// current space unfocused
    Unfocused,

    #[serde(rename = "multiple")]
    /// subscribe to multiple channels at once
    Multiple { subs: Vec<(Subscription, Option<u32>)> },
}

impl Subscription {
    pub fn specific_keypress(key: KeyEvent) -> Self {
        Self::SpecificKeyPress { key }
    }

    pub fn specific_keycode(code: KeyCode) -> Self {
        Self::SpecificKeyCode { code }
    }
    
    pub fn specific_keymodifier(modifier: KeyModifier) -> Self {
        Self::SpecificKeyModifier { modifier }
    }

    pub fn specific_mouse(mousetype: MouseType) -> Self {
        Self::SpecificMouseEvent { mouse: mousetype }
    }

    pub fn specific_message(source: Discriminator) -> Self {
        Self::SpecificMessage { source }
    }
}

impl Into<(Self, Option<u32>)> for Subscription {
    fn into(self) -> (Self, Option<u32>) {
        (self, None)
    }
}
