use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::AoUIExtensionsPlugin;
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


macro_rules! anchor_circle {
    (($commands: expr, $server: expr $(, $ctx: expr)*) {$expr: expr}) => {
        bevy_aoui_widgets::meta_dsl!(($commands, $server $(, $ctx)*) [bevy_aoui_widgets::dsl::builders::ShapeBuilder] {
            default_material: $server.add(::bevy::prelude::ColorMaterial::default()),
            shape: bevy_aoui_widgets::widgets::Shapes::Circle,
            fill: color!(white),
            stroke: (color!(black), 1),
            dimension: [4, 4],
            anchor: $expr
        })
    };
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    vbox! ((commands) {
        anchor: Top,
        margin: 4,
        child: textbox! { 
            anchor: Top,
            text: "Fixed Table Demo",
        },
        child: textbox! { 
            anchor: Top,
            text: "5 columns of 20%, 10%, 20%, 30%, 20%" 
        },
    });
    table! ((commands, assets) {
        anchor: Center,
        dimension: [700, 100],
        columns: [(SizeUnit::Percent, 0.2), (SizeUnit::Percent, 0.1), (SizeUnit::Percent, 0.2), (SizeUnit::Percent, 0.3), (SizeUnit::Percent, 0.2)],
        z: 1,
        extra: Sprite::default(),
        child: rectangle! {
            dimension: size2!([100%, 100%]),
            fill: color!(black),
            extra: IgnoreLayout,
            z: -1,
            child: rectangle! {
                anchor: Left,
                dimension: size2!([20%, 100%]),
                fill: color!(blue300)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!([20%, 0]),
                dimension: size2!([10%, 100%]),
                fill: color!(blue400)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!([30%, 0]),
                dimension: size2!([20%, 100%]),
                fill: color!(blue500)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!([50%, 0]),
                dimension: size2!([30%, 100%]),
                fill: color!(blue600)
            },
            child: rectangle! {
                anchor: Left,
                offset: size2!([80%, 0]),
                dimension: size2!([20%, 100%]),
                fill: color!(blue700)
            }
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            fill: color!(red100),
            child: anchor_circle!{TopLeft},
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            fill: color!(red200),
            child: anchor_circle!{TopCenter}
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            fill: color!(red300),
            child: anchor_circle!{TopRight}
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            fill: color!(red400),
            child: anchor_circle!{CenterLeft}
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            fill: color!(red500),
            child: anchor_circle!{Center}
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            fill: color!(red600),
            child: anchor_circle!{CenterRight}
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            fill: color!(red700),
            child: anchor_circle!{BottomLeft}
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            fill: color!(red800),
            child: anchor_circle!{BottomCenter}
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            fill: color!(red900),
            child: anchor_circle!{BottomRight}
        },
        child: rectangle! {
            anchor: Center,
            dimension: [40, 40],
            fill: color!(red950),
            child: anchor_circle!{Center}
        },


        child: rectangle! {
            anchor: CenterLeft,
            dimension: [60, 40],
            fill: color!(orange100),
            child: anchor_circle!{CenterLeft},
        },

        child: rectangle! {
            anchor: TopRight,
            dimension: [50, 30],
            fill: color!(orange200),
            child: anchor_circle!{TopRight}
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [40, 10],
            fill: color!(orange300),
            child: anchor_circle!{BottomCenter}
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [20, 50],
            fill: color!(orange400),
            child: anchor_circle!{TopLeft}
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [40, 40],
            fill: color!(orange500),
            child: anchor_circle!{BottomLeft}
        },
        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 50],
            fill: color!(orange600),
            child: anchor_circle!{TopCenter}
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [30, 30],
            fill: color!(orange700),
            child: anchor_circle!{CenterRight}
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [30, 60],
            fill: color!(orange800),
            child: anchor_circle!{TopRight}
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [50, 30],
            fill: color!(orange900),
            child: anchor_circle!{BottomLeft}
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [30, 30],
            fill: color!(orange950),
            child: anchor_circle!{BottomRight}
        },
    });
}
