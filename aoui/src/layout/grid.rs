use std::ops::Range;
use bevy::math::*;
use itertools::Itertools;

use crate::layout::{LayoutItem, LayoutControl};

use super::{Layout, FixedGridLayout, Binary, Trinary, LayoutDir, Axis, LayoutOutput, posx, posy, negx, negy, SizedGridLayout, LayoutInfo, TableLayout, DynamicTableLayout};

const R: LayoutDir = LayoutDir::LeftToRight;
const L: LayoutDir = LayoutDir::RightToLeft;
const T: LayoutDir = LayoutDir::BottomToTop;
const B: LayoutDir = LayoutDir::TopToBottom;

impl Layout for FixedGridLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let cell_size = parent.dimension / self.cells.as_vec2();
        let margin = parent.margin;
        let align = match (self.row_dir.into(), self.alignment.into()) {
            (Binary::Lo, Trinary::Neg) => 1.0,
            (Binary::Lo, Trinary::Mid) => 0.5,
            (Binary::Lo, Trinary::Pos) => 0.0,
            (Binary::Hi, Trinary::Neg) => 0.0,
            (Binary::Hi, Trinary::Mid) => 0.5,
            (Binary::Hi, Trinary::Pos) => 1.0, 
        };
        let columns = match self.row_dir.into() {
            Axis::Horizontal => self.cells.x,
            Axis::Vertical => self.cells.y,
        } as usize;
        match (self.row_dir, self.column_dir) {
            (R, T) => grid(margin, entities, columns, cell_size, posx, posy, align),
            (R, B) => grid(margin, entities, columns, cell_size, posx, negy, align),
            (L, T) => grid(margin, entities, columns, cell_size, negx, posy, align),
            (L, B) => grid(margin, entities, columns, cell_size, negx, negy, align),
            (T, R) => grid(margin, entities, columns, cell_size, posy, posx, align),
            (T, L) => grid(margin, entities, columns, cell_size, posy, negx, align),
            (B, R) => grid(margin, entities, columns, cell_size, negy, posx, align),
            (B, L) => grid(margin, entities, columns, cell_size, negy, negx, align),
            _ => panic!("Direction and stack must be othogonal.")
        }.normalized()
    }
}

impl Layout for SizedGridLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let dimension = parent.dimension;
        let cell_size = self.cell_size.as_pixels(dimension, parent.em, parent.em);
        let margin = parent.margin;

        let (cell_count, cell_size) = if self.stretch {
            ((dimension / cell_size).as_uvec2(), cell_size)
        } else {
            let count = (dimension / cell_size).as_uvec2();
            (count, dimension / count.as_vec2())
        };
        let align = match (self.row_dir.into(), self.alignment.into()) {
            (Binary::Lo, Trinary::Neg) => 1.0,
            (Binary::Lo, Trinary::Mid) => 0.5,
            (Binary::Lo, Trinary::Pos) => 0.0,
            (Binary::Hi, Trinary::Neg) => 0.0,
            (Binary::Hi, Trinary::Mid) => 0.5,
            (Binary::Hi, Trinary::Pos) => 1.0, 
        };
        let columns = match self.row_dir.into() {
            Axis::Horizontal => cell_count.x,
            Axis::Vertical => cell_count.y,
        } as usize;
        match (self.row_dir, self.column_dir) {
            (R, T) => grid(margin, entities, columns, cell_size, posx, posy, align),
            (R, B) => grid(margin, entities, columns, cell_size, posx, negy, align),
            (L, T) => grid(margin, entities, columns, cell_size, negx, posy, align),
            (L, B) => grid(margin, entities, columns, cell_size, negx, negy, align),
            (T, R) => grid(margin, entities, columns, cell_size, posy, posx, align),
            (T, L) => grid(margin, entities, columns, cell_size, posy, negx, align),
            (B, R) => grid(margin, entities, columns, cell_size, negy, posx, align),
            (B, L) => grid(margin, entities, columns, cell_size, negy, negx, align),
            _ => panic!("Direction and stack must be orthogonal.")
        }.normalized()
    }
}

