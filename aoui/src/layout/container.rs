use std::ops::Range;

use bevy::prelude::*;

use crate::{Size2, layout::Layout};

/// A configurable container that lays out a sequence of Entities.
#[derive(Debug, Component)]
pub struct Container {
    /// Layout of the container.
    pub layout: Box<dyn Layout>,
    /// Margin between cells, always corresponds to the X and Y axis
    /// regardless of layout directions.
    pub margin: Size2,
    /// Padding around the container.
    pub padding: Size2,
    /// If set, only display a subset of children.
    pub range: Option<Range<usize>>,
}

/// Dimension info of a layout parent.
pub struct LayoutInfo {
    pub dimension: Vec2,
    pub em: f32,
    pub rem: f32,
    pub margin: Vec2
}

impl std::ops::Deref for Container {
    type Target = dyn Layout;

    fn deref(&self) -> &Self::Target {
        self.layout.as_ref()
    }
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect, PartialEq, Eq)]
#[non_exhaustive]
/// Cause special behaviors when inserted into a [`Container`].
pub enum LayoutControl {
    #[default]
    /// Does not cause special behaviors, optional.
    None,
    /// Breaks the line in a container after rendering this item.
    Linebreak,
    /// Breaks the line in a container without taking up space.
    /// 
    /// Dimension is used to determine line height.
    /// 
    /// The sprite will not be rendered and its children will not be updated.
    LinebreakMarker,
    /// Ignore layout and use default rendering.
    IgnoreLayout,
    /// For `compact`, `span` and `paragraph`, trim WhiteSpace at the beginning and end of each layout.
    /// 
    /// If removed this way, the sprite will not be rendered and its children will not be updated.
    WhiteSpace,
    /// Experimental: Unimplemented.
    EntireRow,
}


impl LayoutControl {

    /// Is either [`Linebreak`](LayoutControl::Linebreak) or [`LinebreakMarker`](LayoutControl::LinebreakMarker)
    pub fn is_linebreak(&self) -> bool {
        matches!(self, LayoutControl::Linebreak | LayoutControl::LinebreakMarker)
    }
}