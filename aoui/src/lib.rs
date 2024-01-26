#![doc=include_str!("../README.md")]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_match)]
pub(crate) mod core;
pub mod util;
pub mod layout;
pub mod dsl;
pub mod widgets;
pub mod events;
pub mod anim;

//pub mod signals;
pub use core::*;

#[doc(hidden)]
pub use bevy;

#[doc(hidden)]
pub use bevy_defer as defer;

#[doc(hidden)]
pub use bevy_defer::async_system;

pub mod schedule;

pub use schedule::CorePlugin;

/// Plugin for both widgets and events.
#[derive(Debug)]
pub struct AouiPlugin;

impl bevy::prelude::Plugin for AouiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<util::SignalPool>()
            .add_plugins(schedule::CorePlugin)
            .add_plugins(events::CursorEventsPlugin)
            .add_plugins(anim::AnimationPlugin)
            .add_plugins(widgets::WidgetsPlugin)
            .add_plugins(bevy_defer::DefaultAsyncPlugin)
        ;
    }
}
