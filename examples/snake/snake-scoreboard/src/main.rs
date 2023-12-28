use libccanvas::{
    bindings::{Colour, EventVariant, Subscription},
    client::{Client, ClientConfig},
};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).unwrap();
    // listen to all messages - including broadcasts from snake-main
    client.subscribe(Subscription::AllMessages).await;

    // draws "Score: 0" in canvas
    client.setchar_noflush(0, 0, 'S').await;
    client.setchar_noflush(1, 0, 'c').await;
    client.setchar_noflush(2, 0, 'o').await;
    client.setchar_noflush(3, 0, 'r').await;
    client.setchar_noflush(4, 0, 'e').await;
    client.setchar_noflush(5, 0, ':').await;
    client.setcharcoloured(7, 0, '0', Colour::Red, Colour::Reset).await;

    while let Some(event) = client.recv().await {
        if let EventVariant::Message { content, .. } = event.get() {
            for (x, c) in content.chars().enumerate() {
                // draws the scroe in canvas
                client
                    .setcharcoloured(7 + x as u32, 0, c, Colour::LightRed, Colour::Reset)
                    .await;
            }
        }
    }
}
