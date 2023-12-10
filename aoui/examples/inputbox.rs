use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui::widgets::CursorDefault;


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
        .init_resource::<CursorDefault>()
        .add_plugins(AoUIPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [0, -100],
        font_size: em(4),
        hitbox: Rect(1),
        text: "Hello, World!",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
        extra: Interpolate::<Color>::repeat(
            Some(EaseFunction::QuinticInOut), 
            [(Vec4::from_array(color!(transparent).as_rgba_f32()), 0.0), (Vec4::from_array(color!(gold).as_rgba_f32()), 1.0)], 
            1.0
        )
    });
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [-400, 0],
        rotation: degrees(45),
        font_size: em(4),
        hitbox: Rect(1),
        text: "I'm tilted!",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
    });
}
