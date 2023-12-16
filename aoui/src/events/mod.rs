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
#[derive(Debug)]
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
                event_handle::<Click>,
                event_handle::<Down>,
                event_handle::<DragEnd>,
                event_handle::<RightClick>,
                event_handle::<RightDown>,
                event_handle::<MidClick>,
                event_handle::<MidDown>,
                event_handle::<DoubleClick>,
                event_handle::<DragEnd>,
                event_handle::<ClickOutside>,

                event_handle::<Hover>,
                event_handle::<Pressed>,
                event_handle::<Drag>,
                event_handle::<MidPressed>,
                event_handle::<MidDrag>,
                event_handle::<RightPressed>,
                event_handle::<RightDrag>,
                lose_focus_detection,
            ))
        ;
    }
}