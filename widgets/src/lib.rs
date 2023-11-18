//! UI, events and dsl for the `bevy_aoui` crate.
//! 
//! This crate is highly unstable and subject to change.
//! 
//! If you want to use this crate, pick and choose what you need.
//! 
//! # Event System
//! 
//! Since AoUI presumably sits on top of the bevy app, we provide an event system
//! for detecting cursor activity exclusively for AoUI widgets. Cursor events not
//! catched by our system can be handled by other systems.
//! 
//! We offer a component insertion based core event system as well as
//! a oneshot system based event hadler system for end users.
//! 
//! # Widgets
//! 
//! We currently offer a few simple widgets. 
//! 
//! * `Shape`: a vector shape renderer using `bevy_prototype_lyon`.
//! * `InputBox`: a single line text input.
//! * `Buton`: Since AoUI does not have a standard look,
//! we currently don't offer a standard button widget, but all
//! the building blocks are there to build a button or a checkbox
//! directly through out components.
//! 
//! # DSL
//! 
//! We offer a DSL for streamlining widget construction.
//! 
//! Before you start, always import the prelude for syntax consistancy.
//! ```
//! use bevy_aoui_widgets::dsl::prelude::*;
//! ```
//! 
//! Each "widget" has a struct and its corresponding macro.
//! 
//! ```
//! # /*
//! sprite! ((commands, ..) {
//!     dim: [400, 400],
//!     sprite: assets.load("Ferris.png"),
//! });
//! # */
//! ```
//! This translates to
//! 
//! ```
//! # /*
//! Sprite {
//!     dim: [400, 400].dinto(),
//!     sprite: assets.load("Ferris.png").dinto(),
//!     ..Default::default(),
//! }.spawn_with(&mut commands);
//! # */
//! ```
//! 
//! Where `dinto` is our own [`Into`], [`DslInto`](crate::dsl::DslInto),
//! where all the syntax magic happens.
//! 
//! Check our our book or examples for more info.

use bevy::{prelude::{PreUpdate, PostUpdate}, app::Update, ecs::schedule::{IntoSystemConfigs, SystemSet, IntoSystemSetConfigs}, input::InputSystem};

pub mod dsl;
pub mod widgets;
pub mod events;

mod dto;

pub use dto::Submit;
mod oneshot;
pub use oneshot::{OneShot, OnSubmit, EventQuery};
use widgets::inputbox;

/// Plugin for the event pipeline.
pub(crate) struct AoUICursorEventsPlugin;

#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIEventSet;

#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIEventCleanupSet;

impl bevy::prelude::Plugin for AoUICursorEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use events::*;
        app.init_resource::<events::CursorState>()
            .init_resource::<events::DoubleClickThreshold>()
            .configure_sets(PreUpdate, AoUIEventSet.after(InputSystem))
            .configure_sets(PostUpdate, AoUIEventCleanupSet)
            .add_systems(PreUpdate, events::mouse_button_input.in_set(AoUIEventSet))
            .add_systems(PostUpdate, events::remove_focus.in_set(AoUIEventCleanupSet))
            .add_systems(Update, (
                oneshot::call_oneshot::<EventFlags>,
                oneshot::call_oneshot::<Click>,
                oneshot::call_oneshot::<Down>,
                oneshot::call_oneshot::<DragEnd>,
                oneshot::call_oneshot::<RightClick>,
                oneshot::call_oneshot::<RightDown>,
                oneshot::call_oneshot::<MidClick>,
                oneshot::call_oneshot::<MidDown>,
                oneshot::call_oneshot::<DoubleClick>,
                oneshot::call_oneshot::<DragEnd>,
                oneshot::call_oneshot::<ClickOutside>,

                oneshot::call_oneshot::<Hover>,
                oneshot::call_oneshot::<Pressed>,
                oneshot::call_oneshot::<Drag>,
                oneshot::call_oneshot::<MidPressed>,
                oneshot::call_oneshot::<MidDrag>,
                oneshot::call_oneshot::<RightPressed>,
                oneshot::call_oneshot::<RightDrag>,
                oneshot::call_oneshot::<OnSubmit>,
            ))
        ;
    }
}

/// Plugin for both widgets and events.
pub struct AoUIExtensionsPlugin;

impl bevy::prelude::Plugin for AoUIExtensionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(AoUICursorEventsPlugin)
            .add_plugins(widgets::schedule::FullWidgetsPlugin)
        ;
    }
}