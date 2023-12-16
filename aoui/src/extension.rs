use bevy::{window::CursorIcon, app::{Last, App}, ecs::{world::World, schedule::IntoSystemConfigs}};
use crate::{widgets::button::CursorDefault, schedule::AoUICleanupSet, signals::{SignalSender, signal_cleanup}};

/// Extension methods to `World` and `App`
pub trait WorldExtension {
    /// Register a signal's cleanup routine.
    /// 
    /// Only senders need to be registered.
    fn register_signal<T: SignalSender>(&mut self) -> &mut Self;

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;
}

impl WorldExtension for World {
    fn register_signal<T: SignalSender>(&mut self) -> &mut Self {
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
    fn register_signal<T: SignalSender>(&mut self) -> &mut Self {
        self.add_systems(Last, signal_cleanup::<T>.in_set(AoUICleanupSet));
        self
    }

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
}