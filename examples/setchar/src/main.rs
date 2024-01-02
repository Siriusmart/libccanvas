use libccanvas::{bindings::*, client::*};

#[tokio::main]
async fn main() {
    let mut client = Client::default();
    client
        .subscribe_multiple(vec![
            Subscription::specific_keycode(KeyCode::Up),
            Subscription::specific_keycode(KeyCode::Down),
            Subscription::specific_keycode(KeyCode::Left),
            Subscription::specific_keycode(KeyCode::Right),
            Subscription::specific_keycode(KeyCode::Char('q')),
        ])
        .await;
    client.hidecursor();

    // draw the frame
    for x in 1..41 {
        client.setchar(x, 0, '─');
        client.setchar(x, 21, '─');
    }

    for y in 1..21 {
        client.setchar(0, y, '│');
        client.setchar(41, y, '│');
    }

    client.setchar(41, 0, '╮');
    client.setchar(0, 0, '╭');
    client.setchar(41, 21, '╯');
    client.setchar(0, 21, '╰');

    // the changes are stashed
    // call renderall to flush all those changes to screen
    // reduced amount of renderall would decrease CPU usage
    client.renderall().await;

    let mut x = 10;
    let mut y = 10;

    while let Some(event) = client.recv().await {
        if let EventVariant::Key(key) = event.get() {
            match key.code {
                KeyCode::Char('q') => {
                    client.exit().await;
                }
                KeyCode::Up if y > 1 => {
                    // characters have an almost 1:2 aspect ratio
                    client.setchar(x * 2, y, ' ');
                    y -= 1;
                    client.setchar(x * 2, y, '▄');
                }
                KeyCode::Down if y < 20 => {
                    client.setchar(x * 2, y, ' ');
                    y += 1;
                    client.setchar(x * 2, y, '▄');
                }
                KeyCode::Left if x > 1 => {
                    client.setchar(x * 2, y, ' ');
                    x -= 1;
                    client.setchar(x * 2, y, '▄');
                }
                KeyCode::Right if x < 20 => {
                    client.setchar(x * 2, y, ' ');
                    x += 1;
                    client.setchar(x * 2, y, '▄');
                }
                _ => {}
            }
            client.renderall().await;
        }
    }
}
