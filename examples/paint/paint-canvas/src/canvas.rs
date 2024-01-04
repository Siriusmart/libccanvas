use libccanvas::{bindings::Colour, client::Client};

pub struct Canvas {
    current: [[Rgb; 20]; 20],
    past: Vec<[[Rgb; 20]; 20]>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            current: Default::default(),
            past: vec![Default::default()],
        }
    }
}

impl Canvas {
    // render current state
    pub fn render(&self, client: &mut Client) {
        for (y, row) in self.current.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                client.setcharcoloured(
                    2 * x as u32,
                    y as u32 + 1,
                    ' ',
                    Colour::Reset,
                    (*cell).into(),
                );
                client.setcharcoloured(
                    2 * x as u32 + 1,
                    y as u32 + 1,
                    ' ',
                    Colour::Reset,
                    (*cell).into(),
                );
            }
        }
    }

    // change a pixel
    pub async fn paint(&mut self, mouse_x: u32, mouse_y: u32, client: &mut Client, rgb: Rgb) {
        let x = (mouse_x / 2).min(19);
        let y = mouse_y.min(19);

        self.current[y as usize][x as usize] = rgb;
        client.setcharcoloured(x * 2, y + 1, ' ', Colour::Reset, rgb.into());
        client.setcharcoloured(x * 2 + 1, y + 1, ' ', Colour::Reset, rgb.into());
        client.renderall().await;
    }

    // push new state
    pub fn push(&mut self) {
        self.past.push(self.current)
    }

    // undo previous state
    // return true if there is something to undo
    //
    // notice that the last item would be the newly pushed state, so it doesnt count
    pub fn pop(&mut self) -> bool {
        if self.past.len() == 1 {
            return false;
        }

        self.past.pop();
        self.current = *self.past.last().unwrap();
        true
    }
}

#[derive(Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl Into<Colour> for Rgb {
    fn into(self) -> Colour {
        Colour::Rgb {
            red: self.r,
            green: self.g,
            blue: self.b,
        }
    }
}

impl Rgb {
    pub fn inverse(&self) -> Self {
        Self {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }
}
