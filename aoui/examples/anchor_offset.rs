//! The bread and butter of `bevy_aoui`.
//! 
//! The parent sprite is parented to the window,
//! the child sprite is parented to the parent.
//! 
//! You can experiment on how each parameter affects this simple system.
#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use bevy_aoui::{*, DimensionMut};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, Slider, ComboBox, Ui}, EguiPlugin};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(AouiPlugin)
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

    rectangle!((commands, assets) {
        color: color!(cyan),
        dimension: [200, 200],
        extra: A,
        child: rectangle! {
            color: color!(red),
            dimension: [50, 50],
            extra: B,
        }
    });
}


fn anchor_ui(ui: &mut Ui, anchor: &mut Anchor, name: &str) {
    let mut anc = anchor.str_name();
    ComboBox::from_label(name)
        .selected_text(anc.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut anc, "BottomLeft", "BottomLeft");
            ui.selectable_value(&mut anc, "BottomCenter", "BottomCenter");
            ui.selectable_value(&mut anc, "BottomRight", "BottomRight");
            ui.selectable_value(&mut anc, "CenterLeft", "CenterLeft");
            ui.selectable_value(&mut anc, "Center", "Center");
            ui.selectable_value(&mut anc, "CenterRight", "CenterRight");
            ui.selectable_value(&mut anc, "TopLeft", "TopLeft");
            ui.selectable_value(&mut anc, "TopCenter", "TopCenter");
            ui.selectable_value(&mut anc, "TopRight", "TopRight");
        }
    );

    
    *anchor = match anc {
        "BottomLeft" => Anchor::BottomLeft,
        "BottomCenter" =>  Anchor::BottomCenter,
        "BottomRight" => Anchor::BottomRight,
        "CenterLeft" => Anchor::CenterLeft,
        "Center" => Anchor::Center,
        "CenterRight" => Anchor::CenterRight,
        "TopLeft" => Anchor::TopLeft,
        "TopCenter" => Anchor::TopCenter,
        "TopRight" => Anchor::TopRight,
        _ => unreachable!()
    };
}


pub fn egui_window(mut ctx: EguiContexts, 
    mut root: Query<(&mut Transform2D, DimensionMut), (With<A>, Without<B>)>,
    mut child: Query<(&mut Transform2D, DimensionMut), (With<B>, Without<A>)>,
) {
    let (mut root, mut root_dim) = root.single_mut();
    let (mut child, mut child_dim) = child.single_mut();
    egui::Window::new("Console").show(ctx.ctx_mut(), |ui| {
        ui.label("Root Entity");
        anchor_ui(ui, &mut root.anchor, "Anchor");
        anchor_ui(ui, &mut root.center, "Center");
        ui.add(Slider::new(&mut root.offset.raw_mut().x, -400.0..=400.0).text("Offset X"));
        ui.add(Slider::new(&mut root.offset.raw_mut().y, -400.0..=400.0).text("Offset Y"));
        ui.add(Slider::new(&mut root_dim.raw_mut().x, 0.0..=600.0).text("Dimension X"));
        ui.add(Slider::new(&mut root_dim.raw_mut().y, 0.0..=600.0).text("Dimension Y"));
        ui.add(Slider::new(&mut root.rotation, -PI * 2.0..=PI * 2.0).text("Rotation"));
        ui.add(Slider::new(&mut root.scale.x, 0.0..=10.0).text("Scale X"));
        ui.add(Slider::new(&mut root.scale.y, 0.0..=10.0).text("Scale Y"));
        ui.label("Child Entity");
        anchor_ui(ui, &mut child.anchor, "Child Anchor");
        anchor_ui(ui, &mut child.center, "Child Center");
        ui.add(Slider::new(&mut child.offset.raw_mut().x, -400.0..=400.0).text("Offset X"));
        ui.add(Slider::new(&mut child.offset.raw_mut().y, -400.0..=400.0).text("Offset Y"));
        ui.add(Slider::new(&mut child_dim.raw_mut().x, 0.0..=600.0).text("Dimension X"));
        ui.add(Slider::new(&mut child_dim.raw_mut().y, 0.0..=600.0).text("Dimension Y"));
        ui.add(Slider::new(&mut child.rotation, -PI * 2.0..=PI * 2.0).text("Rotation"));
        ui.add(Slider::new(&mut child.scale.x, 0.0..=10.0).text("Scale X"));
        ui.add(Slider::new(&mut child.scale.y, 0.0..=10.0).text("Scale Y"));
        if ui.button("Reset Parent").clicked() {
            root.anchor = Anchor::Center;
            root.center = Anchor::Center;
            root.offset = Size2::ZERO;
            root.rotation = 0.0;
            root.scale = Vec2::ONE;
            *root_dim.raw_mut() = Vec2::new(200.0, 200.0);
        }
        if ui.button("Reset Child").clicked() {
            child.anchor = Anchor::Center;
            child.center = Anchor::Center;
            child.offset = Size2::ZERO;
            child.rotation = 0.0;
            child.scale = Vec2::ONE;
            *child_dim.raw_mut() = Vec2::new(50.0, 50.0);
        }
    });
    
}
