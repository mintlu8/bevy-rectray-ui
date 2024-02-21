//! A simple material UI renderer for the bevy engine, built on top of [`bevy_rectray`].
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
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]


use aoui::anim::{FgsmPairing, InterpolateAssociation};
use bevy::{asset::load_internal_asset, render::render_resource::Shader, ecs::schedule::IntoSystemConfigs, sprite::Material2dPlugin};
use bevy::app::{Plugin, PostUpdate, Update};
use bevy_rectray::schedule::AouiStoreOutputSet;

use crate::widgets::input::display_if_has_text;
use crate::widgets::menu::{run_dropdown_signals, run_oneshot_menu};
use crate::widgets::slider::{slider_rebase, sync_progress_bar};
use crate::widgets::states::{toggle_opacity_signal, FocusColors, ToggleRotation};
use crate::widgets::window_collapse_transfer;
use crate::shaders::*;
use crate::widgets::{input::text_placeholder, menu::rebuild_dropdown_children, states::{ButtonColors, ToggleColors, ToggleOpacity}, toggle::{ToggleDialDimension, ToggleDialOffset}, StrokeColors};

pub mod shaders;
pub mod builders;
pub mod widgets;
pub mod style;

#[doc(hidden)]
pub use bevy;

#[doc(hidden)]
pub use bevy_rectray as aoui;
#[doc(hidden)]
pub use bevy_defer as defer;

pub struct MatuiPlugin;

impl Plugin for MatuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<RoundedRectangleMaterial>::default());
        app.add_plugins(Material2dPlugin::<RoundedShadowMaterial>::default());
        load_internal_asset!(app, ROUNDED_RECTANGLE_SHADER, "shaders/rounded_rectangle.wgsl", Shader::from_wgsl);
        load_internal_asset!(app, ROUNDED_SHADOW_SHADER, "shaders/rounded_shadow.wgsl", Shader::from_wgsl);
        app.add_systems(PostUpdate, (
            sync_rounded_rect,
            sync_rounded_shadow,
        ).in_set(StoreOutputSet));
        app.add_systems(Update, (
            interpolate_stroke_color,
            slider_rebase,
            sync_progress_bar,
            window_collapse_transfer,
            toggle_opacity_signal,
            run_dropdown_signals,
            StrokeColoring::system,
        ));
        app.add_plugins(ButtonColors::plugin());
        app.add_plugins(StrokeColors::<ButtonColors>::plugin());
        app.add_plugins(ToggleColors::plugin());
        app.add_plugins(StrokeColors::<ToggleColors>::plugin());
        app.add_plugins(ToggleOpacity::plugin());
        app.add_plugins(ToggleRotation::plugin());
        app.add_plugins(ToggleDialOffset::plugin());
        app.add_plugins(ToggleDialDimension::plugin());
        app.add_plugins(FocusColors::plugin());
        app.add_plugins(StrokeColors::<FocusColors>::plugin());
        app.add_systems(Update, (
            rebuild_dropdown_children,
            text_placeholder,
            display_if_has_text,
            run_oneshot_menu,
        ).in_set(StoreOutputSet));
    }
}
