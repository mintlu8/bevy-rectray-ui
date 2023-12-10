//! Showcases support for dragging and interpolation.

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
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
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    sprite! (commands {
        dimension: [100, 100],
        hitbox: Rect(1),
        sprite: assets.load("square.png"),
        extra: EventFlags::Hover|EventFlags::Drag,
        extra: DragBoth,
        extra: SetCursor { 
            flags: EventFlags::Hover|EventFlags::Drag, 
            icon: CursorIcon::Hand,
        },
        extra: DragSnapBack,
        extra: Interpolate::<Offset>::ease(EaseFunction::BounceOut, Vec2::ZERO, 4.0),
    });
}
