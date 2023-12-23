//! A simple material UI renderer for the bevy engine, built on top of [`bevy_aoui`].
//! 
//! # Rendering features:
//! 
//! We provide shaders to render capsules, rounded rectangles and their drop shadows.
//! 
//! # Widgets
//! 
//! We provide widgets that loosely follows google's material design guidelines.
//! 
//! They should provide an out of the box consistent look at the cost of some customizability.
//! 
//! * Button
//! * TextBox
//! * Draggable Frame
//! * Separator
//! * Check Button
//! * Radio Button
//! * Toggle
//! * Slider
//! * Progress Bar
//! * Divider
//! * Tabs


use bevy::{app::{Plugin, PostUpdate, Update}, asset::load_internal_asset, render::render_resource::Shader, ecs::schedule::IntoSystemConfigs, sprite::Material2dPlugin};
use bevy_aoui::schedule::AouiStoreOutputSet;
use shapes::CAPSULE_SHADER;

use crate::{shapes::*, widgets::{btn_color_change, toggle_color_change, toggle_dial_change}};

pub mod shapes;
pub mod builders;
pub mod widgets;

#[doc(hidden)]
pub use bevy_aoui as aoui;

pub struct MatuiPlugin;

impl Plugin for MatuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<CapsuleMaterial>::default());
        app.add_plugins(Material2dPlugin::<CapsuleShadowMaterial>::default());
        app.add_plugins(Material2dPlugin::<RoundedRectangleMaterial>::default());
        app.add_plugins(Material2dPlugin::<RoundedShadowMaterial>::default());
        load_internal_asset!(app, CAPSULE_SHADER, "shaders/capsule.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, CAPSULE_SHADOW_SHADER, "shaders/capsule_shadow.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, ROUNDED_RECTANGLE_SHADER, "shaders/rounded_rectangle.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, ROUNDED_SHADOW_SHADER, "shaders/rounded_shadow.wgsl", Shader::from_wgsl);
        app.add_systems(PostUpdate, (
            sync_capsule,
            sync_capsule_shadow,
            sync_rounded_rect,
            sync_rounded_shadow,
        ).in_set(AouiStoreOutputSet));
        app.add_systems(Update, (
            interpolate_capsule_color,
            interpolate_round_rect_color,
        ));
        app.add_systems(Update, (
            btn_color_change,
            toggle_color_change,
            toggle_dial_change
        ).in_set(AouiStoreOutputSet));
    }
}
