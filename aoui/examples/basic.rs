use bevy_aoui::*;
use bevy::{prelude::*, sprite::Anchor, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};

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
        .add_systems(Update, controls)
        .add_plugins(AoUIPlugin)
        .run();
}

#[derive(Debug, Component)]
pub struct A;
#[derive(Debug, Component)]
pub struct B;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let b = commands.spawn((AoUISpriteBundle {
        sprite: Sprite { 
            anchor: Anchor::Center,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            color: Color::RED,
            ..Default::default()
        },
        texture: assets.load("square.png"),
        ..Default::default()
    },B)).id();

    commands.spawn((AoUISpriteBundle {
        sprite: Sprite { 
            anchor: Anchor::Center,
            custom_size: Some(Vec2::new(200.0, 200.0)),
            color: Color::CYAN,
            ..Default::default()
        },
        texture: assets.load("square.png"),
        ..Default::default()
    },A)).add_child(b);
}

pub fn spin_anc(anc: &Anchor) -> Anchor {
    match anc {
        Anchor::BottomLeft => Anchor::BottomCenter,
        Anchor::BottomCenter => Anchor::BottomRight,
        Anchor::BottomRight => Anchor::CenterLeft,
        Anchor::CenterLeft => Anchor::Center,
        Anchor::Center => Anchor::CenterRight,
        Anchor::CenterRight => Anchor::TopLeft,
        Anchor::TopLeft => Anchor::TopCenter,
        Anchor::TopCenter => Anchor::TopRight,
        Anchor::TopRight => Anchor::BottomLeft,
        Anchor::Custom(_) => unreachable!(),
    }
}

/// Controls: 
/// 1: Spin Parent Anchor
/// 2: Spin Child Anchor
/// Arrows: Move Parent
/// WASD: Move Child
pub fn controls(
    mut a: Query<(&mut Sprite, &mut Transform2D), (With<A>, Without<B>)>, 
    mut b: Query<(&mut Sprite, &mut Transform2D), (With<B>, Without<A>)>, 
    keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        for (mut sp, _) in a.iter_mut() {
            sp.anchor = spin_anc(&sp.anchor)
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        for (mut sp, _) in b.iter_mut() {
            sp.anchor = spin_anc(&sp.anchor)
        }
    }
    if keys.just_pressed(KeyCode::Left) {
        for (_, mut transform) in a.iter_mut() {
            transform.offset.edit_raw(|x| *x -= Vec2::new(10.0, 0.0));
        }
    }
    if keys.just_pressed(KeyCode::Right) {
        for (_, mut transform) in a.iter_mut() {
            transform.offset.edit_raw(|x| *x += Vec2::new(10.0, 0.0));
        }
    }
    if keys.just_pressed(KeyCode::Down) {
        for (_, mut transform) in a.iter_mut() {
            transform.offset.edit_raw(|x| *x -= Vec2::new(0.0, 10.0));
        }
    }
    if keys.just_pressed(KeyCode::Up) {
        for (_, mut transform) in a.iter_mut() {
            transform.offset.edit_raw(|x| *x += Vec2::new(0.0, 10.0));
        }
    }
    if keys.just_pressed(KeyCode::A) {
        for (_, mut transform) in b.iter_mut() {
            transform.offset.edit_raw(|x| *x -= Vec2::new(10.0, 0.0));
        }
    }
    if keys.just_pressed(KeyCode::D) {
        for (_, mut transform) in b.iter_mut() {
            transform.offset.edit_raw(|x| *x += Vec2::new(10.0, 0.0));
        }
    }
    if keys.just_pressed(KeyCode::S) {
        for (_, mut transform) in b.iter_mut() {
            transform.offset.edit_raw(|x| *x -= Vec2::new(0.0, 10.0));
        }
    }
    if keys.just_pressed(KeyCode::W) {
        for (_, mut transform) in b.iter_mut() {
            transform.offset.edit_raw(|x| *x += Vec2::new(0.0, 10.0));
        }
    }
}
