use bevy::prelude::*;
use bevy_egui::EguiContext;

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
    mut contexts: Query<&mut EguiContext>,
    mut wants_focus: ResMut<EguiWantsFocus>,
    include_hover: Res<EguiFocusIncludesHover>,
) -> Result {
    // Check all egui contexts to see if any of them want focus. If any context wants focus,
    // we assume that's the one the user is interacting with and prevent camera input.
    let mut new_wants_focus = false;
    for mut context in contexts.iter_mut() {
        let context = context.get_mut();
        let mut context_wants_focus =
            context.wants_pointer_input() || context.wants_keyboard_input();
        if include_hover.0 {
            context_wants_focus |= context.is_pointer_over_area();
        }
        new_wants_focus |= context_wants_focus;
    }

    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
    Ok(())
}
