use bevy_aoui::*;
use bevy::{prelude::*, sprite::Anchor};
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AoUIPlugin)
        .add_systems(Startup, init)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let texture = assets.load::<Image>("square.png");

    let left = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform2D::DEFAULT.with_anchor(Anchor::CenterLeft),
        dimension: Dimension::percentage(Vec2::new(0.5, 1.0)),
        texture: texture.clone(),
        ..Default::default()
    }).id();

    let _ = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            ..Default::default()
        },
        transform: Transform2D::DEFAULT.with_anchor(Anchor::CenterRight),
        dimension: Dimension::percentage(Vec2::new(0.5, 1.0)),
        texture: texture.clone(),
        ..Default::default()
    }).id();

    let up =  commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE,
            ..Default::default()
        },
        transform: Transform2D::DEFAULT.with_anchor(Anchor::TopCenter),
        dimension: Dimension::percentage(Vec2::new(1.0, 0.25)),
        texture: texture.clone(),
        ..Default::default()
    }).id();


    let down = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            ..Default::default()
        },
        transform: Transform2D::DEFAULT.with_anchor(Anchor::BottomCenter),
        dimension: Dimension::percentage(Vec2::new(1.0, 0.25)),
        texture: texture.clone(),
        ..Default::default()
    }).id();

    commands.entity(left).push_children(&[up, down]);
}