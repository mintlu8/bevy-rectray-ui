//! This is in fact a show case for `Mutation` and not how you typically implement a counter.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{signal_ids, util::{WorldExtension, AouiCommands}, widgets::button::Payload, AouiPlugin};

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


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
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

    let (send, recv, recv2) = signal();

    signal_ids!(SigI32: i32);

    hstack!(commands {
        margin: size2!(0.5 em, 0.5 em),
        font_size: em(2),
        child: text! {
            text: "0",
            signal: receiver::<SigI32>(recv),
            system: |x: SigRecv<SigI32>, text: Ac<Text>| {
                let val = x.recv().await;
                text.set(move |text|format_widget!(text, "{}", val)).await?;
            }
        },
        child: button! {
            event: EventFlags::LeftClick,
            dimension: size2!(1 em, 1 em),
            payload: 1i32,
            on_click: send.type_erase(),
            child: rectangle! {
                dimension: Size2::FULL,
                color: color!(red),
            },
            signal: receiver::<SigI32>(recv2),
            system: |x: SigRecv<SigI32>, payload: Ac<Payload>| {
                x.recv().await;
                payload.set(|payload| payload.mut_dyn(|x: &i32| dbg!(*x + 1))).await?;
            }
        }
    });
}
