# kiss3d  [![Build Status](https://travis-ci.org/sebcrozet/kiss3d.png?branch=master)](https://travis-ci.org/sebcrozet/kiss3d)

Keep It Simple, Stupid 3d graphics engine.

This library is born from the frustration in front of the fact that today’s 3D
graphics library are:
  - either too low level: you have to write your own shaders and opening a
    window steals you 8 hours, 300 lines of code and 10L of coffee.
  - or high level but too hard to understand/use: those are libraries made to
    write beautiful animations or games. They have a lot of feature; too much
    feature if you only want to draw a few geometries on the screen.

**kiss3d** is not designed to be feature-complete or fast.
It is designed to be able to draw simple geometric figures and play with them
with one-liners.

## Features
Most features are one-liners.
  - open a window with a default arc-ball camera and a point light.
  - a first-person camera is available too and user-defined cameras are possible.
  - display boxes, spheres, cones, cylinders, quads and lines.
  - change an object color or texture.
  - change an object transform (we use the [nalgebra](https://github.com/sebcrozet/nalgebra) library to do that).
    An object cannot be scaled though.
  - create basic post-processing effects.

As an example, having a red, rotating cube with the light attached to the camera is as simple as:
```rust
extern mod kiss3d;
extern mod nalgebra;

use nalgebra::mat::Rotation;
use nalgebra::vec::Vec3;
use kiss3d::window;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    do window::Window::spawn("Kiss3d: cube") |window| {
        let mut c = window.add_cube(1.0, 1.0, 1.0);

        c.set_color(1.0, 0.0, 0.0);

        window.set_light(window::StickToCamera);

        do window.render_loop |_| {
            c.rotate_by(&Vec3::new(0.0f64, 0.014, 0.0))
        }
    }
}
```

Some controls are handled by default by the engine (they can be overridden by the user):
  - `scroll`: zoom in / zoom out.
  - `left click + drag`: look around.
  - `right click + drag`: translate the view point.
  - `enter`: look at the origin (0.0, 0.0, 0.0).

## Compilation
You will need the last rust compiler from the `master` branch.
If you encounter problems, make sure you have the last compiler version before creating an issue.

The simplest way to build **kiss3d** and all its dependencies is to do a
recursive clone:


    git clone --recursive git://github.com/sebcrozet/kiss3d.git
    cd kiss3d
    make deps
    make
    make examples


The last command will compile examples on the `bin` folder.
Use `make doc` to compile the documentation on the `doc` folder.

## Contributions
I’d love to see people improving this library for their own needs.  However, keep in mind that
**kiss3d** is KISS.  One-liner features (from the user point of view) are preferred.

## Acknowledgements

  - **bjz** for his awesome [glfw](https://github.com/bjz/glfw-rs.git) and [opengl](https://github.com/bjz/gl-rs.git) bindings.
