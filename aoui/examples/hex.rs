//! This is a demo of `FlexContainer::Span`
//! 
//! use keys
//! 
//! ```?
//! Q W E
//! A S D
//! Z X C
//! ```
//! to add children with different anchors.
//! 
//! And use `backspace` to reset.
//! 
//! Notice how insertion order matters.
//! 

use bevy_aoui::*;
use bevy::{prelude::*, sprite::Anchor, math::Affine2};
use rand::Rng;

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
        .add_plugins(AoUIPlugin)
        .run();
}

#[derive(Component)]
pub struct Root;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image, _>("square.png");
    commands.spawn(Camera2dBundle::default());

    let root = commands.spawn((AoUISpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(600.0, 500.0)),
            anchor: Anchor::Center,
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, SparseContainer {
        layout: SparseLayout::HexGrid { 
            x: HexDir::DownRight, 
            y: HexDir::Up, 
            size: Vec2::new(100.0, 100.0), 
        },
        transform: Affine2::IDENTITY,
        origin: Vec2::new(0.0, 0.0),
        child_rect: None,
    }, Root)).id();

    for i in 0..10 {
        spawn(&mut commands, Vec2::new(i as f32, 0.0), root, &assets);
        spawn(&mut commands, Vec2::new(i as f32, 1.0), root, &assets);
        spawn(&mut commands, Vec2::new(-i as f32, 0.0), root, &assets);
        spawn(&mut commands, Vec2::new(-i as f32, 1.0), root, &assets);
    }
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::Hsla { hue: rng.gen_range(0.0..=360.0), saturation: 1.0, lightness: 0.5, alpha: 1.0 }
}

pub fn spawn(commands: &mut Commands, pos: Vec2, flexbox: Entity, assets: &Res<AssetServer>){
    let child = commands.spawn((AoUISpriteBundle {
        sprite: Sprite {
            color: random_color(),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            anchor: Anchor::TopLeft,
            ..Default::default()
        },
        texture: assets.load::<Image, _>("square.png"),
        ..Default::default()
    }, SparsePosition(pos)
    )).id();
    commands.entity(flexbox).add_child(child);
}
