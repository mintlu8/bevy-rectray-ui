//! Showcases support for dragging and interpolation.

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, sprite::{Material2dPlugin, Material2d}, render::render_resource::AsBindGroup};
use bevy_aoui::AoUIPlugin;

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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_plugins(Material2dPlugin::<Circle>::default())
        .run();
}

#[derive(Debug, Default, Clone, AsBindGroup, TypePath, Asset)]
pub struct Circle{
    #[uniform(0)]
    fill: Color,
    #[uniform(1)]
    stroke: Color,
}

impl Material2d for Circle {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "stroke_circle.wgsl".into()
    }
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    material_rect! ((commands, assets) {
        dimension: [100, 100],
        hitbox: Rect(1),
        material: Circle {
            fill: Color::RED,
            stroke: Color::BLACK
        },
        event: EventFlags::Hover|EventFlags::Drag,
        extra: DragBoth,
        extra: SetCursor { 
            flags: EventFlags::Hover|EventFlags::Drag, 
            icon: CursorIcon::Hand,
        },
        extra: DragSnapBack,
        extra: Interpolate::<Offset>::ease(EaseFunction::BounceOut, Vec2::ZERO, 4.0),
    });
}
