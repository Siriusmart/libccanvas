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
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum RenderRequest {
    #[serde(rename = "set char")]
    SetChar { x: u32, y: u32, c: char },
    #[serde(rename = "set colouredchar")]
    SetCharColoured {
        x: u32,
        y: u32,
        c: char,
        fg: Colour,
        bg: Colour,
    },
    #[serde(rename = "flush")]
    Flush,
    #[serde(rename = "set cursorstyle")]
    SetCursorStyle { style: CursorStyle },
    #[serde(rename = "hide cursor")]
    HideCursor,
    #[serde(rename = "show cursor")]
    ShowCursor,
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
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
