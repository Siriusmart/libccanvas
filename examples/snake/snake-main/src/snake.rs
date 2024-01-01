use std::collections::LinkedList;

use libccanvas::{bindings::Colour, client::Client};

use crate::{Direction, Fruit};

pub struct Snake {
    // x, y
    head: (u32, u32),
    pub heading: Direction,
    pub previous_heading: Direction,
    body: LinkedList<(u32, u32)>,
    fruit: Fruit,
}

impl Snake {
    pub async fn render_forward(&mut self, client: &mut Client, score: &mut u32) {
        // add head to body
        self.body.push_front(self.head);

        // move head by 1 pixel according to the direction
        // note that it wraps around edges
        match self.heading {
            Direction::Up => self.head.1 = self.head.1.checked_sub(1).unwrap_or(20),
            Direction::Down => self.head.1 = (self.head.1 + 1) % 21,
            Direction::Left => self.head.0 = self.head.0.checked_sub(1).unwrap_or(20),
            Direction::Right => self.head.0 = (self.head.0 + 1) % 21,
        }

        // if self is overlapping with fruit
        // it means the snake has ate the fruit
        if self.overlaps_fruit(&self.fruit) {
            // so spawn in a new fruit
            self.new_fruit();
            self.fruit.render(client);
            *score += 1;
            // and broadcast the new score
            // so the scoreboard component will draw the new score
            client.broadcast(score.to_string()).await;
        } else {
            // remove tail
            let (tailx, taily) = self.body.pop_back().unwrap();
            client.setchar(2 * tailx + 1, taily + 2, ' ');
            client.setchar(2 * tailx + 2, taily + 2, ' ');
        }

        // add head
        client.setchar(2 * self.head.0 + 1, self.head.1 + 2, '█');
        client.setchar(2 * self.head.0 + 2, self.head.1 + 2, '█');

        self.previous_heading = self.heading;
    }

    pub async fn new(client: &mut Client) -> Self {
        // the default state of the snake - going up in the middle of the playfield
        let mut out = Self {
            head: (10, 10),
            heading: Direction::Up,
            previous_heading: Direction::Up,
            body: LinkedList::from([(10, 13), (10, 12), (10, 11)]),
            fruit: Fruit::default(),
        };

        out.new_fruit();
        out.fruit.render(client);
        out
    }

    pub fn overlaps_fruit(&self, fruit: &Fruit) -> bool {
        fruit.is_at(self.head) || self.body.iter().any(|location| fruit.is_at(*location))
    }

    pub fn new_fruit(&mut self) {
        loop {
            let fruit = Fruit::new();

            // create a new fruit in an empty space
            if self.overlaps_fruit(&fruit) {
                continue;
            }

            self.fruit = fruit;
            break;
        }
    }

    pub async fn game_over(&self, client: &mut Client) -> bool {
        if self.body.contains(&self.head) {
            // if game over, then return true
            // and draw red on the pixel of collision
            client.setcharcoloured(
                2 * self.head.0 + 1,
                self.head.1 + 2,
                '█',
                Colour::Red,
                Colour::Reset,
            );
            client.setcharcoloured(
                2 * self.head.0 + 2,
                self.head.1 + 2,
                '█',
                Colour::Red,
                Colour::Reset,
            );

            client.renderall().await;
            return true;
        }

        false
    }
}
