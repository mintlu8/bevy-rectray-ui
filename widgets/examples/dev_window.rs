// This tries to be egui

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, widgets::drag::Draggable};
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
    let (send, recv) = signal();
    compact!((commands, assets) {
        direction: TopToBottom,
        hitbox: Rect(1),
        extra: Draggable::BOTH,
        extra: recv.mark::<DragSignal>(),
        child: shape! {
            shape: Shapes::RoundedRectangle(5.0),
            z: -1,
            fill: color!(darkblue),
            dimension: size2!(1 + [5, 5] px),
            extra: IgnoreLayout,
        },
        child: textbox! {
            text: "Egui? Just kidding!",
            event: EventFlags::Drag,
            extra: SetCursor { 
                flags: EventFlags::Hover|EventFlags::Drag, 
                icon: CursorIcon::Hand,
            },
            extra: send.mark::<DragSignal>(),
        },
        child: textbox! {
            text: "Checkbox",
            event: EventFlags::Drag,
        },
    });
}
