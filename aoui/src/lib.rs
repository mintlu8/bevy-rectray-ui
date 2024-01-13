#![doc=include_str!("../README.md")]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_match)]
pub(crate) mod core;
pub mod layout;
pub mod dsl;
pub mod widgets;
pub mod events;
pub mod anim;

pub mod signals;
pub use core::*;

#[doc(hidden)]
pub use bevy;

pub mod schedule;
mod extension;
pub use extension::WorldExtension;

pub use schedule::CorePlugin;

/// Plugin for both widgets and events.
#[derive(Debug)]
pub struct AouiPlugin;

impl bevy::prelude::Plugin for AouiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(schedule::CorePlugin)
            .add_plugins(signals::SignalsPlugin)
            .add_plugins(events::CursorEventsPlugin)
            .add_plugins(anim::AnimationPlugin)
            .add_plugins(widgets::WidgetsPlugin)
        ;
    }
}
