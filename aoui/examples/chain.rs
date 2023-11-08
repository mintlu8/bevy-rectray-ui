use bevy_aoui::*;
use bevy::{prelude::*, sprite::Anchor, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, init)
        .add_systems(Update, rotate_and_scale)
        .add_systems(Update, rotate_and_scale_root)
        .add_plugins(AoUIPlugin)
        .run();
}

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct AnchorMarker;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image>("square.png");
    commands.spawn(Camera2dBundle::default());

    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut last = commands.spawn((AoUISpriteBundle {
        transform: Transform2D::DEFAULT.with_anchor(Anchor::CenterLeft).with_z(0.1),
        sprite: Sprite {
            color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            anchor: Anchor::CenterLeft,
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, Root)).id();
    for _ in 0..120 {
        let curr = commands.spawn(AoUISpriteBundle {
            transform: Transform2D::DEFAULT
                .with_offset(Vec2::new(10.0, 0.0))
                .with_anchor(Anchor::CenterRight)
                .with_center(Anchor::CenterLeft),
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();
        let marker = commands.spawn((AoUISpriteBundle {
            transform: Transform2D::DEFAULT
                .with_offset(Vec2::new(1.0, 0.0))
                .with_anchor(Anchor::CenterRight)
                .with_z(1.0),
            sprite: Sprite {
                color: Color::WHITE,
                anchor: Anchor::CenterRight,
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }, AnchorMarker)).id();
        commands.entity(last).push_children(&[curr, marker]);
        last = curr;
    }
}

pub fn rotate_and_scale(mut query: Query<&mut Transform2D, Without<AnchorMarker>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::W) {
        for mut sp in query.iter_mut() {
            sp.rotation -= 0.004;
        }
    } else if keys.just_pressed(KeyCode::S) {
        for mut sp in query.iter_mut() {
            sp.rotation += 0.004;
        }
    } else if keys.just_pressed(KeyCode::Up) {
        for mut sp in query.iter_mut() {
            sp.rotation -= 0.004;
        }
    } else if keys.just_pressed(KeyCode::Down) {
        for mut sp in query.iter_mut() {
            sp.rotation += 0.004;
        }
    } else if keys.just_pressed(KeyCode::Left) {
        for mut sp in query.iter_mut() {
            sp.scale.x -= 0.02;
        }
    } else if keys.just_pressed(KeyCode::Right) {
        for mut sp in query.iter_mut() {
            sp.scale.x += 0.02;
        }
    }
}


pub fn rotate_and_scale_root(mut query: Query<&mut Transform2D, With<Root>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::A) {
        for mut sp in query.iter_mut() {
            sp.scale.x -= 0.02;
        }
    } else if keys.just_pressed(KeyCode::D) {
        for mut sp in query.iter_mut() {
            sp.scale.x += 0.02;
        }
    }
}
