//! basically copied from the main repo of /ccanvas
use serde::{Deserialize, Serialize};

use crate::bindings::Discriminator;

#[derive(Deserialize, Clone, PartialEq, Debug, Hash)]
#[serde(tag = "type")]
pub enum EventVariant {
    /// keyboard event
    #[serde(rename = "key")]
    Key(KeyEvent),
    /// mouse event
    #[serde(rename = "mouse")]
    Mouse(MouseEvent),
    /// screen resize event (should trigger a rerender)
    #[serde(rename = "resize")]
    Resize { width: u32, height: u32 },
    /// message passed from another process
    #[serde(rename = "message")]
    Message {
        sender: Discriminator,
        target: Discriminator,
        content: String,
    },
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "unfocused")]
    Unfocused,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct KeyEvent {
    /// the keycode represented by the characetr
    pub code: KeyCode,
    /// key modifiers (e.g. ctrl)
    pub modifier: KeyModifier,
}

impl KeyEvent {
    pub fn new(code: KeyCode, modifier: KeyModifier) -> Self {
        Self { code, modifier }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum KeyModifier {
    #[serde(rename = "alt")]
    Alt,
    /// note that certain keys may not be modifiable with ctrl, due to limitations of terminals.
    #[serde(rename = "ctrl")]
    Ctrl,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum KeyCode {
    /// Backspace.
    #[serde(rename = "backspace")]
    Backspace,
    /// Left arrow.
    #[serde(rename = "left")]
    Left,
    /// Right arrow.
    #[serde(rename = "right")]
    Right,
    /// Up arrow.
    #[serde(rename = "up")]
    Up,
    /// Down arrow.
    #[serde(rename = "down")]
    Down,
    /// Home key.
    #[serde(rename = "home")]
    Home,
    /// End key.
    #[serde(rename = "end")]
    End,
    /// Page Up key.
    #[serde(rename = "pageup")]
    PageUp,
    /// Page Down key.
    #[serde(rename = "pagedown")]
    PageDown,
    /// Backward Tab key.
    #[serde(rename = "backtab")]
    BackTab,
    /// Delete key.
    #[serde(rename = "delete")]
    Delete,
    /// Insert key.
    #[serde(rename = "insert")]
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    #[serde(rename = "f")]
    F(u8),
    /// Normal character.
    #[serde(rename = "char")]
    Char(char),
    /// Null byte.
    #[serde(rename = "null")]
    Null,
    /// Esc key.
    #[serde(rename = "esc")]
    Esc,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct MouseEvent {
    /// where the mouse event is
    pub x: u32,
    pub y: u32,
    #[serde(rename = "type")]
    /// what kind of event it is
    pub r#type: MouseType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MouseType {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
    /// mouse release
    Release,
    /// is only emitted when u move the mouse, and only applies to left click
    Hold,
}
