use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, material_sprite};
use bevy_matui::{MatuiPlugin, shapes::{RoundedRectangleMaterial, RoundedShadowMaterial}};

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

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(black),
        extra: fps_signal(|fps: f32, text: &mut Text| {
            format_widget!(text, "FPS: {:.2}", fps);
        })
    });


    material_sprite!((commands, assets) {
        offset: size2!(0, 25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule_image(assets.load("bricks.png"), color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::capsule(color!(black), 20.0),
            z: -1,
        }
    });
    
    material_sprite!((commands, assets) {
        offset: size2!(25%, 25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::from_image(assets.load("bricks.png"), color!(white), 20.0),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 20.0),
            z: -1,
        }
    });


    material_sprite!((commands, assets) {
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::capsule(color!(black), 20.0),
            z: -1,
        }
    });
    material_sprite!((commands, assets) {
        offset: size2!(25%, 0),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(white)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::capsule(color!(black), 5.0),
            z: -1,
        }
    });
    material_sprite!((commands, assets) {
        offset: size2!(-25%, 0),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::capsule(color!(green)).with_stroke((color!(blue), 5.0)),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::capsule(color!(black), 5.0),
            z: -1,
        }
    });
    material_sprite!((commands, assets) {
        offset: size2!(0, -25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::new(color!(white), 20.0),
        child: material_sprite! {
            dimension: size2!(1 + 40 px, 1 + 40 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 20.0),
            z: -1,
        }
    });

    material_sprite!((commands, assets) {
        offset: size2!(25%, -25%),
        dimension: size2!(20%, 20%),
        material: RoundedRectangleMaterial::new(color!(white), 20.0),
        child: material_sprite! {
            dimension: size2!(1 + 10 px, 1 + 10 px),
            material: RoundedShadowMaterial::new(color!(black), 20.0, 5.0),
            z: -1,
        }
    });

    material_sprite!((commands, assets) {
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
