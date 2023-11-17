use bevy::prelude::{Vec2, UVec2};
use crate::span::{compact, span};

use super::span::*;
use super::grid::*;
use super::util::*;

/// Cells in a [`Layout::Grid`]
#[derive(Debug, Copy, Clone, bevy::prelude::Reflect)]
pub enum Cells {
    Counted(UVec2),
    Sized(Vec2),
}

/// Columns in a [`Layout::Table`]
#[derive(Debug, Clone, bevy::prelude::Reflect)]
pub enum Columns {
    Dynamic(usize),
    Porportions(Vec<f32>),
    Sized(Vec<f32>),
}

impl Columns {
    pub const ANY: Self = Columns::Dynamic(usize::MAX);
}


/// A layout that accepts a one dimensional sequence of widgets.
/// 
/// The `Container` is usually a dynamic sized widget, 
/// meaning it is compact with no padding, and will update
/// its size based on the size occupied by its children. 
/// You can parent it to an anchor of
/// a fixed sized widget for alignment.
#[derive(Debug, Clone, bevy::prelude::Reflect)]
#[non_exhaustive]
pub enum Layout {
    /// Dynamic layout that always has the same size as sum of its child, with margin added.
    /// 
    /// Otherwise similar to a regular sprite.
    Dynamic {
        /// Width is dynamic.
        x: bool,
        /// Height is dynamic.
        y: bool,
    },
    /// A size agnostic dynamic mono-directional compact hbox or vbox.
    /// 
    /// # Rules
    /// 
    /// For `HBOX`:
    /// 
    /// Width is sum of children's width, plus margin.
    /// 
    /// Height is the maximum of children's height.
    Compact {
        /// The axis, horizontal or vertical.
        direction: FlexDir,
    },
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
    Span {
        /// The axis, horizontal or vertical.
        direction: FlexDir,
        /// If specified, try increase the margin to fill the span.
        stretch: bool,
    },
    /// A wrapping multiline span.
    ///
    /// # Rules
    ///
    /// Fill up lines in order, then rendered as `span`s.
    ///
    /// A typical English text Paragraph can be seen as
    /// ```
    /// # use bevy_aoui::*;
    /// Layout::Paragraph {
    ///     direction: FlexDir::LeftToRight,
    ///     stack: FlexDir::TopToBottom,
    ///     stretch: false,
    /// }
    /// # ;
    /// ```
    ///
    /// # Panics
    ///
    /// * When supplied a `Anchor::Custom` Anchor.
    Paragraph {
        /// The primary axis, horizontal or vertical
        direction: FlexDir,
        /// The order of which lines are placed.
        stack: FlexDir,
        /// If specified, try increase the margin to fill the span.
        stretch: bool,
    },
    /// A 2D grid wih even pre-subdivided cells.
    /// 
    /// # Panics
    ///
    /// * If `row_dir` is not orthogonal to `column_dir`.
    Grid {
        /// Determines the size of a cell.
        cell: Cells,
        /// The order of which continuous items are placed.
        row_dir: FlexDir,
        /// The order of which rows are placed.
        column_dir: FlexDir,
        /// How items in a incomplete row are aligned.
        /// 
        /// Significant when an early linebreak occurs.
        alignment: Alignment,
        /// if specified, adjust cell size to fill the grid without changing cell count.
        /// 
        /// This only affects [`Cell::Sized`] mode.
        stretch: bool,
    },
    /// A 2D grid with unevenly subdivided cells.
    /// 
    /// # Panics
    ///
    /// * If `row_dir` is not orthogonal to `column_dir`.
    Table {
        /// Determines the number and size of columns
        ///
        /// For porportioned columns
        /// 
        /// `vec![0.4, 0.7]` produces columns `[0.0..0.4, 0.4..0.7, 0.7..1.0]`
        columns: Columns,
        /// The order of which continuous items are placed.
        row_dir: FlexDir,
        /// The order of which rows are placed.
        column_dir: FlexDir,
        /// If specified, adjust row margin to fill the grid.
        /// 
        /// This only affects rows and only on [`Columns::Dynamic`] mode.
        stretch: bool,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Compact { 
            direction: FlexDir::LeftToRight,
        }
    }
}

