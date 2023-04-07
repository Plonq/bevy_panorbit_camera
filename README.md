# Bevy Pan/Orbit Camera

Basic orbit camera controls for Bevy. Supports orbiting, panning, and zooming.

Default controls:
- Left Mouse - Orbit
- Right Mouse - Pan
- Scroll Wheel - Zoom

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

Check out the examples folder for full examples.

## Version Matching

| Bevy Version | `bevy_panorbit_camera` Version |
| ------------ |--------------------------------|
| `0.10.0`     | `0.10.0`                       |

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
