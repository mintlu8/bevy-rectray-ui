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
//! The [`DisplayIf`](crate::widgets::button::DisplayIf)
//! component can be used to change visibility status based on [`CursorFocus`]
//! 
//! * `CursorAction`: Stores a single frame event like `Click` or `Down`.
//! * `CursorClickOutside`: Mouse up outside of the sprite's boundary.
//! * `MouseWheelAction`: Stores the value of mouse wheel scrolling.
//! 
//! # Event Handlers
//! 
//! A [`Handlers`] listens for `CursorAction` and `CursorFocus`,
//! pseudo-events like `EvObtainFocus` and `EvLoseFocus`,
//! widget events like `EvButtonChange` etc and can perform.
//! many action based on the event and its associated input.
//! 
//! Event handlers can do the following things:
//! 
//! * Run a [one-shot system](OneShot).
//! * [Mutate](Mutation) components associated with the entity.
//! * Send a signal.
//! * Write signal input to a [`KeyStorage`](crate::signals::KeyStorage).
//! 
//! # What about Keyboard Events or Joysticks?
//! 
//! We provide abstractions that you can use for other types of input, 
//! but these are outside the scope of this crate.

use bevy::{prelude::*, ecs::query::WorldQuery};
use crate::{schedule::{AouiEventSet, AouiCleanupSet}, Hitbox, Clipping, RotatedRect, Opacity, WorldExtension, widgets::button::CursorDefault};

mod systems;
mod state;
mod event;
mod handler;
mod wheel;
mod cursor;
pub(crate) mod mutation;
mod oneshot;
mod coverage;

pub use event::*;
pub use state::*;
use systems::*;
pub use handler::*;
pub use wheel::{MouseWheelAction, ScrollScaling};
pub use cursor::CustomCursor;
pub use mutation::Mutation;
pub use oneshot::OneShot;

use self::cursor::custom_cursor_controller;
pub use coverage::{ESigCoveragePercent, ESigCoveragePx};

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
            .init_resource::<CursorDefault>()
            .add_systems(PreUpdate, mouse_button_input.in_set(AouiEventSet))
            .add_systems(PreUpdate, wheel::mousewheel_event.in_set(AouiEventSet))
            .add_systems(Last, remove_focus.in_set(AouiCleanupSet))
            .add_systems(Update, (
                handle_event::<EvLeftClick>,
                handle_event::<EvLeftDown>,
                handle_event::<EvDragEnd>,
                handle_event::<EvRightClick>,
                handle_event::<EvRightDown>,
                handle_event::<EvMidClick>,
                handle_event::<EvMidDown>,
                handle_event::<EvDoubleClick>,
                handle_event::<EvDragEnd>,
                handle_event::<EvClickOutside>,
                handle_event::<EvHover>,
                handle_event::<EvLeftPressed>,
                handle_event::<EvLeftDrag>,
                handle_event::<EvMidPressed>,
                handle_event::<EvMidDrag>,
                handle_event::<EvRightPressed>,
                handle_event::<EvRightDrag>,
            ))
            .add_systems(Update, (
                lose_focus_detection,
                obtain_focus_detection,
                custom_cursor_controller,
                coverage::calculate_coverage,
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
            .register_event::<ESigCoveragePercent>()
            .register_event::<ESigCoveragePx>()
        ;
    }
}