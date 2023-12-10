//! A stress test for a large hierarchy.

use std::f32::consts::PI;

use bevy_aoui::{*, bundles::*};
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_egui::{EguiContexts, egui::{self, Slider}, EguiPlugin};
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
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, egui_window)
        .add_plugins(EguiPlugin)
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
                anchor: *anchor,
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        }).id();

        spawn_fractal(commands, count - 1, size / 3.0, child, texture.clone());
        commands.entity(enitity).push_children(&[child]);
    }
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<Image>("square.png");
    commands.spawn(Camera2dBundle::default());

    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let enitity = commands.spawn(AoUISpriteBundle {
        sprite: Sprite {
            color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 1.0),
            custom_size: Some(Vec2::new(800.0, 800.0)),
            ..Default::default()
        },
        texture: texture.clone(),
        ..Default::default()
    }).id();

    spawn_fractal(&mut commands, 5, 250.0, enitity, texture.clone());
}

pub fn egui_window(mut ctx: EguiContexts, 
    mut query: Query<&mut Transform2D>,
) {
    let sp = query.iter().next().unwrap();
    let mut rotation = sp.rotation;
    let mut scale = sp.scale.x;
    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        ui.add(Slider::new(&mut rotation, -PI * 2.0..=PI * 2.0).text("Rotation"));
        ui.add(Slider::new(&mut scale, 0.0..=10.0).text("Scale"));
    });
    for mut sp in query.iter_mut() {
        sp.rotation = rotation;
        sp.scale = Vec2::splat(scale);
    }
}
