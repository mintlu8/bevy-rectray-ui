use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::ecs::entity::Entity;
use bevy::math::bool;
use bevy::prelude::Vec2;
use bevy::prelude::Reflect;

use crate::{layout::LayoutControl, Anchor};

/// Direction of a layout.
pub trait Direction: Sized + Debug + Send + Sync + 'static {
    type Pos: Direction;
    fn unit() -> Vec2;
    fn main(v: Vec2) -> Vec2;
    fn main_vec(v: f32) -> Vec2;
    fn len(v: Vec2) -> f32;
    fn project(v: Vec2) -> f32;
    fn side(v: Vec2) -> Vec2;
    fn side_vec(v: f32) -> Vec2;
    fn signum(v: Vec2) -> Vec2;
    fn reversed() -> bool;
    fn bucket(anc: Anchor) -> Trinary;
}


/// A pair of orthogonal direction.
pub trait DirectionPair {}

/// The direction +X, left to right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X {}

/// The direction +Y, bottom to top.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Y {}

/// Reverse a direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rev<T> {
    _Phantom(PhantomData<T>, X)
}

impl Direction for X {

    type Pos = Self;

    fn unit() -> Vec2 {
        Vec2::new(1.0, 0.0)
    }

    fn main(v: Vec2) -> Vec2 {
        Vec2::new(v.x, 0.0)
    }

    fn main_vec(v: f32) -> Vec2 {
        Vec2::new(v, 0.0)
    }

    fn len(v: Vec2) -> f32 {
        v.x
    }

    fn project(v: Vec2) -> f32 {
        v.x
    }

    fn side(v: Vec2) -> Vec2 {
        Vec2::new(0.0, v.y)
    }

    fn side_vec(v: f32) -> Vec2 {
        Vec2::new(0.0, v)
    }

    fn signum(v: Vec2) -> Vec2 {
        Vec2::new(v.x.signum(), 0.0)
    }

    fn reversed() -> bool {
        false
    }

    fn bucket(anchor: Anchor) -> Trinary {
        match anchor.x() {
            x if x < -0.16 => Trinary::Neg,
            x if x > 0.16 => Trinary::Pos,
            _ => Trinary::Mid,
        }
    }
}

impl Direction for Y {

    type Pos = Self;

    fn unit() -> Vec2 {
        Vec2::new(0.0, 1.0)
    }

    fn main(v: Vec2) -> Vec2 {
        Vec2::new(0.0, v.y)
    }

    fn main_vec(v: f32) -> Vec2 {
        Vec2::new(0.0, v)
    }

    fn len(v: Vec2) -> f32 {
        v.y
    }

    fn project(v: Vec2) -> f32 {
        v.x
    }

    fn side(v: Vec2) -> Vec2 {
        Vec2::new(v.x, 0.0)
    }

    fn side_vec(v: f32) -> Vec2 {
        Vec2::new(v, 0.0)
    }

    fn signum(v: Vec2) -> Vec2 {
        Vec2::new(0.0, v.y.signum())
    }

    fn reversed() -> bool {
        false
    }

    fn bucket(anchor: Anchor) -> Trinary {
        match anchor.y() {
            y if y < -0.16 => Trinary::Neg,
            y if y > 0.16 => Trinary::Pos,
            _ => Trinary::Mid,
        }
    }
}


impl<T: Direction> Direction for Rev<T> {

    type Pos = T::Pos;

    fn unit() -> Vec2 {
        -T::unit()
    }

    fn main(v: Vec2) -> Vec2 {
        -T::main(v)
    }

    fn main_vec(v: f32) -> Vec2 {
        -T::main_vec(v)
    }

    fn len(v: Vec2) -> f32 {
        T::len(v)
    }

    fn project(v: Vec2) -> f32 {
        -T::project(v)
    }

    fn side(v: Vec2) -> Vec2 {
        T::side(v)
    }

    fn side_vec(v: f32) -> Vec2 {
        T::side_vec(v)
    }

    fn signum(v: Vec2) -> Vec2 {
        -T::signum(v)
    }

    fn reversed() -> bool {
        !T::reversed()
    }

    fn bucket(anc: Anchor) -> Trinary {
        match T::bucket(anc) {
            Trinary::Neg => Trinary::Pos,
            Trinary::Mid => Trinary::Mid,
            Trinary::Pos => Trinary::Neg,
        }
    }
}

impl DirectionPair for (X, Y) {}
impl DirectionPair for (Rev<X>, Y) {}
impl DirectionPair for (X, Rev<Y>) {}
impl DirectionPair for (Rev<X>, Rev<Y>) {}

/// Direction and stretch of a layout.
pub trait StretchDir: Direction {
    const STRETCH: bool;
}

impl StretchDir for X {
    const STRETCH: bool = false;
}

impl StretchDir for Y {
    const STRETCH: bool = false;
}

impl<T> StretchDir for Rev<T> where T: StretchDir {
    const STRETCH: bool = T::STRETCH;
}

/// A direction that also signifies stretch.
#[derive(Debug, Clone, Copy)]
pub enum Stretch<T: Direction> {
    _Phantom(PhantomData<T>)
}

