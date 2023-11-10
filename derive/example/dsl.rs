use bevy_aoui::*;
use bevy::prelude::*;

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
    commands.spawn(Camera2dBundle::default());
    bevy_aoui::sprite!(commands, {
        sprite: assets.load("square.png"),
        center: [0.0, 0.0],
        flex: HBox,
        dimension: [500 px, 200 px],
        child: {
            sprite: assets.load("square.png"),
            color: [1.0, 0.1, 0.1, 1.0],
            dimension: [50, 30],
            anchor: CenterLeft
        },
        child: {
            sprite: assets.load("square.png"),
            color: [1.0, 0.1, 0.1, 1.0],
            dimension: [30, 50],
            anchor: CenterLeft
        },
        child:  {
            sprite: assets.load("square.png"),
            color: [0.0, 1.0, 1.0, 1.0],
            dimension: [30, 50],
            anchor: TopRight
        },
        child:  {
            sprite: assets.load("square.png"),
            color: [0.0, 1.0, 1.0, 1.0],
            dimension: [30, 50],
            anchor: CenterRight
        },
        child: {
            text: "Hello World",
            font: Default::default(),
            font_size: 32,
            color: Purple,
            anchor: CenterRight,
        }
    });
}
