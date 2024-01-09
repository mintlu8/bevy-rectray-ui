use bevy::{window::CursorIcon, app::{App, Update}, math::Vec2};
use crate::{widgets::util::CursorDefault, events::ScrollScaling, dsl::DslInto};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;

    /// Register mouse wheel scrolling speed.
    fn register_scrolling_speed(&mut self, line_to_pixels: impl DslInto<Vec2>, speed: impl DslInto<Vec2>) -> &mut Self;

    /// Register addition signal ids, default is `0..=5`. `255` should not be used
    fn register_signal_id<const SIGNAL_ID: u8>(&mut self) -> &mut Self;
}

impl WorldExtension for App {

    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self {
        self.insert_resource(CursorDefault(cursor));
        self
    }
    
    fn register_scrolling_speed(&mut self, line_to_pixels: impl DslInto<Vec2>, speed: impl DslInto<Vec2>) -> &mut Self {
        self.insert_resource(ScrollScaling{
            line_to_pixels: line_to_pixels.dinto(),
            pixel_scale: speed.dinto(),
        })
    }

    fn register_signal_id<const S: u8>(&mut self) -> &mut Self {
        self.add_systems(Update, crate::signals::signal_receive::<S>)
    }
}