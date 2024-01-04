use libccanvas::{bindings::Colour, client::Client};

use crate::Rgb;

#[derive(Debug)]
pub struct Picker {
    // each colour can take value between 0 (least intense)
    // to 19 (most intense)
    red_step: u8,
    blue_step: u8,
    green_step: u8,
}

impl Picker {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red_step: red,
            green_step: green,
            blue_step: blue,
        }
    }

    // set a bar to a value between 0 to 19 inclusive
    pub async fn set_red(&mut self, value: u8, client: &mut Client) {
        self.red_step = value;
        self.render(client);
        client.renderall().await;
    }

    pub async fn set_green(&mut self, value: u8, client: &mut Client) {
        self.green_step = value;
        self.render(client);
        client.renderall().await;
    }

    pub async fn set_blue(&mut self, value: u8, client: &mut Client) {
        self.blue_step = value;
        self.render(client);
        client.renderall().await;
    }

    // render current state
    pub fn render(&self, client: &mut Client) {
        for r in 0..20 {
            let colour = Self::new(r, self.green_step, self.blue_step).to_rgb();
            if r == self.red_step {
                client.setcharcoloured(
                    44,
                    r as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
                client.setcharcoloured(
                    45,
                    r as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
            } else {
                client.setcharcoloured(44, r as u32 + 1, ' ', Colour::Reset, colour.into());
                client.setcharcoloured(45, r as u32 + 1, ' ', Colour::Reset, colour.into());
            }
        }

        for g in 0..20 {
            let colour = Self::new(self.red_step, g, self.blue_step).to_rgb();
            if g == self.green_step {
                client.setcharcoloured(
                    47,
                    g as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
                client.setcharcoloured(
                    48,
                    g as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
            } else {
                client.setcharcoloured(47, g as u32 + 1, ' ', Colour::Reset, colour.into());
                client.setcharcoloured(48, g as u32 + 1, ' ', Colour::Reset, colour.into());
            }
        }

        for b in 0..20 {
            let colour = Self::new(self.red_step, self.green_step, b).to_rgb();
            if b == self.blue_step {
                client.setcharcoloured(
                    50,
                    b as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
                client.setcharcoloured(
                    51,
                    b as u32 + 1,
                    ' ',
                    Colour::Reset,
                    colour.inverse().into(),
                );
            } else {
                client.setcharcoloured(50, b as u32 + 1, ' ', Colour::Reset, colour.into());
                client.setcharcoloured(51, b as u32 + 1, ' ', Colour::Reset, colour.into());
            }
        }
    }

    // to standard 0 - 255 rgb
    pub fn to_rgb(&self) -> Rgb {
        Rgb {
            r: (255_u32 * self.red_step as u32 / 19) as u8,
            g: (255_u32 * self.green_step as u32 / 19) as u8,
            b: (255_u32 * self.blue_step as u32 / 19) as u8,
        }
    }
}
