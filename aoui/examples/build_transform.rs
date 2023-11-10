use bevy_aoui::{*, bundles::*};
use bevy::{prelude::*, sprite::Anchor};

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
        .add_systems(Update, change_anchor)
        .add_plugins(AoUIPlugin)
        .run();
}

macro_rules! add {
    ($commands: expr, $assets: expr, $anchor: ident) => {
        {
            let a = $commands.spawn((AoUISpriteBundle {
                sprite: Sprite { 
                    anchor: Anchor::Center,
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    color: Color::BLUE,
                    ..Default::default()
                },
                texture: $assets.load("square.png"),
                ..Default::default()
            },
            BuildTransformBundle::default()
            )).id();

            let b = $commands.spawn(SpriteBundle {
                sprite: Sprite { 
                    anchor: Anchor::Center,
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    color: Color::GREEN,
                    ..Default::default()
                },
                texture: $assets.load("square.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }).id();

            $commands.entity(a).push_children(&[b]);
        }
    };
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    add!(commands, assets, BottomLeft);
    add!(commands, assets, CenterLeft);
    add!(commands, assets, TopLeft);
    add!(commands, assets, TopCenter);
    add!(commands, assets, BottomCenter);
    add!(commands, assets, TopRight);
    add!(commands, assets, CenterRight);
    add!(commands, assets, BottomRight);
}

pub fn change_anchor(mut query: Query<&mut Transform2D>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut sp in query.iter_mut() {
            match sp.center.as_ref().unwrap() {
                Anchor::BottomLeft => sp.center = Some(Anchor::BottomCenter),
                Anchor::BottomCenter => sp.center = Some(Anchor::BottomRight),
                Anchor::BottomRight => sp.center = Some(Anchor::CenterLeft),
                Anchor::CenterLeft => sp.center = Some(Anchor::Center),
                Anchor::Center => sp.center = Some(Anchor::CenterRight),
                Anchor::CenterRight => sp.center = Some(Anchor::TopLeft),
                Anchor::TopLeft => sp.center = Some(Anchor::TopCenter),
                Anchor::TopCenter => sp.center = Some(Anchor::TopRight),
                Anchor::TopRight => sp.center = Some(Anchor::BottomLeft),
                Anchor::Custom(_) => unreachable!(),
            }
        }
    }
}
