//! Widget primitives for `bevy_aoui`
//!
//! `bevy_aoui` has no standard styles, sprites or shaders,
//! meaning we only provide building blocks for creating widgets.
//! `bevy_matui` is a case study for building real widgets using `bevy_aoui`'s primitives.
//!
//! # Button
//!
//! | Component | Description |
//! | --------- | ----------- |
//! | [`Button`](button::Button) | Marker for enabling the `ButtonClick` event. |
//! | [`CheckButton`](button::CheckButton) | Context, checked or unchecked for a `check_button`. |
//! | [`RadioButton`](button::RadioButton) | Context for a `radio_button`. |
//! | [`Payload`](button::Button) | Data sent by `ButtonClick`. |
//! | [`RadioButtonCancel`](button::RadioButtonCancel) | Allow clicking radio button again to remove its value. |
//!
//! # Dragging And Scrolling
//!
//! | Component | Description |
//! | --------- | ----------- |
//! | [`Dragging`](drag::Dragging) | Enable scrolling of children. |
//! | [`Scrolling`](scroll::Scrolling) | Enable scrolling of children. |
//! | [`Constraint`](constraints::Constraint) | Constraint movement to the parent's dimension. |
//! | [`ScrollDiscrete`](scroll::ScrollDiscrete) | Discrete scrolling for [`Layout`](crate::layout::Layout). |
//! | [`DragSnapBack`](drag::DragSnapBack) | Snap dragged sprite back to the source. |
//! | [`SharedPosition`](constraints::SharedPosition) | Share position between draggable/scrollable widgets. |
//!
//! # Camera
//!
//! | Bundle | Description |
//! | --------- | ----------- |
//! | [`ScopedCameraBundle`](clipping::ScopedCameraBundle) | Bind a camera to a sprite's `RotatedRect`. |
//!
//! # Misc
//!
//! | Component | Description |
//! | --------- | ----------- |
//! | [`PropagateFocus`](util::PropagateFocus) | Propagate `CursorFocus` and `CheckButtonState`. |
//! | [`SetCursor`](util::SetCursor) | Set cursor icon during some cursor events. |
//! | [`DisplayIf`](util::DisplayIf) | Display if some condition is met. |
//!
//! # InputBox
//!
//! | Component | Description |
//! | --------- | ----------- |
//! | [`InputBox`](inputbox::InputBox) | Context of an `input_box`, holding the text and cursor information. |
//! | [`InputBoxText`](inputbox::InputBoxText) | Marker for a container of glyphs in an `input_box` |
//! | [`InputBoxCursorBar`](inputbox::InputBoxCursorBar) | Bar for a cursor. |
//! | [`InputBoxCursorArea`](inputbox::InputBoxCursorArea) | Area for a cursor. |
//!
//! # RichText
//!
//! | Builder | Description |
//! | --------- | ----------- |
//! | [`RichTextBuilder`](richtext::RichTextBuilder) | Builder for `rich_text` (wip) |
//!
pub mod inputbox;
pub mod drag;
pub mod richtext;
pub mod scroll;
pub mod clipping;
pub mod button;
pub mod spinner;
pub mod util;
pub mod signals;
mod text;
use bevy::ecs::system::IntoSystem;
pub use text::TextFragment;
pub mod constraints;
mod atlas;
pub mod misc;
pub use atlas::DeferredAtlasBuilder;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::app::{Plugin, PreUpdate, Update, PostUpdate, Last};

use crate::events::{CursorAction, CursorFocus};
use crate::schedule::{AouiCleanupSet, AouiLoadInputSet, AouiPostEventSet, AouiPostWidgetEventSet, AouiStoreOutputSet, AouiWidgetEventSet};

use self::button::CheckButtonState;
use self::inputbox::InputBoxState;

pub(crate) struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PreUpdate, (
                button::button_on_click,
                button::check_button_on_click,
                button::radio_button_on_click,
                button::generate_check_button_state,
                scroll::propagate_mouse_wheel_action,
                util::propagate_focus::<CursorAction>,
                util::propagate_focus::<CursorFocus>,
            ).in_set(AouiPostEventSet))
            .add_systems(PreUpdate, (
                inputbox::update_inputbox_cursor
                    .before(inputbox::inputbox_keyboard),
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                inputbox::text_propagate_focus,
                drag::drag_start,
                drag::drag_end,
                drag::dragging.after(drag::drag_start),
                scroll::scrolling_senders,
                (
                    scroll::scrolling_system,
                    scroll::scroll_discrete_system,
                ).after(scroll::scrolling_senders),
                clipping::sync_camera_dimension,
            ).in_set(AouiWidgetEventSet))
            .add_systems(PreUpdate, (
                util::propagate_focus::<CheckButtonState>,
                inputbox::text_propagate_focus,
            ).in_set(AouiPostWidgetEventSet))
            .add_systems(Update, (
                util::set_cursor,
                util::event_conditional_visibility,
                util::check_conditional_visibility,
                inputbox::draw_input_box
                    .before(text::sync_text_text_fragment)
                    .before(text::sync_sprite_text_fragment),
                inputbox::inputbox_conditional_visibility,
                atlas::build_deferred_atlas,
                text::sync_text_text_fragment,
                text::sync_sprite_text_fragment,
                spinner::spin_text_change,
                spinner::sync_spin_text_with_text,
                signals::sig_set_text,
                signals::radio_button_clear_widget,
                signals::inputbox_clear_widget,
                signals::text_clear_widget,
            ))
            .add_systems(Update, (
                misc::layout_opacity_limit.pipe(misc::set_layout_opactiy_limit),
            ))
            .add_systems(PostUpdate, (
                richtext::synchronize_glyph_spaces
            ).in_set(AouiLoadInputSet))
            .add_systems(PostUpdate, (
                text::sync_em_text_fragment,
                inputbox::sync_em_inputbox
            ).in_set(AouiStoreOutputSet))
            .add_systems(Last, util::remove_all::<CheckButtonState>.in_set(AouiCleanupSet))
            .add_systems(Last, util::remove_all::<InputBoxState>.in_set(AouiCleanupSet))
        ;
    }
}
