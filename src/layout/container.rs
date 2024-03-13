use std::ops::{Range, RangeFull, RangeInclusive};

use bevy::prelude::*;

use crate::{Size2, util::DslFrom};

use super::{LayoutOutput, LayoutObject};

/// Range of content displayed in the layout, default is `All`.
///
/// This means different things with different layout, could be
/// entities, rows or pages.
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum LayoutRange {
    #[default]
    All,
    // The maximum value is `min + len >= total`,
    // going over that will be auto corrected.
    Bounded {
        min: usize,
        len: usize,
    },
    // The maximum value is `total - min >= 1`,
    // going over that will be auto corrected.
    Capped {
        min: usize,
        len: usize,
    },
    // `min` is len * step, and
    // The maximum value is `total - min >= 1`,
    Stepped {
        step: usize,
        len: usize,
    }
}

impl LayoutRange {

    pub fn is_unbounded(&self) -> bool {
        matches!(self, LayoutRange::All)
    }

    pub fn resolve(&mut self, total: usize) {
        match self {
            LayoutRange::All => (),
            LayoutRange::Bounded { min, len } => *min = usize::min(*min, total.saturating_sub(*len)),
            LayoutRange::Capped { min, .. } => *min = usize::min(*min, total.saturating_sub(1)),
            LayoutRange::Stepped { step, len } => *step = usize::min(*step, total / *len),
        }
    }

    pub fn to_range(&self, total: usize) -> Range<usize> {
        match *self {
            LayoutRange::All => 0..total,
            LayoutRange::Bounded { min, len } => min..(min+len).min(total),
            LayoutRange::Capped { min, len } => min..(min+len).min(total),
            LayoutRange::Stepped { step, len } => step*len..(step*len+step).min(total),
        }
    }
}

impl DslFrom<RangeFull> for LayoutRange {
    fn dfrom(_: RangeFull) -> Self {
        LayoutRange::All
    }
}

impl DslFrom<Range<usize>> for LayoutRange {
    fn dfrom(value: Range<usize>) -> Self {
        LayoutRange::Bounded { min: value.start, len: value.len() }
    }
}


impl DslFrom<RangeInclusive<usize>> for LayoutRange {
    fn dfrom(value: RangeInclusive<usize>) -> Self {
        LayoutRange::Bounded {
            min: *value.start(),
            len: value.end() - value.start() + 1
        }
    }
}

/// A configurable container that lays out a sequence of Entities.
#[derive(Debug, Component, Clone, Reflect)]
pub struct Container {
    /// Layout of the container.
    pub layout: LayoutObject,
    /// Margin between cells, always corresponds to the X and Y axis
    /// regardless of layout directions.
    pub margin: Size2,
    /// Padding around the container.
    pub padding: Size2,
    /// If set, only display a subset of children.
    pub range: LayoutRange,
    /// The runtime computed maximum of a layout, could be number of children, lines, pages, etc.
    pub maximum: usize
}

impl Container {

    pub fn place(&mut self, parent: &LayoutInfo, entities: Vec<super::LayoutItem>) -> LayoutOutput {
        self.layout.place(parent, entities, &mut self.range)
    }

    pub fn get_fac(&self) -> f32 {
        match self.range {
            LayoutRange::All => 0.0,
            LayoutRange::Bounded { min, len } => {
                if self.maximum <= len {
                    0.0
                } else {
                    min as f32 / (self.maximum - len) as f32
                }
            },
            LayoutRange::Capped { min, len:_ } => {
                if self.maximum == 0 {
                    0.0
                } else {
                    min as f32 / self.maximum as f32
                }
            },
            LayoutRange::Stepped { step, len } => {
                let count = self.maximum / len;
                if count == 0 {
                    0.0
                } else {
                    step as f32 / count as f32
                }
            }
        }.clamp(0.0, 1.0)
    }

    pub fn set_fac(&mut self, fac: f32) {
        let fac = fac.clamp(0.0, 1.0);
        match &mut self.range {
            LayoutRange::All => (),
            LayoutRange::Bounded { min, len } => {
                if self.maximum > *len {
                    *min = ((self.maximum - *len) as f32 * fac) as usize
                } else {
                    *min = 0
                }
            },
            LayoutRange::Capped { min, len:_ } => {
                if self.maximum == 0 {
                    *min = 0
                } else {
                    *min = (self.maximum as f32 * fac) as usize
                }
            },
            LayoutRange::Stepped { step, len } => {
                let count = self.maximum / *len;
                if count == 0 {
                    *step = 0
                } else {
                    *step = (count as f32 * fac) as usize
                }
            }
        }
    }

    pub fn decrement(&mut self) {
        match &mut self.range {
            LayoutRange::All => (),
            LayoutRange::Bounded { min, .. } => {
                *min = min.saturating_sub(1)
            },
            LayoutRange::Capped { min, .. } => {
                *min = min.saturating_sub(1)
            },
            LayoutRange::Stepped { step, .. } => {
                *step = step.saturating_sub(1)
            }
        }
    }

    pub fn increment(&mut self) {
        // range doesn't matter since this will be resolved in `pipeline`.
        match &mut self.range {
            LayoutRange::All => (),
            LayoutRange::Bounded { min, .. } => {
                *min += 1;
            },
            LayoutRange::Capped { min, .. } => {
                *min += 1;
            },
            LayoutRange::Stepped { step, .. } => {
                *step += 1;
            }
        }
    }
}

/// Dimension info of a layout parent.
pub struct LayoutInfo {
    pub dimension: Vec2,
    pub em: f32,
    pub rem: f32,
    pub margin: Vec2
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect, PartialEq, Eq)]
#[non_exhaustive]
/// Cause special behaviors when inserted into a [`Container`].
pub enum LayoutControl {
    #[default]
    /// Does not cause special behaviors.
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
