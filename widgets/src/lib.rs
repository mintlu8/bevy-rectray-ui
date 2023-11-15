use bevy::prelude::{PreUpdate, PostUpdate};


mod input;
pub mod dsl;
pub mod widgets;
pub mod events;

/// System used by AoUI widgets.
pub struct AoUIWidgetsPlugin;

impl bevy::prelude::Plugin for AoUIWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(widgets::schedule::WidgetsPlugin)
            .init_resource::<events::CursorState>()
            .init_resource::<events::DoubleClickThreshold>()
            .add_systems(PreUpdate, events::mouse_button_input)
            .add_systems(PostUpdate, events::remove_focus)
        ;
    }
}