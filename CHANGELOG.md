## 0.28.0

- Update `bevy_egui` to 0.36 (#120)

## 0.27.2

- Simplifies the upside down detection

## 0.27.1

- Fixes upside down detection when using a custom axis (#118 thanks @guija)

## 0.27.0

- Update `bevy_egui` to 0.35 (#115 thanks @kiperz)

## 0.26.0

- Update to Bevy 0.16

## 0.25.0

- Adds better trackpad support, with Blender-style controls (thanks @natepiano)

## 0.24.0

- Add ability to change the base axes, e.g. to make Z up instead of Y up

## 0.23.0

- Update `bevy_egui` to 0.33

## 0.22.2

- Fix initial calculation of yaw from the camera's transform (translation)

## 0.22.1

- Add ability to limit the cameras `focus` to a cube or sphere shape (thanks @bytemunch)

## 0.22.0

- Update `bevy_egui` to 0.32

## 0.21.2

- Derive `Reflect` on `PanOrbitCamera`

## 0.21.1

- Add `Camera3d` as a required component (new feature of Bevy 0.15) of `PanOrbitCamera`, so it doesn't need to be added
  manually

## 0.21.0

- Update to Bevy 0.15

## 0.20.1

- Update docs, explaining that setting sensitivity values to 0 will disable the respective control

## 0.20.0

- Change `zoom_lower_limit` from `Option<f32>` to `f32`, defaulting to `0.05`, and remove the hard coded lower zoom
  limit of `0.05`.
  This allows the lower limit to be lowered below 0.05, in case there is a need to work at very small scales.

## 0.19.5

- Update `bevy_egui` to 0.30 in order to resolve an issue with `bevy-inspector-egui`
  0.27 ([#85](https://github.com/Plonq/bevy_panorbit_camera/pull/85))

## 0.19.4

- Re-fix bug that was fixed in 0.19.1 (see below for explanation).

## 0.19.3

- Update `bevy_egui` dependency to 0.29

## 0.19.2

- Fix bug with how egui feature deals with egui side panels and immovable windows. It should now act more
  naturally - if you start dragging in the viewport, dragging over egui will continue to control the camera.
  If you start the drag in any egui area, the camera won't be affected, even if the cursor leaves the egui area.
  Thanks @thmxv!

## 0.19.1

- Fix panic if egui context for window doesn't exist. For example if a new window is created after startup.

## 0.19.0

- Update to Bevy 0.14

## 0.18 and older

Check [GitHub releases](https://github.com/Plonq/bevy_panorbit_camera/releases) for details
