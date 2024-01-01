use bevy::ecs::entity::Entity;
use bevy::prelude::Vec2;
use bevy::prelude::Reflect;

use crate::{layout::LayoutControl, Anchor};

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
