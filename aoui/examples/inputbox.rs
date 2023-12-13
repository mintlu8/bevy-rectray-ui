use bevy::prelude::*;
use bevy_aoui::{AoUIPlugin, WorldExtension};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .register_cursor_default(CursorIcon::Arrow)
        .add_plugins(AoUIPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        font_size: em(4),
        hitbox: Rect(1),
        text: "Hello, World!",
        font: "RobotoCondensed.ttf",
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: 0.1,
            dimension: size2!([2, 1 em]),
            extra: Interpolate::<Color>::repeat(
                Some(EaseFunction::QuinticInOut), 
                gradient![(transparent, 0.0), (gold, 1.0)],
                1.0
            )
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
    });
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [-400, 0],
        rotation: degrees(45),
        font_size: em(4),
        hitbox: Rect(1),
        text: "I'm rotated!",
        font: "RobotoCondensed.ttf",
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: 0.1,
            dimension: size2!([2, 1 em]),
            extra: Interpolate::<Color>::repeat(
                Some(EaseFunction::QuinticInOut), 
                gradient![(transparent, 0.0), (gold, 1.0)],
                1.0
            )
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
    });
}
