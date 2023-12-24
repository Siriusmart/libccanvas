use libccanvas::{bindings::*, client::*};

#[tokio::main]
async fn main() {
    // creates the client using the default configuration
    let client = Client::new(ClientConfig::default()).unwrap();

    // subscribe to all key presses
    // now client.recv() will include key press events
    client.subscribe(Subscription::AllKeyPresses).await;

    // this is basically a forever loop
    // it always waits until the next event
    while let Some(mut event) = client.recv().await {
        match event.get() {
            // check if 'q' is pressed
            EventVariant::Key(key) if key.code == KeyCode::Char('q') => {
                // first release the event, this is important
                // if the event is not released, the ccanvas waits until it is released
                // before the client.exit event
                event.done(true);

                // now that the client is clear, we can send a client.exit event
                // to make it exit
                client.exit().await;

                // realistically, the code will never get to this point
                // as it will be killed automatically as the client exits
                break;
            },
            _ => {}
        }
    }
}
