//! A stress test for a large hierarchy.

use std::f32::consts::PI;

use bevy_rectray::{*, bundles::*, util::RCommands};
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
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
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, egui_window)
        .add_plugins(EguiPlugin)
        .add_plugins(RectrayPlugin)
        .run();
}

static ANCHORS: &[Anchor] = &[
    Anchor::TOP_LEFT, Anchor::TOP_CENTER, Anchor::TOP_RIGHT,
    Anchor::CENTER_LEFT, Anchor::CENTER, Anchor::CENTER_RIGHT,
    Anchor::BOTTOM_LEFT, Anchor::BOTTOM_CENTER, Anchor::BOTTOM_RIGHT,
];

pub fn spawn_fractal(commands: &mut Commands, count: usize, size: f32, entity: Entity, texture: Handle<Image>) {
    if count == 0 {
        return;
    }
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    for anchor in ANCHORS {
        let child = commands.spawn(RSpriteBundle {
            transform: Transform2D {
                anchor: *anchor,
                ..Default::default()
            },
            dimension: Dimension {
                dimension: DimensionType::Owned(Size2::pixels(size, size)),
                ..Default::default()
            },
            texture: texture.clone(),
            color: Coloring::new(Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5)),
            ..Default::default()
        }).id();

        spawn_fractal(commands, count - 1, size / 3.0, child, texture.clone());
        commands.entity(entity).add_child(child);
    }
}

pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    let texture = commands.load::<Image>("square.png");
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        system: |fps: Fps, text: Ac<Text>| {
            let fps = fps.get().await;
            text.set(move |text| format_widget!(text, "FPS: {:.2}", fps)).await?;
        }
    });

    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let entity = commands.spawn_bundle(RSpriteBundle {
        dimension: Dimension {
            dimension: DimensionType::Owned(Size2::pixels(800.0, 800.0)),
            ..Default::default()
        },
        texture: texture.clone(),
        color: Coloring::new(Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5)),
        ..Default::default()
    }).id();

    spawn_fractal(commands.commands(), 5, 250.0, entity, texture.clone());
}

pub fn egui_window(mut ctx: EguiContexts,
    mut query: Query<&mut Transform2D, Without<Text>>,
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
