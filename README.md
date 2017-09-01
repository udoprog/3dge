# 3dge

A simple experimental game engine written using Rust, Vulkan, glTF 2.0.

See [resources.md][resources] for a useful list of resources.

[resources]: /resources.md

## TODO

- [ ] camera should control projection
- [ ] render of 2d elements (menus)
- [ ] projecting mouse clicks using invisible receiver cubes/planes.
- [ ] deform-based animations

## Progress

### 2017-09-01

Movement with WASD keys.
Zoom in/out with scroll wheel.

- [x] rendering thread (60 fps) ([GfxThread](/src/gfx_thread.rs)).
- [x] [scenes](/src/scene.rs)
- [x] render all primitives for a model
- [x] base color textures with UV maps
- [x] base color factor (solid colors, no texture)
- [x] tick-based scheduler that allows for primitive movement and game logic independent of
    graphics (100 ticks per second)

![Progress 1](/images/progress_2017-09-01.png)
