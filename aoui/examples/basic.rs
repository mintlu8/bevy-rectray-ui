use std::f32::consts::PI;

use bevy_aoui::{*, bundles::*};
use bevy::{prelude::*, sprite::Anchor};
use bevy_egui::{EguiContexts, egui::{self, Slider, ComboBox, Ui}, EguiPlugin};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(AoUIPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, egui_window)
        .run();
}

#[derive(Debug, Component)]
pub struct A;
#[derive(Debug, Component)]
pub struct B;

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

    let b = commands.spawn((AoUISpriteBundle {
        sprite: Sprite { 
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform2D { 
            center: Some(Anchor::Center),
            anchor: Anchor::Center,
            ..Default::default()
        },
        dimension: Dimension::pixels(Vec2::new(50.0, 50.0)),
        texture: assets.load("square.png"),
        ..Default::default()
    },B)).id();

    commands.spawn((AoUISpriteBundle {
        sprite: Sprite { 
            color: Color::CYAN,
            ..Default::default()
        },
        transform: Transform2D { 
            center: Some(Anchor::Center),
            anchor: Anchor::Center,
            ..Default::default()
        },
        dimension: Dimension::pixels(Vec2::new(200.0, 200.0)),
        texture: assets.load("square.png"),
        ..Default::default()
    },A)).add_child(b);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor2 {
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    Center,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
}

fn anchor_ui(ui: &mut Ui, anchor: &mut Anchor, name: &str) {
    let mut anc = match anchor {
        Anchor::BottomLeft => Anchor2::BottomLeft,
        Anchor::BottomCenter => Anchor2::BottomCenter,
        Anchor::BottomRight => Anchor2::BottomRight,
        Anchor::CenterLeft => Anchor2::CenterLeft,
        Anchor::Center => Anchor2::Center,
        Anchor::CenterRight => Anchor2::CenterRight,
        Anchor::TopLeft => Anchor2::TopLeft,
        Anchor::TopCenter => Anchor2::TopCenter,
        Anchor::TopRight => Anchor2::TopRight,
        Anchor::Custom(_) => unreachable!(),
    };

    ComboBox::from_label(name)
        .selected_text(format!("{:?}", anc))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut anc, Anchor2::BottomLeft, "BottomLeft");
            ui.selectable_value(&mut anc, Anchor2::BottomCenter, "BottomCenter");
            ui.selectable_value(&mut anc, Anchor2::BottomRight, "BottomRight");
            ui.selectable_value(&mut anc, Anchor2::CenterLeft, "CenterLeft");
            ui.selectable_value(&mut anc, Anchor2::Center, "Center");
            ui.selectable_value(&mut anc, Anchor2::CenterRight, "CenterRight");
            ui.selectable_value(&mut anc, Anchor2::TopLeft, "TopLeft");
            ui.selectable_value(&mut anc, Anchor2::TopCenter, "TopCenter");
            ui.selectable_value(&mut anc, Anchor2::TopRight, "TopRight");
        }
    );

    
    *anchor = match anc {
        Anchor2::BottomLeft => Anchor::BottomLeft,
        Anchor2::BottomCenter =>  Anchor::BottomCenter,
        Anchor2::BottomRight => Anchor::BottomRight,
        Anchor2::CenterLeft => Anchor::CenterLeft,
        Anchor2::Center => Anchor::Center,
        Anchor2::CenterRight => Anchor::CenterRight,
        Anchor2::TopLeft => Anchor::TopLeft,
        Anchor2::TopCenter => Anchor::TopCenter,
        Anchor2::TopRight => Anchor::TopRight,
    };
}


pub fn egui_window(mut ctx: EguiContexts, 
    mut root: Query<(&mut Transform2D, &mut Dimension), (With<A>, Without<B>)>,
    mut child: Query<(&mut Transform2D, &mut Dimension), (With<B>, Without<A>)>,
) {
    let (mut root, mut root_dim) = root.single_mut();
    let (mut child, mut child_dim) = child.single_mut();
    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        ui.label("Root Entity");
        anchor_ui(ui, &mut root.anchor, "Anchor");
        anchor_ui(ui, &mut root.center.as_mut().unwrap(), "Center");
        ui.add(Slider::new(&mut root.offset.raw_mut().x, -400.0..=400.0).text("Offset X"));
        ui.add(Slider::new(&mut root.offset.raw_mut().y, -400.0..=400.0).text("Offset Y"));
        ui.add(Slider::new(&mut root_dim.raw_mut().x, 0.0..=600.0).text("Dimension X"));
        ui.add(Slider::new(&mut root_dim.raw_mut().y, 0.0..=600.0).text("Dimension Y"));
        ui.add(Slider::new(&mut root.rotation, -PI * 2.0..=PI * 2.0).text("Rotation"));
        ui.add(Slider::new(&mut root.scale.x, 0.0..=10.0).text("Scale X"));
        ui.add(Slider::new(&mut root.scale.y, 0.0..=10.0).text("Scale Y"));
        ui.label("Child Entity");
        anchor_ui(ui, &mut child.anchor, "Child Anchor");
        anchor_ui(ui, &mut child.center.as_mut().unwrap(), "Child Center");
        ui.add(Slider::new(&mut child.offset.raw_mut().x, -400.0..=400.0).text("Offset X"));
        ui.add(Slider::new(&mut child.offset.raw_mut().y, -400.0..=400.0).text("Offset Y"));
        ui.add(Slider::new(&mut child_dim.raw_mut().x, 0.0..=600.0).text("Dimension X"));
        ui.add(Slider::new(&mut child_dim.raw_mut().y, 0.0..=600.0).text("Dimension Y"));
        ui.add(Slider::new(&mut child.rotation, -PI * 2.0..=PI * 2.0).text("Rotation"));
        ui.add(Slider::new(&mut child.scale.x, 0.0..=10.0).text("Scale X"));
        ui.add(Slider::new(&mut child.scale.y, 0.0..=10.0).text("Scale Y"));
        if ui.button("Reset Parent").clicked() {
            root.anchor = Anchor::Center;
            root.center = Some(Anchor::Center);
            root.offset = Size2::ZERO;
            root.rotation = 0.0;
            root.scale = Vec2::ONE;
            *root_dim.raw_mut() = Vec2::new(200.0, 200.0);
        }
        if ui.button("Reset Child").clicked() {
            child.anchor = Anchor::Center;
            child.center = Some(Anchor::Center);
            child.offset = Size2::ZERO;
            child.rotation = 0.0;
            child.scale = Vec2::ONE;
            *child_dim.raw_mut() = Vec2::new(50.0, 50.0);
        }
    });
    
}
