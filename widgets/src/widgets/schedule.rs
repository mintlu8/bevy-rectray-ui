use bevy::prelude::{Plugin, PostUpdate, IntoSystemConfigs, Update};
use bevy_aoui::schedule::AoUISyncWrite;

use super::{shape, inputbox};

/// Plugin for widgets that do not depend on events.
pub struct AoUIWidgetsPlugin;

impl Plugin for AoUIWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, shape::sync_shape_size.in_set(AoUISyncWrite))
            .add_systems(PostUpdate, shape::rebuild_shapes.in_set(AoUISyncWrite).after(shape::sync_shape_size))
        ;
    }
}
pub(crate) struct FullWidgetsPlugin;

impl Plugin for FullWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(AoUIWidgetsPlugin)
            .add_systems(Update, inputbox::text_on_mouse_down)
            .add_systems(Update, inputbox::update_inputbox_cursor.after(inputbox::text_on_mouse_down))
            .add_systems(Update, inputbox::text_on_click_outside)
            .add_systems(Update, inputbox::text_on_mouse_double_click)
            .add_systems(Update, inputbox::inputbox_keyboard)
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUISyncWrite))
        ;
    }
}