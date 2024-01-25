//! Utilities for `bevy_aoui`.

mod widget;
mod commands;
mod cloning;
mod extension;
mod mesh;
mod object;
mod compose;
mod queries;
mod to_bundle;

pub mod convert;

pub use mesh::mesh_rectangle;
pub use widget::{Widget, WidgetBuilder, IntoWidgetBuilder};
pub use commands::{AouiCommands, signal};
pub use cloning::CloneSplit;
pub use extension::WorldExtension;
pub use convert::{DslFrom, DslInto};
pub use object::{Object, AsObject};
pub use compose::{ComponentCompose, ComposeExtension};
pub use queries::*;