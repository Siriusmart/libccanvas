# Setchar

This component shows how characters at a specific position on the canvas can be set.

## What this does

1. Subscribe to the `AllKeyPresses` channel.
2. Draws a box on screen.
3. Listens for the `Char('q')`, `Up`, `Down`, `Left`, and `Right`.
4. When `Up`, `Down`, `Left`, or `Right` is received, move the pixel towards the corresponding direction.
5. When `Char('q')` is received, exit the canvas.

## How to run this component

- [`ccanvas`](https://github.com/siriusmart/ccanvas) installed in system.
- Install the component using `cargo install --path .`

Start the canvas using the command below.

```sh
ccanvas setchar ccanvas-setchar
```
