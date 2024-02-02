[![Crates.io](https://img.shields.io/crates/v/bevy_panorbit_camera)](https://crates.io/crates/bevy_panorbit_camera)
[![docs.rs](https://docs.rs/bevy_panorbit_camera/badge.svg)](https://docs.rs/bevy_panorbit_camera)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

<div align="center">
  <h1>Bevy Pan/Orbit Camera</h1>
</div>

![A screen recording showing camera movement](https://user-images.githubusercontent.com/7709415/230715348-eb19d9a8-4826-4a73-a039-02cacdcb3dc9.gif "Demo of bevy_panorbit_camera")

## What Is This?

Bevy Pan/Orbit Camera provides orbit camera controls for Bevy Engine, designed with simplicity and flexibility in mind.
Use it to quickly prototype, experiment, for model viewers, and more!

Default controls:

- Left Mouse - Orbit
- Right Mouse - Pan
- Scroll Wheel - Zoom

Touch controls:

- One finger - Orbit
- Two fingers - Pan
- Pinch - Zoom

## Features:

- Orbiting, panning and zooming
- Smooth motion
- Works with orthographic camera projection in addition to perspective
- Customisable controls, sensitivity, and more
- Touch support
  - Currently 'beta' - please report any issues!
- Works with multiple viewports and/or windows
- Easy to control manually, e.g. for keyboard control or animation
- Can control cameras that render to a texture

## Quick Start

Add the plugin:

```rust ignore
.add_plugins(PanOrbitCameraPlugin)
```

Add `PanOrbitCamera` to a camera:

```rust ignore
commands.spawn((
    Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        ..default()
    },
    PanOrbitCamera::default(),
));
```

This will set up a camera with good defaults.

Check out the [advanced example](https://github.com/Plonq/bevy_panorbit_camera/tree/master/examples/advanced.rs) to see
all the possible configuration options.

## What are `alpha` and `beta`?

Think of this camera as rotating around a point, and always pointing at that point (the `focus`). The sideways rotation,
i.e. the longitudinal rotation, is `alpha`, and the latitudinal rotation is `beta`. Both are measured in radians.
If `alpha` and `beta` are both `0.0`, then the camera will be looking directly forwards (-Z direction). Increasing
`alpha` will rotate around the `focus` to the right, and increasing `beta` will move the camera up and over the `focus`.

## Cargo Features

- `bevy_egui`: makes PanOrbitCamera ignore input when interacting with egui widgets/windows

## Version Compatibility

| bevy | bevy_panorbit_camera |
|------|----------------------|
| 0.12 | 0.9-0.10             |
| 0.11 | 0.6-0.8              |
| 0.10 | 0.1-0.5              |

## Credits

- [Bevy Cheat Book](https://bevy-cheatbook.github.io): For providing an example that I started from
- [babylon.js](https://www.babylonjs.com): I referenced their arc rotate camera for some of this
- [bevy_pancam](https://github.com/johanhelsing/bevy_pancam): For the egui-related code

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
