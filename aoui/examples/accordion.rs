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

#[derive(Component)]
pub struct ScrollDimension(f32);

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    let (first, second, third, fourth) = radio_button_group(0);
    let sig = first.recv::<i32>();

    let (scroll_send1, scroll_send2, scroll_send3, scroll_send4, scroll_recv) = signal();

    let (text1, scroll1) = SharedPosition::many();
    let (text2, scroll2) = SharedPosition::many();
    let (text3, scroll3) = SharedPosition::many();
    let (text4, scroll4) = SharedPosition::many();



    let (cov3_send, cov3_recv) = signal();

    let (main_in, main_out) = render_target(&assets, [800, 800]);
    camera_frame!((commands, assets){
        dimension: [400, 400],
        render_target: main_in,
        layer: 1,
        child: sprite! {
            dimension: Size2::FULL,
            sprite: main_out,
        }
    });

    scrolling!((commands, assets) {
        dimension: [400, 400],
        scroll: Scrolling::POS_Y
            .with_recv(scroll_recv),
        child: vbox! {
            anchor: Top,
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 1",
                    layer: 1,
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
                        layer: 1,
                        extra: sig.clone().recv_select(0, 
                            Interpolate::<Rotation>::signal_to(PI),
                            Interpolate::<Rotation>::signal_to(0.0),
                        ),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: hbox!{
                anchor: Top,
                extra: sig.clone().recv_select(0, 
                    Interpolate::<Opacity>::signal_to(1.0),
                    Interpolate::<Opacity>::signal_to(0.0),
                ),
                extra: transition! (Opacity 0.5 CubicInOut default 1.0),
                child: scrolling! {
                    anchor: Top,
                    dimension: [380, 200],
                    scroll: Scrolling::Y
                        .with_shared_position(text1)
                        .with_invoke(scroll_send1),
                    extra: sig.clone().recv_select(0, 
                        Interpolate::<Dimension>::signal_to_y(200.0),
                        Interpolate::<Dimension>::signal_to_y(0.0),
                    ),
                    extra: transition! (Dimension 0.5 Linear default [380, 200]),
                    layer: 1,
                    child: text! {
                        anchor: Top,
                        bounds: [370, 999999],
                        color: color!(gold),
                        wrap: true,
                        layer: 1,
                        text: TEXT,
                    }
                },
                child: rectangle! {
                    anchor: Right,
                    dimension: size2!(20, 100%),
                    color: color!(orange),
                    layer: 1,
                    child: rectangle! {
                        anchor: Top,
                        event: EventFlags::LeftDrag,
                        dimension: [20, 40],
                        color: color!(red),
                        layer: 1,
                        extra: DragY.with_position(scroll1.flip(false, true)),
                    }
                },
            },
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 2",
                    layer: 1,
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
                        layer: 1,
                        extra: sig.clone().recv_select(1, 
                            Interpolate::<Rotation>::signal_to(PI),
                            Interpolate::<Rotation>::signal_to(0.0),
                        ),                        
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: hbox!{
                anchor: Top,
                extra: sig.clone().recv_select(1, 
                    Interpolate::<Opacity>::signal_to(1.0),
                    Interpolate::<Opacity>::signal_to(0.0),
                ),
                extra: transition! (Opacity 0.5 CubicInOut default 0.0),
                child: scrolling! {
                    anchor: Top,
                    dimension: [380, 100],
                    scroll: Scrolling::Y
                        .with_shared_position(text2)
                        .with_invoke(scroll_send2),
                    extra: sig.clone().recv_select(1, 
                        Interpolate::<Dimension>::signal_to_y(100.0),
                        Interpolate::<Dimension>::signal_to_y(0.0),
                    ),
                    extra: transition! (Dimension 0.5 Linear default [380, 0]),
                    layer: 1,
                    child: text! {
                        anchor: Top,
                        bounds: [370, 999999],
                        color: color!(gold),
                        wrap: true,
                        layer: 1,
                        text: TEXT,
                    }
                },
                child: rectangle! {
                    anchor: Right,
                    dimension: size2!(20, 100%),
                    color: color!(orange),
                    layer: 1,
                    child: rectangle! {
                        anchor: Top,
                        event: EventFlags::LeftDrag,
                        dimension: [20, 40],
                        color: color!(red),
                        layer: 1,
                        extra: DragY.with_position(scroll2.flip(false, true)),
                    }
                },
            },
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 3",
                    layer: 1,
                },
                child: radio_button! {
                    anchor: Right,
                    center: Center,
                    dimension: size2!(2 em, 2 em),
                    context: third,
                    cancellable: true,
                    value: 2,
                    child: text! {
                        text: "v",
                        rotation: PI,
                        layer: 1,
                        extra: sig.clone().recv_select(2, 
                            Interpolate::<Rotation>::signal_to(PI),
                            Interpolate::<Rotation>::signal_to(0.0),
                        ),                        
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: hbox!{
                anchor: Top,
                extra: sig.clone().recv_select(2, 
                    Interpolate::<Opacity>::signal_to(1.0),
                    Interpolate::<Opacity>::signal_to(0.0),
                ),
                extra: transition! (Opacity 0.5 CubicInOut default 0.0),
                child: scrolling! {
                    anchor: Top,
                    dimension: [380, 500],
                    scroll: Scrolling::Y
                        .with_shared_position(text3)
                        .with_invoke(scroll_send3),
                    coverage_px: cov3_send,
                    extra: ScrollDimension(0.0),
                    extra: sig.clone().recv_select(2, 
                        |dim: &ScrollDimension, interpolate: &mut Interpolate<Dimension>| {
                            interpolate.interpolate_to_y(dim.0)
                        },
                        Interpolate::<Dimension>::signal_to_y(0.0),
                    ),
                    extra: cov3_recv.recv(
                        |fac: Vec2, dim: &mut ScrollDimension| {
                            dim.0 = fac.y;
                        }
                    ).with_slot::<1>(),
                    extra: transition! (Dimension 0.5 Linear default [380, 0]),
                    layer: 1,
                    child: text! {
                        anchor: Top,
                        bounds: [370, 999999],
                        color: color!(gold),
                        wrap: true,
                        layer: 1,
                        text: "Hello, Hello, Hello!",
                    }
                },
                child: rectangle! {
                    anchor: Right,
                    dimension: size2!(20, 100%),
                    color: color!(orange),
                    layer: 1,
                    child: rectangle! {
                        anchor: Top,
                        event: EventFlags::LeftDrag,
                        dimension: [20, 40],
                        color: color!(red),
                        layer: 1,
                        extra: DragY.with_position(scroll3.flip(false, true)),
                    }
                },
            },
            child: hspan! {
                dimension: size2!(400, 2 em),
                child: text! {
                    anchor: Left,
                    text: "Accordion 4",
                    layer: 1,
                },
                child: radio_button! {
                    anchor: Right,
                    center: Center,
                    dimension: size2!(2 em, 2 em),
                    context: fourth,
                    cancellable: true,
                    value: 3,
                    child: text! {
                        text: "v",
                        rotation: PI,
                        layer: 1,
                        extra: sig.clone().recv_select(3, 
                            Interpolate::<Rotation>::signal_to(PI),
                            Interpolate::<Rotation>::signal_to(0.0),
                        ),
                        extra: transition! (Rotation 0.5 CubicInOut default PI)
                    },
                }
            },
            child: hbox!{
                anchor: Top,
                extra: sig.clone().recv_select(3, 
                    Interpolate::<Opacity>::signal_to(1.0),
                    Interpolate::<Opacity>::signal_to(0.0),
                ),
                extra: transition! (Opacity 0.5 CubicInOut default 0.0),
                child: scrolling! {
                    anchor: Top,
                    dimension: [380, 300],
                    scroll: Scrolling::Y
                        .with_shared_position(text4)
                        .with_invoke(scroll_send4),
                        extra: sig.clone().recv_select(3, 
                            Interpolate::<Dimension>::signal_to_y(300.0),
                            Interpolate::<Dimension>::signal_to_y(0.0),
                        ),
                    extra: transition! (Dimension 0.5 Linear default [380, 0]),
                    layer: 1,
                    child: text! {
                        anchor: Top,
                        bounds: [370, 999999],
                        color: color!(gold),
                        wrap: true,
                        layer: 1,
                        text: TEXT,
                    }
                },
                child: rectangle! {
                    anchor: Right,
                    dimension: size2!(20, 100%),
                    color: color!(orange),
                    layer: 1,
                    child: rectangle! {
                        anchor: Top,
                        event: EventFlags::LeftDrag,
                        dimension: [20, 40],
                        color: color!(red),
                        layer: 1,
                        extra: DragY.with_position(scroll4.flip(false, true)),
                    }
                },
            },
        }
    });
    
}
