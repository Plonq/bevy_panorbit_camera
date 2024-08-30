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
