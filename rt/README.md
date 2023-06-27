# Jank Tracer

This is just a fairly basic CPU path tracer.
It aims to be PBR accurate, but I'm not the best at calculus, so I'm sure it often fails to do so.

## Features

- Real-time rendering preview
- Multithreaded rendering
- Uses a BVH for intersection acceleration
- Various primitive shapes
- Transformation matrix support for all objects
- Supports different kinds of materials
- Shared material support
- Point and area lights with Next Event Estimation
- Support for Constructive Solid Geometry
- Loading scenes from a config file
- Saving to different output formats
- Lots of camera options, including bokeh
- Tone mapping

#### Supported primitives

- Sphere
- Box
- Plane
- Cylinder

#### Supported materials

- Lambertian
- Cook-Torrance with GGX
- Perfect mirror
- Perfect glass

## Usage

```
Usage: jank_tracer.exe [OPTIONS] [SCENE]

Arguments:
  [SCENE]  What scene file to load in. Relative to working directory [default: scene.ron]

Options:
  -o <OUTPUT_FORMAT>          What format to output the renders in [default: png] [possible values: png, ppm, exr]
      --no-preview            When this flag is set, a render preview window will not be opened
      --post-process <IMAGE>  Apply opened scene's post process pipeline to an image. This disables normal rendering. Output format is defined by -o
  -h, --help                  Print help information
```

Building is
the [usual rust way](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project) with
cargo.
However, it is important to use the `--release` flag, as otherwise the program will be really slow.

See the [scene documentation](SCENE.md) for how set up your scene file.

## To-Do

Here's a wish-list of things I want to do, but probably won't get around to actually doing:

- Textured materials
- Normal mapped materials
- Skybox
- Implementing RIS
- Proper denoising
