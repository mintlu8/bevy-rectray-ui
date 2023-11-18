use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{events::DoubleClick, AoUIExtensionsPlugin, handler, Submit, widgets::CursorDefault};
use bevy_prototype_lyon::prelude::*;


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
        .init_resource::<CursorDefault>()
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        font_size: em(4),
        hitbox: Rect(1),
        text: "Hello, World!",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: shape! {
            shape: Shapes::Rectangle,
            fill: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: shape! {
            shape: Shapes::Rectangle,
            fill: color!(green) * 0.5,
            z: -0.1,
            dimension: size2!([12, 1 em]),
        },
        extra: handler!{LeftDown => fn name() {
            println!("LMB Down")
        }},
        extra: handler!{MidDown => fn name() {
            println!("MMB Down")
        }},
        extra: handler!{RightDown => fn name() {
            println!("RMB Down")
        }},
        extra: handler!{DragEnd => fn name() {
            println!("Drag End")
        }},
        extra: handler!{DoubleClick => fn name() {
            println!("Double Click")
        }},
        extra: handler!{ClickOutside => fn name() {
            println!("Clicked Outside")
        }},
        extra: handler!{OnSubmit => fn name(submit: Query<&Submit>) {
            match submit.get_single(){
                Ok(s) => {
                    let s: &str = s.get();
                    println!("Submit: {}", s)
                },
                Err(_) => print!("Submit: Value missing"),
            };
        }},
    });
}
