//! Demo for the span based layouts.

use bevy::{prelude::*, render::render_resource::{AsBindGroup, PrimitiveTopology}, sprite::{Material2d, Material2dPlugin}};
use bevy_aoui::{AoUIPlugin, bundles::AoUIMaterialMesh2dBundle, Dimension, Anchor, Transform2D, dsl::DslInto};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
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

fn anchor_circle(commands: &mut Commands, assets: &Res<AssetServer>, anchor: impl DslInto<Anchor>) -> Entity{
    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, 
            vec![[-4.0, -4.0, 0.0], [4.0, -4.0, 0.0], [-4.0, 4.0, 0.0], [4.0, 4.0, 0.0]]
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, 
            vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, 
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
        )
        .with_indices(Some(bevy::render::mesh::Indices::U32(vec![
            0, 1, 2,
            1, 2, 3
        ])));
    let mesh_handle = bevy::sprite::Mesh2dHandle(assets.add(mesh));
    let material = assets.add(Circle {
        fill: Color::WHITE,
        stroke: Color::BLACK,
    });
    commands.spawn(AoUIMaterialMesh2dBundle {
        transform: Transform2D::UNIT.with_anchor(anchor.dinto()),
        dimension: Dimension::pixels(Vec2::new(4.0, 4.0)),
        mesh: mesh_handle,
        material,
        ..Default::default()
    }).id()
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    vbox! (commands {
        anchor: Top,
        margin: 4,
        child: textbox! { 
            anchor: Top,
            text: "Layout Demo",
        },
        child: textbox! { 
            anchor: Top,
            text: "Insertion Order: light to dark." 
        },
        child: textbox! { 
            anchor: Top,
            text: "Anchor: circle." 
        },
        child: textbox! { 
            anchor: Top,
            color: color!(red),
            text: "Red: Compact HBox" 
        },
        child: textbox! { 
            anchor: Top,
            color: color!(yellow),
            text: "Yellow: HSpan" 
        },
        child: textbox! { 
            anchor: Top,
            color: color!(green),
            text: "green: Compact VBox" 
        },
        child: textbox! { 
            anchor: Top,
            color: color!(blue),
            text: "blue: VSpan" 
        },
    });
    hbox! ((commands, assets) {
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
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            color: color!(red100),
            child: anchor_circle(&mut commands, &assets, TopLeft),
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            color: color!(red200),
            child: anchor_circle(&mut commands, &assets, TopCenter)
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            color: color!(red300),
            child: anchor_circle(&mut commands, &assets, TopRight)
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            color: color!(red400),
            child: anchor_circle(&mut commands, &assets, CenterLeft)
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            color: color!(red500),
            child: anchor_circle(&mut commands, &assets, Center)
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            color: color!(red600),
            child: anchor_circle(&mut commands, &assets, CenterRight)
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            color: color!(red700),
            child: anchor_circle(&mut commands, &assets, BottomLeft)
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            color: color!(red800),
            child: anchor_circle(&mut commands, &assets, BottomCenter)
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            color: color!(red900),
            child: anchor_circle(&mut commands, &assets, BottomRight)
        },
    });

    hspan! ((commands, assets) {
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
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            color: color!(yellow100),            
            child: anchor_circle(&mut commands, &assets, TopLeft),
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            color: color!(yellow200),
            child: anchor_circle(&mut commands, &assets, TopCenter),
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            color: color!(yellow300),
            child: anchor_circle(&mut commands, &assets, TopRight),
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            color: color!(yellow400),
            child: anchor_circle(&mut commands, &assets, CenterLeft),
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            color: color!(yellow500),
            child: anchor_circle(&mut commands, &assets, Center),
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            color: color!(yellow600),
            child: anchor_circle(&mut commands, &assets, CenterRight),
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            color: color!(yellow700),
            child: anchor_circle(&mut commands, &assets, BottomLeft),
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            color: color!(yellow800),
            child: anchor_circle(&mut commands, &assets, BottomCenter),
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            color: color!(yellow900),
            child: anchor_circle(&mut commands, &assets, BottomRight),
        },
    });

    vbox! ((commands, assets) {
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
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            color: color!(green100),
            child: anchor_circle(&mut commands, &assets, TopLeft),
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            color: color!(green200),
            child: anchor_circle(&mut commands, &assets, TopCenter),
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            color: color!(green300),
            child: anchor_circle(&mut commands, &assets, TopRight),
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            color: color!(green400),
            child: anchor_circle(&mut commands, &assets, CenterLeft),
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            color: color!(green500),
            child: anchor_circle(&mut commands, &assets, Center),
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            color: color!(green600),
            child: anchor_circle(&mut commands, &assets, CenterRight),
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            color: color!(green700),
            child: anchor_circle(&mut commands, &assets, BottomLeft),
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            color: color!(green800),
            child: anchor_circle(&mut commands, &assets, Bottom),
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            color: color!(green900),
            child: anchor_circle(&mut commands, &assets, BottomRight),
        },
    });

    vspan! ((commands, assets) {
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
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            color: color!(blue100),
            child: anchor_circle(&mut commands, &assets, TopLeft),
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            color: color!(blue200),
            child: anchor_circle(&mut commands, &assets, TopCenter),
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            color: color!(blue300),
            child: anchor_circle(&mut commands, &assets, TopRight),
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            color: color!(blue400),
            child: anchor_circle(&mut commands, &assets, CenterLeft),
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            color: color!(blue500),
            child: anchor_circle(&mut commands, &assets, Center),
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            color: color!(blue600),
            child: anchor_circle(&mut commands, &assets, CenterRight),
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            color: color!(blue700),
            child: anchor_circle(&mut commands, &assets, BottomLeft),
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            color: color!(blue800),
            child: anchor_circle(&mut commands, &assets, BottomCenter),
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            color: color!(blue900),
            child: anchor_circle(&mut commands, &assets, BottomRight),
        },
    });
}
