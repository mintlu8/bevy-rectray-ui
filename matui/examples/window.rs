#![recursion_limit = "256"]

use bevy::log::LogPlugin;
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, util::WorldExtension, util::AouiCommands};
use bevy_matui::{MatuiPlugin, mbutton, mtoggle, mframe, palette, mwindow, mslider, minput, mmenu, menu_items, mspinner, mdropdown};
use bevy_aoui::layout::BoundsLayout;
use bevy_matui::widgets::states::ToggleRotation;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                //resolution: WindowResolution::new(1600.0, 800.0).with_scale_factor_override(1.0),
                ..Default::default()
            }),
            ..Default::default()
        }).set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_systems(PostStartup, |w: &mut World| {dbg!(w.archetypes().iter().filter(|x| x.entities().len() > 0).count());})

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
        system: |fps: Fps, text: Ac<Text>| {
            let fps = fps.get().await;
            text.set(move |text| format_widget!(text, "FPS: {:.2}", fps)).await?;
        }
    });

    let palette_idle = palette! {
        background: red500,
        background_lite: red600,
        foreground: white,
        stroke: white,
        stroke_lite: neutral100,
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

    let (collapse_send, collapse_recv, collapse_spin) = signal();
    mwindow!(commands {
        radius: 5,
        palette: palette! {
            background: white,
            stroke: neutral400,
            stroke_lite: red,
        },
        margin: size2!(0, 0.2 em),
        padding: size2!(1 em, 1 em),
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
                child: sprite! {
                    dimension: size2!(1 em, 1 em),
                    sprite: "tri.png",
                    color: color!(black),
                    extra: transition!(Rotation 0.2 Linear default 0.0),
                    signal: receiver::<ToggleChange>(collapse_spin),
                    system: |sig: SigRecv<ToggleChange>, inter: Ac<Interpolate<Rotation>>|{
                        if sig.recv().await {
                            inter.interpolate_to(0.0).await?;
                        } else {
                            inter.interpolate_to(PI).await?;
                        }
                    }
                }
            }
        },
        child: text! {
            text: "Title!",
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
            cancel: button! {
                z: 1,
                dimension: size2!(1.2 em, 1.2 em),
                anchor: Right,
                offset: size2!(-1 em, 0),
                extra: transition!(Opacity 0.2 Linear default 1.0),
                child: sprite! {
                    dimension: Size2::FULL,
                    rotation: degrees(45),
                    sprite: "plus.png",
                }
            }
        },
        child: {
            let (callback_send, callback_recv) = signal();

            mdropdown!(commands {
                placeholder: "Favorite Language:",
                width: 20,
                radius: 5,
                palette: palette_idle,
                callback: callback_recv,
                dial: check_button! {
                    dimension: size2!(1.2 em, 1.2 em),
                    anchor: CenterRight,
                    offset: size2!(-1 em, 0),
                    child: sprite! {
                        sprite: "tri.png",
                        dimension: size2!(full),
                        extra: transition!(Rotation 0.2 Linear default 0.0),
                        extra: ToggleRotation::new(0.0, PI),
                    },
                },
                cancel: button! {
                    z: 1,
                    dimension: size2!(1 em, 1 em),
                    offset: size2!(-2.4 em, -0.4 em),
                    anchor: Right,
                    extra: transition!(Opacity 0.2 Linear default 1.0),
                    child: sprite! {
                        dimension: Size2::FULL,
                        rotation: degrees(45),
                        sprite: "plus.png",
                    }
                },
                menu: mmenu! {
                    parent_anchor: BottomRight,
                    anchor: TopRight,
                    opacity: 0.0,
                    z: 1,
                    shadow: 5,
                    radius: 5,
                    padding: [0, 10],
                    callback: callback_send,
                    palette: palette_idle,
                    items: menu_items! {
                        "Rust", "Go", "Python", "Zig", |, 
                        "C" { "C", "C++", "C#", "Carbon" }, 
                        "Java" { "Java", "JavaScript" },
                    },
                }
            }
        )},
        child: hstack! {
            margin: size2!(1 em, 0),
            child: text! {
                color: color!(black),
                text: "Crates:",
            },
            child: mspinner!{
                capsule: true,
                palette: palette_idle,
                palette_focus: palette_pressed,
                decrement_icon: "left.png",
                increment_icon: "right.png",
                content: ["serde", "tokio", "bevy", "actix"],
                button_hitbox: Hitbox::rect(2.0),
            },
            child: mspinner!{
                capsule: true,
                axis: bevy_aoui::layout::Axis::Vertical,
                radius: 5,
                width: 1,
                palette: palette_idle,
                palette_focus: palette_pressed,
                decrement_icon: "left.png",
                increment_icon: "right.png",
                content: 0..=9,
                button_hitbox: Hitbox::rect(2.0),
            },
        },
    });

    mbutton!(commands{
        dimension: size2![100, 100],
        shadow: 5,
        capsule: true,
        palette: palette_idle,
        palette_hover: palette_hover,
        palette_pressed: palette_pressed,
        icon: "plus.png",
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
        icon: "plus.png",
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
