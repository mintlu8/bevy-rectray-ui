use bevy::ecs::entity::Entity;
use bevy::prelude::Vec2;
use bevy::prelude::Reflect;

use crate::{layout::LayoutControl, Anchor};

/// Horizontal or Vertical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub(crate) enum Axis {
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


/// Order items are laid out in a [`Container`](crate::Container).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum FlexDir {
    LeftToRight, RightToLeft,
    BottomToTop, TopToBottom,
}

impl FlexDir {
    pub fn is_reversed(&self) -> bool {
        matches!(self, Self::RightToLeft|Self::TopToBottom)
    }

    pub fn flip(&self) -> Self {
        match self {
            FlexDir::LeftToRight => FlexDir::RightToLeft,
            FlexDir::RightToLeft => FlexDir::LeftToRight,
            FlexDir::BottomToTop => FlexDir::TopToBottom,
            FlexDir::TopToBottom => FlexDir::BottomToTop,
        }
    }

    pub fn transpose(&self) -> Self {
        match self {
            FlexDir::LeftToRight => FlexDir::BottomToTop,
            FlexDir::RightToLeft => FlexDir::TopToBottom,
            FlexDir::BottomToTop => FlexDir::LeftToRight,
            FlexDir::TopToBottom => FlexDir::RightToLeft,
        }
    }
}

impl From<FlexDir> for Axis {
    fn from(value: FlexDir) -> Self {
        match value {
            FlexDir::LeftToRight => Axis::Horizontal,
            FlexDir::RightToLeft => Axis::Horizontal,
            FlexDir::BottomToTop => Axis::Vertical,
            FlexDir::TopToBottom => Axis::Vertical,
        }
    }
}

impl From<&FlexDir> for Axis {
    fn from(value: &FlexDir) -> Self {
        match value {
            FlexDir::LeftToRight => Axis::Horizontal,
            FlexDir::RightToLeft => Axis::Horizontal,
            FlexDir::BottomToTop => Axis::Vertical,
            FlexDir::TopToBottom => Axis::Vertical,
        }
    }
}

/// Where items are aligned to in a [`Container`](crate::Container).
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

impl From<FlexDir> for Binary {
    fn from(value: FlexDir) -> Self {
        match value {
            FlexDir::RightToLeft|FlexDir::TopToBottom => Self::Lo,
            FlexDir::LeftToRight|FlexDir::BottomToTop => Self::Hi,
        }
    }
}

impl From<&FlexDir> for Binary {
    fn from(value: &FlexDir) -> Self {
        match value {
            FlexDir::RightToLeft|FlexDir::TopToBottom => Self::Lo,
            FlexDir::LeftToRight|FlexDir::BottomToTop => Self::Hi,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Trinary {
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


pub(super) fn hbucket(anchor: &Anchor) -> Trinary {
    match anchor.x() {
        x if x < -0.16 => Trinary::Neg,
        x if x > 0.16 => Trinary::Pos,
        _ => Trinary::Mid,
    }
}


pub(super) fn vbucket(anchor: &Anchor) -> Trinary {
    match anchor.y() {
        y if y < -0.16 => Trinary::Neg,
        y if y > 0.16 => Trinary::Pos,
        _ => Trinary::Mid,
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
