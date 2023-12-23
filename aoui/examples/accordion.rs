use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::AouiPlugin;
use bevy_aoui::widgets::scroll::ScrollDirection;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .insert_resource(ScrollDirection::INVERTED)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });
    
    let (send1, recv11, recv12, recv13) = signal();
    let (send2, recv21, recv22, recv23) = signal();

    let (scroll_send1, scroll_recv) = signal();
    let scroll_send2 = scroll_send1.clone();

    clipping_layer!((commands, assets) {
        dimension: [400, 400],
        scroll: Scrolling::Y,
        scroll_recv: scroll_recv,
        buffer: [800, 800],
        layer: 3,
        child: vbox! {
            anchor: Top,
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 1",
                },
                child: check_button! {
                    anchor: Right,
                    dimension: size2!(2 em, 2 em),
                    checked: true,
                    on_change: send1,
                    child: text! {
                        text: "v",
                        extra: recv13.map::<SigRotation>(|x: bool| if x {PI} else {0.0}),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: clipping_layer! {
                anchor: Top,
                dimension: [400, 400],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                scroll_send: scroll_send1,
                extra: recv11.map::<SigDimensionY>(|x: bool| if x {400.0f32} else {0.0f32}),
                extra: transition! (Dimension 0.5 CubicInOut default [400, 400]),
                layer: 1,
                child: text! {
                        anchor: TopLeft,
                        bounds: [390, 999999],
                        color: color!(gold),
                        wrap: true,
                        extra: recv12.map::<SigOpacity>(|x: bool| if x {1.0f32} else {0.0f32}),
                        extra: transition! (Opacity 0.5 CubicInOut default 1.0),
                        extra: OpacityWriter,
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.
Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.
Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.",
                }
            },
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 2",
                },
                child: check_button! {
                    anchor: Right,
                    dimension: size2!(2 em, 2 em),
                    checked: true,
                    on_change: send2,
                    child: text! {
                        text: "v",
                        extra: recv23.map::<SigRotation>(|x: bool| if x {PI} else {0.0}),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: clipping_layer! {
                anchor: Top,
                dimension: [400, 400],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                scroll_send: scroll_send2,
                extra: recv21.map::<SigDimensionY>(|x: bool| if x {400.0f32} else {0.0f32}),
                extra: Interpolate::<Dimension>::ease(EaseFunction::CubicInOut, Vec2::new(400.0, 400.0), 0.5),
                layer: 2,
                child: text! {
                    anchor: TopLeft,
                    bounds: [390, 999999],
                    color: color!(gold),
                    wrap: true,
                    extra: recv22.map::<SigOpacity>(|x: bool| if x {1.0f32} else {0.0f32}),
                    extra: Interpolate::<Opacity>::ease(EaseFunction::CubicInOut, 1.0, 0.5),
                    extra: OpacityWriter,
                    text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.
Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.
Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.",
                }
            },
        }
    });
    
}
