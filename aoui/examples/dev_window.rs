// This tries to be egui

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_aoui::{AoUIPlugin, widgets::drag::Draggable};

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
    let (send, recv) = signal();
    compact!((commands, assets) {
        direction: TopToBottom,
        hitbox: Rect(1),
        extra: Draggable::BOTH,
        extra: recv.mark::<SigDrag>(),
        child: rectangle! {
            z: -1,
            color: color!(darkblue),
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
            extra: send.mark::<SigDrag>(),
        },
        child: textbox! {
            text: "Checkbox",
            event: EventFlags::Drag,
        },
    });
}
