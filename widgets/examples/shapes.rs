use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, linebreak};
use bevy_prototype_lyon::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .add_plugins(ShapePlugin)
        .run();
}

#[derive(Debug, Component)]
pub struct Marker;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    shape! ((commands, assets) {
        shape: Shapes::Rectangle,
        stroke: (color!(purple), 2.8),
        fill: color!(black),
        dimension: [400.0, 400.0],
        child: shape! {
            shape: Shapes::Rectangle,
            anchor: TopLeft,
            stroke: (color!(blue), 1.4),
            fill: color!(orange),
            dimension: [120, 120],
        },
        child: shape! {
            shape: Shapes::RoundedRectangle(10.0),
            anchor: CenterRight,
            stroke: (color!(cyan), 1.4),
            fill: color!(darkgreen),
            dimension: [120, 120],
            extra: Marker,
        },
        child: shape! {
            shape: Shapes::Circle,
            anchor: BottomCenter,
            stroke: (color!(yellow), 1.4),
            dimension: [240, 120],
        },
        child: linebreak! {},
        child: linebreak! { 10 },
    });
}
