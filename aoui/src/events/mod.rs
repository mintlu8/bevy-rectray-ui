//! This module provides cursor related event detection for `bevy_aoui`.
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
//! You can use these with queries.
//!
//! They should be safe to use during `Update` and `PostUpdate`.
//!
//! * `CursorFocus`: Stores a persistent state like `Hover` or `Pressed`.
//! The [`DisplayIf`](crate::widgets::util::DisplayIf)
//! component can be used to change visibility status based on [`CursorFocus`]
//!
//! * `CursorAction`: Stores a single frame event like `Click` or `Down`.
//! * `CursorClickOutside`: Mouse up outside of the sprite's boundary.
//! * `MouseWheelAction`: Stores the value of mouse wheel scrolling.
//!
//! # What about Keyboard Events or Joysticks?
//!
//! We provide abstractions that you can use for other types of input,
//! but these are outside the scope of this crate.

use bevy::{prelude::*, ecs::query::WorldQuery};
use crate::{Hitbox, Clipping, RotatedRect, Opacity};
use crate::widgets::util::{CursorDefault, remove_all};
use crate::schedule::{AouiCleanupSet, AouiEventSet, AouiWidgetEventSet};

pub(crate) mod systems;
pub(crate) mod wheel;
mod state;
mod event;
mod cursor;
mod gbb;
mod focus;

pub use event::*;
pub use state::*;
use systems::*;
pub use wheel::{MovementUnits, ScrollScaling, MouseWheelAction};
pub use cursor::{CustomCursor, TrackCursor};
pub use cursor::CameraQuery;
pub use gbb::{GreaterBoundingBox, GreaterBoundingBoxPercent, GreaterBoundingBoxPx};
pub use focus::*;

use self::gbb::calculate_greater_bounding_box;
use self::cursor::{custom_cursor_controller, track_cursor};

/// Marker component for Aoui's camera, optional.
///
/// Used for cursor detection and has no effect on rendering.
/// If not present, we will try the `.get_single()` method instead.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
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
            .init_resource::<CursorDefault>()
            .add_systems(PreUpdate, mouse_button_input.in_set(AouiEventSet))
            .add_systems(PreUpdate, mouse_button_click_outside.in_set(AouiEventSet).after(mouse_button_input))
            .add_systems(PreUpdate, wheel::mousewheel_event.in_set(AouiEventSet))
            .add_systems(PreUpdate, focus::run_focus_signals.in_set(AouiWidgetEventSet))
            .add_systems(PreUpdate, focus::run_strong_focus_signals.in_set(AouiWidgetEventSet))
            .add_systems(FixedUpdate, (
                track_cursor,
                custom_cursor_controller,
                calculate_greater_bounding_box,
            ))
            .add_systems(Last, (
                remove_all::<CursorAction>,
                remove_all::<CursorFocus>,
                remove_all::<CursorClickOutside>,
                remove_all::<MouseWheelAction>,
                remove_all::<DescendantHasFocus>,
            ).in_set(AouiCleanupSet))
        ;
    }
}
