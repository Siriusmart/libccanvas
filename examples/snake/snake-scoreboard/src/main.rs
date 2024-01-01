use libccanvas::{
    bindings::{Colour, EventVariant, Subscription},
    client::Client,
};

#[tokio::main]
async fn main() {
    let mut client = Client::default();
    // listen to all messages - including broadcasts from snake-main
    client.subscribe(Subscription::AllMessages).await;

    // draws "Score: 0" in canvas
    client.setchar(0, 0, 'S');
    client.setchar(1, 0, 'c');
    client.setchar(2, 0, 'o');
    client.setchar(3, 0, 'r');
    client.setchar(4, 0, 'e');
    client.setchar(5, 0, ':');
    client.setcharcoloured(7, 0, '0', Colour::Red, Colour::Reset);
    client.renderall().await;

    while let Some(event) = client.recv().await {
        if let EventVariant::Message { content, .. } = event.get() {
            for (x, c) in content.chars().enumerate() {
                // draws the scroe in canvas
                client.setcharcoloured(7 + x as u32, 0, c, Colour::LightRed, Colour::Reset);
                client.renderall().await;
            }
        }
    }
}
