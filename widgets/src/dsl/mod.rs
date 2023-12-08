mod convert;
mod util;

use bevy::{prelude::{Commands, Entity, BuildChildren, Bundle}, asset::AssetServer};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod layouts;
mod shapes;
mod widgets;
mod oneshot;
mod meta_dsl;
mod context;

#[doc(hidden)]
pub use layouts::{SpanContainerNames, GridContainerNames};
#[doc(hidden)]
pub use util::OneOrTwo;

pub mod prelude;
pub use convert::DslInto;
pub use context::get_layer;

pub mod builders {
    use crate::widget_extension;

    widget_extension!(pub struct FrameBuilder {}, this, commands, assets, components: ());
    widget_extension!(pub struct SpriteBuilder: Sprite {}, this, commands, assets, components: ());
    widget_extension!(pub struct TextBoxBuilder: Text {}, this, commands, assets, components: ());

    pub use super::shapes::ShapeBuilder;
    pub use super::layouts::{PaddingBuilder, SpanContainerBuilder, GridContainerBuilder};
    pub use super::widgets::{InputBoxBuilder, ButtonBuilder, ClippingFrameBuilder};
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
    fn spawn_aoui(&mut self, a: (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;
    /// Spawn an aoui widget.
    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, a: (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;

}

impl<'w, 's> AoUICommands for Commands<'w, 's> {
    fn spawn_aoui(&mut self, (widget, extras, children): (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self, None);
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }

    fn spawn_aoui_with_assets(&mut self, assets: &AssetServer, (widget, extras, children): (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self, Some(assets));
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }
}

pub trait AoUIWidget: Sized {
    fn spawn_with<'w, 's>(self, commands: &mut Commands<'w, 's>, assets: Option<&AssetServer>) -> Entity;
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
