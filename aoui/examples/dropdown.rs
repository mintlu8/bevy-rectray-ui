#![recursion_limit = "256"]
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::AouiPlugin;
use bevy_aoui::dsl::AouiCommands;

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
    
    let (send, recv_rot, fold_recv) = commands.signal();

    let elements = [
        "Water", "Earth", "Fire", "Air"
    ];

    let text_ctx = radio_button_group::<[_; 4]>("");
    let text_recv = text_ctx[0].recv();
    frame!(commands{
        dimension: size2!(22 em, 2 em),
        child: check_button! {
            dimension: Size2::FULL,
            on_change: send,
            child: hbox! {
                dimension: size2!(22 em, 2 em),
                font_size: em(2),
                child: text! {
                    anchor: Left,
                    text: "Selected Element:",
                    font: "ComicNeue-Bold.ttf",
                },
                child: text! {
                    anchor: Left,
                    text: "",
                    font: "ComicNeue-Bold.ttf",
                    extra: text_recv.recv(|x: &str, text: &mut Text| format_widget!(text, "{}", x))
                },
            },
            child: text! {
                font_size: em(2),
                anchor: Right,
                center: Center,
                rotation: degrees(90),
                text: "v",
                extra: recv_rot.recv(|x: bool, rot: &mut Interpolate<Rotation>| if x {
                    rot.interpolate_to(0.0)
                } else {
                    rot.interpolate_to(PI / 2.0)
                }),
                extra: transition! (Rotation 0.5 CubicInOut default PI/2.0)
            },
        },
        child: scrolling! {
            anchor: TopRight,
            parent_anchor: BottomRight,
            layer: 1,
            scroll: Scrolling::Y,
            clipping: false,
            extra: fold_recv.recv(|x: bool, op: &mut Interpolate<Opacity>| if x {
                op.interpolate_to(1.0)
            } else {
                op.interpolate_to(0.0)
            }),
            extra: transition! (Opacity 0.5 Linear default 0.0),
            dimension: size2!(14 em, 4 em),
            child: vstack! {
                anchor: Top,
                child: #radio_button! {
                    dimension: size2!(14 em, 2 em),
                    context: #text_ctx,
                    value: #elements,
                    child: sprite!{
                        anchor: Left,
                        dimension: size2!(2 em, 2 em),
                        sprite: "radio.png",
                        extra: DisplayIf(CheckButtonState::Checked),
                    },
                    child: sprite!{
                        anchor: Left,
                        dimension: size2!(2 em, 2 em),
                        sprite: "unchecked.png",
                        extra: DisplayIf(CheckButtonState::Unchecked)
                    },
                    child: text!{
                        anchor: Left,
                        offset: size2!(2.5 em, 0),
                        text: #elements,
                    },
                },
            },
        }
    });
}
