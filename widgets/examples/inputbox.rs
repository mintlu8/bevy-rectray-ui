use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, handler, Submit, widgets::CursorDefault};
use bevy_prototype_lyon::prelude::*;


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
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [0, -100],
        font_size: em(4),
        hitbox: Rect(1),
        text: "Hello, World!",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: shape! {
            shape: Shapes::Rectangle,
            fill: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: shape! {
            shape: Shapes::Rectangle,
            fill: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
        extra: handler!{OnSubmit => fn name(submit: Query<&Submit>) {
            match submit.get_single(){
                Ok(s) => {
                    let s: &str = s.get();
                    println!("Submit: {}", s)
                },
                Err(_) => print!("Submit: Value missing"),
            };
        }},
    });
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [0, 100],
        font_size: em(4),
        hitbox: Rect(1),
        z: 1,
        text: "Hello, Bevy!",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: sprite! {
            sprite: assets.load("square.png"),
            color: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
            extra: Interpolate::<Offset>::ease(EaseFunction::QuinticInOut, Vec2::ZERO, 0.2),
            extra: Interpolate::<Color>::repeat(
                Some(EaseFunction::QuinticInOut), 
                [(Vec4::from_array(color!(transparent).as_rgba_f32()), 0.0), (Vec4::from_array(color!(gold).as_rgba_f32()), 1.0)], 
                1.0
            )
        },
        cursor_area: sprite! {
            sprite: assets.load("square.png"),
            color: color!(darkblue),
            z: -0.1,
            dimension: size2!([12, 1 em]),
            extra: Interpolate::<Offset>::ease(EaseFunction::QuinticInOut, Vec2::ZERO, 0.2),
            extra: Interpolate::<Dimension>::ease(EaseFunction::QuinticInOut, Vec2::new(0.0, 1.0), 0.2),
        },
        extra: handler!{OnSubmit => fn name(submit: Query<&Submit>) {
            match submit.get_single(){
                Ok(s) => {
                    let s: &str = s.get();
                    println!("Submit: {}", s)
                },
                Err(_) => print!("Submit: Value missing"),
            };
        }},
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
        cursor_bar: shape! {
            shape: Shapes::Rectangle,
            fill: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: shape! {
            shape: Shapes::Rectangle,
            fill: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
    });
}
