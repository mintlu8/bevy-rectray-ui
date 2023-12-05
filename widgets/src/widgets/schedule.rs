use bevy::{prelude::{Plugin, PostUpdate, IntoSystemConfigs, Update}, ecs::schedule::{SystemSet, IntoSystemSetConfigs}, app::PreUpdate};
use bevy_aoui::schedule::{AoUIStoreOutput, AoUILoadInput};

use crate::events::AoUIEventSet;

use super::{shape, inputbox, button::{self, CursorDefault}, drag::{self, drag_start}, richtext, scroll};

/// Plugin for widgets that do not depend on events.
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, shape::sync_shape_size.in_set(AoUIStoreOutput))
            .add_systems(PostUpdate, shape::rebuild_shapes.in_set(AoUIStoreOutput).after(shape::sync_shape_size))
        ;
    }
}




#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIWidgetsEventSet;

pub(crate) struct FullWidgetsPlugin;

impl Plugin for FullWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<CursorDefault>()
            .add_plugins(CoreWidgetsPlugin)
            .configure_sets(PreUpdate, AoUIWidgetsEventSet.after(AoUIEventSet))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(AoUIEventSet)
                .before(AoUIWidgetsEventSet))
            .add_systems(PreUpdate, (
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                button::propagate_focus,
            ).in_set(AoUIWidgetsEventSet))
            .add_systems(Update, (
                inputbox::update_inputbox_cursor,
                button::set_cursor,
                button::conditional_visibility,
                drag::drag_start,
                drag::drag_end,
                drag::dragging.after(drag_start),
                scroll::scrolling,
            ))
            .add_systems(PostUpdate, drag::apply_constraints.in_set(AoUILoadInput))
            .add_systems(PostUpdate, richtext::synchronize_glyph_spaces.in_set(AoUILoadInput))
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUIStoreOutput))
        ;
    }
}