//! `bevy_aoui`'s DSL.
//!
//! See the [main page](crate) for documentation.

mod convert;
mod util;
mod core;
use std::iter::Copied;
use bevy::prelude::Entity;

#[doc(hidden)]
pub use colorthis::rgbaf;

mod layouts;
mod widgets;
mod meta_dsl;
mod frame;
mod mesh2d;
mod atlas;
mod interpolate;
mod clipping;
//mod rich_text;


pub use util::{OneOrTwo, Scale, Aspect, WidgetWrite, ParentAnchor};
pub use crate::util::convert::{OptionEx, DslFromOptionEx, IntoAsset};
#[doc(hidden)]
pub use itertools::izip;

pub mod prelude;
use crate::util::{DslFrom, convert::DslConvert};

pub mod builders {
    pub use super::core::{FrameBuilder, SpriteBuilder, RectangleBuilder, TextBuilder};

    pub use super::atlas::AtlasBuilder;

    pub use super::layouts::PaddingBuilder;
    pub use super::widgets::{InputBoxBuilder, CheckButtonBuilder, RadioButtonBuilder, ButtonBuilder};
    pub use super::mesh2d::{MaterialSpriteBuilder, MaterialMeshBuilder};
    pub use super::clipping::{CameraFrameBuilder, ScrollingFrameBuilder};
}


/// The auto convert function for bevy_aoui's DSL,
/// uses `DslInto` as the normal backend
/// while specializes for functions
/// and some other cases normally requiring specialization with a single trait.
pub fn parse<A, B, const N: char>(item: A) -> B where A: DslConvert<B, N> {
    item.parse()
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

impl<'t, T> IntoChildren<'t, 1> for T where T: IntoIterator<Item = Entity> + 't, T::IntoIter: 't {
    type Out = T;
    fn into_entities(self) -> Self::Out {
        self
    }
}

impl<'t, T> IntoChildren<'t, 2> for T where T: IntoIterator<Item = &'t Entity> + 't, T::IntoIter: 't {
    type Out = Copied<T::IntoIter>;
    fn into_entities(self) -> Self::Out {
        self.into_iter().copied()
    }
}

#[doc(hidden)]
pub fn into_children<'t, E: IntoChildren<'t, M>, const M:u8>(entity: E) -> E::Out {
    entity.into_entities()
}

/// Documents intrinsic fields of bevy_aoui's dsl.
#[doc(hidden)]
pub mod intrinsics {

    /// Pseudo-type of a mutable reference to an [`Entity`](bevy::ecs::entity::Entity).
    /// ```
    /// let mut item: Entity;
    /// ```
    pub struct MutEntity;
    pub struct IntrinsicEntity {
        /// Fetches an entity from a nested macro invocation.
        /// ```
        /// let mut item: Entity;
        /// sprite!(commands {
        ///     child: sprite! {
        ///         entity: item
        ///     }
        /// })
        /// ```
        pub entity: MutEntity
    }

    pub struct ImplBundle;
    pub struct IntrinsicExtra {
        /// Adds a [`Bundle`](bevy::ecs::bundle::Bundle) to an [`Entity`](bevy::ecs::entity::Entity).
        pub extra: ImplBundle
    }

    pub struct EntityOrIterator;
    pub struct IntrinsicChild {
        /// Adds a [`Entity`](bevy::ecs::bundle::Bundle), `Option<Entity>`, 
        /// `IntoIterator<Item = Entity>` or `IntoIterator<Item = &Entity>`
        /// as children.
        pub child: EntityOrIterator
    }

    pub struct RoleSignal;

    pub struct IntrinsicSignal {
        /// A [`RoleSignal`](crate::sync::RoleSignal), either sender or receiver. 
        /// 
        /// Created by the 
        /// [`sender`](crate::dsl::prelude::sender) and
        /// [`receiver`](crate::dsl::prelude::receiver) functions.
        pub signal: RoleSignal
    }

    pub struct AsyncSystem;

    pub struct IntrinsicSystem {
        /// An asynchronous system function [`AsyncSystem`](crate::sync::AsyncSystem).
        pub system: AsyncSystem
    }
}