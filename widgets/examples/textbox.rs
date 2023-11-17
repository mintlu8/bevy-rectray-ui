use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{widgets::{inputbox::{*}, TextColor}, events::EventFlags, AoUIExtensionsPlugin, handler};
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
        hitbox: Rect([1, 40]),
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
        extra: handler!{mouse(DRAG_END) => fn drag_end() {
            println!("Drag end")
        }},
    });
}

#[allow(unused)]
pub fn equiv(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    frame! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        font_size: em(4),
        hitbox: Rect(1),
        extra: InputBox::new("Hello, World!"),
        extra: EventFlags::DOUBLE_CLICK|EventFlags::DRAG|EventFlags::CLICK_OUTSIDE,
        extra: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: TextColor(color!(red)),
        child: frame! {
            dimension: size2!([100%, 100%]),
            anchor: Left,
            extra: InputBoxText,
        },
        child: shape! {
            shape: Shapes::Rectangle,
            fill: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
            extra: InputBoxCursorBar,
        },
        child: shape! {
            shape: Shapes::Rectangle,
            fill: color!(green) * 0.5,
            dimension: size2!([12, 1 em]),
            extra: InputBoxCursorArea,
        },
    });
}
