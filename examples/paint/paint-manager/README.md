# Paint manager

This component creates multiple spaces, and loads in a `ccanvas-paint-canvas` for each of the space. This also handles switching workspace focus and quitting.

## What this does

1. Loads in [`ccanvas-paint-canvas`](../paint-canvas).
2. Watches when `Char(c)` is received, switch focus to the corresponding space.
3. Watches when `Char('q')` is received, exit the canvas.
