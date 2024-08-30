use bevy::prelude::*;

/// A resource that tracks whether egui wants focus on the current and previous frames,
/// in order to determine whether PanOrbitCamera should react to input events.
///
/// The reason the previous frame's value is saved is because when you click inside an
/// egui window, Context::wants_pointer_input() still returns false once before returning
/// true. If the camera stops taking input only when it returns false, there's one frame
/// where both egui and the camera are using the input events, which is not desirable.
///
/// This is re-exported in case it's useful. I recommend only using input events if both
/// `prev` and `curr` are false.
#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiWantsFocus {
    /// Whether egui wanted focus on the previous frame
    pub prev: bool,
    /// Whether egui wants focus on the current frame
    pub curr: bool,
}

/// When true, just hovering over an egui panel/window will prevent PanOrbitCamera
/// from reacting to input events. This is an optional, and hopefully temporary,
/// workaround to this issue: https://github.com/Plonq/bevy_panorbit_camera/issues/75.
/// Note that this will prevent PanOrbitCamera using reacting to input whenever the cursor
/// is over an egui area, even if you're in the middle of dragging to rotate, so only use
/// this if you use egui Panels (as opposed to Windows). If you use Windows exclusively
/// then no workaround is required.
#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiFocusIncludesHover(pub bool);

pub fn check_egui_wants_focus(
    mut contexts: bevy_egui::EguiContexts,
    mut wants_focus: ResMut<EguiWantsFocus>,
    include_hover: Res<EguiFocusIncludesHover>,
    windows: Query<Entity, With<Window>>,
) {
    // The window that the user is interacting with and the window that contains the egui context
    // that the user is interacting with are always going to be the same. Therefore, we can assume
    // that if any of the egui contexts want focus, then it must be the one that the user is
    // interacting with.
    let new_wants_focus = windows.iter().any(|window| {
        let ctx = contexts.ctx_for_entity_mut(window);
        let mut value = ctx.wants_pointer_input() || ctx.wants_keyboard_input();
        if include_hover.0 {
            value |= ctx.is_pointer_over_area()
        }
        value
    });
    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
}
