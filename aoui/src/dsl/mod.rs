//! `bevy_aoui`'s DSL.
//! 
//! See the [main page](crate) for documentation.

mod convert;
mod util;
mod core;

use std::iter::Copied;

use bevy::hierarchy::DespawnRecursiveExt;
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
pub use util::{OneOrTwo, Scale, Aspect, WidgetWrite};
#[doc(hidden)]
pub use itertools::izip;

pub use mesh2d::mesh_rectangle;

pub mod prelude;
pub use convert::{DslFrom, DslInto};

use crate::signals::{SignalPool, AsObject, SignalBuilder};
use crate::widgets::clipping::render_target;

use self::convert::DslConvert;

pub mod builders {
    pub use super::core::{FrameBuilder, SpriteBuilder, RectangleBuilder, TextBuilder};

    pub use super::atlas::AtlasBuilder;

    pub use super::layouts::PaddingBuilder;
    pub use super::widgets::{InputBoxBuilder, CheckButtonBuilder, RadioButtonBuilder, ButtonBuilder};
    pub use super::mesh2d::{MaterialSpriteBuilder, MaterialMeshBuilder};
    pub use super::clipping::{CameraFrameBuilder, ScrollingFrameBuilder};
}

/// [`SystemParam`] combination of [`Commands`], [`AssetServer`] and [`SignalPool`].
#[derive(SystemParam)]
pub struct AouiCommands<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    signals: Res<'w, SignalPool>,
}

/// A dynamic function that builds an entity.
pub struct WidgetBuilder<T>(Box<dyn Fn(&mut AouiCommands, T) -> Entity + Send + Sync + 'static>);

impl<T> std::fmt::Debug for WidgetBuilder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityBuilder").finish()
    }
}

/// Trait for functions that are considered function builders.
pub trait IntoWidgetBuilder<T, const N: u8> {
    fn into_builder(self) -> impl Fn(&mut AouiCommands, T) -> Entity + Send + Sync + 'static;
}

impl<F> IntoWidgetBuilder<(), 0> for F where F: Fn(&mut AouiCommands) -> Entity + Send + Sync + 'static {
    fn into_builder(self) -> impl Fn(&mut AouiCommands, ()) -> Entity + Send + Sync + 'static {
        move |commands, _|self(commands)
    }
}

impl<F, T> IntoWidgetBuilder<T, 1> for F where F: Fn(&mut AouiCommands, T) -> Entity + Send + Sync + 'static {
    fn into_builder(self) -> impl Fn(&mut AouiCommands, T) -> Entity + Send + Sync + 'static {
        self
    }
}

impl<T> WidgetBuilder<T> {
    pub fn new<const M: u8>(f: impl IntoWidgetBuilder<T, M>) -> Self {
        Self(Box::new(f.into_builder()))
    }

    /// Build a widget entity with commands.
    pub fn build(&self, commands: &mut AouiCommands, item: T) -> Entity{
        (self.0)(commands, item)
    }
}

/// A powerful auto convert function, uses `DslInto` as the normal backend
/// while specializes for functions.
#[doc(hidden)]
pub fn parse<A, B, const N: u8>(item: A) -> B where A: DslConvert<B, N> {
    item.parse()
}

impl<'w, 's> AouiCommands<'w, 's> {
    /// Obtain the underlying [`Commands`].
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }

    /// Obtain an [`EntityCommands`].
    pub fn entity<'a>(&'a mut self, entity: Entity) -> EntityCommands<'w, 's, 'a> {
        self.commands.entity(entity)
    }

    /// Obtain the underlying [`AssetServer`].
    pub fn assets(&self) -> &AssetServer {
        &self.asset_server
    }

    /// Add an [`Asset`].
    pub fn add<T: Asset>(&self, item: T) -> Handle<T> {
        self.assets().add(item)
    }

    /// Load an [`Asset`] from an asset path.
    pub fn load<'a, T: Asset>(&self, name: impl Into<AssetPath<'a>>) -> Handle<T> {
        self.assets().load(name)
    }

    /// Spawn a bundle.
    pub fn spawn_bundle<'a>(&'a mut self, bundle: impl Bundle) -> EntityCommands<'w, 's, 'a>{
        self.commands.spawn(bundle)
    }

    /// Create a sprite as a render target.
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

    /// Created a tracked unnamed signal.
    pub fn signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self) -> S {
        self.signals.signal()
    }

    /// Created a tracked named signal.
    pub fn named_signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self, name: &str) -> S {
        self.signals.named(name)
    }

    /// Created a named untracked signal.
    pub fn shared_storage<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self, name: &str) -> S {
        self.signals.shared_storage(name)
    }

    /// Recursively despawn an entity, calls `despawn_recursive`. 
    pub fn despawn(&mut self, entity: Entity) {
        self.commands.entity(entity).despawn_recursive()
    }

    /// Despawn descendants.
    pub fn despawn_descendants(&mut self, entity: Entity) {
        self.commands.entity(entity).despawn_descendants();
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

#[doc(hidden)]
pub trait IntoChildren<'t, const M: u8> where Self::Out: 't {
    type Out: IntoIterator<Item = Entity>;
    fn into_entities(self) -> Self::Out;
}

impl IntoChildren<'static, 0> for Entity {
    type Out = [Entity; 1];
    fn into_entities(self) -> Self::Out {
        [self]
    }
}

impl<'t, T> IntoChildren<'t, 2> for T where T: IntoIterator<Item = Entity> + 't, T::IntoIter: 't {
    type Out = T;
    fn into_entities(self) -> Self::Out {
        self
    }
}

impl<'t, T> IntoChildren<'t, 3> for T where T: IntoIterator<Item = &'t Entity> + 't, T::IntoIter: 't {
    type Out = Copied<T::IntoIter>;
    fn into_entities(self) -> Self::Out {
        self.into_iter().copied()
    }
}

#[doc(hidden)]
pub fn into_children<'t, E: IntoChildren<'t, M>, const M:u8>(entity: E) -> E::Out {
    entity.into_entities()
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