pub const L2R: u8 = 0;
pub const R2L: u8 = 1;
pub const B2T: u8 = 2;
pub const T2B: u8 = 3;


pub const LO: u8 = 0;
pub const HI: u8 = 1;

use {
    FlexDir::LeftToRight as R,
    FlexDir::RightToLeft as L,
    FlexDir::BottomToTop as T,
    FlexDir::TopToBottom as B,
};

impl Layout {

    /// Horizontal compact box
    pub const HBOX: Self = Self::Compact { direction: FlexDir::LeftToRight };
    /// Horizontal compact box
    pub const VBOX: Self = Self::Compact { direction: FlexDir::TopToBottom };
    /// Horizontal span
    pub const HSPAN: Self = Self::Span { direction: FlexDir::LeftToRight, stretch: false };
    /// Vertical span
    pub const VSPAN: Self = Self::Span { direction: FlexDir::TopToBottom, stretch: false };

    /// Alias for a standard english style layout.
    pub const PAGE: Self = Self::Paragraph { 
        direction: FlexDir::LeftToRight, 
        stack: FlexDir::TopToBottom, 
        stretch: false,
    };

    pub fn place_all<T>(&self, dim: Vec2, margin: Vec2, items: T) -> (Vec<Vec2>, Vec2) where T: IntoIterator<Item = LayoutItem>, T::IntoIter: DoubleEndedIterator{
        match self {
            Layout::Dynamic {x, y} => {
                let mut max = Vec2::ZERO;
                let mut anchors: Vec<_> = items.into_iter().map(|x| {
                    max = max.max(x.dimension);
                    x.anchor.as_vec()
                }).collect();

                let dim = Vec2::new(
                    if *x {max.x} else {dim.x},
                    if *y {max.y} else {dim.y},
                );
                anchors.iter_mut().for_each(|x| *x *= dim);
                (anchors, dim)
            },
            Layout::Compact { direction } => {
                match direction {
                    R => compact(margin, items, posx, posy),
                    L => compact(margin, items, negx, posy),
                    T => compact(margin, items, posy, posx),
                    B => compact(margin, items, negy, posx),
                }
            },
            Layout::Span { direction, stretch } => {
                (match direction{
                    R => span::<false>(dim, margin, *stretch, items, hbucket, posx, posy),
                    L => span::<true>(dim, margin, *stretch, items.into_iter().rev(), hbucket, posx, posy),
                    T => span::<false>(dim, margin, *stretch, items, vbucket, posy, posx),
                    B => span::<true>(dim, margin, *stretch, items.into_iter().rev(), vbucket, posy, posx),
                }, dim)
            },
            Layout::Paragraph { direction, stack, stretch } => {
                match (direction, stack) {
                    (R, B) => paragraph::<false>(dim, margin, *stretch, items, hbucket, posx, negy),
                    (L, B) => paragraph::<true>(dim, margin, *stretch, items, hbucket, posx, negy),
                    (T, L) => paragraph::<false>(dim, margin, *stretch, items, vbucket, posy, negx),
                    (B, L) => paragraph::<true>(dim, margin, *stretch, items, vbucket, posy, negx),
                    (R, T) => paragraph::<false>(dim, margin, *stretch, items, hbucket, posx, posy),
                    (L, T) => paragraph::<true>(dim, margin, *stretch, items, hbucket, posx, posy),
                    (T, R) => paragraph::<false>(dim, margin, *stretch, items, vbucket, posy, posx),
                    (B, R) => paragraph::<true>(dim, margin, *stretch, items, vbucket, posy, posx),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            },
            Layout::Grid { cell, row_dir, column_dir, alignment, stretch } => {
                let (cell_count, cell_size) = match (cell, stretch) {
                    (Cells::Counted(count), _) => (*count, dim / count.as_vec2()),
                    (Cells::Sized(size), false) => ((dim / *size).as_uvec2(), *size),
                    (Cells::Sized(size), true) =>  {
                        let count = (dim / *size).as_uvec2();
                        (count, dim / count.as_vec2())
                    },
                };
                let align = match (row_dir.into(), alignment.into()) {
                    (Binary::Lo, Trinary::Neg) => 1.0,
                    (Binary::Lo, Trinary::Mid) => 0.5,
                    (Binary::Lo, Trinary::Pos) => 0.0,
                    (Binary::Hi, Trinary::Neg) => 0.0,
                    (Binary::Hi, Trinary::Mid) => 0.5,
                    (Binary::Hi, Trinary::Pos) => 1.0, 
                };
                let columns = match row_dir.into() {
                    Axis::Horizontal => cell_count.x,
                    Axis::Vertical => cell_count.y,
                } as usize;
                match (row_dir, column_dir) {
                    (R, T) => grid(margin, items, columns, cell_size, posx, posy, align),
                    (R, B) => grid(margin, items, columns, cell_size, posx, negy, align),
                    (L, T) => grid(margin, items, columns, cell_size, negx, posy, align),
                    (L, B) => grid(margin, items, columns, cell_size, negx, negy, align),
                    (T, R) => grid(margin, items, columns, cell_size, posy, posx, align),
                    (T, L) => grid(margin, items, columns, cell_size, posy, negx, align),
                    (B, R) => grid(margin, items, columns, cell_size, negy, posx, align),
                    (B, L) => grid(margin, items, columns, cell_size, negy, negx, align),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
            Layout::Table { columns: Columns::Dynamic(columns), row_dir, column_dir, stretch } => {
                match (row_dir, column_dir) {
                    (R, T) => flex_table(dim, margin, items, *columns, posx, posy, *stretch),
                    (R, B) => flex_table(dim, margin, items, *columns, posx, negy, *stretch),
                    (L, T) => flex_table(dim, margin, items, *columns, negx, posy, *stretch),
                    (L, B) => flex_table(dim, margin, items, *columns, negx, negy, *stretch),
                    (T, R) => flex_table(dim, margin, items, *columns, posy, posx, *stretch),
                    (T, L) => flex_table(dim, margin, items, *columns, posy, negx, *stretch),
                    (B, R) => flex_table(dim, margin, items, *columns, negy, posx, *stretch),
                    (B, L) => flex_table(dim, margin, items, *columns, negy, negx, *stretch),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
            Layout::Table { columns: Columns::Porportions(columns), row_dir, column_dir, stretch: _ } => {
                match (row_dir, column_dir) {
                    (R, T) => porportional_table(dim, margin, items, columns.into_iter().cloned(), posx, posy),
                    (R, B) => porportional_table(dim, margin, items, columns.into_iter().cloned(), posx, negy),
                    (L, T) => porportional_table(dim, margin, items, columns.into_iter().cloned(), negx, posy),
                    (L, B) => porportional_table(dim, margin, items, columns.into_iter().cloned(), negx, negy),
                    (T, R) => porportional_table(dim, margin, items, columns.into_iter().cloned(), posy, posx),
                    (T, L) => porportional_table(dim, margin, items, columns.into_iter().cloned(), posy, negx),
                    (B, R) => porportional_table(dim, margin, items, columns.into_iter().cloned(), negy, posx),
                    (B, L) => porportional_table(dim, margin, items, columns.into_iter().cloned(), negy, negx),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
            Layout::Table { columns: Columns::Sized(columns), row_dir, column_dir, stretch } => {
                match (row_dir, column_dir) {
                    (R, T) => fixed_table(dim, margin, items, columns.into_iter().cloned(), posx, posy, *stretch),
                    (R, B) => fixed_table(dim, margin, items, columns.into_iter().cloned(), posx, negy, *stretch),
                    (L, T) => fixed_table(dim, margin, items, columns.into_iter().cloned(), negx, posy, *stretch),
                    (L, B) => fixed_table(dim, margin, items, columns.into_iter().cloned(), negx, negy, *stretch),
                    (T, R) => fixed_table(dim, margin, items, columns.into_iter().cloned(), posy, posx, *stretch),
                    (T, L) => fixed_table(dim, margin, items, columns.into_iter().cloned(), posy, negx, *stretch),
                    (B, R) => fixed_table(dim, margin, items, columns.into_iter().cloned(), negy, posx, *stretch),
                    (B, L) => fixed_table(dim, margin, items, columns.into_iter().cloned(), negy, negx, *stretch),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
        }
    }
}
