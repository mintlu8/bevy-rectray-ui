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

    rectangle!(commands {
        anchor: CenterLeft,
        dimension: size2!(50%, 100%),
        color: color!(red),
        child: rectangle! {
            anchor: TopCenter,
            dimension: size2!(100%, 25%),
            color: color!(orange),
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: size2!(100%, 25%),
            color: color!(purple),
        }
    });
    rectangle!(commands {
        anchor: CenterRight,
        dimension: size2!(50%, 100%),
        color: color!(blue)
    });
}