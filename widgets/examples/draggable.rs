use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::AoUIExtensionsPlugin;
use bevy_prototype_lyon::prelude::*;

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
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    sprite! ((commands, assets) {
        dimension: [100, 100],
        hitbox: Rect(1),
        //fill: color!(lavender),
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
