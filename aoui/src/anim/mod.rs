use bevy::{ecs::schedule::{SystemSet, IntoSystemConfigs, IntoSystemSetConfigs}, app::{Update, Plugin}};

mod interpolation;

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