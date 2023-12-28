# Basic

This component shows how key events can be handled, and how the client can be exited.

## What this does

1. Subscribe to the `AllKeyPresses` channel.
2. Listens for the `Char('q')` event.
3. When received, tell the canvas to exit.

## How to run this component

- [`ccanvas`](https://github.com/siriusmart/ccanvas) installed in system.
- Install the component using `cargo install --path .`

Start the canvas using the command below.

```sh
ccanvas basic ccanvas-basic
```
