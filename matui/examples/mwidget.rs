use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, WorldExtension, dsl::AouiCommands};
use bevy_matui::{MatuiPlugin, mbutton, mtoggle, widgets::{util::WidgetPalette, toggle::DialPalette, frame::WindowPalette}, palette, mwindow, mslider};

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
        .add_plugins(MatuiPlugin)
        .insert_resource(ClearColor(Color::WHITE))
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

    let palette_idle = WidgetPalette { 
        background: color!(red500), 
        foreground: color!(white), 
        stroke: color!(none),
    };

    let palette_hover = WidgetPalette { 
        background: color!(red600), 
        foreground: color!(white), 
        stroke: color!(none),
    };

    let palette_pressed = WidgetPalette { 
        background: color!(red800), 
        foreground: color!(white), 
        stroke: color!(none),
    };

    let (collapse_send, collapse_recv, collapse_spin) = commands.signal();

    mwindow!(commands {
        radius: 5,
        palette: palette!(WindowPalette { 
            background: white,
            banner: purple, 
        }),
        margin: size2!(0, 0.5 em),
        z: 40,
        shadow: 12,
        collapse: collapse_recv,
        banner: hbox! {
            dimension: size2!(100%, 2 em),
            margin: size2!(1 em, 0),
            child: text! {
                text: "Hello, World!",
                color: color!(black),
            },
            child: check_button! {
                anchor: Right,
                offset: size2!(-1 em, 0),
                dimension: size2!(1 em, 1 em),                    
                checked: true,
                on_change: collapse_send,
                child: text! {
                    text: "v",
                    color: color!(black),
                    extra: transition!(Rotation 0.2 Linear default 0.0),
                    extra: collapse_spin.recv_select(true,
                        Interpolate::<Rotation>::signal_to(0.0),
                        Interpolate::<Rotation>::signal_to(PI),
                    )
                }
            }
        },
        child: text! {
            text: "Inner!",
            color: color!(black),
        },
        child: hstack! {
            margin: size2!(1 em, 0),
            child: text! {
                color: color!(black),
                text: "Button:",
            },
            child: mbutton!{
                capsule: true,
                palette: palette_idle,
                palette_hover: palette_hover,
                palette_pressed: palette_pressed,
                text: "Click Me!"
            },
        },
        child: hstack! {
            margin: size2!(1 em, 0),
            child: text! {
                color: color!(black),
                text: "Button:",
            },
            child: mtoggle! {
                background_size: 1.0,
                length: 2,
                dial_size: 1.6,
                palette: palette!(DialPalette {
                    background: white, 
                    dial: red500, 
                }),
                checked_palette: palette!(DialPalette {
                    background: red700, 
                    dial: red500, 
                }),
                dial_shadow: (4, color!(grey)),
            }
        },

        child: mslider! {
            range: (0..5),
            palette: palette!(DialPalette {
                background: grey, 
                dial: red500, 
            })
        }
    });

    mbutton!(commands{
        dimension: size2![100, 100],
        shadow: 5,
        capsule: true,
        palette: palette_idle,
        palette_hover: palette_hover,
        palette_pressed: palette_pressed,
        icon: "cross.png",
        text: "Hello",
    });

    mtoggle!(commands{
        offset: [0, 100],
        palette: palette!(DialPalette {
            background: white, 
            dial: red500, 
            background_stroke: red700, 
            icon: white,
        }),
        checked_palette: palette!(DialPalette {
            background: red700, 
            dial: white, 
            background_stroke: red700,
            icon: red700,
        }),
        icon: "cross.png",
        shadow: 5,
        background_stroke: 2,
        //background_stroke: (color!(darkred), 3),
    });

    mtoggle!(commands{
        offset: [0, 200],
        background_size: 1.0,
        length: 2,
        dial_size: 1.6,
        palette: palette!(DialPalette {
            background: white, 
            dial: red500, 
        }),
        checked_palette: palette!(DialPalette {
            background: red700, 
            dial: red500, 
        }),
        dial_shadow: (4, color!(grey)),
    });

}