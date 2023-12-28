use libccanvas::{client::{Client, ClientConfig}, bindings::{EventVariant, KeyCode, Subscription}};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).unwrap();
    // spawns in the 2 processes
    client.spawn("snake-main".to_string(), "ccanvas-snake-main".to_string(), Vec::new()).await;
    client.spawn("snake-scoreboard".to_string(), "ccanvas-snake-scoreboard".to_string(), Vec::new()).await;

    client.subscribe(Subscription::AllKeyPresses).await;

    // and listens for 'q' to exit the canvas
    while let Some(event) = client.recv().await {
        if let EventVariant::Key(key) = event.get() {
            if key.code == KeyCode::Char('q') {
                client.exit().await;
            }
        }
    }
}