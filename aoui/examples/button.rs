//! Showcases the features of a button widget.

use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui::button;

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
        .add_plugins(AoUIPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    button! ((commands, assets) {
        dimension: size2!([12 em, 2 em]),
        font_size: em(2),
        hitbox: Rect(1),
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
            extra: DisplayIf(EventFlags::Hover)
        },
        child: textbox!{
            text: "Hovering!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Hover),
            z: 0.1
        },
        child: rectangle!{
            dimension: size2!([100%, 100%]),
            color: color!(blue400),
            extra: DisplayIf(EventFlags::Pressed)
        },
        child: textbox!{
            text: "Clicked!",
            color: color!(gold),
            extra: DisplayIf(EventFlags::Pressed),
            z: 0.1
        },
        extra: handler!{LeftPressed => fn name() {
            println!("LMB Down")
        }},
        extra: handler!{LeftClick => fn name() {
            println!("Clicked")
        }},
        extra: handler!{Hover => fn name() {
            println!("Hovering")
        }},
    });
}
