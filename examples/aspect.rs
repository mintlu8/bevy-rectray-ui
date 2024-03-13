//! A simple test case for percentage size.
use bevy::prelude::*;
use bevy_rectray::{RectrayPlugin, util::RCommands};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RectrayPlugin)
        .add_systems(Startup, init)
        .run();
}

pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    sprite!(commands {
        sprite: "check.png",
        dimension: size2!(50%, 50%),
        color: color!(gold),
        aspect: Preserve,
    });
}