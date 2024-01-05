use std::fmt::Debug;

use bevy::ecs::entity::Entity;
use bevy::prelude::{Vec2, UVec2};
use downcast_rs::{impl_downcast, Downcast};
use crate::{Size2, SizeUnit, Size};

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
    /// Returns a reliable minimum size for percentage base dimension.
    #[allow(unused_variables)]
    fn reliable_dimension(&self, computed_size: Vec2) -> Vec2 { Vec2::ZERO }
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

/// A dynamic dimensioned `Frame` that by default have size equal
/// to the maximum of its children.
/// 
/// This layout usually should contain only one child with no offset.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct BoundsLayout {
    /// If set, use `Dimension` on that axis.
    pub fixed: [bool; 2],
    /// Minimum bounds.
    pub min: Size2,
    /// Maximum bounds.
    pub max: Size2,
}

impl BoundsLayout {
    /// Ignore constraints and use `BoundsLayout` as padding.
    pub const PADDING: Self = Self {
        fixed: [false; 2],
        min: Size2::ZERO, 
        max: Size2::MAX
    };

    pub const fn from_max(max: Size2) -> Self{
        BoundsLayout {
            fixed: [false; 2],
            min: Size2::MAX,
            max,
        }
    }

    pub const fn from_min(min: Size2) -> Self{
        BoundsLayout {
            fixed: [false; 2],
            min,
            max: Size2::MAX
        }
    }

    pub const fn x_bounds(min: Size, max: Size) -> Self{
        BoundsLayout {
            fixed: [false, true],
            min: Size2::splat(min),
            max: Size2::splat(max),
        }
    }

    pub const fn y_bounds(min: Size, max: Size) -> Self{
        BoundsLayout {
            fixed: [true, false],
            min: Size2::splat(min),
            max: Size2::splat(max),
        }
    }


}

impl Default for BoundsLayout {
    fn default() -> Self {
        Self::PADDING
    }
}

impl Layout for BoundsLayout {
    fn place(&self, info: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let mut max_dim = Vec2::ZERO;
        let entity_anchors: Vec<_> = entities.into_iter().map(|x| {
            max_dim = max_dim.max(x.dimension);
            (x.entity, x.anchor.as_vec())
        }).collect();
        
        let min = self.min.as_pixels(info.dimension, info.em, info.rem);
        let max = self.max.as_pixels(info.dimension, info.em, info.rem);

        let dim = max_dim.clamp(min, max);

        let dimension = Vec2::new(
            if !self.fixed[0] {dim.x} else {info.dimension.x},
            if !self.fixed[1] {dim.y} else {info.dimension.y},
        );
        LayoutOutput { entity_anchors, dimension }
    }

    fn reliable_dimension(&self, computed_size: Vec2) -> Vec2 {
        Vec2::new(
            if self.fixed[0] {0.0} else {computed_size.x},
            if self.fixed[1] {0.0} else {computed_size.y},
        )
    }
}

/// A size agnostic mono-directional container.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct StackLayout {
    pub direction: LayoutDir,
}

impl StackLayout {
    pub const HSTACK: Self = Self { direction: LayoutDir::LeftToRight };
    pub const VSTACK: Self = Self { direction: LayoutDir::TopToBottom };
}

/// A fix-sized mono-directional container.
#[derive(Debug, Clone, Copy, bevy::prelude::Reflect)]
pub struct SpanLayout {
    /// The axis, horizontal or vertical.
    pub direction: LayoutDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}

impl SpanLayout {
    pub const HBOX: Self = Self { direction: LayoutDir::LeftToRight, stretch: false };
    pub const VBOX: Self = Self { direction: LayoutDir::TopToBottom, stretch: false };
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
    pub direction: LayoutDir,
    /// The order of which lines are placed.
    pub stack: LayoutDir,
    /// If specified, try increase the margin to fill the span.
    pub stretch: bool,
}

impl Default for ParagraphLayout {
    fn default() -> Self {
        Self {
            direction: LayoutDir::LeftToRight,
            stack: LayoutDir::TopToBottom,
            stretch: false,
        }
    }
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
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
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
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
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
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
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
    pub row_dir: LayoutDir,
    /// The order of which rows are placed.
    pub column_dir: LayoutDir,
    /// If specified, adjust row margin to fill the table.
    pub stretch: bool,
}

impl TableLayout {
    pub fn from_columns(columns: impl Into<Vec<(SizeUnit, f32)>>) -> Self {
        Self {
            columns: columns.into(),
            row_dir: LayoutDir::LeftToRight,
            column_dir: LayoutDir::TopToBottom,
            stretch: false,
        }
    }
}