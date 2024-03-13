//! This showcases discrete scrolling.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rectray::{RectrayPlugin, widgets::scroll::ScrollDiscrete, util::RCommands};

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
        .add_plugins(RectrayPlugin)
        // classic macos stuff
        .run();
}


pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
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

    let s = "abcdefghijklmnopqrstuvwxyz".chars();

    vstack! (commands {
        dimension: [200, 60],
        hitbox: Hitbox::rect(1),
        event: EventFlags::MouseWheel,
        children_range: 0..5,
        font_size: em(4),
        extra: ScrollDiscrete::new(),
        child: #text! {
            text: #s
        }
    });
}
