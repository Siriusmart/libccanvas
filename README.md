# libccanvas

libccanvas makes easy for creating ccanvas components. For more information, see the [ccanvas](https://github.com/Siriusmart/ccanvas) main repository.

## Quickstart

Your component will be ran by the canvas when loaded, get started by creating a client with `Client::default()`.

> Please name your Rust project in format of `ccanvas-XXXXX` for identification.

```rust
#[tokio::main]
async fn main() {
    let mut client = Client::default();

    client.hidecursor();
    client.setchar(0, 0, 'H');
    client.setchar(1, 0, 'e');
    client.setchar(2, 0, 'l');
    client.setchar(3, 0, 'l');
    client.setchar(4, 0, 'o');
    client.setchar(5, 0, '!');

    // flush all changes
    client.renderall().await;


    // listen to all key presses
    client.subscribe(Subscription::AllKeyPresses).await;

    // exit when 'q' is pressed
    while let Some(event) = client.recv().await {

        if let EventVariant::Key(key) = event.get() {
            if key.code == KeyCode::Char('q') => {
                client.exit().await;
                break;
            }
        }

    }
}
```

### Running the component

> You will need [ccanvas](https://github.com/Siriusmart/ccanvas) installed on your system.

1. Install the component using cargo.
```sh
cargo install --path . # where `.` is your project directory
```
2. Run ccanvas with the component loaded.
```sh
ccanvas hello ccanvas-hello # where `ccanvas-hello` is the command to run your component
```

You can find more examples of advance usage in [`/examples`](https://github.com/Siriusmart/libccanvas/tree/master/examples).

## Implementation details

***This is not just ccanvas bindings***, here are things `Client` does under the hood to simplify developer ergonomics.

### Confirm receiving events

When the *server* (from now on I will refer to the ccanvas *server* as just the server) pass an event to the client, it expects the client to confirm that it has received the event. And whether the event should be passed to other components.

- A confirmation is automatically sent when an `Event` goes out of scope.
- You can manually trigger this behaviour using `Event.done(true)` as well.

### Handling responses

To preserve the order of requests, a client request will *await* until the server respond with a confirmation of the task being completed.

### Auto self destruct

When `Client` goes out of scope, it will automatically call `drop` of self - thus removing it from the ccanvas session, avoiding the situation of a "ghost component".
