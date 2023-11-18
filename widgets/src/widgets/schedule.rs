use bevy::{prelude::{Plugin, PostUpdate, IntoSystemConfigs, Update}, ecs::schedule::{SystemSet, IntoSystemSetConfigs}, app::PreUpdate};
use bevy_aoui::schedule::AoUISyncWrite;

use crate::AoUIEventSet;

use super::{shape, inputbox, button::{self, CursorDefault}};

/// Plugin for widgets that do not depend on events.
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, shape::sync_shape_size.in_set(AoUISyncWrite))
            .add_systems(PostUpdate, shape::rebuild_shapes.in_set(AoUISyncWrite).after(shape::sync_shape_size))
        ;
    }
}
pub(crate) struct FullWidgetsPlugin;




#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIWidgetsSet;

impl Plugin for FullWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<CursorDefault>()
            .add_plugins(CoreWidgetsPlugin)
            .configure_sets(PreUpdate, AoUIWidgetsSet.after(AoUIEventSet))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(AoUIEventSet)
                .before(AoUIWidgetsSet))
            .add_systems(PreUpdate, (
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                button::propagate_focus,
            ).in_set(AoUIWidgetsSet))
            .add_systems(Update, (
                inputbox::update_inputbox_cursor,
                button::set_cursor,
                button::conditional_visibility,
            ))
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUISyncWrite))
        ;
    }
}