## 0.20

- Change `zoom_lower_limit` from `Option<f32>` to `f32`, defaulting to `0.05`, and remove the hard coded lower zoom limit of `0.05`.
  This allows the lower limit to be lowered below 0.05, in case there is a need to work at very small scales.

## 0.19.5

- Update `bevy_egui` to 0.30 in order to resolve an issue with `bevy-inspector-egui` 0.27 ([#85](https://github.com/Plonq/bevy_panorbit_camera/pull/85))

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

## 0.19

- Update to Bevy 0.14

## 0.18 and older

Check [GitHub releases](https://github.com/Plonq/bevy_panorbit_camera/releases) for details
