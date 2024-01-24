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
    /// The entire rectangular area of the sprite.
    pub const FULL: Self = Self {
        shape: HitboxShape::Rect,
        scale: Vec2::ONE,
    };
}

impl Hitbox {
    pub fn contains(&self, rect: &RotatedRect, point: Vec2) -> bool {
        let local = point - rect.center();
        let x = rect.affine.transform_vector2(Vec2::new(0.5, 0.0));
        let y = rect.affine.transform_vector2(Vec2::new(0.0, 0.5));
        let x_squared = (x * self.scale.x).length_squared();
        let y_squared = (y * self.scale.y).length_squared();
        match self.shape {
            HitboxShape::Rect => {
                local.dot(x).abs() < x_squared && local.dot(y).abs() < y_squared
            },
            HitboxShape::Ellipse => {
                local.dot(x).abs() / x_squared + local.dot(y).abs() / y_squared <= 1.0
            }
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
