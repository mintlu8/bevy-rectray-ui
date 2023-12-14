use bevy::{prelude::{Plugin, PostUpdate, IntoSystemConfigs, Update}, app::{PreUpdate, Last}};
use crate::schedule::{AoUIStoreOutputSet, AoUILoadInputSet, AoUIWidgetsEventSet, AoUICleanupSet};

use super::{inputbox, button, drag::{self, drag_start}, richtext, scroll, scrollframe};


pub(crate) struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PreUpdate, (
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                button::propagate_focus,
                button::button_on_click,
                button::check_button_on_click,
                button::radio_button_on_click,
                drag::drag_start,
                drag::drag_end,
                drag::dragging.after(drag_start),
                scroll::drag_and_scroll,
                scrollframe::clipping_layer,
            ).in_set(AoUIWidgetsEventSet))
            .add_systems(Update, (
                inputbox::update_inputbox_cursor,
                button::set_cursor,
                button::event_conditional_visibility,
                button::check_conditional_visibility,
            ))
            .add_systems(PostUpdate, richtext::synchronize_glyph_spaces.in_set(AoUILoadInputSet))
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUIStoreOutputSet))
            .add_systems(Last, button::remove_check_button_state.in_set(AoUICleanupSet))
        ;
    }
}