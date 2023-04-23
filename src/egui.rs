use bevy::prelude::{Deref, DerefMut, DetectChangesMut, Query, ResMut, Resource};

#[derive(Resource, Deref, DerefMut, PartialEq, Eq, Default)]
#[cfg(feature = "bevy_egui")]
pub struct EguiWantsFocus(pub bool);

#[cfg(feature = "bevy_egui")]
pub fn check_egui_wants_focus(
    mut contexts: Query<&mut bevy_egui::EguiContext>,
    mut wants_focus: ResMut<EguiWantsFocus>,
) {
    let ctx = contexts.iter_mut().next();
    let new_wants_focus = if let Some(ctx) = ctx {
        let ctx = ctx.into_inner().get_mut();
        ctx.wants_pointer_input() || ctx.wants_keyboard_input()
    } else {
        false
    };
    wants_focus.set_if_neq(EguiWantsFocus(new_wants_focus));
}
