use bevy::{window::CursorIcon, app::{App, Last}, ecs::system::Query};
use crate::{widgets::button::CursorDefault, events::{EventHandling, Handlers}};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;

    /// Register an event which cleansup its associated signals.
    fn register_event<T: EventHandling + 'static>(&mut self);
}

impl WorldExtension for App {

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }

    fn register_event<T: EventHandling>(&mut self) {
        fn eveny_cleanup<T: EventHandling>(mut query: Query<&mut Handlers<T>>) {
            query.iter_mut().for_each(|x| x.cleanup());
        }
        self.add_systems(Last, eveny_cleanup::<T>);
    }
}