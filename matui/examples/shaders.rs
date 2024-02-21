use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rectray::{material_sprite, util::RCommands, AouiPlugin, Coloring};
use bevy_matui::{MatuiPlugin, shaders::{RoundedRectangleMaterial, RoundedShadowMaterial}};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        .add_plugins(MatuiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .run();
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


    material_sprite!(commands {
        offset: size2!(0, 25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule_image(commands.load("bricks.png"), color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::capsule(color!(black), 20.0),
            extra: Coloring{ color: color!(black) },
            z: -1,
        }
    });

    material_sprite!(commands {
        offset: size2!(25%, 25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::from_image(commands.load("bricks.png"), color!(white), 20.0)
            .with_stroke((color!(brown), 5.0)),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 20.0),
            z: -1,
        }
    });


    material_sprite!(commands {
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::capsule(color!(black), 20.0),
            z: -1,
        }
    });
    material_sprite!(commands {
        offset: size2!(25%, 0),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::capsule(color!(black), 5.0),
            z: -1,
        }
    });
    material_sprite!(commands {
        offset: size2!(-25%, 0),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(green)).with_stroke((color!(blue), 5.0)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::capsule(color!(black), 5.0),
            z: -1,
        }
    });
    material_sprite!(commands {
        offset: size2!(-25%, 25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::rect(color!(green)).with_stroke((color!(red), 5.0)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::new(color!(black), 0.0, 5.0),
            z: -1,
        }
    });
    material_sprite!(commands {
        offset: size2!(0, -25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::new(color!(white), 20.0),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 20.0),
            z: -1,
        }
    });

    material_sprite!(commands {
        offset: size2!(25%, -25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::new(color!(white), 20.0),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 5.0),
            z: -1,
        }
    });

    material_sprite!(commands {
        offset: size2!(-25%, -25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::new(color!(white), 20.0)
            .with_stroke((color!(red), 2.0)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 5.0),
            z: -1,
        }
    });

}
