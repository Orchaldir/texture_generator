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
See [an example](resources/tilemaps/example.tm).

The rendered color image with [ambient occlusion](https://en.wikipedia.org/wiki/Ambient_occlusion):

![Color image with ambient occlusion](../assets/v0.3/color-ambient-occlusion.png)

The rendered color image with ambient occlusion & lighting:

![Color image with lighting](../assets/v0.3/color-lighting.png)

And the depth image:

![Depth Image](../assets/v0.3/depth.png)