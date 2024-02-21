use bevy::prelude::*;
use bevy_rectray::{RectrayPlugin, util::RCommands};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ime_enabled: true,
                ime_position: Vec2::new(300.0, 300.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .add_plugins(RectrayPlugin)
        .run();
}


pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    inputbox! (commands {
        dimension: size2!(400, 1 em),
        font_size: em(4),
        hitbox: Hitbox::rect(1),
        text: "Hello, World!",
        font: "RobotoCondensed.ttf",
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(red),
            z: 0.1,
            dimension: size2!(2, 1 em),
            extra: transition!(
                Opacity 0.5 QuadraticInOut loop [(0.0, 0.0), (1.0, 0.5), (1.0, 1.0)];
            ),

        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!(12, 1 em),
        },
        child: rectangle! {
            dimension: Size2::FULL,
            color: color!(darkgray),
            z: -0.2
        }
    });
    inputbox! (commands {
        dimension: size2!(400, 1 em),
        offset: [-400, 0],
        rotation: degrees(45),
        font_size: em(4),
        hitbox: Hitbox::rect(1),
        text: "I'm rotated!",
        font: "ComicNeue-Regular.ttf",
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(red),
            z: 0.1,
            dimension: size2!(2, 1 em),
            extra: transition!(
                Opacity 0.5 QuadraticInOut loop [(0.0, 0.0), (1.0, 0.5), (1.0, 1.0)];
            ),
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!(12, 1 em),
        },
        child: rectangle! {
            dimension: Size2::FULL,
            color: color!(darkgray),
            z: -0.2
        }
    });
}
