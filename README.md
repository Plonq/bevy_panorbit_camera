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
    - Currently 'beta' - please report any issues!
- Works with multiple viewports and/or windows
- Easy to control manually, e.g. for keyboard control or animation
- Can control cameras that render to a texture
- Supports the 'roll' axis and changing the 'up' vector, and thus controlling all 3 rotational axes
    - Comes with caveats as explained in the documentation for `PanOrbitCamera.key_roll_left` /
      `PanOrbitCamera.key_roll_right` and `PanOrbitCamera.base_transform`

## Controls

By default, you can only orbit, pan, and zoom. Optionally, you can enable the 'roll' axis, which modifies the 'up'
vector of the camera. You can also set the 'up' vector manually - see `PanOrbitCamera.base_transform`.

Default mouse controls:

- Left Mouse - Orbit
- Right Mouse - Pan
- Scroll Wheel - Zoom

Touch controls:

- One finger - Orbit
- Two fingers - Pan
- Pinch - Zoom
- Two finger rotate - Roll (disabled by default)

Note: touch controls are currently not customisable. Please create an issue if you would like to customise the touch
controls.

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

Typically you don't need to worry about the inner workings of this plugin - the defaults work well and are suitable for
most use cases. However, if you want to customise the behaviour, for example restricting the camera movement or
adjusting sensitivity, you probably want to know what the `alpha` and `beta` values represent.

While not strictly accurate, you can think of `alpha` as yaw and `beta` as tilt. More accurately, `alpha` represents the
angle around the _global_ Y axis, and `beta` represents the angle around the _local_ X axis (i.e. the X axis after Y
axis rotation has been applied). When both `alpha` and `beta` are `0.0`, the camera is pointing directly forward (-Z).
Thus, increasing `alpha` orbits around to the right (counter clockwise if looking from above), and increasing `beta`
orbits up and over (e.g. a `beta` value of 90 degrees (`PI / 2.0`) results in the camera looking straight down).

Note that if you change the up vector either by enabling roll controls or changing `PanOrbitCamera.base_transform`,
the concept of 'up' and 'down' change, and so the above explanation changes accordingly.

## Cargo Features

- `bevy_egui` (optional): makes `PanOrbitCamera` ignore any input that `egui` uses

## Version Compatibility

| bevy | bevy_panorbit_camera |
|------|----------------------|
| 0.12 | 0.9-0.11             |
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