impl<T> Direction for Stretch<T> where T: Direction {
    type Pos = T::Pos;
    fn unit() -> Vec2 { T::unit() }
    fn main(v: Vec2) -> Vec2 { T::main(v) }
    fn main_vec(v: f32) -> Vec2 { T::main_vec(v) }
    fn len(v: Vec2) -> f32 { T::len(v) }
    fn project(v: Vec2) -> f32 { T::project(v) }
    fn side(v: Vec2) -> Vec2 { T::side(v) }
    fn side_vec(v: f32) -> Vec2 { T::side_vec(v) }
    fn signum(v: Vec2) -> Vec2 { T::signum(v) }
    fn reversed() -> bool { T::reversed() }
    fn bucket(anc: Anchor) -> Trinary { T::bucket(anc) }
}

impl<T> StretchDir for Stretch<T> where T: Direction {
    const STRETCH: bool = true;
}

impl DirectionPair for (Stretch<X>, Y) {}
impl DirectionPair for (Stretch<Rev<X>>, Y) {}
impl DirectionPair for (Stretch<X>, Rev<Y>) {}
impl DirectionPair for (Stretch<Rev<X>>, Rev<Y>) {}

/// Horizontal or Vertical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum Axis {
    #[default]
    Horizontal, Vertical,
}

impl From<bool> for Axis {
    fn from(value: bool) -> Self {
        match value {
            false => Axis::Horizontal,
            true => Axis::Vertical,
        }
    }
}


/// Order items are laid out in a [`Container`](crate::layout::Container).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum LayoutDir {
    LeftToRight, RightToLeft,
    BottomToTop, TopToBottom,
}

impl LayoutDir {
    pub fn is_reversed(&self) -> bool {
        matches!(self, Self::RightToLeft|Self::TopToBottom)
    }

    pub fn flip(&self) -> Self {
        match self {
            LayoutDir::LeftToRight => LayoutDir::RightToLeft,
            LayoutDir::RightToLeft => LayoutDir::LeftToRight,
            LayoutDir::BottomToTop => LayoutDir::TopToBottom,
            LayoutDir::TopToBottom => LayoutDir::BottomToTop,
        }
    }

    pub fn transpose(&self) -> Self {
        match self {
            LayoutDir::LeftToRight => LayoutDir::BottomToTop,
            LayoutDir::RightToLeft => LayoutDir::TopToBottom,
            LayoutDir::BottomToTop => LayoutDir::LeftToRight,
            LayoutDir::TopToBottom => LayoutDir::RightToLeft,
        }
    }
}

impl From<LayoutDir> for Axis {
    fn from(value: LayoutDir) -> Self {
        match value {
            LayoutDir::LeftToRight => Axis::Horizontal,
            LayoutDir::RightToLeft => Axis::Horizontal,
            LayoutDir::BottomToTop => Axis::Vertical,
            LayoutDir::TopToBottom => Axis::Vertical,
        }
    }
}

impl From<&LayoutDir> for Axis {
    fn from(value: &LayoutDir) -> Self {
        match value {
            LayoutDir::LeftToRight => Axis::Horizontal,
            LayoutDir::RightToLeft => Axis::Horizontal,
            LayoutDir::BottomToTop => Axis::Vertical,
            LayoutDir::TopToBottom => Axis::Vertical,
        }
    }
}

/// Where items are aligned to in a [`Container`](crate::layout::Container).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Alignment {
    Center, Bottom, Top, Left, Right
}


impl Alignment {
    /// Returns -1 if Lo, 1 if Hi
    pub fn transpose(&self) -> Self {
        match self {
            Alignment::Center => Alignment::Center,
            Alignment::Bottom => Alignment::Top,
            Alignment::Top => Alignment::Bottom,
            Alignment::Left => Alignment::Right,
            Alignment::Right => Alignment::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub(crate) enum Binary {
    Lo, Hi
}

impl From<LayoutDir> for Binary {
    fn from(value: LayoutDir) -> Self {
        match value {
            LayoutDir::RightToLeft|LayoutDir::TopToBottom => Self::Lo,
            LayoutDir::LeftToRight|LayoutDir::BottomToTop => Self::Hi,
        }
    }
}

impl From<&LayoutDir> for Binary {
    fn from(value: &LayoutDir) -> Self {
        match value {
            LayoutDir::RightToLeft|LayoutDir::TopToBottom => Self::Lo,
            LayoutDir::LeftToRight|LayoutDir::BottomToTop => Self::Hi,
        }
    }
}

/// Info for positioning an item in a [`Container`].
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct LayoutItem {
    /// entity of the item
    pub entity: Entity,
    /// anchor of this item
    pub anchor: Anchor,
    /// dimension of this item
    pub dimension: Vec2,
    /// Force a linebreak on or after this item.
    pub control: LayoutControl,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trinary {
    Neg, Mid, Pos
}

impl From<Alignment> for Trinary {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Center => Self::Mid,
            Alignment::Left|Alignment::Bottom => Self::Neg,
            Alignment::Right|Alignment::Top => Self::Pos,
        }
    }
}

impl From<&Alignment> for Trinary {
    fn from(value: &Alignment) -> Self {
        match value {
            Alignment::Center => Self::Mid,
            Alignment::Left|Alignment::Bottom => Self::Neg,
            Alignment::Right|Alignment::Top => Self::Pos,
        }
    }
}

pub(super) fn posx(v: Vec2) -> Vec2 {
    Vec2::new(v.x, 0.0)
}

pub(super) fn negx(v: Vec2) -> Vec2 {
    Vec2::new(-v.x, 0.0)
}

pub(super) fn posy(v: Vec2) -> Vec2 {
    Vec2::new(0.0, v.y)
}

pub(super) fn negy(v: Vec2) -> Vec2 {
    Vec2::new(0.0, -v.y)
}
