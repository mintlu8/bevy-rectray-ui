use std::fmt::Debug;

use bevy::ecs::entity::Entity;
use bevy::prelude::{Vec2, UVec2};
use downcast_rs::{impl_downcast, Downcast};
use crate::{Size2, SizeUnit};

use super::{util::*, LayoutInfo};

// asserts layout is object safe
const _: Option<Box<dyn Layout>> = None;

/// A layout that accepts a one dimensional sequence of widgets.
/// 
/// The `Container` is usually a dynamic sized widget, 
/// meaning it will update its size based on the size occupied by its children. 
/// You can parent it to an anchor of
/// a fixed sized widget for alignment.
pub trait Layout: Downcast + Debug + Send + Sync + 'static {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput;
}

impl_downcast!(Layout);

#[derive(Debug)]
pub struct LayoutOutput {
    pub entity_anchors: Vec<(Entity, Vec2)>,
    pub dimension: Vec2,
}

impl LayoutOutput {
    pub fn normalized(mut self) -> Self{
        self.entity_anchors.iter_mut().for_each(|(_, x)| *x = *x / self.dimension - 0.5);
        self
    }
}

/// Dynamic layout that always has the same size as sum of its child, with margin added.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct Padding {
    pub x: bool,
    pub y: bool,
}

impl Layout for Padding {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let mut max = Vec2::ZERO;
        let margin = parent.margin;
        let mut entity_anchors: Vec<_> = entities.into_iter().map(|x| {
            max = max.max(x.dimension);
            (x.entity, x.anchor.as_vec())
        }).collect();

        let dimension = Vec2::new(
            if self.x {max.x} else {parent.dimension.x},
            if self.y {max.y} else {parent.dimension.y},
        );
        let m = (dimension - margin) / dimension;
        entity_anchors.iter_mut().for_each(|(_, anchor)| *anchor *= m);
        let dimension = dimension + margin * 2.0;
        LayoutOutput { entity_anchors, dimension: dimension + margin * 2.0 }
    }
}

/// A size agnostic mono-directional compact hbox or vbox.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct CompactLayout {
    pub direction: FlexDir,
}

/// A fix-sized mono-directional hbox or vbox.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct SpanLayout {
    /// The axis, horizontal or vertical.
    pub direction: FlexDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}

/// A fix-sized mono-directional hbox or vbox.
/// 
/// The width is dynamic compared to `SpanLayout`
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct DynamicSpanLayout {
    /// The axis, horizontal or vertical.
    pub direction: FlexDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}


/// A statically sized mono-directional hbox or vbox
///
/// # Rules
///
/// Children are ordered by their main axis alignment,
/// then by their index in the Children component.
///
/// # Panics
///
/// * When supplied a [`Anchor::Custom`](bevy::sprite::Anchor) Anchor.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct ParagraphLayout {
    /// The primary axis, horizontal or vertical
    pub direction: FlexDir,
    /// The order of which lines are placed.
    pub stack: FlexDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}

/// A 2D grid wih even pre-subdivided cells.
/// 
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct SizedGridLayout {
    /// Determines the size of a cell.
    pub cell_size: Size2,
    /// The order of which continuous items are placed.
    pub row_dir: FlexDir,
    /// The order of which rows are placed.
    pub column_dir: FlexDir,
    /// How items in a incomplete row are aligned.
    /// 
    /// Significant when an early linebreak occurs.
    pub alignment: Alignment,
    /// if specified, adjust cell size to fill the grid without changing cell count.
    /// 
    /// This only affects [`Cells::Sized`] mode.
    pub stretch: bool,
}

/// A 2D grid wih even pre-subdivided cells.
/// 
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct FixedGridLayout {
    /// Determines the number of cells
    pub cells: UVec2,
    /// The order of which continuous items are placed.
    pub row_dir: FlexDir,
    /// The order of which rows are placed.
    pub column_dir: FlexDir,
    /// How items in a incomplete row are aligned.
    /// 
    /// Significant when an early linebreak occurs.
    pub alignment: Alignment,
}

/// A 2D grid with unevenly subdivided cells.
/// 
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, bevy::prelude::Reflect)]
pub struct DynamicTableLayout {
    /// Determines the number of columns, use a large number for infinite.
    pub columns: usize,
    /// The order of which continuous items are placed.
    pub row_dir: FlexDir,
    /// The order of which rows are placed.
    pub column_dir: FlexDir,
    /// If specified, adjust row margin to fill the table.
    pub stretch: bool,
}

/// A 2D grid with unevenly subdivided cells.
/// 
/// # Panics
///
/// * If `row_dir` is not orthogonal to `column_dir`.
#[derive(Debug, Clone, bevy::prelude::Reflect)]
pub struct TableLayout {
    /// Determines the number and size of columns
    pub columns: Vec<(SizeUnit, f32)>,
    /// The order of which continuous items are placed.
    pub row_dir: FlexDir,
    /// The order of which rows are placed.
    pub column_dir: FlexDir,
    /// If specified, adjust row margin to fill the table.
    pub stretch: bool,
}
