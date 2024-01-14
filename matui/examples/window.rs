use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, util::WorldExtension, util::AouiCommands};
use bevy_matui::{MatuiPlugin, mbutton, mtoggle, mframe, palette, mwindow, mslider, minput, mmenu, menu_items};
use bevy_aoui::layout::BoundsLayout;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                //resolution: WindowResolution::new(1600.0, 800.0).with_scale_factor_override(1.0),
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
        color: color!(black),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    let palette_idle = palette! {
        background: red500,
        foreground: white,
        stroke: none,
    };

    let palette_hover = palette! {
        background: red600,
        foreground: white,
        stroke: none,
    };

    let palette_pressed = palette! {
        background: red800,
        foreground: white,
        stroke: none,
    };

    let (collapse_send, collapse_recv, collapse_spin) = commands.signal();

    mwindow!(commands {
        radius: 5,
        palette: palette!(FramePalette {
            background: white,
            stroke: neutral400,
        }),
        margin: size2!(0, 0.2 em),
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
                text: "Click Me!",
                child: mframe! {
                    anchor: TopLeft,
                    palette: palette!(FramePalette {
                        background: white,
                    }),
                    radius: 5,
                    shadow: 5,
                    padding: 5,
                    extra: TrackCursor::DEFAULT,
                    extra: Detach,
                    z: 100,
                    layout: BoundsLayout::PADDING,
                    extra: DisplayIf(EventFlags::Hover|EventFlags::LeftPressed),
                    child: text! {
                        text: "Just please, click me, my friend!",
                        color: color!(darkblue),
                    }
                }
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
                dial_shadow: 2.0,
                palette: palette!(
                    background: red300,
                    foreground: red500,
                ),
                checked_palette: palette!(
                    background: red700,
                    foreground: red500,
                ),
            }
        },

        child: mslider! {
            range: (0..5),
            dial_shadow: 2.0,
            palette: palette!(
                background: grey,
                foreground: red500,
            ),
            hover_palette: palette!(
                background: grey,
                foreground: red600,
            ),
        },

        child: minput! {
            text: "Hello, World!",
            placeholder: "Say Hello:",
            width: 20,
            radius: 5,
            palette: palette_idle,
        },
        child: mmenu! {
            shadow: 5,
            radius: 10.0,
            padding: [0, 10],
            palette: palette!(
                background: white,
                stroke: green,
            ),
            hover_palette: palette!(
                background: white,
                stroke: green,
            ),
            items: menu_items!(
                "Hello", "Hi", |, "Good Bye"
            ),
        },
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
        palette: palette!(
            background: red300,
            stroke: red700,
            foreground: red500,
            on_foreground: white,
        ),
        checked_palette: palette!(
            background: red700,
            stroke: red700,
            foreground: white,
            on_foreground: red700,
        ),
        icon: "cross.png",
        shadow: 5,
        background_stroke: 2,
    });

    mtoggle!(commands{
        offset: [0, 200],
        background_size: 1.0,
        length: 2,
        dial_size: 1.6,
        palette: palette!(
            background: white,
            foreground: red500,
        ),
        checked_palette: palette!(
            background: red700,
            foreground: red500,
        ),
        dial_shadow: (4, color!(grey)),
    });
}
