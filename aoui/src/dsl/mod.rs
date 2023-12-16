mod convert;
mod util;

use bevy::{prelude::{Commands, Entity, BuildChildren, Bundle}, asset::AssetServer};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod layouts;
mod widgets;
mod oneshot;
mod meta_dsl;
mod context;
mod shapes;
mod mesh2d;
mod atlas;
mod interpolate;

#[doc(hidden)]
pub use layouts::{SpanContainerNames, GridContainerNames};
#[doc(hidden)]
pub use util::{OneOrTwo, HandleOrString};
#[doc(hidden)]
pub use itertools::izip;

pub mod prelude;
pub use convert::{DslFrom, DslInto};
pub use context::{get_layer, is_using_opacity};

pub mod builders {
    use crate::widget_extension;

    widget_extension!(pub struct FrameBuilder {}, this, commands, assets, components: ());
    widget_extension!(pub struct SpriteBuilder: Sprite {}, this, commands, assets, components: ());
    widget_extension!(pub struct TextBoxBuilder: Text {}, this, commands, assets, components: ());
    pub use super::atlas::AtlasBuilder;

    pub use super::layouts::{PaddingBuilder, SpanContainerBuilder, GridContainerBuilder};
    pub use super::widgets::{InputBoxBuilder, CheckButtonBuilder, RadioButtonBuilder, ButtonBuilder, ClippingFrameBuilder};
    pub use super::shapes::RectangleBuilder;
    pub use super::mesh2d::{MaterialSpriteBuilder, MaterialMeshBuilder};
}

/// Construct an empty sprite.
#[macro_export]
macro_rules! frame {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {$($tt)*})};
}
/// Construct an image based sprite.
#[macro_export]
macro_rules! sprite {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::SpriteBuilder] {$($tt)*})};
}
/// Construct a textbox.
#[macro_export]
macro_rules! textbox {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::TextBoxBuilder] {$($tt)*})};
}

/// Enable commands to spawn our widgets.
pub trait AoUICommands {
    /// Spawn an aoui widget.
    fn spawn_aoui(&mut self, a: (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;
    /// Spawn an aoui widget.
    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, a: (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;

}

impl<'w, 's> AoUICommands for Commands<'w, 's> {
    /// Spawn a `Widget` without passing in an `AssetServer`, this may panic.
    fn spawn_aoui(&mut self, (widget, extras, children): (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self, None);
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }

    /// Spawn a `Widget` with an `AssetServer`.
    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, (widget, extras, children): (impl Widget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self, Some(assets));
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }
}

/// A widget for `bevy_aoui`.
/// 
/// You can construct it with the [`widget_extension`](crate::widget_extension) macro.
pub trait Widget: Sized {
    /// This function should panic if assets is needed but is `None`.
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> Entity;
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
