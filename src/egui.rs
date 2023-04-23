use bevy::prelude::{DetectChangesMut, Query, ResMut, Resource};

#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiWantsFocus {
    // Must track previous value too, because when you click inside an egui window,
    // Context::wants_pointer_input() returns false once before returning true again, which
    // is undesirable.
    pub prev: bool,
    pub curr: bool,
}

pub fn check_egui_wants_focus(
    mut contexts: Query<&mut bevy_egui::EguiContext>,
    mut wants_focus: ResMut<EguiWantsFocus>,
) {
    let ctx = contexts.iter_mut().next();
    let new_wants_focus = if let Some(mut ctx) = ctx {
        let ctx = ctx.get_mut();
        ctx.wants_pointer_input() || ctx.wants_keyboard_input()
    } else {
        false
    };
    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
}
