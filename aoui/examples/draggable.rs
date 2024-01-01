//! Showcases support for dragging and interpolation.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin, sprite::{Material2dPlugin, Material2d}, render::render_resource::AsBindGroup};
use bevy_aoui::{AouiPlugin, WorldExtension};

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

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!((commands, assets) {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });
    material_sprite! ((commands, assets) {
        dimension: [100, 100],
        hitbox: Rect(1),
        z: 10,
        material: Circle {
            fill: Color::RED,
            stroke: Color::BLACK
        },
        event: EventFlags::Hover|EventFlags::LeftDrag,
        extra: DragBoth.with_snap_back(),
        extra: SetCursor { 
            flags: EventFlags::Hover|EventFlags::LeftDrag, 
            icon: CursorIcon::Hand,
        },
        extra: transition!(Offset 4.0 BounceOut default Vec2::ZERO),
    });

    let (send1, recv1) = signal();

    rectangle!((commands, assets) {
        dimension: [400, 50],
        offset: [0, 100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Right,
            color: color!(aqua),
            event: EventFlags::Hover|EventFlags::LeftDrag,
            extra: SetCursor { 
                flags: EventFlags::Hover|EventFlags::LeftDrag, 
                icon: CursorIcon::Hand,
            },
            extra: DragX.with_handler(send1),
        }
    });

    text! ((commands, assets) {
        offset: [300, 100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        extra: recv1.map_recv::<SigText>(|x: f32| format!("<= has value {:.2}!", x))
    });

    let (send2, recv2) = signal();
    let (send3, recv3) = signal();

    rectangle!((commands, assets) {
        dimension: [400, 50],
        offset: [0, -100],
        child: rectangle! {
            dimension: [50, 50],
            anchor: Left,
            color: color!(aqua),
            extra: DragX
                .with_recv(recv2)
                .with_handler(send3),
        }
    });

    material_sprite! ((commands, assets) {
        dimension: [100, 100],
        offset: [-300, -100],
        hitbox: Rect(1),
        event: EventFlags::Hover|EventFlags::LeftDrag,
        material: Circle {
            fill: color!(aqua),
            stroke: color!(blue),
        },
        extra: SetCursor { 
            flags: EventFlags::Hover|EventFlags::LeftDrag, 
            icon: CursorIcon::Hand,
        },
        extra: Handlers::<EvMouseDrag>::new(send2),
    });

    text! ((commands, assets) {
        offset: [300, -100],
        color: color!(gold),
        text: "<= Drag and this will change!",
        extra: recv3.map_recv::<SigText>(|x: f32| format!("<= has value {:.2}!", x))
    });
}
