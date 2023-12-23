use bevy::{window::CursorIcon, app::{Last, App}, ecs::schedule::IntoSystemConfigs};
use crate::{widgets::button::CursorDefault, schedule::AouiCleanupSet};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;
}

impl WorldExtension for App {

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
}