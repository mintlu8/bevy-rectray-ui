//! `bevy_aoui`'s DSL.

mod convert;
mod util;
mod core;

use bevy::prelude::{Commands, Entity, BuildChildren, Bundle};
use bevy::ecs::system::{SystemParam, Res, EntityCommands};
use bevy::asset::{AssetServer, Asset, Handle, AssetPath};
use bevy::render::texture::Image;
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

use crate::signals::{SignalPool, AsObject, SignalBuilder};
use crate::widgets::clipping::render_target;

pub mod builders {
    pub use super::core::{FrameBuilder, SpriteBuilder, RectangleBuilder, TextBuilder};

    pub use super::atlas::AtlasBuilder;

    pub use super::layouts::PaddingBuilder;
    pub use super::widgets::{InputBoxBuilder, CheckButtonBuilder, RadioButtonBuilder, ButtonBuilder};
    pub use super::mesh2d::{MaterialSpriteBuilder, MaterialMeshBuilder};
    pub use super::clipping::{CameraFrameBuilder, ScrollingFrameBuilder};
}

#[derive(SystemParam)]
pub struct AouiCommands<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    signals: Res<'w, SignalPool>,
}

impl<'w, 's> AouiCommands<'w, 's> {
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }

    pub fn entity<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 's, 'a> {
        self.commands.entity(entity)
    }

    pub fn assets(&self) -> &AssetServer {
        &self.asset_server
    }

    pub fn add<T: Asset>(&self, item: T) -> Handle<T> {
        self.assets().add(item)
    }

    pub fn load<'a, T: Asset>(&self, name: impl Into<AssetPath<'a>>) -> Handle<T> {
        self.assets().load(name)
    }

    pub fn spawn_bundle<'a>(&'a mut self, bundle: impl Bundle) -> EntityCommands<'w, 's, 'a>{
        self.commands.spawn(bundle)
    }

    pub fn render_target<T: CloneSplit<Handle<Image>>>(&self, dimension: [u32; 2]) -> T{
        render_target(&self.asset_server, dimension)
    }

    /// Spawn a `Widget` without passing in an `AssetServer`, this may panic.
    pub fn spawn_aoui(&mut self, widget: impl Widget, extras: impl Bundle, children: impl AsRef<[Entity]>) -> Entity {
        let (id, container) = widget.spawn(self);
        self.entity(container).push_children(children.as_ref());
        self.entity(id)
            .insert(extras);
        id
    }

    pub fn signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self) -> S {
        self.signals.signal()
    }

    pub fn named_signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self, name: &str) -> S {
        self.signals.named(name)
    }
}

impl AsRef<AssetServer> for AouiCommands<'_, '_> {
    fn as_ref(&self) -> &AssetServer {
        &self.asset_server
    }
}

impl<'w, 's> AsMut<Commands<'w, 's>> for AouiCommands<'w, 's> {
    fn as_mut(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }
}

/// A widget for `bevy_aoui`.
/// 
/// You can construct it with the [`widget_extension`](crate::widget_extension) macro.
pub trait Widget: Sized {
    /// This function should panic if assets is needed but is `None`.
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity);
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
