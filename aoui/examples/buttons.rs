//! Showcases the features of a button widget.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui::WorldExtension;

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
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(AoUIPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    textbox!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: sig_fps().mark::<SigText>().map(|x: f32| format!("FPS: {:.2}", x))
    });
    

    let (send1, recv1) = signal();
    let (send2, recv2) = signal();

    vbox!((commands, assets) {
        offset: [0, 100],
        child: check_button! {
            dimension: size2!([14 em, 2 em]),
            checked: true,
            change: send1,
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "This is a checkbox",
            },
        },
        child: check_button! {
            dimension: size2!([14 em, 2 em]),
            change: send2,
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "check.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "This is also a checkbox",
            },
        }
    });

    textbox! (commands {
        offset: [300, 120],
        color: color!(gold),
        text: "<= true!",
        extra: recv1.mark::<SigText>().map(|x: bool| format!("<= {}!", x))
    });
    textbox! (commands {
        offset: [300, 80],
        color: color!(gold),
        text: "<= false!",
        extra: recv2.mark::<SigText>().map(|x: bool| format!("<= {}!", x))
    });


    
    let ((fire, water, earth, air), sig) = radio_button_group("Fire");


    textbox! (commands {
        offset: [300, -100],
        color: color!(gold),
        text: "<= This reflects the value of the radio button.",
        extra: sig.mark::<SigText>().map(|x: &str| format!("<= has value {}!", x))
    });

    vbox!((commands, assets) {
        offset: [0, -150],
        child: radio_button! {
            dimension: size2!([14 em, 2 em]),
            context: fire,
            value: "Fire",
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "Fire",
            },
        },
        child: radio_button! {
            dimension: size2!([14 em, 2 em]),
            context: water,
            value: "Water",
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "Water",
            },
        },
        child: radio_button! {
            dimension: size2!([14 em, 2 em]),
            context: earth,
            value: "Earth",
            cursor: CursorIcon::Hand,
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "Earth",
            },
        },
        child: radio_button! {
            dimension: size2!([14 em, 2 em]),
            context: air,
            value: "Air",
            cursor: CursorIcon::Hand,
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "radio.png",
                extra: DisplayIf(CheckButtonState::Checked)
            },
            child: sprite!{
                anchor: Left,
                dimension: size2!([2 em, 2 em]),
                sprite: "unchecked.png",
                extra: DisplayIf(CheckButtonState::Unchecked)
            },
            child: textbox!{
                anchor: Left,
                offset: size2!([2.5 em, 0]),
                text: "Air",
            },
        },
    });
    
    let (send, recv) = signal();

    textbox! (commands {
        offset: [300, 0],
        color: color!(gold),
        text: "<= Click that button.",
        extra: recv.mark::<SigText>().map(|_: ()| format!("<= You clicked it!"))
    });


    button! ((commands, assets) {
        dimension: size2!([12 em, 2 em]),
        font_size: em(2),
        cursor: CursorIcon::Hand,
        child: rectangle!{
            dimension: size2!([100%, 100%]),
            color: color!(blue500),
            extra: DisplayIf(EventFlags::Idle)
        },
        child: textbox!{
            text: "Click Me!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Idle),
            z: 0.1
        },
        child: rectangle!{
            dimension: size2!([100%, 100%]),
            color: color!(blue800),
            extra: DisplayIf(EventFlags::Hover|EventFlags::Pressed)
        },
        child: textbox!{
            text: "Hovering!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Hover),
            z: 0.1
        },
        child: textbox!{
            text: "Clicked!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Pressed),
            z: 0.1
        },
        extra: handler!{LeftClick => {
            fn (){ println!("Clicked"); },
            send,
        }},
    });
}
