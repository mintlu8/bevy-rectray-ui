use bevy::{window::CursorIcon, app::{App, Last}, ecs::system::Query, math::Vec2};
use crate::{widgets::{button::CursorDefault, scroll::ScrollDirection}, events::{EventHandling, Handlers}, dsl::DslInto};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;

    /// Register mouse wheel scrolling speed.
    fn register_scrolling_speed(&mut self, speed: impl DslInto<Vec2>) -> &mut Self;
    
    /// Register mouse wheel scrolling speed but inverted.
    fn register_inverted_scrolling(&mut self, speed: impl DslInto<Vec2>) -> &mut Self;

    /// Register an event which cleans up its associated signals.
    fn register_event<T: EventHandling + 'static>(&mut self) -> &mut Self;
}

impl WorldExtension for App {

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
    
    fn register_scrolling_speed(&mut self, speed: impl DslInto<Vec2>) -> &mut Self {
        self.insert_resource(ScrollDirection::new(speed.dinto()))
    }

    fn register_inverted_scrolling(&mut self, speed: impl DslInto<Vec2>) -> &mut Self {
        self.insert_resource(ScrollDirection::inverted(speed.dinto()))
    }

    fn register_event<T: EventHandling>(&mut self) -> &mut Self {
        fn event_cleanup<T: EventHandling>(mut query: Query<&mut Handlers<T>>) {
            query.iter_mut().for_each(|x| x.cleanup());
        }
        self.add_systems(Last, event_cleanup::<T>);
        self
    }
}