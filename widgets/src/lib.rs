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
//! Unlike the event system in `bevy_aoui`, this api is optimized for library use,
//! and might not be as ergonomic for firing off one-shot events.
//! 
//! # Widgets
//! 
//! We currently offer a few simple widgets. 
//! 
//! * `Shape`: a vector shape renderer using `bevy_prototype_lyon`.
//! * `InputBox`: a single line text input.
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

use bevy::{prelude::{PreUpdate, PostUpdate}, app::Update};

pub mod dsl;
pub mod widgets;
pub mod events;

pub mod oneshot;
pub use oneshot::OneShot;

pub use widgets::schedule::AoUIWidgetsPlugin;

/// Plugin for the event pipeline.
pub struct AoUICursorEventsPlugin;

impl bevy::prelude::Plugin for AoUICursorEventsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<events::CursorState>()
            .init_resource::<events::DoubleClickThreshold>()
            .add_systems(PreUpdate, events::mouse_button_input)
            .add_systems(PostUpdate, events::remove_focus)
            .add_systems(Update, oneshot::call_oneshot_mouse)
        ;
    }
}

/// Plugin for both widgets and events.
pub struct AoUIExtensionsPlugin;

impl bevy::prelude::Plugin for AoUIExtensionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(widgets::schedule::FullWidgetsPlugin)
            .add_plugins(AoUICursorEventsPlugin)
        ;
    }
}