//! This is a proof of concept as stacking camera is basically not gonna work in real games.

use bevy::log::LogPlugin;
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_defer::{TypedSignal, Object, signal_ids};
use bevy_rectray::util::WorldExtension;
use bevy_rectray::RectrayPlugin;
use bevy_rectray::util::RCommands;
use bevy_rectray::events::MovementUnits;
use bevy_rectray::widgets::button::RadioButton;
use futures_lite::future;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }).set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_plugins(RectrayPlugin)
        .register_scrolling_speed([16, 16], [0.5, -0.5])
        .run();
}

static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris semper magna nibh, nec tincidunt metus fringilla id. Phasellus viverra elit volutpat orci lacinia, non suscipit odio egestas. Praesent urna ipsum, viverra non dui id, auctor sodales sem. Quisque ut mi sit amet quam ultricies cursus at vitae justo. Morbi egestas pulvinar dui id elementum. Aliquam non aliquam eros. Nam euismod in lectus sit amet blandit. Aenean mauris diam, auctor ut massa sed, convallis congue leo. Maecenas non nibh semper, tempor velit sit amet, facilisis lacus. Curabitur nec leo nisl. Proin vitae fringilla nisl. Sed vel hendrerit mi. Donec et cursus risus, at euismod justo.
Ut luctus tellus mi. Donec non lacus ex. Vivamus non rutrum quam. Curabitur in bibendum tellus. Fusce eu gravida massa. Ut viverra vestibulum convallis. Morbi ullamcorper gravida fringilla. Morbi ullamcorper sem eget eleifend sagittis. Mauris interdum odio eget luctus pretium. In non dapibus risus.";

#[derive(Component)]
pub struct ScrollDimension(f32);


signal_ids!(
    SyncDim: usize,
);

pub fn accordion_page(
    commands: &mut RCommands,
    index: usize,
    group: &RadioButton,
    scroll: &TypedSignal<MovementUnits>,
    text: &str,
) -> [Entity; 2] {
    use bevy_rectray::dsl::prelude::*;
    let sig = group.recv::<Object>();

    const HEIGHT: f32 = 200.0;

    let (pos_text, pos_scroll) = signal();
    let (cov_send, cov_recv) = signal();
    let (cov_percent_send, cov_percent_recv) = signal();
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
                    signal: receiver::<Invocation>(sig.clone()),
                    system: |recv: SigRecv<Invocation>, rot: Ac<Interpolate<Rotation>>| {
                        let index = index;
                        if recv.recv().await.equal_to(&index) {
                            rot.interpolate_to(PI).await
                        } else {
                            rot.interpolate_to(0.0).await
                        }
                    },
                    extra: transition! (Rotation 0.5 CubicInOut default (if index == 0 {PI} else {0.0}))
                },
            }
        }),
        hstack! (commands {
            anchor: Top,
            signal: receiver::<Invocation>(sig.clone()),
            extra: ScrollParent,
            system: |recv: SigRecv<Invocation>, rot: Ac<Interpolate<Opacity>>| {
                if recv.recv().await.equal_to(&index) {
                    rot.interpolate_to(1.0).await
                } else {
                    rot.interpolate_to(0.0).await
                }
            },
            extra: transition! (Opacity 0.5 CubicInOut default (if index == 0 {1.0} else {0.0})),
            child: frame! {
                anchor: Top,
                dimension: [380, 200],
                signal: receiver::<Scrolling>(scroll.clone()),
                extra: ScrollDimension(200.0),

                signal: receiver::<Fac<Vec2>>(cov_recv),
                system: |recv: SigRecv<Fac<Vec2>>, dim: Ac<ScrollDimension>| {
                    let fac = recv.recv().await.y;
                    dbg!(fac);
                    dim.set(move |x| x.0 = fac.min(HEIGHT)).await?;
                },
                signal: receiver::<Invocation>(sig.clone()),
                system: |recv: SigRecv<Invocation>, dim: Ac<Interpolate<Dimension>>, sd: Ac<ScrollDimension>| {
                    let index = index;
                    let (invoke, dim_y) = future::zip(
                        recv.recv(), sd.get(|x| x.0)
                    ).await;
                    if invoke.equal_to(&index) {
                        dim.interpolate_to_y(dim_y?).await?;
                    } else {
                        dim.interpolate_to_y(0.0).await?;
                    }
                },
                extra: transition! (Dimension 0.5 Linear default [380, if index == 0 {200} else {0}]),
                layer: 1,
                child: text! {
                    event: EventFlags::MouseWheel,
                    extra: Scrolling::Y,
                    extra: SharedPosition::new(false, false),
                    signal: sender::<SharedPosition>(pos_text),
                    extra: GreaterBoundingBox::new(),
                    signal: sender::<GreaterBoundingBoxPx>(cov_send),
                    signal: sender::<GreaterBoundingBoxPercent>(cov_percent_send),
                    // coverage_px: cov_send,
                    // coverage_percent: cov_percent_send,
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
                    extra: Dragging::Y,
                    extra: SharedPosition::new(false, true),
                    signal: sender::<SharedPosition>(pos_scroll),
                    signal: receiver::<Fac<Vec2>>(cov_percent_recv),
                    system: |recv: SigRecv<Fac<Vec2>>, dim: Ac<Dimension>| {
                        let fac = recv.recv().await;
                        dim.set(move |dim| dim.edit_raw(|v| v.y = if fac.y > 1.0 {
                            1.0 / fac.y
                        } else {
                            fac.y
                        })).await?;
                    },
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

pub fn init(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    commands.spawn_bundle(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        system: |fps: Fps, text: Ac<Text>| {
            let fps = fps.get().await;
            text.set(move |text| format_widget!(text, "FPS: {:.2}", fps)).await?;
        }
    });

    let group = radio_button_group(0usize);

    let (scroll_send, scroll_recv) = signal();

    let texts = [TEXT, TEXT, "Hello, Hello, Hello!", &format!("{TEXT}{TEXT}"), "apple\norange\nbanana", TEXT];

    let children: Vec<_> = texts.into_iter().enumerate()
        .flat_map(|(idx, text)| accordion_page(&mut commands, idx, &group, &scroll_send, text))
        .collect();

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

    frame!(commands {
        dimension: [400, 400],
        child: vstack! {
            anchor: Top,
            child: children,
            extra: Scrolling::POS_Y,
            signal: receiver::<Scrolling>(scroll_recv),
        }
    });

}
