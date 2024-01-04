use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, WorldExtension};
use bevy_matui::{MatuiPlugin, mbutton, mtoggle, widgets::{util::WidgetPalette, toggle::TogglePalette, frame::WindowPalette}, palette, mwindow};

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

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    
    mwindow!((commands, assets) {
        radius: 5,
        palette: palette!(WindowPalette { 
            background: white,
            banner: purple, 
        }),
        margin: 0.5,
        z: 100,
        padding: 1,
        shadow: 12,
        banner: padding!{
            padding: size2!(2 em, 0.5 em),
            child: text! {
                text: "Hello, World!",
                color: color!(black),
            }
        },
        child: text! {
            text: "Inner!",
            color: color!(black),
        },
    });

    mbutton!((commands, assets){
        dimension: size2![100, 100],
        shadow: 5,
        capsule: true,
        palette: WidgetPalette { 
            background: color!(red500), 
            foreground: color!(white), 
            stroke: color!(none),
        },
        palette_hover: WidgetPalette { 
            background: color!(red600), 
            foreground: color!(white), 
            stroke: color!(none),
        },
        palette_pressed: WidgetPalette { 
            background: color!(red800), 
            foreground: color!(white), 
            stroke: color!(none),
        },
        //capsule: true,
        icon: "cross.png",
        text: "Hello",
    });

    mtoggle!((commands, assets){
        offset: [0, 100],
        palette: palette!(TogglePalette {
            background: white, 
            dial: red500, 
            background_stroke: red700, 
            icon: white,
        }),
        checked_palette: palette!(TogglePalette {
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

    mtoggle!((commands, assets){
        offset: [0, 200],
        background_size: 1.0,
        length: 2,
        dial_size: 1.6,
        palette: palette!(TogglePalette {
            background: white, 
            dial: red500, 
        }),
        checked_palette: palette!(TogglePalette {
            background: red700, 
            dial: red500, 
        }),
        dial_shadow: (4, color!(grey)),
    });
}