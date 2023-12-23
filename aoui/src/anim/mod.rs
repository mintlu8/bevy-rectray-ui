//! Interpolation module for bevy_aoui.
//! 
//! # Getting Started:
//! 
//! We use a CSS-like syntax in the `transition!` macro:
//! ```
//! # /*
//! text! (commands {
//!     text: "I'm Spinning!",
//!     extra: OpacityWriter,
//!     extra: transition!(
//!         Offset 2 Linear default Vec2::new(-200.0, 0.0);
//!         Opacity 0.5 CubicOut init (0.0, 1.0);
//!         Rotation 2 Linear repeat (0.0, 2.0 * PI);
//!         Color 2 Linear loop [cyan, blue];
//!     )
//! });
//! # */
//! ```
//! The syntax is
//! ```js
//! transition!(
//!     Field seconds Easing mode value
//! )
//! ```
//! 
//! ## Mode
//! 
//! * default: 
//! A watcher that you can write to either manually or with signals,
//! value has to be a single value.
//! 
//! * init:
//! A watcher that runs once on initialization.
//! 
//! * repeat
//! Repeat the animation forever, time value is `0->1, 0->1, 0->1, ...`
//! 
//! * loop
//! Repeat the animation forever, time value is `0->1->0->1->0->1, ...`
//! 
//! 
//! ## Easing
//! 
//! * Linear
//! * [Ease Functions](EaseFunction)
//! * Cubic Bézier `[f32; 4]`
//! 
//! ## Value
//! 
//! * Single Value
//! * Tuple `(T, T)`
//! * Gradient `[(T, 0.0..=1.0); N]`
//! 
//! # Smart Tweening
//! 
//! `Interpolation` is a simple state machine. When setting a new target:
//! 
//! * If target is the same, ignore.
//! * If target is the source of current animation, reverse.
//! * Otherwise interpolate to the target.

use bevy::{ecs::schedule::{SystemSet, IntoSystemConfigs, IntoSystemSetConfigs}, app::{Update, Plugin}};

use ::interpolation::Ease;
pub use ::interpolation::EaseFunction;
mod interpolation;
pub use interpolation::{Interpolate, Offset, Rotation, Scale, Index};
//mod state_machine;
//pub use state_machine::WidgetState;

/// A easing function.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Easing {
    #[default]
    Linear,
    Ease(EaseFunction),
    Bezier([f32; 4]),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Playback {
    #[default]
    Once,
    Loop,
    Repeat,
}

impl Playback {
    pub fn is_once(&self) -> bool {
        self == &Playback::Once
    }
}

impl Easing {
    pub fn get(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::Ease(f) => t.calc(*f),
            Easing::Bezier([a,b,c,d]) => ::interpolation::cub_bez(a, b, c, d, &t),
        }
    }
}

/// SystemSet for interpolation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct InterpolationSet;

/// SystemSet for time update of interpolation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct InterpolationUpdateSet;

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .configure_sets(Update, InterpolationSet)
            .configure_sets(Update, InterpolationUpdateSet.after(InterpolationSet))
            .add_systems(Update, (
                interpolation::interpolate_offset,
                interpolation::interpolate_rotation,
                interpolation::interpolate_scale,
                interpolation::interpolate_dimension,
                interpolation::interpolate_index,
                interpolation::interpolate_color,
                interpolation::interpolate_opacity,
            ).in_set(InterpolationSet))
            .add_systems(Update, interpolation::update_interpolate.in_set(InterpolationUpdateSet))
        ;
    }
}