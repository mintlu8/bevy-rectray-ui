use bevy::{window::CursorIcon, app::App, math::Vec2};
use crate::{widgets::util::CursorDefault, events::ScrollScaling, util::DslInto};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;

    /// Register mouse wheel scrolling speed.
    fn register_scrolling_speed(&mut self, line_to_pixels: impl DslInto<Vec2>, speed: impl DslInto<Vec2>) -> &mut Self;
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
}