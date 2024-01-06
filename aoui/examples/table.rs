use bevy::{prelude::*, render::render_resource::AsBindGroup, sprite::{Material2d, Material2dPlugin}};
use bevy_aoui::{AouiPlugin, dsl::{DslInto, AouiCommands}, Anchor, material_sprite, layout::TableLayout};

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

fn anchor_circle(commands: &mut AouiCommands, anchor: impl DslInto<Anchor>) -> Entity{
    material_sprite!(commands {
        anchor: anchor,
        dimension: [6, 6],
        z: 10,
        material: Circle {
            fill: Color::WHITE,
            stroke: Color::BLACK,
        }
    })
}


pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());
    vstack! (commands {
        anchor: Top,
        margin: 4,
        child: text! { 
            anchor: Top,
            text: "Fixed Table Demo",
        },
        child: text! { 
            anchor: Top,
            text: "5 columns of 20%, 10%, 20%, 30%, 20%" 
        },
    });
    frame! (commands {
        anchor: Center,
        dimension: [700, 100],
        z: 1,
        layout: TableLayout::from_columns(
            [
                (SizeUnit::Percent, 0.2), 
                (SizeUnit::Percent, 0.1), 
                (SizeUnit::Percent, 0.2), 
                (SizeUnit::Percent, 0.3), 
                (SizeUnit::Percent, 0.2)
            ]
        ),
        extra: Sprite::default(),
        child: rectangle! {
            dimension: size2!(100%, 100%),
            color: color!(black),
            extra: IgnoreLayout,
            z: -1,
            child: rectangle! {
                anchor: Left,
                dimension: size2!(20%, 100%),
                color: color!(blue300)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!(20%, 0),
                dimension: size2!(10%, 100%),
                color: color!(blue400)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!(30%, 0),
                dimension: size2!(20%, 100%),
                color: color!(blue500)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!(50%, 0),
                dimension: size2!(30%, 100%),
                color: color!(blue600)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!(80%, 0),
                dimension: size2!(20%, 100%),
                color: color!(blue700)
            }
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            color: color!(red100),
            child: anchor_circle(&mut commands, TopLeft),
        },
        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            color: color!(red200),
            child: anchor_circle(&mut commands, TopCenter)
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            color: color!(red300),
            child: anchor_circle(&mut commands, TopRight)
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            color: color!(red400),
            child: anchor_circle(&mut commands, CenterLeft)
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            color: color!(red500),
            child: anchor_circle(&mut commands, Center)
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            color: color!(red600),
            child: anchor_circle(&mut commands, CenterRight)
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            color: color!(red700),
            child: anchor_circle(&mut commands, BottomLeft)
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            color: color!(red800),
            child: anchor_circle(&mut commands, BottomCenter)
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            color: color!(red900),
            child: anchor_circle(&mut commands, BottomRight)
        },
        child: rectangle! {
            anchor: Center,
            dimension: [40, 40],
            color: color!(red950),
            child: anchor_circle(&mut commands, Center)
        },


        child: rectangle! {
            anchor: CenterLeft,
            dimension: [60, 40],
            color: color!(orange100),
            child: anchor_circle(&mut commands, CenterLeft),
        },

        child: rectangle! {
            anchor: TopRight,
            dimension: [50, 30],
            color: color!(orange200),
            child: anchor_circle(&mut commands, TopRight)
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [40, 10],
            color: color!(orange300),
            child: anchor_circle(&mut commands, BottomCenter)
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [20, 50],
            color: color!(orange400),
            child: anchor_circle(&mut commands, TopLeft)
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [40, 40],
            color: color!(orange500),
            child: anchor_circle(&mut commands, BottomLeft)
        },
        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 50],
            color: color!(orange600),
            child: anchor_circle(&mut commands, TopCenter)
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [30, 30],
            color: color!(orange700),
            child: anchor_circle(&mut commands, CenterRight)
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [30, 60],
            color: color!(orange800),
            child: anchor_circle(&mut commands, TopRight)
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [50, 30],
            color: color!(orange900),
            child: anchor_circle(&mut commands, BottomLeft)
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [30, 30],
            color: color!(orange950),
            child: anchor_circle(&mut commands, BottomRight)
        },
    });
}
