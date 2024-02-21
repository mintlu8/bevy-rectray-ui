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

use bevy::window::CursorIcon;
#[doc(hidden)]
pub use bevy_defer as defer;

#[doc(hidden)]
pub use bevy_defer::async_system;

pub mod schedule;

pub use schedule::CorePlugin;
use util::WorldExtension;

/// The core plugin for bevy_rectray.
#[derive(Debug)]
pub struct RectrayPlugin;

impl bevy::prelude::Plugin for RectrayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<util::SignalPool>()
            .register_cursor_default(CursorIcon::Default)
            .add_plugins(schedule::CorePlugin)
            .add_plugins(events::CursorEventsPlugin)
            .add_plugins(anim::AnimationPlugin)
            .add_plugins(widgets::WidgetsPlugin)
            .add_plugins(bevy_defer::DefaultAsyncPlugin)
        ;
    }
}
