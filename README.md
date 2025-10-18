[![Crates.io](https://img.shields.io/crates/v/bevy_panorbit_camera)](https://crates.io/crates/bevy_panorbit_camera)
[![docs.rs](https://docs.rs/bevy_panorbit_camera/badge.svg)](https://docs.rs/bevy_panorbit_camera)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

<div style="text-align: center">
  <h1>Bevy Pan/Orbit Camera</h1>
</div>

![A screen recording showing camera movement](https://user-images.githubusercontent.com/7709415/230715348-eb19d9a8-4826-4a73-a039-02cacdcb3dc9.gif "Demo of bevy_panorbit_camera")

## Summary

Bevy Pan/Orbit Camera provides orbit camera controls for Bevy Engine, designed with simplicity and flexibility in mind.
Use it to quickly prototype, experiment, for model viewers, and more!

## Features:

- Smoothed orbiting, panning, and zooming
- Works with orthographic camera projection in addition to perspective
- Customisable controls, sensitivity, and more
- Touch support
- Works with multiple viewports and/or windows
- Easy to control manually, e.g. for keyboard control or animation
- Can control cameras that render to a texture

## Controls

Default mouse controls:

- Left Mouse - Orbit
- Right Mouse - Pan
- Scroll Wheel - Zoom

Default touch controls:

- One finger - Orbit
- Two fingers - Pan
- Pinch - Zoom

## Quick Start

Add the plugin:

```rust ignore
.add_plugins(PanOrbitCameraPlugin)
```

Add `PanOrbitCamera` (this will automatically add a `Camera3d` but you can add it manually if necessary):

```rust ignore
commands.spawn((
    Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
    PanOrbitCamera::default(),
));
```

This will set up a camera with good defaults.

Check out the [advanced example](https://github.com/Plonq/bevy_panorbit_camera/tree/master/examples/advanced.rs) to see
all the possible configuration options.

## Cargo Features

- `bevy_egui` (optional): Makes `PanOrbitCamera` ignore any input that `egui` uses, thus preventing moving the camera
  when interacting with egui windows

## Version Compatibility

| bevy | bevy_panorbit_camera |
|------|----------------------|
| 0.17 | 0.29-0.31            |
| 0.16 | 0.26-0.28            |
| 0.15 | 0.21-0.25            |
| 0.14 | 0.19-0.20            |
| 0.13 | 0.14-0.18            |
| 0.12 | 0.9-0.13             |
| 0.11 | 0.6-0.8              |
| 0.10 | 0.1-0.5              |

## Credits

- [Bevy Cheat Book](https://bevy-cheatbook.github.io): For providing an example that I started from
- [babylon.js](https://www.babylonjs.com): I referenced their arc rotate camera for some of this
- [bevy_pancam](https://github.com/johanhelsing/bevy_pancam): For the egui feature idea

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
