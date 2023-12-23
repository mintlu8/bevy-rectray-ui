use bevy::prelude::*;
use bevy_aoui::{AouiPlugin, WorldExtension};
use bevy_matui::{MatuiPlugin, mbutton, mtoggle};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .add_plugins(MatuiPlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    mbutton!((commands, assets){
        dimension: size2![100, 100],
        radius: 5,
        shadow: 5,
        shadow_z: -10,
        background: color!(red500),
        background_hover: color!(red600),
        background_pressed: color!(darkred800),
        capsule: true,
        icon: "cross.png",
        text: "Hello",
    });

    mtoggle!((commands, assets){
        offset: [0, 200],
        background_color: color!(red200),
        background_active: color!(red700),
        dial_color: color!(red500),
        checked_color: color!(white),
        background_stroke: (color!(darkred), 3),
    });
}