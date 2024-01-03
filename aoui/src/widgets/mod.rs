//! Widget primitives for `bevy_aoui`
//! 
//! `bevy_aoui` has no standard styles, sprites or shaders, 
//! meaning we only provide behaviors.
//! 
//! # Button
//! 
//! | Component | Description |
//! | --------- | ----------- |
//! | [`Button`](button::Button) | Marker for enabling the `EvButtonClick` event. |
//! | [`CheckButton`](button::CheckButton) | Context, checked or unchecked for a `check_button`. |
//! | [`RadioButton`](button::RadioButton) | Context for a `radio_button`. |
//! | [`Payload`](button::Button) | Data sent by `EvButtonClick`. |
//! | [`PropagateFocus`](button::PropagateFocus) | Propagate `CursorFocus` and `CheckButtonState`. |
//! | [`SetCursor`](button::SetCursor) | Set cursor icon during some cursor events. |
//! | [`DisplayIf`](button::DisplayIf) | Display if some condition is met. |
//! | [`RadioButtonCancel`](button::RadioButtonCancel) | Allow clicking radio button again to remove its value. |
//!
//! # Scrolling
//! 
//! | Component | Description |
//! | --------- | ----------- |
//! | [`Scrolling`](scroll::Scrolling) | Enable scrolling of children. |
//! | [`ScrollConstraint`](scroll::ScrollConstraint) | Constraint scrolling to the sprite's dimension. |
//! | [`ScrollDiscrete`](scroll::ScrollDiscrete) | Discrete scrolling for [`Layout`](crate::layout::Layout). |
//! | [`SharedPosition`] | Share position between draggable/scrollable widgets. |
//! 
//! # Dragging
//! 
//! | Component | Description |
//! | --------- | ----------- |
//! | [`Dragging`](drag::Dragging) | Enable scrolling of children. |
//! | [`DragConstraint`](drag::DragConstraint) | Constraint scrolling to the sprite's dimension. |
//! | [`DragSnapBack`](drag::DragSnapBack) | Snap dragged sprite back to the source. |
//! | [`SharedPosition`] | Share position between draggable/scrollable widgets. |
//! 
//! # Clipping
//! 
//! | Bundle | Description |
//! | --------- | ----------- |
//! | [`ScopedCameraBundle`](clipping::ScopedCameraBundle) | Bind a camera to a sprite's `RotatedRect`. |
//! 
//! # InputBox
//! 
//! | Component | Description |
//! | --------- | ----------- |
//! | [`InputBox`](inputbox::InputBox) | Context of an `input_box`, holding the text and cursor information. |
//! | [`TextColor`](inputbox::TextColor) | Color of an `input_box`. |
//! | [`InputBoxText`](inputbox::InputBoxText) | Marker for a container of glyphs in an `input_box` |
//! | [`InputBoxCursorBar`](inputbox::InputBoxCursorBar) | Bar for a cursor. |
//! | [`InputBoxCursorArea`](inputbox::InputBoxCursorArea) | Area for a cursor. |
//! 
//! # RichText
//! 
//! | Builder | Description |
//! | --------- | ----------- |
//! | [`RichTextBuilder`](richtext::RichTextBuilder) | Builder for `rich_text` |
//! 
pub mod inputbox;
pub mod drag;
pub mod richtext;
pub mod scroll;
pub mod clipping;
pub mod button;
mod constraints;
mod atlas;
pub use atlas::DeferredAtlasBuilder;
pub use constraints::SharedPosition;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::app::{Plugin, PreUpdate, Update, PostUpdate, Last};

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
                clipping::sync_camera_dimension,
            ).in_set(AouiWidgetEventSet))
            .add_systems(Update, (
                constraints::scroll_constraint,
                constraints::drag_constraint,
                constraints::discrete_scroll_sync,
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