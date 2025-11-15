use bevy::prelude::Resource;
/// just toggle this on when you don't want [`PanOrbitCamera`] to process input.
/// you can see a live example in `examples/toggle_input.rs`, but here's the
/// simplified pseudo-code version:
///
/// ```rust
/// fn main () {
///   ...
///
///   // timing-wise, this is a total guess that I blindly copied from the egui
///   // version; In the real example, I'm just doing it during Update and that
///   // seems to work fine.
///   app.add_systems(
///     PostUpdate,
///       toggle_camera_input_system
///        .after(YourGuiOrWhatever)
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
/// ```
///
/// [`PanOrbitCameraIgnoreInput`] controls the same thing as the egui version,
/// expect the implementation is on the consumer's side.
///
/// Changing this resource while already dragging the camera seems to have no
/// effect, but it's possible that's just an effect of how I did the example and
/// I'm unable to understand the details due to my own relative inexperience
/// with bevy etc. That said, I'm pretty sure that's actually the behavior you'd
/// actually want in any case.
///

#[derive(Resource, PartialEq, Eq, Default)]
pub struct PanOrbitCameraIgnoreInput(pub bool);
