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
        .add_plugins(AoUIPlugin)
        .run();
}

static ANCHORS: &[Anchor] = &[
    Anchor::TopLeft, Anchor::TopCenter, Anchor::TopRight,
    Anchor::CenterLeft, Anchor::Center, Anchor::CenterRight,
    Anchor::BottomLeft, Anchor::BottomCenter, Anchor::BottomRight,
];

pub fn spawn_fractal(commands: &mut Commands, count: usize, size: f32, enitity: Entity, texture: Handle<Image>) {
    if count == 0 {
        return;
    }
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    for anchor in ANCHORS {
        let child = commands.spawn(AoUISpriteBundle {
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                custom_size: Some(Vec2::new(size, size)),
                ..Default::default()
            },
            transform: Transform2D { 
                anchor: anchor.clone(),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();

        spawn_fractal(commands, count - 1, size / 4.0, child, texture.clone());
        commands.entity(enitity).push_children(&[child]);
    }
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image>("square.png");
    commands.spawn(Camera2dBundle::default());

    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    for anchor in ANCHORS {
        let enitity = commands.spawn(AoUISpriteBundle {
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 1.0),
                custom_size: Some(Vec2::new(200.0, 200.0)),
                anchor: anchor.clone(),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();

        spawn_fractal(&mut commands, 3, 50.0, enitity, texture.clone());
    }
}

pub fn rotate_and_scale(mut query: Query<&mut Transform2D>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Left) {
        for mut sp in query.iter_mut() {
            sp.rotation -= 0.1;
        }
    } else if keys.just_pressed(KeyCode::Right) {
        for mut sp in query.iter_mut() {
            sp.rotation += 0.1;
        }
    } else if keys.just_pressed(KeyCode::Up) {
        for mut sp in query.iter_mut() {
            sp.scale += Vec2::splat(0.1);
        }
    } else if keys.just_pressed(KeyCode::Down) {
        for mut sp in query.iter_mut() {
            sp.scale -= Vec2::splat(0.1);
        }
    }
}
