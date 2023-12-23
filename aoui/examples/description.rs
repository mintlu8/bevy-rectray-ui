/// A simple example of rendering description using the paragraph layout.

use bevy::prelude::*;
use bevy_aoui::AouiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    paragraph!((commands, assets) {
        dimension: [400, 700],
        child: rectangle! {
            color: color!(neutral800),
            dimension: Size2::FULL,
            extra: IgnoreLayout,
        },
        child: text! {
            text: "Thunderbolt",
            color: color!(white),
            anchor: TopLeft,
        },
        child: text! {
            text: "Special Attack",
            color: color!(white),
            anchor: TopRight,
        },
        child: linebreak! {},
        child: sprite! {
            sprite: "electric_type.png",
            dimension: size2!(0.8 em, 0.8 em),
            color: color!(white),
            anchor: TopLeft,
        },
        child: text! {
            text: "Electric Type",
            color: color!(white),
            anchor: TopLeft,
            font_size: em(0.8),
        },
        child: text! {
            text: "90 bp",
            color: color!(white),
            anchor: TopRight,
            font_size: em(0.8),
        },
        child: linebreak! {},
        child: text! {
            text: "The user attacks the target with a strong electric blast. This may also leave the target with paralysis.",
            color: color!(white),
            anchor: TopLeft,
            bounds: [399, 9999],
            wrap: true
        },
    });
}
