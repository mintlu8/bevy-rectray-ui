use bevy::{window::CursorIcon, app::{Last, App}, ecs::{world::World, schedule::IntoSystemConfigs}};
use crate::{widgets::button::CursorDefault, schedule::AoUICleanupSet, signals::{SignalMarker, signal_cleanup}};

pub trait WorldExtension {
    fn register_signal<T: SignalMarker>(&mut self) -> &mut Self;
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;
}

impl WorldExtension for World {
    fn register_signal<T: SignalMarker>(&mut self) -> &mut Self {
        self.schedule_scope(Last, |_, s| {
            s.add_systems(signal_cleanup::<T>.in_set(AoUICleanupSet));
        });
        self
    }

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
}

impl WorldExtension for App {
    fn register_signal<T: SignalMarker>(&mut self) -> &mut Self {
        self.add_systems(Last, signal_cleanup::<T>.in_set(AoUICleanupSet));
        self
    }

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
}