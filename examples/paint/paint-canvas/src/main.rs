use ccanvas_paint_canvas::*;
use libccanvas::{
    bindings::{EventVariant, KeyCode, KeyEvent, KeyModifier, MouseType, Subscription},
    client::Client,
};

#[tokio::main]
async fn main() {
    let mut client = Client::default();

    // subscribe to these events
    client
        .subscribe_multiple(vec![
            Subscription::Focused,
            Subscription::Unfocused,
            Subscription::specific_mouse(MouseType::Left),
            Subscription::specific_mouse(MouseType::Hold),
            Subscription::specific_mouse(MouseType::Release),
            Subscription::specific_keypress(KeyEvent {
                modifier: KeyModifier::Ctrl,
                code: KeyCode::Char('z'),
            }),
        ])
        .await;

    // create a new blank canvas and a colour picker
    let mut canvas = Canvas::default();
    let mut picker = Picker::new(0, 0, 0);

    let mut mouse_mode: Option<MouseMode> = None;

    while let Some(event) = client.recv().await {
        match event.get() {
            // if ctrl z and there is a previous history state
            // then revert to that state and render
            EventVariant::Key(KeyEvent {
                code: KeyCode::Char('z'),
                modifier: KeyModifier::Ctrl,
            }) => {
                if mouse_mode != Some(MouseMode::Paint) && canvas.pop() {
                    canvas.render(&mut client);
                    client.renderall().await;
                }
            }
            // when current space is focused, then render current state
            EventVariant::Focused => {
                canvas.render(&mut client);
                picker.render(&mut client);
                client.renderall().await;
            }
            EventVariant::Unfocused => mouse_mode = None,
            EventVariant::Mouse(mouse) => match mouse.mousetype {
                // if mouse is clicked on canvas, then mouse mode to paint
                MouseType::Left if mouse.x < 40 && mouse.y < 20 => {
                    mouse_mode = Some(MouseMode::Paint);
                    canvas
                        .paint(
                            mouse.x,
                            mouse.y.saturating_sub(1),
                            &mut client,
                            picker.to_rgb(),
                        )
                        .await;
                }
                // if mouse is clicked on one of the pickers, determine which picker it is clicking on
                // and change value of the picker
                MouseType::Left => {
                    let val = mouse.y.saturating_sub(1).min(19) as u8;
                    let mode = match mouse.x {
                        44 | 45 => {
                            picker.set_red(val, &mut client).await;
                            MouseMode::PickerRed
                        }
                        47 | 48 => {
                            picker.set_green(val, &mut client).await;
                            MouseMode::PickerGreen
                        }
                        50 | 51 => {
                            picker.set_blue(val, &mut client).await;
                            MouseMode::PickerBlue
                        }
                        _ => continue,
                    };
                    mouse_mode = Some(mode)
                }

                // if mouse is dragging, then what it does depends on where did it click when the
                // dragging began
                MouseType::Hold => match mouse_mode {
                    Some(MouseMode::Paint) => {
                        canvas
                            .paint(
                                mouse.x,
                                mouse.y.saturating_sub(1),
                                &mut client,
                                picker.to_rgb(),
                            )
                            .await;
                    }
                    Some(MouseMode::PickerRed) => {
                        picker
                            .set_red(mouse.y.saturating_sub(1).min(19) as u8, &mut client)
                            .await
                    }
                    Some(MouseMode::PickerGreen) => {
                        picker
                            .set_green(mouse.y.saturating_sub(1).min(19) as u8, &mut client)
                            .await
                    }
                    Some(MouseMode::PickerBlue) => {
                        picker
                            .set_blue(mouse.y.saturating_sub(1).min(19) as u8, &mut client)
                            .await
                    }
                    None => continue,
                },

                MouseType::Release => {
                    if mouse_mode == Some(MouseMode::Paint) {
                        canvas.push();
                    }
                    mouse_mode = None
                }
                _ => {}
            },
            _ => {}
        }
    }
}

// mouse mode remembers whats the mouse currently doing
// so mouse hold/drags would also do that thing
#[derive(PartialEq, Eq)]
pub enum MouseMode {
    Paint,
    PickerRed,
    PickerGreen,
    PickerBlue,
}
