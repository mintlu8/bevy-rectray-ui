use bevy::prelude::*;

use crate::{flex::FlexLayout, Size2};

/// A configurable container that lays out a sequence of AoUI Sprites.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct FlexContainer {
    /// Layout of the flexbox.
    pub layout: FlexLayout,
    /// Margin between cells, always corresponds to the X and Y axis.
    pub margin: Size2,
}

impl std::ops::Deref for FlexContainer {
    type Target = FlexLayout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect, PartialEq, Eq)]
#[non_exhaustive]
/// Cause special behaviors when inserted into a [`FlexBox`].
pub enum FlexControl {
    #[default]
    /// Does not cause special behaviors, optional.
    None,
    /// Breaks the line in a flex box after rendering this item.
    Linebreak,
    /// Breaks the line without taking up space
    /// using returned position for rendering is unspecified behavior.
    LinebreakMarker,
}


impl FlexControl {
    pub fn is_linebreak(&self) -> bool {
        matches!(self, FlexControl::Linebreak | FlexControl::LinebreakMarker)
    }
}