impl Layout for TableLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let dim = parent.dimension;
        let margin = parent.margin;
        let stretch = self.stretch;
        let main_axis = match self.row_dir.into() {
            Axis::Horizontal => parent.dimension.x,
            Axis::Vertical => parent.dimension.y,
        };

        let columns = self.columns.iter().map(|(unit, raw)| 
            unit.as_pixels(*raw, main_axis, parent.em, parent.rem)
        ).collect();

        match (self.row_dir, self.column_dir) {
            (R, T) => fixed_table(dim, margin, entities, columns, posx, posy, stretch),
            (R, B) => fixed_table(dim, margin, entities, columns, posx, negy, stretch),
            (L, T) => fixed_table(dim, margin, entities, columns, negx, posy, stretch),
            (L, B) => fixed_table(dim, margin, entities, columns, negx, negy, stretch),
            (T, R) => fixed_table(dim, margin, entities, columns, posy, posx, stretch),
            (T, L) => fixed_table(dim, margin, entities, columns, posy, negx, stretch),
            (B, R) => fixed_table(dim, margin, entities, columns, negy, posx, stretch),
            (B, L) => fixed_table(dim, margin, entities, columns, negy, negx, stretch),
            _ => panic!("Direction and stack must be orthogonal.")
        }.normalized()
    }
}

impl Layout for DynamicTableLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let dim = parent.dimension;
        let margin = parent.margin;
        let stretch = self.stretch;
        let columns = self.columns;

        match (self.row_dir, self.column_dir) {
            (R, T) => flex_table(dim, margin, entities, columns, posx, posy, stretch),
            (R, B) => flex_table(dim, margin, entities, columns, posx, negy, stretch),
            (L, T) => flex_table(dim, margin, entities, columns, negx, posy, stretch),
            (L, B) => flex_table(dim, margin, entities, columns, negx, negy, stretch),
            (T, R) => flex_table(dim, margin, entities, columns, posy, posx, stretch),
            (T, L) => flex_table(dim, margin, entities, columns, posy, negx, stretch),
            (B, R) => flex_table(dim, margin, entities, columns, negy, posx, stretch),
            (B, L) => flex_table(dim, margin, entities, columns, negy, negx, stretch),
            _ => panic!("Direction and stack must be orthogonal.")
        }
    }
}


fn xy(v: Vec2) -> f32 {
    v.x + v.y
}

pub(crate) fn grid(
    margin: Vec2,
    items: Vec<LayoutItem>,
    columns: usize,
    cell_size: Vec2,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
    alignment: f32,
) -> LayoutOutput {
    let mut cursor = Vec2::ZERO;
    let mut dimension = Vec2::ZERO;
    let mut max_columns = 0;
    let mut result = Vec::new();
    let mut row_ranges: Vec<Range<usize>> = Vec::new();
    let mut row_start = 0;
    let half_size = cell_size - margin / 2.0;
    let half_dir = row_dir(half_size / 2.0) + column_dir(half_size / 2.0);

    let delta_cell = row_dir(cell_size);
    let delta_row = column_dir(cell_size);
    let mut row_cursor = cursor;
    for (i, item) in items.into_iter().enumerate() {
        if item.control != LayoutControl::LinebreakMarker {
            result.push((item.entity, row_cursor + half_dir + half_size * item.anchor.as_vec()));
            row_cursor += delta_cell;
        } 
        if result.len() - row_start >= columns || item.control.is_linebreak() {
            row_ranges.push(row_start..result.len());
            max_columns = max_columns.max(result.len() - row_start);
            dimension = dimension.max((row_cursor + delta_row).abs());
            row_start = i + 1;
            cursor += delta_row;
            row_cursor = cursor;
        }
    }
    if row_start < result.len() {
        row_ranges.push(row_start..result.len());
        max_columns = max_columns.max(result.len() - row_start);
        dimension = dimension.max((row_cursor + delta_row).abs());
    }
    for row in row_ranges {
        let roll = (max_columns - row.len()) as f32 / max_columns as f32;
        let roll = row_dir(dimension) * roll * alignment;
        result[row].iter_mut().for_each(|(_,x)| *x += roll);
    }
    let normalize = (row_dir(dimension) + column_dir(dimension)).min(Vec2::ZERO);
    result.iter_mut().for_each(|(_,x)| *x -= normalize);
    LayoutOutput {
        entity_anchors: result,
        dimension,
    }
}


