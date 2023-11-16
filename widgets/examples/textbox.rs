use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIWidgetsPlugin, widgets::inputbox::{*}, events::EventFlags};
use bevy_prototype_lyon::prelude::*;


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIWidgetsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    frame! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        em: em(4),
        hitbox: Hitbox::Rect(1),
        extra: InputBox::new("Hello, World!"),
        extra: EventFlags::DOUBLE_CLICK|EventFlags::DRAG|EventFlags::CLICK_OUTSIDE,
        extra: assets.load::<Font>("RobotoCondensed.ttf"),
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
