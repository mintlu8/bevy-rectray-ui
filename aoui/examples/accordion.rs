use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::AouiPlugin;

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
        .register_scrolling_speed([16, 16], [0.5, -0.5])
        .run();
}

static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.";

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });

    let (first, second) = radio_button_group(0);
    let sig = first.recv::<i32>();

    let (scroll_send1, scroll_recv) = signal();
    let scroll_send2 = scroll_send1.clone();

    let (drag1_send, drag1_recv) = signal();
    clipping_layer!((commands, assets) {
        dimension: [400, 400],
        scroll: Scrolling::POS_Y,
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
                child: radio_button! {
                    anchor: Right,
                    center: Center,
                    dimension: size2!(2 em, 2 em),
                    context: first,
                    cancellable: true,
                    value: 0,
                    child: text! {
                        text: "v",
                        extra: sig.clone().cond_recv::<SigRotation>(0, PI, 0.0),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: clipping_layer! {
                anchor: Top,
                dimension: [400, 200],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                scroll_send: scroll_send1,
                extra: sig.clone().cond_recv::<SigDimensionY>(0, 200.0, 0.0),
                extra: transition! (Dimension 0.5 CubicInOut default [400, 400]),
                extra: drag1_recv.recv::<SigPositionSync>(),
                layer: 1,
                child: text! {
                    anchor: Top,
                    offset: [-10, 0],
                    bounds: [370, 999999],
                    color: color!(gold),
                    wrap: true,
                    extra: sig.clone().cond_recv::<SigOpacity>(0, 1.0, 0.0),
                    extra: transition! (Opacity 0.5 CubicInOut default 1.0),
                    extra: OpacityWriter,
                    text: TEXT,
                }
            },
            child: rectangle! {
                anchor: Right,
                dimension: [20, 200],
                color: color!(orange),
                extra: IgnoreLayout,
                child: rectangle! {
                    anchor: Top,
                    event: EventFlags::LeftDrag,
                    dimension: [20, 40],
                    color: color!(red),
                    extra: DragY,
                    extra: DragConstraint,
                    extra: handler! {
                        EvPositionSync => {drag1_send}
                    }
                }
            },
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 2",
                },
                child: radio_button! {
                    anchor: Right,
                    center: Center,
                    dimension: size2!(2 em, 2 em),
                    context: second,
                    cancellable: true,
                    value: 1,
                    child: text! {
                        text: "v",
                        rotation: PI,
                        extra: sig.clone().cond_recv::<SigRotation>(1, PI, 0.0),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: clipping_layer! {
                anchor: Top,
                dimension: [400, 0],
                buffer: [800, 800],
                scroll: Scrolling::Y,
                scroll_send: scroll_send2,
                extra: sig.clone().cond_recv::<SigDimensionY>(1, 200.0, 0.0),
                extra: Interpolate::<Dimension>::ease(EaseFunction::CubicInOut, Vec2::new(400.0, 400.0), 0.5),
                layer: 2,
                child: text! {
                    anchor: TopLeft,
                    bounds: [390, 999999],
                    color: color!(gold),
                    opacity: 0.0,
                    wrap: true,
                    extra: sig.clone().cond_recv::<SigOpacity>(1, 1.0, 0.0),
                    extra: Interpolate::<Opacity>::ease(EaseFunction::CubicInOut, 1.0, 0.5),
                    extra: OpacityWriter,
                    text: TEXT
                }
            },
        }
    });
    
}
