use std::ops::{Mul, MulAssign};

use bevy::prelude::*;

use crate::RotatedRect;

/// Shape of a hitbox.
#[derive(Debug, Clone, Copy, Reflect)]
#[non_exhaustive]
pub enum HitboxShape{
    Rect,
    Ellipse,
}

/// Provides cursor detection on [`RotatedRect`]
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Hitbox {
    pub shape: HitboxShape,
    pub scale: Vec2,
}

impl Default for Hitbox {
    fn default() -> Self {
        Self {
            shape: HitboxShape::Rect,
            scale: Vec2::ONE,
        }
    }
}

impl Hitbox {
    pub const FULL: Self = Self {
        shape: HitboxShape::Rect,
        scale: Vec2::ONE,
    };

    pub const ELLIPSE: Self = Self {
        shape: HitboxShape::Ellipse,
        scale: Vec2::ONE,
    };
}

impl Hitbox {

    pub fn rect(scale: f32) -> Self {
        Self {
            shape: HitboxShape::Rect,
            scale: Vec2::splat(scale),
        }
    }

    pub fn ellipse(scale: f32) -> Self {
        Self {
            shape: HitboxShape::Ellipse,
            scale: Vec2::splat(scale),
        }
    }

    pub fn contains(&self, rect: &RotatedRect, point: Vec2) -> bool {
        let local_pt = rect.local_space(point);
        let local_dim = rect.dimension * self.scale / 2.0;
        match self.shape {
            HitboxShape::Rect => local_pt.abs().cmple(local_dim.abs()).all(),
            HitboxShape::Ellipse =>
                local_pt.x.powi(2) / local_dim.x.powi(2) +
                local_pt.y.powi(2) / local_dim.y.powi(2) <= 1.0
        }
    }
}

impl Mul<Vec2> for Hitbox {
    type Output = Hitbox;

    fn mul(mut self, rhs: Vec2) -> Self::Output {
        self.scale *= rhs;
        self
    }
}

impl MulAssign<Vec2> for Hitbox {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.scale *= rhs;
    }
}