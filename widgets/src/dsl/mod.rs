mod convert;
mod util;

use bevy::prelude::{Commands, Entity, BuildChildren, Bundle};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod layouts;
mod shapes;
mod inputbox;
mod oneshot;
mod meta_dsl;

#[doc(hidden)]
pub use layouts::{SpanContainerNames, GridContainerNames};
#[doc(hidden)]
pub use util::OneOrTwo;

pub mod prelude;
pub use convert::DslInto;

pub mod builders {
    use crate::widget_extension;

    widget_extension!(pub struct FrameBuilder {}, this, commands, components: ());
    widget_extension!(pub struct SpriteBuilder: Sprite {}, this, commands, components: ());
    widget_extension!(pub struct TextBoxBuilder: Text {}, this, commands, components: ());

    pub use super::shapes::ShapeBuilder;
    pub use super::layouts::{PaddingBuilder, SpanContainerBuilder, GridContainerBuilder};
    pub use super::inputbox::{InputBoxBuilder, ButtonBuilder};
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
    fn spawn_with<'w, 's>(self, commands: &mut Commands<'w, 's>) -> Entity;
}

/// Construct marker components by name.
#[macro_export]
macro_rules! markers {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ::bevy::prelude::Component)]
            struct $name;
        )*
    };
}
