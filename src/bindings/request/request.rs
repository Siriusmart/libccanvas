use std::path::PathBuf;

use serde::Serialize;

use crate::{bindings::Discriminator, client::Client};

use super::Subscription;

#[derive(Serialize, Debug, Clone)]
/// request to send to the server
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
/// the real content of the request sent to server
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

    #[serde(rename = "Unsubscribe")]
    /// remove subscription from a channel
    Unsubscribe {
        channel: Subscription,
        component: Option<Discriminator>,
    },

    #[serde(rename = "set socket")]
    /// sent responses to this socket
    SetSocket { path: PathBuf },

    #[serde(rename = "drop")]
    /// remove a single component
    Drop { discrim: Option<Discriminator> },

    #[serde(rename = "render")]
    /// render something to the terminal
    Render { content: RenderRequest, flush: bool },

    #[serde(rename = "spawn")]
    /// spawn a new process
    Spawn {
        command: String,
        args: Vec<String>,
        label: String,
    },

    #[serde(rename = "message")]
    /// send a message to another component
    /// if target specifies a space,
    /// all components under that space will recieve the message
    Message {
        content: String,
        sender: Discriminator,
        target: Discriminator,
    },

    /// create a new space at a space
    #[serde(rename = "new space")]
    NewSpace { label: String },

    /// focus a specific space
    #[serde(rename = "focus at")]
    FocusAt,
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
/// a render request to the server
pub enum RenderRequest {
    /// change a single character
    #[serde(rename = "set char")]
    SetChar { x: u32, y: u32, c: char },
    /// change a single character, coloured
    #[serde(rename = "set colouredchar")]
    SetCharColoured {
        x: u32,
        y: u32,
        c: char,
        fg: Colour,
        bg: Colour,
    },
    /// all the terminal to flush all changes, this is usually not needed
    #[serde(rename = "flush")]
    Flush,
    /// set cursor looks
    #[serde(rename = "set cursorstyle")]
    SetCursorStyle { style: CursorStyle },
    /// hide cursor
    #[serde(rename = "hide cursor")]
    HideCursor,
    /// unhide cursor
    #[serde(rename = "show cursor")]
    ShowCursor,

    /// render multiple items at the same time - guaranteed to be rendered at the same time, and
    /// socket performance is significantly better than sending individual requests.
    #[serde(rename = "render multiple")]
    RenderMultiple { tasks: Vec<Self> },
}

impl RenderRequest {
    /// create a setchar request
    pub fn setchar(x: u32, y: u32, c: char) -> Self {
        Self::SetChar { x, y, c }
    }

    /// create a setchar coloured request
    pub fn setchar_coloured(x: u32, y: u32, c: char, fg: Colour, bg: Colour) -> Self {
        Self::SetCharColoured { x, y, c, fg, bg }
    }

    /// create a setcursor request
    pub fn setcursor(style: CursorStyle) -> Self {
        Self::SetCursorStyle { style }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
/// how the cursor should look
pub enum CursorStyle {
    #[serde(rename = "blinking bar")]
    BlinkingBar,
    #[serde(rename = "blinking block")]
    BlinkingBlock,
    #[serde(rename = "blinking underline")]
    BlinkingUnderline,
    #[serde(rename = "steady bar")]
    SteadyBar,
    #[serde(rename = "steady block")]
    SteadyBlock,
    #[serde(rename = "steady underline")]
    SteadyUnderline,
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
/// generic colours
pub enum Colour {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "magenta")]
    Magenta,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "yellow")]
    Yellow,

    #[serde(rename = "lightblack")]
    LightBlack,
    #[serde(rename = "lightblue")]
    LightBlue,
    #[serde(rename = "lightcyan")]
    LightCyan,
    #[serde(rename = "lightgreen")]
    LightGreen,
    #[serde(rename = "lightmagenta")]
    LightMagenta,
    #[serde(rename = "lightred")]
    LightRed,
    #[serde(rename = "lightwhite")]
    LightWhite,
    #[serde(rename = "lightyellow")]
    LightYellow,

    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "ansi")]
    Ansi { value: u8 },
    #[serde(rename = "rgb")]
    Rgb { red: u8, green: u8, blue: u8 },
}
