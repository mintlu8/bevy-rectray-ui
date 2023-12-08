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
            fill: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
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
    });

    hspan! ((commands, assets) {
        anchor: Left,
        dimension: [600, 100],
        offset: [20, 0],
        margin: [10, 10],
        z: 2,
        child: rectangle! {
            dimension: size2!(1 + [6, 6] px),
            fill: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            fill: color!(yellow100),            
            child: anchor_circle!{TopLeft},
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            fill: color!(yellow200),
            child: anchor_circle!{TopCenter},
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            fill: color!(yellow300),
            child: anchor_circle!{TopRight},
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            fill: color!(yellow400),
            child: anchor_circle!{CenterLeft},
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            fill: color!(yellow500),
            child: anchor_circle!{Center},
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            fill: color!(yellow600),
            child: anchor_circle!{CenterRight},
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            fill: color!(yellow700),
            child: anchor_circle!{BottomLeft},
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            fill: color!(yellow800),
            child: anchor_circle!{BottomCenter},
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            fill: color!(yellow900),
            child: anchor_circle!{BottomRight},
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
            fill: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            fill: color!(green100),
            child: anchor_circle!{TopLeft},
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            fill: color!(green200),
            child: anchor_circle!{TopCenter},
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            fill: color!(green300),
            child: anchor_circle!{TopRight},
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            fill: color!(green400),
            child: anchor_circle!{CenterLeft},
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            fill: color!(green500),
            child: anchor_circle!{Center},
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            fill: color!(green600),
            child: anchor_circle!{CenterRight},
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            fill: color!(green700),
            child: anchor_circle!{BottomLeft},
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            fill: color!(green800),
            child: anchor_circle!{Bottom},
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            fill: color!(green900),
            child: anchor_circle!{BottomRight},
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
            fill: color!(neutral700),
            extra: IgnoreLayout,
            z: -1,
        },
        child: rectangle! {
            anchor: TopLeft,
            dimension: [80, 50],
            fill: color!(blue100),
            child: anchor_circle!{TopLeft},
        },

        child: rectangle! {
            anchor: TopCenter,
            dimension: [40, 90],
            fill: color!(blue200),
            child: anchor_circle!{TopCenter},
        },
        child: rectangle! {
            anchor: TopRight,
            dimension: [60, 30],
            fill: color!(blue300),
            child: anchor_circle!{TopRight},
        },
        child: rectangle! {
            anchor: CenterLeft,
            dimension: [70, 50],
            fill: color!(blue400),
            child: anchor_circle!{CenterLeft},
        },
        child: rectangle! {
            anchor: Center,
            dimension: [50, 60],
            fill: color!(blue500),
            child: anchor_circle!{Center},
        },
        child: rectangle! {
            anchor: CenterRight,
            dimension: [40, 90],
            fill: color!(blue600),
            child: anchor_circle!{CenterRight},
        },
        child: rectangle! {
            anchor: BottomLeft,
            dimension: [10, 10],
            fill: color!(blue700),
            child: anchor_circle!{BottomLeft},
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: [30, 70],
            fill: color!(blue800),
            child: anchor_circle!{BottomCenter},
        },
        child: rectangle! {
            anchor: BottomRight,
            dimension: [90, 30],
            fill: color!(blue900),
            child: anchor_circle!{BottomRight},
        },
    });
}
