use bevy::{window::CursorIcon, app::{App, Last}, ecs::system::{Query, Res}, math::Vec2};
use crate::{widgets::button::CursorDefault, events::{EventHandling, Handlers, ScrollScaling}, dsl::DslInto, signals::DropFlag, schedule::AouiCleanupSet};

/// Extension methods to `World` and `App`
pub trait WorldExtension {

    /// Register a routine that resets the cursor every frame.
    fn register_cursor_default(&mut self, cursor: CursorIcon) -> &mut Self;

    /// Register mouse wheel scrolling speed.
    fn register_scrolling_speed(&mut self, line_to_pixels: impl DslInto<Vec2>, speed: impl DslInto<Vec2>) -> &mut Self;

    /// Register an event which cleans up its associated signals.
    fn register_event<T: EventHandling + 'static>(&mut self) -> &mut Self;
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

    fn register_event<T: EventHandling>(&mut self) -> &mut Self {
        use bevy::prelude::IntoSystemConfigs;
        fn event_cleanup<T: EventHandling>(
            drop_flag: Res<DropFlag>,
            mut query: Query<&mut Handlers<T>>
        ) {
            query.iter_mut().for_each(|x| x.cleanup(drop_flag.get()));
        }
        self.add_systems(Last, event_cleanup::<T>.in_set(AouiCleanupSet));
        self
    }
}