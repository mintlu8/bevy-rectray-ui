#![recursion_limit = "256"]
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rectray::RectrayPlugin;
use bevy_rectray::util::RCommands;

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
    
    let (send, rot_recv, fold_recv) = signal();

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
                    signal: receiver::<FormatTextStatic>(text_recv),
                    system: |x: Receiver<FormatTextStatic>, text: Ac<Text>| {
                        let t = x.recv().await;
                        text.set(move |w| format_widget!(w, "{t}")).await?;
                    }
                },
            },
            child: text! {
                font_size: em(2),
                anchor: Right,
                center: Center,
                rotation: degrees(90),
                text: "v",
                signal: receiver::<ToggleChange>(rot_recv),
                system: |x: Receiver<ToggleChange>, text: Ac<Interpolate<Rotation>>| {
                    let b = x.recv().await;
                    text.set(move |rot| rot.interpolate_to(if b {0.0} else {PI / 2.0})).await?;
                },
                extra: transition! (Rotation 0.5 CubicInOut default PI/2.0)
            },
        },
        child: frame! {
            anchor: TopRight,
            parent_anchor: BottomRight,
            layer: 1,
            clipping: false,
            signal: receiver::<ToggleChange>(fold_recv),
            system: |x: Receiver<ToggleChange>, text: Ac<Interpolate<Opacity>>| {
                let b = x.recv().await;
                text.set(move |rot| rot.interpolate_to(if b {1.0} else {0.0})).await?;
            },
            extra: transition! (Opacity 0.5 Linear default 0.0),
            dimension: size2!(14 em, 4 em),
            child: vstack! {
                anchor: Top,
                extra: Scrolling::Y,
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
