# Orchaldir's Texture Generator in Rust

![CI](https://github.com/Orchaldir/texture_generator/workflows/CI/badge.svg)

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