// This tries to be egui

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_aoui::{AouiPlugin, widgets::drag::Draggable, WorldExtension};

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
        .add_plugins(AouiPlugin)
        .register_cursor_default(CursorIcon::Hand)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!((commands, assets) {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });

    let (send, recv) = signal();
    compact!((commands, assets) {
        direction: TopToBottom,
        hitbox: Rect(1),
        extra: Draggable::BOTH,
        extra: recv.build::<SigDrag>(),
        child: rectangle! {
            z: -1,
            color: color!(darkblue),
            dimension: size2!(1 + [5, 5] px),
            extra: IgnoreLayout,
        },
        child: text! {
            text: "Egui? Just kidding!",
            event: EventFlags::LeftDrag,
            extra: SetCursor { 
                flags: EventFlags::Hover|EventFlags::LeftDrag, 
                icon: CursorIcon::Hand,
            },
            extra: handler!{EvMouseDrag => {send}},
        },
        child: text! {
            text: "Checkbox",
            event: EventFlags::LeftDrag,
        },
    });
}
