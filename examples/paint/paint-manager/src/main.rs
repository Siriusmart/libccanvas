use libccanvas::{
    bindings::{
        Colour, Discriminator, EventVariant, KeyCode, ResponseContent, ResponseSuccess,
        Subscription,
    },
    client::Client,
};

#[tokio::main]
async fn main() {
    let mut client = Client::default();

    client.hidecursor();

    // subscribe to various events
    client
        .subscribe_multiple(vec![
            Subscription::specific_keycode(KeyCode::Char('1')),
            Subscription::specific_keycode(KeyCode::Char('2')),
            Subscription::specific_keycode(KeyCode::Char('3')),
            Subscription::specific_keycode(KeyCode::Char('4')),
            Subscription::specific_keycode(KeyCode::Char('5')),
            Subscription::specific_keycode(KeyCode::Char('q')),
        ])
        .await;

    // this vector holds the 5 discriminators of the 5 spawned canvas spaces
    let mut canvas: Vec<Discriminator> = Vec::with_capacity(5);

    for _ in 0..5 {
        canvas.push(resp_to_discrim(
            client
                .new_space(Discriminator::master(), "canvas".to_string())
                .await,
        ));
    }

    // add 1 paint-canvas to each of those spaces
    for subspace in canvas.iter() {
        client
            .spawn_at(
                "painter".to_string(),
                "ccanvas-paint-canvas".to_string(),
                Vec::new(),
                subspace.clone(),
            )
            .await;
    }

    // render some text to screen
    for (x, c) in "Use mouse to draw on canvas. Ctrl + Z to undo."
        .chars()
        .enumerate()
    {
        client.setchar(x as u32, 21, c);
    }

    // set focus to canvas 1 by default
    let mut focused = 0;
    client.focus_at(canvas[focused as usize].clone()).await;

    render_workspaces(focused, &mut client);
    client.renderall().await;

    while let Some(event) = client.recv().await {
        if let EventVariant::Key(key) = event.get() {
            match key.code {
                KeyCode::Char('q') => {
                    client.exit().await;
                    break;
                }
                // change focus to the corresponding workspace if number keys are pressed
                KeyCode::Char('1') => {
                    focused = 0;
                    client.focus_at(canvas[0].clone()).await;
                }
                KeyCode::Char('2') => {
                    focused = 1;
                    client.focus_at(canvas[1].clone()).await;
                }
                KeyCode::Char('3') => {
                    focused = 2;
                    client.focus_at(canvas[2].clone()).await;
                }
                KeyCode::Char('4') => {
                    focused = 3;
                    client.focus_at(canvas[3].clone()).await;
                }
                KeyCode::Char('5') => {
                    focused = 4;
                    client.focus_at(canvas[4].clone()).await;
                }
                _ => continue,
            }
            render_workspaces(focused, &mut client);
            client.renderall().await;
        }
    }
}

// returns the discriminator of the created space
pub fn resp_to_discrim(res: ResponseContent) -> Discriminator {
    if let ResponseContent::Success {
        content: ResponseSuccess::SpaceCreated { discrim },
    } = res
    {
        discrim
    } else {
        unreachable!("this is not a space spawned request")
    }
}

// this function renders the workspace bar, highlighting the current selected workspace
pub fn render_workspaces(focused: u32, client: &mut Client) {
    for x in 0..5 {
        if focused == x {
            client.setcharcoloured(
                x * 2,
                0,
                (x + 1).to_string().chars().next().unwrap(),
                Colour::Magenta,
                Colour::Reset,
            );
        } else {
            client.setcharcoloured(
                x * 2,
                0,
                (x + 1).to_string().chars().next().unwrap(),
                Colour::Red,
                Colour::Reset,
            );
        }
    }

    // again render some text to screen
    for (x, c) in "Press a key: 1 - 5 to goto canvas.".chars().enumerate() {
        client.setchar(x as u32 + 12, 0, c);
    }
}
