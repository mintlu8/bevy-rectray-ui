//! Showcases the features of a button widget.
#![recursion_limit = "256"]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_aoui::AouiPlugin;
use bevy_aoui::WorldExtension;
use bevy_aoui::dsl::AouiCommands;
use bevy_aoui::signals::Invoke;
use bevy_aoui::signals::ReceiveInvoke;

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
        .add_systems(Update, recv)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(AouiPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}

#[derive(Debug, Component)]
pub struct Listen(Invoke<Listen>);

impl ReceiveInvoke for Listen {
    type Type = ();
}

pub fn recv(query: Query<&Listen>) {
    for item in query.iter() {
        if item.0.poll_any() {
            println!("Signal is received!");
        }
    }
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
    

    let (send1, recv1) = commands.signal();
    let (send2, recv2) = commands.signal();

    vstack!(commands {
        offset: [0, 100],
        child: check_button! {
            dimension: size2!(14 em, 2 em),
            checked: true,
            on_change: send1,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
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
                text: "This is a checkbox",
            },
        },
        child: check_button! {
            dimension: size2!(14 em, 2 em),
            on_change: send2,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
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
                text: "This is also a checkbox",
            },
        }
    });

    text! (commands {
        offset: [300, 120],
        color: color!(gold),
        text: "<= true!",
        extra: recv1.recv0(|x: bool, text: &mut Text| format_widget!(text, "<= {}!", x))
    });
    text! (commands {
        offset: [300, 80],
        color: color!(gold),
        text: "<= false!",
        extra: recv2.recv0(|x: bool, text: &mut Text| format_widget!(text, "<= {}!", x))
    });
    
    let ctx = radio_button_group::<[_; 4]>("Fire");
    let sig = ctx[0].recv();
    let elements = ["Fire", "Water", "Earth", "Wind"];

    text! (commands {
        offset: [300, -100],
        color: color!(gold),
        text: "<= This reflects the value of the radio button.",
        extra: sig.recv0(|x: &str, text: &mut Text| format_widget!(text, "<= has value {}!", x))
    });

    vstack!(commands {
        offset: [0, -150],
        child: #radio_button! {
            dimension: size2!(14 em, 2 em),
            context: #ctx,
            value: #elements,
            child: sprite!{
                anchor: Left,
                dimension: size2!(2 em, 2 em),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
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
    });
    
    let (send, recv, recv2) = commands.signal();

    commands.spawn_bundle(Listen(recv2.invoke()));

    text! (commands {
        offset: [300, 0],
        color: color!(gold),
        text: "<= Click this button.",
        extra: recv.recv0(|text: &mut Text| format_widget!(text, "You clicked it!"))
    });

    button! (commands {
        dimension: size2!(12 em, 2 em),
        font_size: em(2),
        cursor: CursorIcon::Hand,
        child: rectangle!{
            dimension: size2!(100%, 100%),
            color: color!(blue500),
            extra: DisplayIf(EventFlags::Idle)
        },
        child: text!{
            text: "Click Me!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Idle),
            z: 0.1
        },
        child: rectangle!{
            dimension: size2!(100%, 100%),
            color: color!(blue800),
            extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed)
        },
        child: text!{
            text: "Hovering!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Hover),
            z: 0.1
        },
        child: text!{
            text: "Clicked!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::LeftPressed),
            z: 0.1
        },
        extra: Handlers::<EvLeftClick>::oneshot(&mut commands, ||println!("Clicked"))
            .and(send),
        extra: Handlers::<EvHover>::oneshot(&mut commands, ||println!("Hovering")),
    });
}
