use bevy::{window::CursorIcon, app::{Last, App}, ecs::world::World};
use crate::widgets::CursorDefault;
use super::signals;



pub trait WorldExtension {
    fn register_signal<T: 'static>(&mut self);
    fn register_cursor_default(&mut self, cursor: CursorIcon);
}

impl WorldExtension for World {
    fn register_signal<T: 'static>(&mut self) {
        self.schedule_scope(Last, |_, s| {
            s.add_systems(signals::signal_cleanup::<T>);
        });
    }

    fn register_cursor_default(&mut self, cursor: CursorIcon) {
        self.insert_resource(CursorDefault(cursor));
    }
}

impl WorldExtension for App {
    fn register_signal<T: 'static>(&mut self) {
        self.add_systems(Last, signals::signal_cleanup::<T>);
    }

    fn register_cursor_default(&mut self, cursor: CursorIcon) {
        self.insert_resource(CursorDefault(cursor));
    }
}