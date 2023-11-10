use bevy_aoui::*;
use bevy::{prelude::*, sprite::{Anchor, Mesh2dHandle}};
use bevy_prototype_lyon::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .add_plugins(ShapePlugin)
        .run();
}

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        AoUIBundle {
            dimension: Dimension::pixels(Vec2::new(200.0, 200.0)),
            transform: Transform2D::UNIT.with_scale(Vec2::new(100.0, 100.0)),
            ..Default::default()
        }, 
        BuildGlobalBundle::default(),
        GeometryBuilder::build_as(&shapes::RegularPolygon {
            sides: 6,
            ..Default::default()
        }),
        Fill::color(Color::CYAN),
        Stroke::new(Color::NAVY, 0.05),
        Mesh2dHandle::default(),
        Handle::<ColorMaterial>::default(),
    ));

    commands.spawn((
        AoUIBundle {
            dimension: Dimension::pixels(Vec2::new(2.0, 2.0)),
            transform: Transform2D::UNIT
                .with_anchor(Anchor::CenterLeft)
                .with_scale(Vec2::new(100.0, 100.0))
                .with_z(1.0),
            ..Default::default()
        }, 
        BuildGlobalBundle::at_anchor(Anchor::Center),
        GeometryBuilder::build_as(&shapes::RegularPolygon {
            sides: 3,
            ..Default::default()
        }),
        Fill::color(Color::YELLOW),
        Stroke::new(Color::GRAY, 0.05),
        Mesh2dHandle::default(),
        Handle::<ColorMaterial>::default(),
    ));
}
