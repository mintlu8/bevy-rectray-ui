//! A simple test case for percentage size.
use bevy::prelude::*;
use bevy_aoui::AouiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AouiPlugin)
        .add_systems(Startup, init)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    sprite!((commands, assets) {
        sprite: "check.png",
        dimension: size2!(50%, 50%),
        color: color!(gold),
        aspect: Preserve,
    });
}