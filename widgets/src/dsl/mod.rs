mod convert;
mod util;
use std::fmt::Debug;

use bevy::prelude::{Commands, Entity, BuildChildren, Bundle};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod core;
mod layouts;
mod shapes;
mod inputbox;
mod oneshot;
mod meta_dsl;

pub mod prelude;

pub use convert::DslInto;
pub use layouts::{SpanContainerNames, GridContainerNames};
pub use util::OneOrTwo;

pub mod builders {
    pub use super::shapes::ShapeBuilder;
    pub use super::layouts::{DynamicFrameBuilder, SpanContainerBuilder, GridContainerBuilder};
    pub use super::core::{FrameBuilder, SpriteBuilder, TextBoxBuilder};
    pub use super::inputbox::{InputBoxBuilder, ButtonBuilder};
}

#[doc(hidden)]
/// Implementation detail for meta_dsl.
pub trait FnChildren {
    type Out: AsRef<[Entity]> + Default;
    fn exec(self, commands: &mut Commands) -> Self::Out;
}

impl<F, Out> FnChildren for F where F: FnOnce(&mut Commands) -> Out, Out: AsRef<[Entity]> + Default {
    type Out = Out;

    fn exec(self, commands: &mut Commands) -> Self::Out {
        self(commands)
    }
}

#[doc(hidden)]
#[derive(Debug, Default)]
/// Implementation detail for meta_dsl.
pub enum EntitiesBuilder<F: FnChildren>{
    Some(F),
    #[default]
    None,
}

impl<F: FnChildren> EntitiesBuilder<F> {
    pub fn build_entities(self, commands: &mut Commands) -> F::Out{
        match self {
            EntitiesBuilder::Some(f) => f.exec(commands),
            EntitiesBuilder::None => Default::default(),
        }
    }
}

/// Enable commands to spawn our widgets.
pub trait AoUICommands {
    /// Spawn an aoui widget.
    fn spawn_aoui(&mut self, a: (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;
}

impl<'w, 's> AoUICommands for Commands<'w, 's> {
    fn spawn_aoui(&mut self, (widget, extras, children): (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self);
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }
}

pub trait AoUIWidget: Sized {
    fn spawn_with(self, commands: &mut Commands) -> Entity;
}

/// Construct a marker component by name.
#[macro_export]
macro_rules! marker {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ::bevy::prelude::Component)]
        struct $name;
    };
}