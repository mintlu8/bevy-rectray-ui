use std::ops::{Deref, DerefMut};

use bevy::math::Vec2;

/// A conditional Vec modifier.
/// 
/// If a field is `NAN`, all operators with another `Vec2` do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CondVec(Vec2);

impl CondVec {
    pub fn new(v: Vec2) -> Self {
        CondVec(v)
    }

    pub fn x(v: f32) -> Self {
        CondVec(Vec2::new(v, f32::NAN))
    }

    pub fn y(v: f32) -> Self {
        CondVec(Vec2::new(f32::NAN, v))
    }

    pub fn cond(&self, f: Vec2, t: Vec2) -> Vec2{
        Vec2::new(
            if self.x.is_nan() {f.x} else {t.x}, 
            if self.y.is_nan() {f.y} else {t.y}, 
        )
    }

    pub fn normalized(&self) -> Self {
        CondVec(Vec2::new(
            if self.x.is_nan() {self.x} else {1.0}, 
            if self.y.is_nan() {self.y} else {1.0}, 
        ))
    }
}

impl From<Vec2> for CondVec {
    fn from(value: Vec2) -> Self {
        CondVec(value)
    }
}

impl Deref for CondVec {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CondVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_op {
    ($($trait: ident, $fn: ident, $op:tt);* $(;)?) => {
        $(impl ::std::ops::$trait<Vec2> for CondVec {
            type Output = Vec2;
            fn $fn(self, rhs: Vec2) -> Self::Output {
                Vec2::new(
                    if self.x.is_nan() {rhs.x} else {self.x $op rhs.x}, 
                    if self.y.is_nan() {rhs.y} else {self.y $op rhs.y}, 
                )
            }
        }

        impl ::std::ops::$trait<CondVec> for Vec2 {
            type Output = Vec2;
            fn $fn(self, rhs: CondVec) -> Self::Output {
                Vec2::new(
                    if rhs.x.is_nan() {self.x} else {self.x $op rhs.x}, 
                    if rhs.y.is_nan() {self.y} else {self.y $op rhs.y}, 
                )
            }
        })*

    };
}

impl_op!(
    Add, add, +;
    Sub, sub, -;
    Mul, mul, *;
    Div, div, /;
    Rem, rem, %;
);