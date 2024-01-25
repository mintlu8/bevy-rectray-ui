//! Showcases support for dragging and interpolation.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, sprite::{Material2dPlugin, Material2d}, render::render_resource::AsBindGroup};
use bevy_aoui::{util::{WorldExtension, AouiCommands}, widgets::constraints::PositionFac, AouiPlugin};

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
        .add_plugins(Material2dPlugin::<Circle>::default())
        .register_cursor_default(CursorIcon::Arrow)
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

pub fn init(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
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

    material_sprite! (commands {
        dimension: [100, 100],
        hitbox: Hitbox::rect(1),
        z: 10,
        material: Circle {
            fill: Color::RED,
            stroke: Color::BLACK
        },
        event: EventFlags::Hover|EventFlags::LeftDrag,
        extra: Dragging::BOTH.without_constraint().with_snap_back(),
        extra: SetCursor {
            flags: EventFlags::Hover|EventFlags::LeftDrag,
            icon: CursorIcon::Hand,
        },
        extra: transition!(Offset 4.0 BounceOut default Vec2::ZERO),
    });

    let (send1, recv1) = signal();

    rectangle!(commands {
        dimension: [400, 50],
        offset: [0, 100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Right,
            center: Center,
            color: color!(aqua),
            event: EventFlags::Hover|EventFlags::LeftDrag,
            extra: SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftDrag,
                icon: CursorIcon::Hand,
            },
            extra: Dragging::X,
            signal: sender::<PositionFac>(send1),
            system: |fac: SigSend<PositionFac>, transform: Ac<Transform2D>, dim: Ac<Dimension>| {
                let fac = fac.recv().await;
                futures::try_join!(
                    transform.set(move |x| x.rotation = fac * 2.0 * PI),
                    dim.set(move |v| v.edit_raw(|v| v.y = 50.0 + (1.0 - fac) * 50.0))
                )?;
            }
        }
    });

    text! (commands {
        offset: [300, 100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        signal: receiver::<PositionFac>(recv1),
        system: |x: SigRecv<PositionFac>, text: Ac<Text>| {
            let fac = x.recv().await;
            text.set(move |text| format_widget!(text, "<= has value {:.2}!", fac)).await?;
        }
    });

    let (send2, recv2) = signal();
    let (send3, recv3) = signal();

    rectangle!(commands {
        dimension: [400, 50],
        offset: [0, -100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Left,
            color: color!(aqua),
            extra: Dragging::X,
            signal: sender::<PositionFac>(send3),
            signal: receiver::<Dragging>(recv2),
        }
    });

    material_sprite! (commands {
        dimension: [100, 100],
        offset: [-300, -100],
        hitbox: Hitbox::rect(1),
        event: EventFlags::Hover|EventFlags::LeftDrag,
        material: Circle {
            fill: color!(aqua),
            stroke: color!(blue),
        },
        extra: SetCursor {
            flags: EventFlags::Hover|EventFlags::LeftDrag,
            icon: CursorIcon::Hand,
        },
        signal: sender::<Dragging>(send2),
    });

    text! (commands {
        offset: [300, -100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        signal: receiver::<PositionFac>(recv3),
        system: |x: SigRecv<PositionFac>, text: Ac<Text>| {
            let fac = x.recv().await;
            text.set(move |text| format_widget!(text, "<= has value {:.2}!", fac)).await?;
        }   
    });
}
