use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::{AoUIPlugin, widgets::button::CheckButton};
use bevy_aoui::widgets::scroll::{Scrolling, ScrollDirection};

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
        .add_plugins(AoUIPlugin)
        .register_cursor_default(CursorIcon::Arrow)
        .insert_resource(ScrollDirection::INVERTED)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    textbox!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: sig_fps().mark::<SigText>().map(|x: f32| format!("FPS: {:.2}", x))
    });
    
    let (send1, recv1, recv2, recv3) = signal();
    let (send2, recv21, recv22, recv23) = signal();

    clipping_frame!((commands, assets) {
        dimension: [400, 400],
        buffer: [800, 800],
        scroll: Scrolling::Y,
        layer: 1,
        container: with_layer(1, ||vbox!((commands, assets) {
            anchor: Top,
            child: hspan! {
                dimension: size2!([400, 2 em]),
                child: textbox! {
                    anchor: Left,
                    text: "Accordion 1",
                },
                child: button! {
                    anchor: Right,
                    dimension: size2!([2 em, 2 em]),
                    extra: CheckButton::Checked,
                    cursor: CursorIcon::Hand,
                    extra: send1.mark::<SigChange>(),
                    child: textbox! {
                        text: "v",
                        extra: recv3.mark::<SigRotation>().map(|x: bool| if x {PI} else {0.0}),
                        extra: Interpolate::<Rotation>::ease(EaseFunction::CubicInOut, 0.0, 0.5),
                    },
                }
            },
            child: clipping_frame! {
                anchor: Top,
                dimension: [400, 400],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                layer: 2,
                extra: recv1.mark::<SigDimensionY>().map(|x: bool| if x {400.0f32} else {0.0f32}),
                extra: Interpolate::<Dimension>::ease(EaseFunction::CubicInOut, Vec2::new(400.0, 400.0), 0.5),
                container: with_layer(2, ||frame!( commands{
                    dimension: [400, 400],
                    extra: recv2.mark::<SigOpacity>().map(|x: bool| if x {1.0f32} else {0.0f32}),
                    extra: Interpolate::<Opacity>::ease(EaseFunction::CubicInOut, 1.0, 0.5),
                    child: textbox! {
                        anchor: TopLeft,
                        bounds: [390, 999999],
                        color: color!(gold),
                        wrap: true,
                        extra: OpacityWriter,
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.
Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.
Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.",
                    }
                }))
            },
            child: hspan! {
                dimension: size2!([400, 2 em]),
                child: textbox! {
                    anchor: Left,
                    text: "Accordion 2",
                },
                child: button! {
                    anchor: Right,
                    dimension: size2!([2 em, 2 em]),
                    extra: CheckButton::Checked,
                    cursor: CursorIcon::Hand,
                    extra: send2.mark::<SigChange>(),
                    child: textbox! {
                        text: "v",
                        extra: recv23.mark::<SigRotation>().map(|x: bool| if x {PI} else {0.0}),
                        extra: Interpolate::<Rotation>::ease(EaseFunction::CubicInOut, 0.0, 0.5),
                    },
                }
            },
            child: clipping_frame! {
                anchor: Top,
                dimension: [400, 400],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                layer: 3,
                extra: recv21.mark::<SigDimensionY>().map(|x: bool| if x {400.0f32} else {0.0f32}),
                extra: Interpolate::<Dimension>::ease(EaseFunction::CubicInOut, Vec2::new(400.0, 400.0), 0.5),
                container: with_layer(3, ||frame!((commands, assets){
                    dimension: [400, 400],
                    extra: recv22.mark::<SigOpacity>().map(|x: bool| if x {1.0f32} else {0.0f32}),
                    extra: Interpolate::<Opacity>::ease(EaseFunction::CubicInOut, 1.0, 0.5),
                    child: textbox! {
                        anchor: TopLeft,
                        bounds: [390, 999999],
                        color: color!(gold),
                        wrap: true,
                        extra: OpacityWriter,
                        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.
Quisque id odio urna. Maecenas aliquam ullamcorper semper. Duis eu pulvinar magna, vel malesuada odio. Morbi lobortis porttitor metus sit amet pellentesque. In convallis feugiat sem, eget varius risus vulputate eget. Ut nec massa cursus, tempor quam nec, vulputate lorem. Nullam nec nisl et odio blandit vulputate. Morbi porta risus dui, nec efficitur sem euismod quis. Integer vel elit massa. Mauris ornare sollicitudin nunc venenatis laoreet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean aliquet egestas ipsum.
Aenean fringilla faucibus augue, at commodo lectus vestibulum placerat. Fusce et placerat velit. Suspendisse vel risus leo. Aenean in justo nec mauris porta lobortis a vitae nisi. Fusce fermentum ipsum et aliquet varius. Proin vel aliquam risus, et ornare libero. Proin non luctus dui.",
                    }
                }))
            },
        }))
    });
    
}
