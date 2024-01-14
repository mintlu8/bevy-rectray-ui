use bevy::prelude::*;
use bevy_aoui::{AouiPlugin, linebreak, util::AouiCommands};
use bevy_aoui_shapes::{shape, Shapes, AouiShapesPlugin};
use bevy_prototype_lyon::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(ShapePlugin)
        .add_plugins(AouiPlugin)
        .add_plugins(AouiShapesPlugin)
        .run();
}

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    shape! (commands {
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
