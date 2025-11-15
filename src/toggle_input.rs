use bevy::prelude::Resource;

/// just toggle this off when your ui is stealing inputs, and on when it's not
/// totally guessing, but just run your thing here-ish? or just in Update i
/// guess, that seems to work as well.
/// fn main () {
///   ...
///   app.add_systems(
///     PostUpdate,
///       toggle_camera_input_system
///        .after(YourGuiCodeOrWhatever)
///        .before(PanOrbitCameraSystemSet)
///   );
///   ...
/// }
///
/// fn should_the_camera_ignore_input (...) -> bool {
///   /* it's up to you */
///   ...
///   true
/// }
///
/// fn toggle_camera_input_system (
///   mut ignore_input: ResMut<PanOrbitCameraIgnoreInput>,
///   ...
/// ) {
///   ignore_input.set_if_neq(should_the_camera_ignore_input(...));
/// }
///
/// uses the same mechanics as the egui version, but allows the logic to happen
/// on your side instead of having some big interaction with the ui lib deep
/// within the camera code itself. kinda doesn't do anything if you move into
/// the ui when already dragging, but I think that's actually the behavior you'd
/// want. see the example in `examples/toggle_input.rs`

#[derive(Resource, PartialEq, Eq, Default)]
pub struct PanOrbitCameraIgnoreInput(pub bool);
