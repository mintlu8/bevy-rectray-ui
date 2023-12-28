//! This module provides cursor related event detection for `bevy_aoui`.
//! 
//! # Relation to Signals
//! 
//! Signals are designed to be polled by systems and has the capability to carry arbitrary data.
//! Events are design to trigger systems or send signals without the ability to send data directly.
//! 
//! # Event Listeners
//! 
//! Add components `Hitbox` and `EventFlags` to a sprite, this allows a sprite to
//! listen to a subset of events.
//! 
//! Only a subset of EventFlags are valid event listeners,
//! `*` is Left, Mid or Center, other mouse buttons are ignored.
//! 
//! * `Hover` listens for `Hover`,
//! * `*Click` listens for `*Down`, `*Up` and `*Pressed`
//! * `*Drag` listens for `*Down`, `*Drag` and `DragEnd`
//! * `*DoubleClick` listens for `DoubleClick`, which replaces `LeftClick` or `DragEnd`
//! * `Drop` listens for `Drop`
//! * `ClickOutside` listens for mouse up outside of the sprite's boundary.
//! 
//! # Event Propagation
//! 
//! We use component insertion to send events to widgets. These are
//! `CursorFocus`, `CursorAction`, `CursorClickOutside` and `MouseWheelAction`.
//! 
//! They should be safe to use in `Update` and `PostUpdate` like signals.
//! 
//! * `CursorFocus`: Stores a persistent state like `Hover` or `Pressed`, this
//! can be used with the [`With`] constraint. The [`DisplayIf`](widgets::button::DisplayIf)
//! component can be used to change visibility status based on [`CursorFocus`]
//! 
//! * `CursorAction`: Stores a single frame event like `Click` or `Down`.
//! * `CursorClickOutside`: Mouse up outside of the sprite's boundary.
//! * `MouseWheelAction`: Mouse wheel scrolling. Unlike others this receives data from scrolling.
//! 
//! # Event Handlers
//! 
//! A handler listens for `CursorAction` and `CursorFocus` alongside pseudo-events `ObtainFocus` and `LoseFocus`.
//! 
//! You can use the macro `handler!` to create an event handler 
//! using either one-shot systems or signals.
//! 
//! ```
//! # /*
//! sprite! {
//!     ...
//!     extra: handler! { LeftClick => 
//!         // this is a one-shot system function
//!         fn click_handler(mut commands: Commands) {
//!             commands.spawn(Fruit("Apple"));
//!         },
//!         // this is a signal sender
//!         // Notice the signal's default type is `()`.
//!         score_sender.map(|_: ()| 100),
//!     }
//! }
//! # */
//! ```
//! 
//! # Keyboard Events? Joysticks?
//! 
//! We provide abstractions that you can use for other types of input, but that's
//! outside the scope of this crate.

use bevy::{prelude::*, ecs::query::WorldQuery};
use crate::{schedule::{AouiEventSet, AouiCleanupSet}, Hitbox, Clipping, RotatedRect, Opacity, WorldExtension};

mod systems;
mod state;
mod event;
mod handler;
mod wheel;
mod cursor;

pub use event::*;
pub use state::*;
use systems::*;
pub use handler::*;
pub use wheel::{MouseWheelAction, ScrollScaling};
pub use cursor::CustomCursor;

use self::cursor::custom_cursor_controller;

/// Marker component for Aoui's camera, optional.
/// 
/// Used for cursor detection and has no effect on rendering.
/// If not present, we will try the `.get_single()` method instead.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct AouiCamera;


/// Query for checking whether a widget is active and can receive interactions.
#[derive(WorldQuery)]
pub struct ActiveDetection {
    vis: &'static Visibility,
    computed_vis: &'static InheritedVisibility,
    opacity: &'static Opacity,
}

impl ActiveDetectionItem<'_> {
    pub fn is_active(&self) -> bool {
        self.vis != Visibility::Hidden && self.computed_vis.get()
            && self.opacity.is_active()
    }
}

/// Query for checking whether cursor is in bounds of a widget.
#[derive(WorldQuery)]
pub struct CursorDetection {
    hitbox: &'static Hitbox,
    rect: &'static RotatedRect,
    clipping: &'static Clipping,
}

impl CursorDetectionItem<'_> {
    pub fn contains(&self, pos: Vec2) -> bool{
        self.hitbox.contains(self.rect, pos) 
            && self.clipping.contains(pos)
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.rect.z.total_cmp(&other.rect.z)
    }

    pub fn z(&self) -> f32 {
        self.rect.z
    }
}

/// Plugin for the event pipeline.
#[derive(Debug)]
pub(crate) struct CursorEventsPlugin;

impl bevy::prelude::Plugin for CursorEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CursorState>()
            .init_resource::<ScrollScaling>()
            .init_resource::<DoubleClickThreshold>()
            .add_systems(PreUpdate, mouse_button_input.in_set(AouiEventSet))
            .add_systems(PreUpdate, wheel::mousewheel_event.in_set(AouiEventSet))
            .add_systems(Last, remove_focus.in_set(AouiCleanupSet))
            .add_systems(Update, (
                event_handle::<EvLeftClick>,
                event_handle::<EvLeftDown>,
                event_handle::<EvDragEnd>,
                event_handle::<EvRightClick>,
                event_handle::<EvRightDown>,
                event_handle::<EvMidClick>,
                event_handle::<EvMidDown>,
                event_handle::<EvDoubleClick>,
                event_handle::<EvDragEnd>,
                event_handle::<EvClickOutside>,
                event_handle::<EvHover>,
                event_handle::<EvLeftPressed>,
                event_handle::<EvLeftDrag>,
                event_handle::<EvMidPressed>,
                event_handle::<EvMidDrag>,
                event_handle::<EvRightPressed>,
                event_handle::<EvRightDrag>,
                lose_focus_detection,
                obtain_focus_detection,
                custom_cursor_controller,
            ))
            .register_event::<EvLeftClick>()
            .register_event::<EvLeftDown>()
            .register_event::<EvDragEnd>()
            .register_event::<EvRightClick>()
            .register_event::<EvRightDown>()
            .register_event::<EvMidClick>()
            .register_event::<EvMidDown>()
            .register_event::<EvDoubleClick>()
            .register_event::<EvDragEnd>()
            .register_event::<EvClickOutside>()
            .register_event::<EvHover>()
            .register_event::<EvLeftPressed>()
            .register_event::<EvLeftDrag>()
            .register_event::<EvMidPressed>()
            .register_event::<EvMidDrag>()
            .register_event::<EvRightPressed>()
            .register_event::<EvRightDrag>()
            .register_event::<EvMouseWheel>()
            .register_event::<EvMouseDrag>()
            .register_event::<EvObtainFocus>()
            .register_event::<EvLoseFocus>()
            .register_event::<EvObtainFocus>()
            .register_event::<EvButtonClick>()
            .register_event::<EvToggleChange>()
            .register_event::<EvTextChange>()
            .register_event::<EvTextSubmit>()
            .register_event::<EvPositionFactor>()
        ;
    }
}