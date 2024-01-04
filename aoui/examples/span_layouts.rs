//! Demo for the span based layouts.

use bevy::{prelude::*, render::render_resource::AsBindGroup, sprite::{Material2d, Material2dPlugin}};
use bevy_aoui::{AouiPlugin, material_sprite};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .add_plugins(Material2dPlugin::<Circle>::default())
        .run();
}

#[derive(Debug, Default, Clone, AsBindGroup, TypePath, Asset)]
pub struct Circle{
    #[uniform(0)]
    fill: Color,
    #[uniform(1)]
    stroke: Color,
}

impl Material2d for Circle {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "stroke_circle.wgsl".into()
    }
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    let text = [ 
        "Layout Demo",
        "Insertion Order: light to dark.",
        "Anchor: circle.",
        "Red: HStack",
        "Yellow: HBox",
        "Green: VStack",
        "Blue: VBox",
    ];
    let colors = colors![
        white, white, white,
        red, yellow, green, blue,
    ];

    vstack! ((commands, assets) {
        anchor: Top,
        margin: 4,
        child: #text! { 
            anchor: Top,
            text: #text,
            color: #colors,
        },
    });

    let anchors = [
        TopLeft, TopCenter, TopRight,
        CenterLeft, Center, CenterRight,
        BottomLeft, BottomCenter, BottomRight,
    ];

    let dimensions = [
        [80, 50], [40, 90], [60, 30],
        [70, 50], [50, 60], [40, 90],
        [20, 20], [30, 70], [90, 30],
    ];

    let reds = colors! [
        red100, red200, red300,
        red400, red500, red600,
        red700, red800, red900
    ];

    let yellows = colors! [
        yellow100, yellow200, yellow300,
        yellow400, yellow500, yellow600,
        yellow700, yellow800, yellow900
    ];

    let greens = colors! [
        green100, green200, green300,
        green400, green500, green600,
        green700, green800, green900
    ];

    let blues = colors! [
        blue100, blue200, blue300,
        blue400, blue500, blue600,
        blue700, blue800, blue900
    ];

    hstack! ((commands, assets) {
        anchor: Left,
        dimension: [400, 100],
        offset: [20, 120],
        margin: [10, 10],
        z: 1,
        child: rectangle! {
            dimension: size2!(1 + [6, 6] px),
            color: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: #rectangle! {
            anchor: #anchors,
            dimension: #dimensions,
            color: #reds,
            child: material_sprite! {
                anchor: #anchors,
                dimension: [6, 6],
                material: Circle {
                    fill: Color::WHITE,
                    stroke: Color::BLACK,
                }
            }
        },
    });

    hbox! ((commands, assets) {
        anchor: Left,
        dimension: [600, 100],
        offset: [20, 0],
        margin: [10, 10],
        z: 2,
        child: rectangle! {
            dimension: size2!(1 + [6, 6] px),
            color: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: #rectangle! {
            anchor: #anchors,
            dimension: #dimensions,
            color: #yellows,
            child: material_sprite! {
                anchor: #anchors,
                dimension: [6, 6],
                material: Circle {
                    fill: Color::WHITE,
                    stroke: Color::BLACK,
                }
            }
        },
    });

    vstack! ((commands, assets) {
        anchor: Right,
        dimension: [100, 700],
        offset: [-200, 0],
        margin: [10, 10],
        z: 3,
        child: rectangle! {
            dimension: size2!(1 + [6, 6] px),
            color: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: #rectangle! {
            anchor: #anchors,
            dimension: #dimensions,
            color: #greens,
            child: material_sprite! {
                anchor: #anchors,
                dimension: [6, 6],
                material: Circle {
                    fill: Color::WHITE,
                    stroke: Color::BLACK,
                }
            }
        },
    });

    vbox! ((commands, assets) {
        anchor: Right,
        dimension: [100, 700],
        offset: [-20, 0],
        margin: [10, 10],
        z: 4,
        child: rectangle! {
            dimension: size2!(1 + [6, 6] px),
            color: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: #rectangle! {
            anchor: #anchors,
            dimension: #dimensions,
            color: #blues,
            child: material_sprite! {
                anchor: #anchors,
                dimension: [6, 6],
                material: Circle {
                    fill: Color::WHITE,
                    stroke: Color::BLACK,
                }
            }
        },
    });
}
