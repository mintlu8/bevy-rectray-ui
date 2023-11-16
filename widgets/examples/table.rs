use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::AoUIWidgetsPlugin;
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


macro_rules! anchor_circle {
    (($commands: expr, $server: expr $(, $ctx: expr)*) {$expr: expr}) => {
        bevy_aoui_widgets::meta_dsl!(($commands, $server $(, $ctx)*) [bevy_aoui_widgets::dsl::Shape] {
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
    fixed_table! ((commands, assets) {
        anchor: Center,
        dimension: [400, 100],
        margin: [10, 10],
        columns: [0.2, 0.3, 0.5, 0.7],
        z: 1,
        extra: Sprite::default(),
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
    });
}
