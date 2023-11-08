use bevy::prelude::*;

use crate::{Layout, Size2};

/// A configurable container that lays out a sequence of AoUI Sprites.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct Container {
    /// Layout of the container.
    pub layout: Layout,
    /// Margin between cells, always corresponds to the X and Y axis
    /// regardless of layout directions.
    pub margin: Size2,
}

impl std::ops::Deref for Container {
    type Target = Layout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect, PartialEq, Eq)]
#[non_exhaustive]
/// Cause special behaviors when inserted into a [`Container`].
pub enum LayoutControl {
    #[default]
    /// Does not cause special behaviors, optional.
    None,
    /// Breaks the line in a flex box after rendering this item.
    Linebreak,
    /// Breaks the line without taking up space.
    /// 
    /// Dimension might be used to determine line height.
    /// 
    /// Using returned position for rendering is unspecified behavior.
    LinebreakMarker,
}


impl LayoutControl {

    /// Is either [`Linebreak`](LayoutControl::Linebreak) or [`LinebreakMarker`](LayoutControl::LinebreakMarker)
    pub fn is_linebreak(&self) -> bool {
        matches!(self, LayoutControl::Linebreak | LayoutControl::LinebreakMarker)
    }
}