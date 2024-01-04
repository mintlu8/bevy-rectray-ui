//! This is in fact a show case for `Mutation` and not how you typically implement a counter.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, widgets::button::Payload, WorldExtension};

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
        // classic macos stuff
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

    let (sender, receiver) = signal();

    hbox!((commands, assets) {
        margin: size2!(0.5 em, 0.5 em),
        font_size: em(2),
        child: text! {
            text: "0",
            extra: receiver.recv0(|x: i32, text: &mut Text| format_widget!(text, "{}", x))
        },
        child: button! {
            event: EventFlags::LeftClick,
            dimension: size2!(1 em, 1 em),
            payload: 1i32,
            on_click: Handlers::new(
                Mutation::dynamic::<i32, _, _>(|payload: i32, x: &mut Payload| x.mut_dyn(|_: &i32| payload + 1))
            ).and(sender.type_erase()),
            child: rectangle! {
                dimension: Size2::FULL,
                color: color!(red),
            }
        }
    });
}
