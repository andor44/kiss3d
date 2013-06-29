kiss3d
======

Keep It Simple, Stupid 3d graphics engine.

This library is born from the frustration in front of the fact that today’s 3D graphics library are:
  - either too low level: you have to write your own shaders and openning a window steals you 8 hours, 300 lines of code and 10L of coffee.
  - or high level but too hard to understand/use: those are libraries made to write beautiful animations or games. They have a lot of feature; too much feature if you only want to draw a few geometries on the screen.

**kiss3d** is not designed to be feature-complete or fast.
It is designed to be able to draw simple geometric figures and play with them with one-liners.

## Features
All features are one-liners.
  - open a window.
  - display a box.
  - change an object's color.
  - change an object's transform (we use the **nalgebra** library to do that). An object cannot be scaled though.

That’s all.
Other geometric primitive (ball, cone, cylinder), and camera (fps, arc-ball) will be added soon.
One light (yes, only one!) will be added soon.

## Won’t
Anything not in the previous list is _not_ suported.
In particular common features like texturing, mesh loading, and coffee making are not suported and wont be unless someone manages to write them in one line. Don’t hesitate to contribute =)


## Contributions
I intend to work on this library to suit my needs only (to write demo for my physics engine **nphysics**).
Therefore, I’d love to see people improving this library for their own needs.

However, keep in mind that kiss3d is KISS.
Only one-liner features (from the user point of view) are accepted (there might be exceptions).

## Acknowledgements

I am more a physics guy than a graphics guy. I did not want to spend too much
time to be able to display things on the screen. Thus I thank:
  - **bjz** for its glfw binding and its demos
    (git://github.com/bjz/open.gl-tutorials.git) from which I took a great
    bunch of initialization code.