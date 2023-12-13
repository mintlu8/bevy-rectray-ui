use bevy::prelude::*;
use crate::schedule::{AoUIEventSet, AoUICleanupSet};

mod systems;
mod state;
mod event;
mod oneshot;
mod wheel;
mod cursor;

pub use event::*;
pub use state::*;
use systems::*;
pub use oneshot::*;
pub use wheel::MouseWheelAction;
pub use cursor::CustomCursor;

/// Marker component for AoUI's camera, optional.
/// 
/// Used for cursor detection and has no effect on rendering.
/// If not present, we will try the `.get_single()` method instead.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct AoUICamera;


/// Plugin for the event pipeline.
pub struct CursorEventsPlugin;

impl bevy::prelude::Plugin for CursorEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CursorState>()
            .init_resource::<DoubleClickThreshold>()
            .add_systems(PreUpdate, mouse_button_input.in_set(AoUIEventSet))
            .add_systems(PreUpdate, wheel::mousewheel_event.in_set(AoUIEventSet))
            .add_systems(Last, remove_focus.in_set(AoUICleanupSet))
            .add_systems(Update, cursor::custom_cursor_controller)
            .add_systems(Update, (
                call_oneshot::<EventFlags>,
                call_oneshot::<Click>,
                call_oneshot::<Down>,
                call_oneshot::<DragEnd>,
                call_oneshot::<RightClick>,
                call_oneshot::<RightDown>,
                call_oneshot::<MidClick>,
                call_oneshot::<MidDown>,
                call_oneshot::<DoubleClick>,
                call_oneshot::<DragEnd>,
                call_oneshot::<ClickOutside>,

                call_oneshot::<Hover>,
                call_oneshot::<Pressed>,
                call_oneshot::<Drag>,
                call_oneshot::<MidPressed>,
                call_oneshot::<MidDrag>,
                call_oneshot::<RightPressed>,
                call_oneshot::<RightDrag>,
                lose_focus_detection,
            ))
        ;
    }
}