pub(crate) fn table(
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: Vec<(Vec2, Vec2)>,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
) -> LayoutOutput {

    let rabs = |x| row_dir(x).abs();
    let cabs = |x| column_dir(x).abs();

    let mut cursor = Vec2::ZERO;
    let mut result = Vec::new();

    let as_cell_size = |x| rabs(x) + cabs(Vec2::ONE);
    let mut line_height = Vec2::ZERO;
    let line_margin = column_dir(margin);
    let mut col = 0;
    let max = columns.first().map(|x| x.0 + x.1).unwrap_or(Vec2::ZERO).max(
        columns.last().map(|x| x.0 + x.1).unwrap_or(Vec2::ZERO)
    );
    let unit_row = rabs(Vec2::ONE);
    for item in items {
        line_height = line_height.max(column_dir(item.dimension).abs());
        let (offset, dim) = columns[col];
        let dim = as_cell_size(dim);
        if item.control != LayoutControl::LinebreakMarker {
            result.push((item.entity, offset + dim / 2.0 + dim * item.anchor.as_vec()));
            col += 1;
        } 
        if col >= columns.len() || item.control.is_linebreak() {
            let len = result.len();
            let height = column_dir(line_height);
            cursor += height.min(Vec2::ZERO);
            for (_, item) in &mut result[(len - col)..] {
                *item = *item * (height.abs() + unit_row) + cursor;
            }
            cursor += height.max(Vec2::ZERO);
            cursor += line_margin;
            col = 0;
            line_height = Vec2::ZERO;
        }
    }
    if col > 0 {
        let len = result.len();
        let height = column_dir(line_height);
        cursor += height.min(Vec2::ZERO);
        for (_, item) in &mut result[(len - col)..] {
            *item = *item * (height.abs() + unit_row) + cursor;
        }
        cursor += height.max(Vec2::ZERO);
    } else if cursor != Vec2::ZERO {
        cursor -= line_margin;
    }

    let normalize = cursor.min(Vec2::ZERO);
    result.iter_mut().for_each(|(_, x)| *x -= normalize);
    LayoutOutput {
        entity_anchors: result,
        dimension: max + cursor.abs(),
    }
}

pub(crate) fn fixed_table(
    dimension: Vec2,
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: Vec<f32>,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
    stretch: bool,
) -> LayoutOutput {
    let len = row_dir(dimension);
    let columns: Vec<Vec2> = columns.into_iter().map(|x| row_dir(x * Vec2::ONE)).collect_vec();
    let row_margin = match stretch {
        false => row_dir(margin),
        true => match columns.len() {
            0|1 => Vec2::ZERO,
            count => (len - columns.iter().sum::<Vec2>()) / (count - 1) as f32,
        },
    };

    let mut result = Vec::new();
    if len.cmplt(Vec2::ZERO).any() {
        let mut cursor = len.abs();
        for item in columns {
            result.push((cursor + item, item.abs()));
            cursor += item + row_margin;
        }
    } else {
        let mut cursor = Vec2::ZERO;
        for item in columns {
            result.push((cursor, item.abs()));
            cursor += item + row_margin;
        }
    }
    table(margin, items, result, row_dir, column_dir)
}

pub fn flex_table(
    dimension: Vec2,
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: usize,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
    stretch: bool,
) -> LayoutOutput {
    assert_ne!(columns, 0, "Columns should not be 0.");
    let mut index = 0;
    let mut cols: Vec<f32> = Vec::new();
    let items = items.into_iter().map(|item| {
        let len = xy(row_dir(item.dimension).abs());
        match cols.get_mut(index) {
            Some(x) => *x = (*x).max(len),
            None => cols.push(len),
        }
        index += 1;
        if index >= columns || item.control.is_linebreak() {
            index = 0;
        }
        item
    }).collect_vec();

    let row_len = row_dir(dimension);
    let row_one = row_dir(Vec2::ONE).abs();
    
    let col_margin = if stretch {
        (xy(row_len).abs() - cols.iter().sum::<f32>()).max(0.0) / (cols.len() - 1) as f32 * row_one
    } else {
        row_dir(margin).abs()
    };
    let total = if stretch {
        row_len.abs()
    } else { 
        cols.iter().sum::<f32>() * row_one + row_dir(margin).abs() * (cols.len() - 1) as f32
    };
    let columns = if row_len.cmplt(Vec2::ZERO).any() {
        let mut cursor = total;
        cols.into_iter()
            .map(|dim|{
                let dim = (dim * row_one).abs();
                let result = (cursor - dim, dim);
                cursor -= dim + col_margin;
                result
            })
            .collect_vec()
    } else {
        let mut cursor = Vec2::ZERO;
        cols.into_iter()
            .map(|dim|{
                let dim = (dim * row_one).abs();
                let result = (cursor, dim);
                cursor += dim + col_margin;
                result
            })
            .collect_vec()
    };
    table(margin, items, columns, row_dir, column_dir)
}