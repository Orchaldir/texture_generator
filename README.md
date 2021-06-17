# Orchaldir's Texture Generator in Rust

![CI](https://github.com/Orchaldir/texture_generator/workflows/CI/badge.svg)

## Textures

A library & application to generate procedural textures based on texture definition files.
See [an example](resources/textures/brick.yaml).

The example can be generated with:

```terminal
texture_generator resources/textures/brick.yaml brick 1024
```

This generates a color image:

![Color Image](../assets/v0.2/brick_wall-color.png)

And a depth image:

![Depth Image](../assets/v0.2/brick_wall-depth.png)

## Tilemaps

A library & editor to create tilemaps and render them with the previous procedural textures.
See [an example](resources/tilemaps/example.otm).

The rendered color image with [ambient occlusion](https://en.wikipedia.org/wiki/Ambient_occlusion) & lighting:

![Color image](../assets/v0.5/color.png)

And the depth image:

![Depth Image](../assets/v0.5/depth.png)

The editor uses the GUI library [Iced](https://github.com/hecrj/iced):

![Editor with Iced](../assets/v0.5/editor.png)