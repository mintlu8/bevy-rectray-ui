use bevy::prelude::{Vec2, UVec2};
use bevy::sprite::Anchor;
use crate::RotatedRect;
use crate::grid::{flex_table, fixed_table};

use super::span::{span, paragraph};
use super::grid::grid;
use super::util::*;

#[derive(Debug, Copy, Clone, bevy::prelude::Reflect)]
pub enum Cells {
    Counted(UVec2),
    Sized(Vec2),
}

#[derive(Debug, Clone, bevy::prelude::Reflect)]
pub enum Columns {
    Dynamic(usize),
    Fixed(Vec<f32>),
}


/// A layout that accepts a one dimensional sequence of widgets.
#[derive(Debug, Clone, bevy::prelude::Reflect)]
#[non_exhaustive]
pub enum FlexLayout {
    /// A mono-directional hbox or vbox
    ///
    /// # Rules
    ///
    /// Childrens are ordered by their main axis alignment,
    /// (i.e, Anchor::*Left, Anchor::*Center, Anchor::*Right for Axis::Horizonal),
    /// then by their index in the Children component.
    /// This produces 3 continuous blocks with FlexBox::margin, the margin between blocks are uniform and maximized.
    ///
    /// # Panics
    ///
    /// When a child uses a custom anchor.
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
    /// First we look for children until a line fills up,
    /// then render them as if they were a Span.
    ///
    /// Line are stacked along the direction of [`stack`],
    /// and placed in its rect area with alignment [`align`].
    ///
    /// A typical English text Paragraph can be seen as
    /// ```
    /// FlexLayout::Paragraph {
    ///     direction: Direction::LeftToRight,
    ///     alignment: Alignment::Top,
    ///     stack: FlexDir::TopToBottom,
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// * When supplied a [`Anchor::Custom`] Anchor.
    /// * If `align` is not orthogonal to `direction`.
    Paragraph {
        /// The primary axis, horizontal or vertical
        direction: FlexDir,
        /// Where spans are places relative to the parent box,
        /// must be othogonal to `direction`.
        alignment: Alignment,
        /// The order of which lines are placed.
        stack: FlexDir,
        /// This only affects the primary axis.
        stretch: bool,
    },
    /// A 2D grid wih even pre-subdivided cells.
    /// 
    /// # Note 
    /// 
    /// Margin does not increase cell size, 
    /// but decreases the size of rects sprites are placed against.
    /// 
    /// # Panics
    ///
    /// * If `row` is not orthogonal to `column`.
    Grid {
        /// Determines the size of a cell.
        cell: Cells,
        /// The order of which continuous items are placed.
        row: FlexDir,
        /// The order of which rows are placed.
        column: FlexDir,
        /// How items in a row are aligned.
        /// 
        /// Significant when an early linebreak occurs.
        row_align: Alignment,
        /// How rows are placed relative to the parent,
        column_align: Alignment,
        /// if specified, adjust cell size slightly to fill the grid.
        /// 
        /// This only affects rows and only on [`Cell::Sized`] mode.
        stretch: bool,
    },
    /// A 2D grid with unevenly subdivided cells.
    /// 
    /// # Note 
    /// 
    /// In fixed tables, row margin decrease 
    /// the size of rects sprites are placed against.
    /// 
    /// # Panics
    ///
    /// * If `row` is not orthogonal to `column`.
    Table {
        /// Determines the number and size of columns
        ///
        /// For sized columns
        /// 
        /// `vec![0.4, 0.7]` => `[0.0..0.4, 0.4..0.7, 0.7..1.0]`
        columns: Columns,
        /// The order of which continuous items are placed.
        row: FlexDir,
        /// The order of which rows are placed.
        column: FlexDir,
        /// How items in a row are aligned.
        row_align: Alignment,
        /// How rows are placed relative to the parent,
        column_align: Alignment,
        /// If specified, adjust row margin to fill the grid.
        /// 
        /// This only affects rows and only on [`Columns::Dynamic`] mode.
        stretch: bool,
    },
}

