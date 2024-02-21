use std::sync::Arc;
use bevy::ecs::entity::Entity;
use crate::util::RCommands;


/// A widget for `bevy_rectray`.
///
/// You can construct it with the [`frame_extension`](crate::frame_extension) macro.
pub trait Widget: Sized {
    /// This function should panic if assets is needed but is `None`.
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity);

    /// Construct a widget builder from a clonable widget.
    fn into_bulider(self) -> WidgetBuilder<()> where Self: Clone + Send + Sync + 'static {
        WidgetBuilder::new(move |commands: &mut RCommands| self.clone().spawn(commands).0)
    }
}

#[derive(Clone)]
/// A dynamic function that builds an entity.
pub struct WidgetBuilder<T>(Arc<dyn Fn(&mut RCommands, T) -> Entity + Send + Sync + 'static>);

impl<T> std::fmt::Debug for WidgetBuilder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetBuilder").finish()
    }
}

/// Trait for functions that can create an entity with an argument.
pub trait IntoWidgetBuilder<T, const N: u8> {
    fn into_builder(self) -> impl Fn(&mut RCommands, T) -> Entity + Send + Sync + 'static;
}

impl<F> IntoWidgetBuilder<(), 0> for F where F: Fn(&mut RCommands) -> Entity + Send + Sync + 'static {
    fn into_builder(self) -> impl Fn(&mut RCommands, ()) -> Entity + Send + Sync + 'static {
        move |commands, _|self(commands)
    }
}

impl<F, T> IntoWidgetBuilder<T, 1> for F where F: Fn(&mut RCommands, T) -> Entity + Send + Sync + 'static {
    fn into_builder(self) -> impl Fn(&mut RCommands, T) -> Entity + Send + Sync + 'static {
        self
    }
}

impl<T> WidgetBuilder<T> {
    pub fn new<const M: u8>(f: impl IntoWidgetBuilder<T, M>) -> Self {
        Self(Arc::new(f.into_builder()))
    }

    /// Build a widget entity with commands.
    pub fn build(&self, commands: &mut RCommands, item: T) -> Entity{
        (self.0)(commands, item)
    }
}