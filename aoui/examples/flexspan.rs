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
use bevy::{prelude::*, sprite::Anchor};
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
        .add_systems(Update, reord)
        .run();
}

#[derive(Component)]
pub struct Root;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image, _>("square.png");
    commands.spawn(Camera2dBundle::default());

    commands.spawn((AoUISpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(600.0, 100.0)),
            anchor: Anchor::Center,
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, FlexContainer {
        layout: FlexLayout::Span {
            direction: FlexDir::LeftToRight,
            stretch: false,
        },
        margin: Size2::pixels(10.0, 10.0),
    }, Root));
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::Hsla { hue: rng.gen_range(0.0..=360.0), saturation: 1.0, lightness: 0.5, alpha: 1.0 }
}

pub fn spawn(commands: &mut Commands, anchor: Anchor, flexbox: Entity, assets: &Res<AssetServer>){
    let child = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: random_color(),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            anchor,
            ..Default::default()
        },
        texture: assets.load::<Image, _>("square.png"),
        ..Default::default()
    }).id();
    commands.entity(flexbox).add_child(child);
}

pub fn reord(mut commands: Commands, mut query: Query<(Entity, &mut FlexContainer), With<Root>>, spawned: Query<Entity, (With<AoUI>, Without<Root>)>, keys: Res<Input<KeyCode>>, assets: Res<AssetServer>) {
    let (flexbox, mut container) = query.single_mut();
    if keys.just_pressed(KeyCode::Q) {
        spawn(&mut commands, Anchor::TopLeft, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::A) {
        spawn(&mut commands, Anchor::CenterLeft, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::Z) {
        spawn(&mut commands, Anchor::BottomLeft, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::W) {
        spawn(&mut commands, Anchor::TopCenter, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::S) {
        spawn(&mut commands, Anchor::Center, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::X) {
        spawn(&mut commands, Anchor::BottomCenter, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::E) {
        spawn(&mut commands, Anchor::TopRight, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::D) {
        spawn(&mut commands, Anchor::CenterRight, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::C) {
        spawn(&mut commands, Anchor::BottomRight, flexbox, &assets)    
    }
    if keys.just_pressed(KeyCode::Back) {
        for entity in spawned.iter() {
            commands.entity(entity).despawn();
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        match &mut container.layout {
            FlexLayout::Span { stretch, .. } => *stretch = !*stretch,
            _ => todo!(),
        }
    }
}