impl Default for FlexLayout {
    fn default() -> Self {
        FlexLayout::Span { 
            direction: FlexDir::LeftToRight,
            stretch: false,
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

impl FlexLayout {
    pub fn place_all(&self, rect: &RotatedRect, margin: Vec2, items: impl IntoIterator<Item = FlexItem>) -> Vec<Vec2>{
        let size = BoxSize {
            offset: rect.anchor(&Anchor::BottomLeft),
            dimension: rect.dimension,
            margin,
        };
        let mut points = match self {
            FlexLayout::Span { direction, stretch } => {
                match (direction, stretch) {
                    (R, false) => span::<L2R, false>(size, items),
                    (L, false) => span::<R2L, false>(size, items),
                    (T, false) => span::<B2T, false>(size, items),
                    (B, false) => span::<T2B, false>(size, items),
                    (R, true) => span::<L2R, true>(size, items),
                    (L, true) => span::<R2L, true>(size, items),
                    (T, true) => span::<B2T, true>(size, items),
                    (B, true) => span::<T2B, true>(size, items),
                }
            },
            FlexLayout::Paragraph { direction, alignment, stack, stretch } => {
                match (direction, stack, stretch) {
                    (R, B, false) => paragraph::<L2R, LO, false>(size, items, alignment),
                    (L, B, false) => paragraph::<R2L, LO, false>(size, items, alignment),
                    (T, L, false) => paragraph::<B2T, LO, false>(size, items, alignment),
                    (B, L, false) => paragraph::<T2B, LO, false>(size, items, alignment),
                    (R, T, false) => paragraph::<L2R, HI, false>(size, items, alignment),
                    (L, T, false) => paragraph::<R2L, HI, false>(size, items, alignment),
                    (T, R, false) => paragraph::<B2T, HI, false>(size, items, alignment),
                    (B, R, false) => paragraph::<T2B, HI, false>(size, items, alignment),
                    (R, B, true) => paragraph::<L2R, LO, true>(size, items, alignment),
                    (L, B, true) => paragraph::<R2L, LO, true>(size, items, alignment),
                    (T, L, true) => paragraph::<B2T, LO, true>(size, items, alignment),
                    (B, L, true) => paragraph::<T2B, LO, true>(size, items, alignment),
                    (R, T, true) => paragraph::<L2R, HI, true>(size, items, alignment),
                    (L, T, true) => paragraph::<R2L, HI, true>(size, items, alignment),
                    (T, R, true) => paragraph::<B2T, HI, true>(size, items, alignment),
                    (B, R, true) => paragraph::<T2B, HI, true>(size, items, alignment),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            },
            FlexLayout::Grid { cell, row: line, column: stack, row_align: line_align, column_align: stack_align, stretch } => {
                let (cell_count, cell_size) = match cell {
                    Cells::Counted(count) => (*count, rect.dimension / count.as_vec2()),
                    Cells::Sized(size) => match stretch {
                        false => ((rect.dimension / *size).as_uvec2(), *size),
                        true => {
                            let count = (rect.dimension / *size).as_uvec2();
                            (count, rect.dimension / count.as_vec2())
                        },
                    },
                };
                match (line, stack) {
                    (R, T) => grid::<L2R, HI>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (R, B) => grid::<L2R, LO>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (L, T) => grid::<R2L, HI>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (L, B) => grid::<R2L, LO>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (T, R) => grid::<B2T, HI>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (T, L) => grid::<B2T, LO>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (B, R) => grid::<T2B, HI>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    (B, L) => grid::<T2B, LO>(size, items, cell_count, cell_size, *line_align, *stack_align),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
            FlexLayout::Table { columns: Columns::Dynamic(columns), row: line, column: stack, row_align: line_align, column_align: stack_align, stretch } => {
                match (line, stack) {
                    (R, T) => flex_table::<L2R, HI>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (R, B) => flex_table::<L2R, LO>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (L, T) => flex_table::<R2L, HI>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (L, B) => flex_table::<R2L, LO>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (T, R) => flex_table::<B2T, HI>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (T, L) => flex_table::<B2T, LO>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (B, R) => flex_table::<T2B, HI>(size, items, *columns, *line_align, *stack_align, *stretch),
                    (B, L) => flex_table::<T2B, LO>(size, items, *columns, *line_align, *stack_align, *stretch),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
            FlexLayout::Table { columns: Columns::Fixed(columns), row: line, column: stack, row_align:_, column_align: stack_align, stretch:_ } => {
                match (line, stack) {
                    (R, T) => fixed_table::<L2R, HI>(size, items, columns, *stack_align),
                    (R, B) => fixed_table::<L2R, LO>(size, items, columns, *stack_align),
                    (L, T) => fixed_table::<R2L, HI>(size, items, columns, *stack_align),
                    (L, B) => fixed_table::<R2L, LO>(size, items, columns, *stack_align),
                    (T, R) => fixed_table::<B2T, HI>(size, items, columns, *stack_align),
                    (T, L) => fixed_table::<B2T, LO>(size, items, columns, *stack_align),
                    (B, R) => fixed_table::<T2B, HI>(size, items, columns, *stack_align),
                    (B, L) => fixed_table::<T2B, LO>(size, items, columns, *stack_align),
                    _ => panic!("Direction and stack must be othogonal.")
                }
            }
        };
        for pt in points.iter_mut(){
            *pt = rect.center + Vec2::from_angle(rect.rotation).rotate(*pt - rect.center);
        }
        points
    }
}
