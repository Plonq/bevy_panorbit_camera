use bevy::prelude::*;

/// A resource that tracks whether egui wants focus, i.e. user is interacting with egui.
///
/// This is re-exported in case it's useful.
#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiWantsFocus(pub bool);

pub fn check_egui_wants_focus(
    mut contexts: bevy_egui::EguiContexts,
    mut wants_focus: ResMut<EguiWantsFocus>,
    windows: Query<Entity, With<Window>>,
) {
    // The window that the user is interacting with and the window that contains the egui context
    // that the user is interacting with are always going to be the same. Therefore, we can assume
    // that if any of the egui contexts want focus, then it must be the one that the user is
    // interacting with.
    let new_wants_focus = windows.iter().any(|window| {
        let ctx = contexts.ctx_for_window_mut(window);
        ctx.wants_pointer_input() || ctx.wants_keyboard_input()
    });
    let new_res = EguiWantsFocus(new_wants_focus);
    wants_focus.set_if_neq(new_res);
}
