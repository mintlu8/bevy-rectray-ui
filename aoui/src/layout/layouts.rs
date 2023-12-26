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
/// 
/// # Object Safety
/// 
/// This trait is object safe.
pub trait Layout: Downcast + Debug + Send + Sync + 'static {
    /// Place sprites in the layout.
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput;
    /// If specified, dimension is considered reliable 
    /// even if `Dimension` is a percentage of parent's.
    fn dimension_agnostic(&self) -> bool { false }
}

impl_downcast!(Layout);

/// Output of a layout, containing anchors of entities, and the computed dimension of the layout.
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

/// Dynamic layout perfectly containing its child, best used with padding.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct FitLayout {
    pub x: bool,
    pub y: bool,
}

impl FitLayout {
    pub const XY: Self = Self { x: true, y: true };
    pub const X: Self = Self { x: true, y: false };
    pub const Y: Self = Self { x: false, y: true };
}

impl Default for FitLayout {
    fn default() -> Self {
        FitLayout { x: true, y: true }
    }
}

impl Layout for FitLayout {
    fn place(&self, info: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let mut max = Vec2::ZERO;
        let entity_anchors: Vec<_> = entities.into_iter().map(|x| {
            max = max.max(x.dimension);
            (x.entity, x.anchor.as_vec())
        }).collect();

        let dimension = Vec2::new(
            if self.x {max.x} else {info.dimension.x},
            if self.y {max.y} else {info.dimension.y},
        );
        LayoutOutput { entity_anchors, dimension }
    }

    fn dimension_agnostic(&self) -> bool {
        true
    }
}

/// A size agnostic mono-directional compact HBox or VBox.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct CompactLayout {
    pub direction: FlexDir,
}

/// A fix-sized mono-directional HBox or VBox.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct SpanLayout {
    /// The axis, horizontal or vertical.
    pub direction: FlexDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}

/// A fix-sized mono-directional HBox or VBox.
/// 
/// The width is dynamic compared to `SpanLayout`
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct DynamicSpanLayout {
    /// The axis, horizontal or vertical.
    pub direction: FlexDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}


/// A statically sized mono-directional HBox or VBox
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
