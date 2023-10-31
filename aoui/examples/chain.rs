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

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image, _>("square.png");
    commands.spawn(Camera2dBundle::default());

    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut last = commands.spawn((AoUISpriteBundle {
        anchors: Anchors::inherit(Anchor::CenterLeft),
        transform: Transform2D::DEFAULT.with_z(0.1),
        sprite: Sprite {
            color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            anchor: Anchor::CenterLeft,
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }, Root)).id();
    for index in 0..120 {
        let curr = commands.spawn(AoUISpriteBundle {
            anchors: Anchors::inherit(Anchor::CenterLeft),
            transform: Transform2D::new(Vec2::new(10.0, 0.0)).with_z(0.1 * index as f32),
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                anchor: Anchor::CenterRight,
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();
        commands.entity(last).push_children(&[curr]);
        last = curr;
    }
}

pub fn rotate_and_scale(mut query: Query<&mut Transform2D>, keys: Res<Input<KeyCode>>) {
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
