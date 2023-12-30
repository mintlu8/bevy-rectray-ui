//! Building blocks for creating widgets.

pub mod inputbox;
pub mod drag;
pub mod richtext;
pub mod scroll;
pub mod scrollframe;
pub mod button;
mod constraints;
mod atlas;
pub use atlas::DeferredAtlasBuilder;
pub use constraints::SharedPosition;
use bevy::{ecs::schedule::IntoSystemConfigs, app::{Plugin, PreUpdate, Update, PostUpdate, Last}};

use crate::{schedule::{AouiButtonEventSet, AouiWidgetEventSet, AouiLoadInputSet, AouiStoreOutputSet, AouiCleanupSet, AouiEventSet}, events::{CursorAction, CursorFocus}};

use self::button::CheckButtonState;

pub(crate) struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PreUpdate, (
                button::button_on_click,
                button::check_button_on_click,
                button::radio_button_on_click,
            ).in_set(AouiButtonEventSet))
            .add_systems(PreUpdate, (
                button::generate_check_button_state,
            ).in_set(AouiEventSet))
            .add_systems(PreUpdate, (
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                button::propagate_focus::<CursorAction>,
                button::propagate_focus::<CursorFocus>,
                button::propagate_focus::<CheckButtonState>,
                drag::drag_start,
                drag::drag_end,
                drag::dragging.after(drag::drag_start),
                scroll::scrolling_system,
                scroll::scrolling_discrete.after(scroll::scrolling_system),
                scrollframe::clipping_layer,
            ).in_set(AouiWidgetEventSet))
            .add_systems(Update, (
                constraints::scroll_constraint,
                constraints::drag_constraint,
                constraints::discrete_scroll_sync,
            ))
            .add_systems(Update, (
                inputbox::update_inputbox_cursor,
                button::set_cursor,
                button::event_conditional_visibility,
                button::check_conditional_visibility,
                atlas::build_deferred_atlas,
            ))
            .add_systems(PostUpdate, richtext::synchronize_glyph_spaces.in_set(AouiLoadInputSet))
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AouiStoreOutputSet))
            .add_systems(Last, button::remove_check_button_state.in_set(AouiCleanupSet))
            .add_systems(Last, constraints::remove_position_changed.in_set(AouiCleanupSet))
        ;
    }
}