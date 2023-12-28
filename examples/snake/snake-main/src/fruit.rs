use libccanvas::{bindings::Colour, client::Client};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Fruit {
    pub x: u32,
    pub y: u32,
}

impl Fruit {
    pub fn is_at(&self, (x, y): (u32, u32)) -> bool {
        x == self.x && y == self.y
    }

    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..21),
            y: rng.gen_range(0..21),
        }
    }

    pub async fn render(&self, client: &Client) {
        client
            .setcharcoloured_noflush(
                2 * self.x + 1,
                self.y + 2,
                '█',
                Colour::LightMagenta,
                Colour::Reset,
            )
            .await;
        client
            .setcharcoloured_noflush(
                2 * self.x + 2,
                self.y + 2,
                '█',
                Colour::LightMagenta,
                Colour::Reset,
            )
            .await;
    }
}
