[![Crates.io](https://img.shields.io/crates/v/bevy_panorbit_camera)](https://crates.io/crates/bevy_panorbit_camera) [![docs.rs](https://docs.rs/bevy_panorbit_camera/badge.svg)](https://docs.rs/bevy_panorbit_camera) [![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

# Bevy Pan/Orbit Camera

Basic orbit camera controls for Bevy. Supports orbiting, panning, and zooming.

It was designed as a plug and play camera to get up and running quickly, with good defaults, but also the ability to
customise some aspects to your liking.

Default controls:

- Left Mouse - Orbit
- Right Mouse - Pan
- Scroll Wheel - Zoom

## Demo

![A screen recording showing camera movement](https://user-images.githubusercontent.com/7709415/230715348-eb19d9a8-4826-4a73-a039-02cacdcb3dc9.gif "Demo of bevy_panorbit_camera")

## Features

- Smooth orbiting motion
- Line-by-line and pixel scrolling
- Orthographic camera projection in addition to perspective
- Customisable controls and sensitivity

## Quick Start

Simply add the `PanOrbitCameraPlugin`, then add `PanOrbitCamera` to an entity
with a `Camera3dBundle`:

```rust
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle::default(),
            PanOrbitCamera::default(),
        ));
}
```

Be sure to check out the [examples](https://github.com/Plonq/bevy_panorbit_camera/tree/master/examples).

## Version Compatibility

| bevy | bevy_panorbit_camera |
|------|----------------------|
| 0.10 | 0.1, 0.2             |

## Credits

- [Bevy Cheat Book](https://bevy-cheatbook.github.io): For providing an example that I started from
- [babylon.js](https://www.babylonjs.com): I referenced their arc rotate camera for some of this

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
