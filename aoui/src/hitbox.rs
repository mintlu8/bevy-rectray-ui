use std::ops::{Mul, MulAssign, BitAnd};

use bevy::prelude::*;

use crate::RotatedRect;

#[derive(Debug, Clone, Copy, Reflect)]
#[non_exhaustive]
pub enum HitboxShape{
    Rect,
    Ellipse,
}

/// Provides cursor detection on [`RotatedRect`]
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Hitbox<TFLag: BitAnd<TFLag, Output = TFLag> + Copy + Default + Eq + Send + Sync=u32>{
    pub shape: HitboxShape,
    pub scale: Vec2,
    pub flags: TFLag,
}

impl<T> Default for Hitbox<T> where T: BitAnd<T, Output = T> + Copy + Default + Eq + Send + Sync {
    fn default() -> Self {
        Self {
            shape: HitboxShape::Rect,
            scale: Vec2::ONE,
            flags: Default::default(),
        }
    }
}

impl Hitbox {
    pub const FULL: Self = Self {
        shape: HitboxShape::Rect,
        scale: Vec2::ONE,
        flags: 0,
    };

    pub const ELLIPSE: Self = Self {
        shape: HitboxShape::Ellipse,
        scale: Vec2::ONE,
        flags: 0,
    };
}

impl<T> Hitbox<T> where T: BitAnd<T, Output = T> + Copy + Default + Eq + Send + Sync {

    pub fn rect(scale: f32) -> Self {
        Self {
            shape: HitboxShape::Rect,
            scale: Vec2::splat(scale),
            flags: Default::default(),
        }
    }

    pub fn ellipse(scale: f32) -> Self {
        Self {
            shape: HitboxShape::Ellipse,
            scale: Vec2::splat(scale),
            flags: Default::default(),
        }
    }

    pub fn contains(&self, rect: &RotatedRect, point: Vec2) -> bool {
        let local_pt = Vec2::from_angle(-rect.rotation).rotate(point - rect.center);
        let local_dim = rect.dimension * self.scale / 2.0;
        match self.shape {
            HitboxShape::Rect => local_pt.abs().cmple(local_dim.abs()).all(),
            HitboxShape::Ellipse =>
                local_pt.x.powi(2) / local_dim.x.powi(2) +
                local_pt.y.powi(2) / local_dim.y.powi(2) <= 1.0
        }
    }

    pub fn with_flag(self, flags: T) -> Self {
        Self {
            shape: self.shape,
            scale: self.scale,
            flags,
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