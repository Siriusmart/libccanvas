use std::time::Duration;

use ccanvas_snake_main::{Direction, Snake};
use libccanvas::{
    bindings::{EventVariant, KeyCode, Subscription},
    client::{Client, ClientConfig},
};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).unwrap();
    client.subscribe(Subscription::AllKeyPresses).await;
    client.hidecursor_noflush().await;

    // draw the frame
    for x in 1..43 {
        client.setchar_noflush(x, 1, '─').await;
        client.setchar_noflush(x, 23, '─').await;
    }

    for y in 2..24 {
        client.setchar_noflush(0, y, '│').await;
        client.setchar_noflush(43, y, '│').await;
    }

    client.setchar_noflush(43, 1, '╮').await;
    client.setchar_noflush(0, 1, '╭').await;
    client.setchar_noflush(43, 23, '╯').await;
    client.setchar(0, 23, '╰').await;

    let mut snake = Snake::new(&client).await;
    // give it a bit of suspense
    let mut next_tick = Instant::now() + Duration::from_millis(500);

    let mut score = 0;

    loop {
        tokio::select! {
            // if there are no inputs until the next tick, then great
            _ = tokio::time::sleep_until(next_tick) => {},
            Some(event) = client.recv() => {
                // otherwise, do something with the event
                if let EventVariant::Key(key) = event.get() {
                    match key.code {
                        KeyCode::Up if snake.previous_heading != Direction::Down => snake.heading = Direction::Up,
                        KeyCode::Down if snake.previous_heading != Direction::Up => snake.heading = Direction::Down,
                        KeyCode::Left if snake.previous_heading != Direction::Right => snake.heading = Direction::Left,
                        KeyCode::Right if snake.previous_heading != Direction::Left => snake.heading = Direction::Right,
                        _ => {}
                    }

                    // and continue waiting until the next game tick
                    continue;
                }
            }
        }

        // render and forward the snake by 1 pixel
        snake.render_forward(&client, &mut score).await;

        // if game is over, then just exit the game script
        // this will freeze the screen and not exit yet
        if snake.game_over(&client).await {
            break;
        }

        // do some maths to make the snake go faster and faster
        // plot the graph in desmos to see how it looks like
        next_tick = Instant::now()
            + Duration::from_millis(60 - (((score as f32 + 2_f32).log2() * 8_f32) as u64));
    }
}
