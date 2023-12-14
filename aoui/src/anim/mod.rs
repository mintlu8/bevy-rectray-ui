use bevy::{ecs::schedule::{SystemSet, IntoSystemConfigs, IntoSystemSetConfigs}, app::{Update, Plugin}};

use ::interpolation::Ease;
pub use ::interpolation::EaseFunction;
mod interpolation;

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct InterpolationSet;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SystemSet)]
pub struct InterpolationUpdateSet;

pub use interpolation::{Interpolate, Offset, Rotation, Scale};

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
                interpolation::interpolate_color,
                interpolation::interpolate_opacity,
            ).in_set(InterpolationSet))
            .add_systems(Update, interpolation::update_interpolate.in_set(InterpolationUpdateSet))
        ;
    }
}