use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, WorldExtension, dsl::AouiCommands};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .register_cursor_default(CursorIcon::Arrow)
        .add_plugins(AouiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .run();
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_channel(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    text! (commands {
        dimension: size2!(400, 1 em),
        font_size: em(4),
        text: "I'm Spinning!",
        font: "ComicNeue-Bold.ttf",
        color: color!(cyan),
        extra: transition!(
            Opacity 5 CubicOut init (0.0, 1.0);
            Offset 2 Linear loop (Vec2::new(-200.0, 0.0), Vec2::new(200.0, 0.0));
            Rotation 2 Linear repeat (0.0, 2.0 * PI);
            Color 2 Linear loop [cyan, blue];
        )
    });
}
