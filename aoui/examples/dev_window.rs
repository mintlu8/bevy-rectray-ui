// This tries to be egui

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, widgets::drag::Dragging, WorldExtension};

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
        .add_plugins(AouiPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    let (send, recv) = signal();
    vstack!((commands, assets) {
        hitbox: Rect(1),
        extra: Dragging::BOTH,
        extra: recv.invoke::<Dragging>(),
        child: rectangle! {
            z: -1,
            color: color!(darkblue),
            dimension: size2!(1 + [5, 5] px),
            extra: IgnoreLayout,
        },
        child: text! {
            text: "Egui? Just kidding!",
            event: EventFlags::LeftDrag | EventFlags::Hover,
            extra: SetCursor { 
                flags: EventFlags::Hover|EventFlags::LeftDrag, 
                icon: CursorIcon::Hand,
            },
            extra: Handlers::<EvMouseDrag>::new(send),
        },
        child: text! {
            text: "Checkbox",
            event: EventFlags::LeftDrag,
        },
    });
}
