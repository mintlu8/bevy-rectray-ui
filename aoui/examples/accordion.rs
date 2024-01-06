use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::AouiPlugin;
use bevy_aoui::dsl::AouiCommands;
use bevy_aoui::events::MouseWheelAction;
use bevy_aoui::signals::Object;
use bevy_aoui::signals::SignalBuilder;
use bevy_aoui::widgets::button::RadioButton;

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
        .register_scrolling_speed([16, 16], [0.5, -0.5])
        .run();
}

static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.";

#[derive(Component)]
pub struct ScrollDimension(f32);


pub fn accordion_page(
    commands: &mut AouiCommands, 
    index: usize,
    group: &RadioButton, 
    scroll: &SignalBuilder<MouseWheelAction>,
    text: &str,
) -> [Entity; 2] {
    use bevy_aoui::dsl::prelude::*;
    let sig = group.recv::<Object>();

    const HEIGHT: f32 = 200.0;

    let (pos_text, pos_scroll) = SharedPosition::many();
    let (cov_send, cov_recv) = commands.signal();
    let (cov_percent_send, cov_percent_recv) = commands.signal();
    let (render_in, render_out) = commands.render_target([800, 800]);
    [
        hbox! (commands{
            dimension: size2!(400, 2 em),
            child: text! {
                anchor: Left,
                text: format!("Accordion {}", index),
                layer: 1,
            },
            child: radio_button! {
                anchor: Right,
                center: Center,
                dimension: size2!(2 em, 2 em),
                context: group.clone(),
                cancellable: true,
                value: index,
                z: 0.01,
                child: text! {
                    text: "v",
                    layer: 1,
                    extra: sig.clone().recv_select(index, 
                        Interpolate::<Rotation>::signal_to(PI),
                        Interpolate::<Rotation>::signal_to(0.0),
                    ),
                    extra: transition! (Rotation 0.5 CubicInOut default (if index == 0 {PI} else {0.0}))
                },
            }
        }),
        hstack! (commands {
            anchor: Top,
            extra: sig.clone().recv_select(index, 
                Interpolate::<Opacity>::signal_to(1.0),
                Interpolate::<Opacity>::signal_to(0.0),
            ),
            extra: transition! (Opacity 0.5 CubicInOut default (if index == 0 {1.0} else {0.0})),
            child: scrolling! {
                anchor: Top,
                dimension: [380, 200],
                scroll: Scrolling::Y
                    .with_shared_position(pos_text)
                    .with_invoke(scroll.clone()),
                coverage_px: cov_send,
                coverage_percent: cov_percent_send,
                extra: ScrollDimension(200.0),
                extra: sig.clone().recv_select(index, 
                    |dim: &ScrollDimension, interpolate: &mut Interpolate<Dimension>| {
                        interpolate.interpolate_to_y(dim.0)
                    },
                    Interpolate::<Dimension>::signal_to_y(0.0),
                ),
                extra: cov_recv.recv(
                    |fac: Vec2, dim: &mut ScrollDimension| {
                        dim.0 = fac.y.min(HEIGHT);
                    }
                ).with_slot::<1>(),
                extra: transition! (Dimension 0.5 Linear default [380, if index == 0 {200} else {0}]),
                layer: 1,
                child: text! {
                    anchor: Top,
                    bounds: [370, 999999],
                    color: color!(gold),
                    wrap: true,
                    layer: (index + 4) as u8,
                    text: text,
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
                    dimension: size2!(20, 20 %),
                    color: color!(red),
                    layer: 1,
                    extra: DragY.with_position(pos_scroll.flip(false, true)),
                    extra: cov_percent_recv.recv(|fac: Vec2, dim: &mut Dimension| {
                        dim.edit_raw(|v| v.y = if fac.y > 1.0 {1.0 / fac.y} else {fac.y})
                    }),
                }
            },
            child: camera_frame! {
                dimension: Size2::FULL,
                render_target: render_in,
                layer: (index + 4) as u8,
                extra: IgnoreLayout,
                child: sprite! {
                    dimension: Size2::FULL,
                    layer: 1,
                    sprite: render_out,
                }
            }
        })
    ]
}

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_channel(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });

    let group = radio_button_group(0usize);

    let (scroll_send, scroll_recv) = commands.signal();

    let texts = [TEXT, TEXT, "Hello, Hello, Hello!", &format!("{TEXT}{TEXT}"), "apple\norange\nbanana", TEXT];

    let children: Vec<_> = texts.into_iter().enumerate()
        .map(|(idx, text)| accordion_page(&mut commands, idx, &group, &scroll_send, text))
        .flatten().collect();

    let (main_in, main_out) = commands.render_target([800, 800]);
    camera_frame!(commands{
        dimension: [400, 400],
        render_target: main_in,
        layer: 1,
        child: sprite! {
            dimension: Size2::FULL,
            sprite: main_out,
        }
    });

    scrolling!(commands {
        dimension: [400, 400],
        scroll: Scrolling::POS_Y
            .with_recv(scroll_recv),
        child: vstack! {
            anchor: Top,
            children: children,
        }
    });
    
}
