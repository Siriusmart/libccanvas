use libccanvas::{bindings::*, client::*};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).unwrap();
    client.subscribe(Subscription::AllKeyPresses).await;

    // draw the frame
    for x in 1..41 {
        client.setchar(x, 0, '─').await;
        client.setchar(x, 21, '─').await;
    }

    for y in 1..21 {
        client.setchar(0, y, '│').await;
        client.setchar(41, y, '│').await;
    }

    client.setchar(41, 0, '╮').await;
    client.setchar(0, 0, '╭').await;
    client.setchar(41, 21, '╯').await;
    client.setchar(0, 21, '╰').await;

    let mut x = 10;
    let mut y = 10;

    while let Some(mut event) = client.recv().await {
        event.done(true);
        match event.get() {
            EventVariant::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    client.exit().await;
                }
                KeyCode::Up if y > 1 => {
                    // characters have an almost 1:2 aspect ratio
                    client.setchar(x * 2, y, ' ').await;
                    y -= 1;
                    client.setchar(x * 2, y, '▄').await;
                }
                KeyCode::Down if y < 20 => {
                    client.setchar(x * 2, y, ' ').await;
                    y += 1;
                    client.setchar(x * 2, y, '▄').await;
                }
                KeyCode::Left if x > 1 => {
                    client.setchar(x * 2, y, ' ').await;
                    x -= 1;
                    client.setchar(x * 2, y, '▄').await;
                }
                KeyCode::Right if x < 20 => {
                    client.setchar(x * 2, y, ' ').await;
                    x += 1;
                    client.setchar(x * 2, y, '▄').await;
                }
                _ => {
                }
            },
            _ => {}
        }
    }
}
