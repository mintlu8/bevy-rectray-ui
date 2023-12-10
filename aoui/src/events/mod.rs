use bevy::{prelude::*, input::InputSystem};
use crate::util::{Submit, Change, signal_cleanup};

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
pub struct AoUICursorEventsPlugin;

#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIEventSet;

#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIEventCleanupSet;

impl bevy::prelude::Plugin for AoUICursorEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CursorState>()
            .init_resource::<DoubleClickThreshold>()
            .configure_sets(PreUpdate, AoUIEventSet.after(InputSystem))
            .configure_sets(PostUpdate, AoUIEventCleanupSet)
            .add_systems(PreUpdate, mouse_button_input.in_set(AoUIEventSet))
            .add_systems(PreUpdate, wheel::mousewheel_event.in_set(AoUIEventSet))
            .add_systems(PostUpdate, remove_focus.in_set(AoUIEventCleanupSet))
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
            .add_systems(Last, (
                signal_cleanup::<()>,
                signal_cleanup::<Submit>,
                signal_cleanup::<Change>,
            ))
        ;
    }
}