//! `bevy_aoui`'s DSL.

mod convert;
mod util;
mod core;

use bevy::{prelude::{Commands, Entity, BuildChildren, Bundle}, asset::AssetServer};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod layouts;
mod widgets;
mod meta_dsl;
mod mesh2d;
mod atlas;
mod interpolate;
mod converters;
mod clipping;
//mod rich_text;

pub use converters::*;

#[doc(hidden)]
pub use util::{OneOrTwo, Aspect, WidgetWrite};
#[doc(hidden)]
pub use itertools::izip;

pub use mesh2d::mesh_rectangle;

pub mod prelude;
pub use convert::{DslFrom, DslInto};

pub mod builders {
    pub use super::core::{FrameBuilder, SpriteBuilder, RectangleBuilder, TextBuilder};

    pub use super::atlas::AtlasBuilder;

    pub use super::layouts::PaddingBuilder;
    pub use super::widgets::{InputBoxBuilder, CheckButtonBuilder, RadioButtonBuilder, ButtonBuilder};
    pub use super::mesh2d::{MaterialSpriteBuilder, MaterialMeshBuilder};
    pub use super::clipping::{CameraFrameBuilder, ScrollingFrameBuilder};
}


/// Enable commands to spawn our widgets.
pub trait AouiCommands {
    /// Spawn an aoui widget.
    fn spawn_aoui(&mut self, a: (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;
    /// Spawn an aoui widget.
    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, a: (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;

}

impl<'w, 's> AouiCommands for Commands<'w, 's> {
    /// Spawn a `Widget` without passing in an `AssetServer`, this may panic.
    fn spawn_aoui(&mut self, (widget, extras, children): (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let (id, container) = widget.spawn(self);
        self.entity(container).push_children(children.as_ref());
        self.entity(id)
            .insert(extras);
        id
    }

    /// Spawn a `Widget` with an `AssetServer`.
    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, (widget, extras, children): (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let (id, container) = widget.spawn_with(self, Some(assets));
        self.entity(container).push_children(children.as_ref());
        self.entity(id)
            .insert(extras);
        id
    }
}

/// A widget for `bevy_aoui`.
/// 
/// You can construct it with the [`widget_extension`](crate::widget_extension) macro.
pub trait Widget: Sized {
    /// This function should panic if assets is needed but is `None`.
    fn spawn(self, commands: &mut Commands) -> (Entity, Entity) {
        self.spawn_with(commands, None)
    }
    /// This function should panic if assets is needed but is `None`.
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> (Entity, Entity);
}

/// Construct marker components by name.
#[macro_export]
macro_rules! markers {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, $crate::bevy::prelude::Component)]
            struct $name;
        )*
    };
}

/// Allow a struct to create many clones of itself as either 
/// itself T, an array `[T; N]` or a tuple `(T, T, T, ...)`.
pub trait CloneSplit<T: Clone> {
    fn clone_split(item: T) -> Self;
}

impl<T: Clone> CloneSplit<T> for T {
    fn clone_split(item: T) -> Self {
        item
    }
}


impl<T: Clone, const N: usize> CloneSplit<T> for [T; N] {
    fn clone_split(item: T) -> Self {
        std::array::from_fn(|_| item.clone())
    }
}

macro_rules! impl_clone_split {
    () => {};
    ($first: ident $(,$rest: ident)*) => {
        impl<$first: Clone> CloneSplit<$first> for ($first, $($rest),*) {
            fn clone_split(item: T) -> Self {
                (
                    $({
                        let v: $rest = item.clone();
                        v
                    },)*
                    item,
                )
            }
        }
        impl_clone_split!($($rest),*);
    };
}

impl_clone_split!(
    T,T,T,T,T,
    T,T,T,T,T,
    T,T,T,T,T
);
