use bevy::prelude::Vec2;
use bevy::sprite::Anchor;
use bevy::prelude::Reflect;

use crate::FlexControl;

/// Horizontal or Vertical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub(crate) enum Axis {
    Horizontal, Vertical,
}

impl Axis {
    pub fn rev(&self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

impl From<bool> for Axis {
    fn from(value: bool) -> Self {
        match value {
            false => Axis::Horizontal,
            true => Axis::Vertical,
        }
    }
}

/// Order items are laid out in a [`FlexContainer`].
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

impl From<u8> for FlexDir {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::LeftToRight,
            1 => Self::RightToLeft,
            2 => Self::BottomToTop,
            3 => Self::TopToBottom,
            _ => panic!("Not a valid direction.")
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

/// Where items are aligned to in a [`FlexContainer`]
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

/// Where to place the next line in a [wrapping layout](FlexLayout::Paragraph).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub(crate) enum Stacking {
    Lo, Hi
}

impl Stacking {
    /// Returns -1 if Lo, 1 if Hi
    pub fn signum(&self) -> f32 {
        match self {
            Stacking::Lo => -1.0,
            Stacking::Hi => 1.0,
        }
    }
}

impl From<u8> for Stacking {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Lo,
            1 => Self::Hi,
            _ => panic!("Not a valid WrapTo.")
        }
    }
}

impl From<FlexDir> for Stacking {
    fn from(value: FlexDir) -> Self {
        match value {
            FlexDir::RightToLeft|FlexDir::TopToBottom => Self::Lo,
            FlexDir::LeftToRight|FlexDir::BottomToTop => Self::Hi,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BoxSize {
    pub offset: Vec2,
    pub dimension: Vec2,
    pub margin: Vec2,
}

impl BoxSize {
    pub fn with_max_dim(&self, max: Vec2) -> Self{
        BoxSize {
            offset: self.offset,
            dimension: self.dimension.min(max),
            margin: self.margin
        }
    }
}

/// Info for positioning an item in a [`FlexContainer`].
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct FlexItem {
    /// anchor of this item
    pub anchor: Anchor,
    /// dimension of this item
    pub dimension: Vec2,
    /// Force a linebreak on or after this item.
    pub flex_control: FlexControl,
}

pub(crate) enum SpanAlign {
    Neg, Mid, Pos
}

impl From<Alignment> for SpanAlign {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Center => Self::Mid,
            Alignment::Left|Alignment::Bottom => Self::Neg,
            Alignment::Right|Alignment::Top => Self::Pos,
        }
    }
}

impl From<&Alignment> for SpanAlign {
    fn from(value: &Alignment) -> Self {
        match value {
            Alignment::Center => Self::Mid,
            Alignment::Left|Alignment::Bottom => Self::Neg,
            Alignment::Right|Alignment::Top => Self::Pos,
        }
    }
}

impl Axis {
    pub(crate) fn span_align(&self, anchor: &Anchor) -> SpanAlign {
        use Anchor::*;
        match self {
            Axis::Horizontal => match anchor {
                BottomLeft|CenterLeft|TopLeft => SpanAlign::Neg,
                BottomCenter|Center|TopCenter => SpanAlign::Mid,
                BottomRight|CenterRight|TopRight => SpanAlign::Pos,
                _ => panic!("Custom anchor not supported"),
            },
            Axis::Vertical => match anchor {
                BottomLeft|BottomCenter|BottomRight => SpanAlign::Neg,
                CenterLeft|Center|CenterRight => SpanAlign::Mid,
                TopLeft|TopCenter|TopRight => SpanAlign::Pos,
                _ => panic!("Custom anchor not supported"),
            },
        }
    }
}

pub(crate) fn maxlen_minor<'t, const DIR: u8>(items: impl IntoIterator<Item = &'t FlexItem>) -> Vec2 {
    match DIR / 2 == 0{
        false => maxlen::<false>(items),
        true => maxlen::<true>(items),
    }
}

pub(crate) fn maxlen<'t, const AXIS: bool>(items: impl IntoIterator<Item = &'t FlexItem>) -> Vec2 {
    let axis: Axis = AXIS.into();
    match axis {
        Axis::Horizontal => Vec2::new(
            items.into_iter()
                .map(|x: &FlexItem| x.dimension.x)
                .max_by(|a, b| a.total_cmp(b))
                .unwrap_or(0.0), 0.0),
        Axis::Vertical => Vec2::new(0.0,
            items.into_iter()
                .map(|x: &FlexItem| x.dimension.y)
                .max_by(|a, b| a.total_cmp(b))
                .unwrap_or(0.0))
    }
}