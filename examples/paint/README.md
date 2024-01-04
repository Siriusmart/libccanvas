# Paint

This bundle utilises spaces as "workspaces".

I would recommend looking the 2 components in the following order:

1. [`paint-manager`](./paint-manager)
2. [`paint-canvas`](./paint-canvas)

## How to run this component

- [`ccanvas`](https://github.com/siriusmart/ccanvas) installed in system.
- Install all 2 components components using `cargo install --path .`

Start the canvas using the command below.

```sh
ccanvas paint ccanvas-paint
```

## Component structure

```
/
└── 1 (master)
   ├── 2 (manager)
   ├── 3 (space)
   │  └── 8 (process)
   │  // ...
   └── 7 (space)
      └── 12 (process)
```

## Showcase

Click on the image for video.

[![](https://gmtex.siri.sh/api/usercontent/v1/file/id/1/tex/Dump/Showcases/ccanvas-paint.png)](https://gmtex.siri.sh/fs/1/Dump/Showcases/ccanvas-paint.webm)
