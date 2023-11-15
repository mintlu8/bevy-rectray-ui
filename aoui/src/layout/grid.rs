use std::ops::Range;
use bevy::math::*;
use itertools::Itertools;

use crate::{LayoutItem, LayoutControl};

fn xy(v: Vec2) -> f32 {
    v.x + v.y
}

pub fn grid(
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: usize,
    cell_size: Vec2,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
    alignment: f32,
) -> (Vec<Vec2>, Vec2) {
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
            result.push(row_cursor + half_dir + half_size * item.anchor.as_vec());
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
        if item.control == LayoutControl::LinebreakMarker {
            result.push(Vec2::ZERO);
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
        result[row].iter_mut().for_each(|x| *x += roll);
    }
    let normalize = (row_dir(dimension) + column_dir(dimension)).min(Vec2::ZERO);
    //if normalize.cmplt(Vec2::ZERO).any() {
        result.iter_mut().for_each(|x| *x -= normalize);
    //}
    (result, dimension)
}


pub fn table(
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: Vec<(Vec2, Vec2)>,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
) -> (Vec<Vec2>, Vec2) {

    let rabs = |x| row_dir(x).abs();
    let cabs = |x| column_dir(x).abs();

    let mut cursor = Vec2::ZERO;
    let mut result = Vec::new();

    let half_size = |x| rabs(x) / 2.0 + cabs(Vec2::ONE) / 2.0;
    let mut line_height = Vec2::ZERO;
    let line_margin = column_dir(margin);
    let mut col = 0;
    let max = columns.first().map(|x| x.0 + x.1).unwrap_or(Vec2::ZERO).max(
        columns.last().map(|x| x.0 + x.1).unwrap_or(Vec2::ZERO)
    );
    let unit_row = row_dir(Vec2::ONE).abs();
    for item in items {
        line_height = line_height.max(column_dir(item.dimension).abs());
        let (offset, dim) = columns[col];
        let dim = half_size(dim);
        if item.control != LayoutControl::LinebreakMarker {
            result.push(offset + dim + dim * item.anchor.as_vec());
            col += 1;
        } 
        if col >= columns.len() || item.control.is_linebreak() {
            let len = result.len();
            let height = column_dir(line_height);
            for item in &mut result[(len - col)..] {
                *item = *item * (height + unit_row) + cursor;
            }
            cursor += height + line_margin;
            col = 0;
            line_height = Vec2::ZERO;
        }
        if item.control == LayoutControl::LinebreakMarker {
            result.push(Vec2::ZERO);
        }
    }
    if col > 0 {
        let len = result.len();
        let height = column_dir(line_height);
        for item in &mut result[(len - col)..] {
            *item = *item * (height + unit_row) + cursor;
        }
        cursor += height;
    } else if cursor != Vec2::ZERO {
        cursor -= line_margin;
    }

    let normalize = cursor.min(Vec2::ZERO);
    result.iter_mut().for_each(|x| *x -= normalize);
    (result, max + cursor.abs())
}

pub fn porportional_table(
    dimension: Vec2,
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: impl IntoIterator<Item = f32> + ExactSizeIterator,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
) -> (Vec<Vec2>, Vec2) {

    let len = columns.len();
    if len == 0 {
        assert_ne!(len, 0, "Columns should not be 0.");
    }
    let row = xy(row_dir(dimension).abs());
    let margin_per_item = xy(margin) * (len - 1) as f32 / len as f32;
    let columns = columns.into_iter().map(|x| x * row - margin_per_item);

    fixed_table(dimension, margin, items, columns, row_dir, column_dir, false)
}

pub fn fixed_table(
    dimension: Vec2,
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    columns: impl IntoIterator<Item = f32>,
    row_dir: impl Fn(Vec2) -> Vec2,
    column_dir: impl Fn(Vec2) -> Vec2,
    stretch: bool,
) -> (Vec<Vec2>, Vec2) {
    let len = row_dir(dimension);
    let signum = len.signum();
    let columns: Vec<Vec2> = columns.into_iter().map(|x| x * signum).collect_vec();
    let row_margin = match stretch {
        false => row_dir(margin),
        true => match columns.len() {
            0|1 => Vec2::ZERO,
            count => (len - columns.iter().sum::<Vec2>()) / (count - 1) as f32
        },
    };

    let mut result = Vec::new();
    if len.cmplt(Vec2::ZERO).any() {
        let mut cursor = len.abs();
        for item in columns {
            result.push((cursor + item, cursor));
            cursor += item + row_margin;
        }
    } else {
        let mut cursor = Vec2::ZERO;
        for item in columns {
            result.push((cursor, cursor + item));
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
) -> (Vec<Vec2>, Vec2) {
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