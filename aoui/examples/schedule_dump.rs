use bevy::{prelude::*, log::LogPlugin};
use bevy_aoui::AouiPlugin;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }).disable::<LogPlugin>())
        .add_plugins(AouiPlugin);
    //bevy_mod_debugdump::print_schedule_graph(&mut app, PreUpdate);
    //bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    bevy_mod_debugdump::print_schedule_graph(&mut app, PostUpdate);
    //bevy_mod_debugdump::print_schedule_graph(&mut app, Last);
    //bevy_mod_debugdump::print_render_graph(&mut app)
}

