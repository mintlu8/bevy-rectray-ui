use super::Axis;
use bevy::prelude::*;

use crate::{FlexDir, BoxSize, FlexItem, FlexControl, Stacking, Alignment, SpanAlign, maxlen_minor};

pub fn grid<const DIR: u8, const STACK: u8>(
    size: BoxSize,
    items: impl IntoIterator<Item = FlexItem>,
    cell_count: UVec2,
    cell_size: Vec2,
    line_align: Alignment,
    stack_align: Alignment,
) -> Vec<Vec2> {
    let dir: FlexDir = DIR.into();
    let stack: Stacking = STACK.into();
    let axis: Axis = dir.into();

    let line_len = match axis {
        Axis::Horizontal => cell_count.x,
        Axis::Vertical => cell_count.x,
    } as usize;

    let line_dir = match axis {
        Axis::Horizontal => |a: Vec2| Vec2::new(a.x, 0.0),
        Axis::Vertical => |a: Vec2| Vec2::new(0.0, a.y),
    };

    let stack_dir = match axis {
        Axis::Horizontal => |a: Vec2| Vec2::new(0.0, a.y),
        Axis::Vertical => |a: Vec2| Vec2::new(a.x, 0.0),
    };

    let delta_cell = match dir {
        FlexDir::LeftToRight => Vec2::new(cell_size.x, 0.0),
        FlexDir::RightToLeft => Vec2::new(-cell_size.x, 0.0),
        FlexDir::BottomToTop => Vec2::new(0.0, cell_size.y),
        FlexDir::TopToBottom => Vec2::new(0.0, -cell_size.y),
    };
    let delta_line = match axis {
        Axis::Vertical => Vec2::new(cell_size.x, 0.0),
        Axis::Horizontal => Vec2::new(0.0, cell_size.y),
    };
    let line_roll = match (line_align.into(), dir.into()) {
        (SpanAlign::Neg, Stacking::Hi) => Vec2::ZERO,
        (SpanAlign::Mid, Stacking::Hi) => cell_size / 2.0,
        (SpanAlign::Pos, Stacking::Hi) => cell_size,
        (SpanAlign::Neg, Stacking::Lo) => -cell_size,
        (SpanAlign::Mid, Stacking::Lo) => -cell_size / 2.0,
        (SpanAlign::Pos, Stacking::Lo) => Vec2::ZERO,
    };
    let margin = size.dimension - cell_size * cell_count.as_vec2();
    let line_margin = match (line_align.into(), dir.into()) {
        (SpanAlign::Neg, Stacking::Hi) => Vec2::ZERO,
        (SpanAlign::Mid, Stacking::Hi) => line_dir(margin / 2.0),
        (SpanAlign::Pos, Stacking::Hi) => line_dir(margin),
        (SpanAlign::Neg, Stacking::Lo) => -line_dir(margin),
        (SpanAlign::Mid, Stacking::Lo) => -line_dir(margin / 2.0),
        (SpanAlign::Pos, Stacking::Lo) => Vec2::ZERO,
    };
    let cell_offset = size.margin / 2.0;
    let cell_size = cell_size - size.margin;

    let mut result = Vec::new();
    let mut buffer = Vec::new();

    let mut cursor = size.offset;

    for item in items {
        if item.flex_control != FlexControl::LinebreakMarker {
            buffer.push(item.anchor);
        }
        if buffer.len() == line_len || item.flex_control.is_linebreak() {
            if stack == Stacking::Lo { cursor -= delta_line }
            let mut c = cursor;
            let roll = line_roll * ((line_len - buffer.len()) as f32) + line_margin;
            for anc in buffer.drain(..) {
                result.push(c + cell_size * (anc.as_vec() + 0.5) + cell_offset + roll);
                c += delta_cell;
            }
            if stack == Stacking::Hi { cursor += delta_line }
        }
        if item.flex_control == FlexControl::LinebreakMarker {
            result.push(Vec2::ZERO);
        }
    }
    if !buffer.is_empty() {
        if stack == Stacking::Lo { cursor -= delta_line }
        let mut c = cursor;
        let roll = line_roll * ((line_len - buffer.len()) as f32) + line_margin;
        for anc in buffer.drain(..) {
            result.push(c + cell_size * (anc.as_vec() + 0.5) + cell_offset + roll);
            c += delta_cell;
        }
        if stack == Stacking::Hi { cursor += delta_line }
    }

    let height = stack_dir(cursor - size.offset);
    let dim = stack_dir(size.dimension);

    match stack_align.into() {
        SpanAlign::Neg => {
            let dh = -height.min(Vec2::ZERO);
            result.iter_mut().for_each(|v| *v += dh);
        },
        SpanAlign::Mid => {
            let dh = dim / 2.0 - height / 2.0;
            result.iter_mut().for_each(|v| *v += dh)
        },
        SpanAlign::Pos => {
            let dh = dim - height.max(Vec2::ZERO);
            result.iter_mut().for_each(|v| *v += dh);
        },
    }
    result
}

pub fn flex_table<const DIR: u8, const STACK: u8>(
    size: BoxSize,
    items: impl IntoIterator<Item = FlexItem>,
    columns: usize,
    line_align: Alignment,
    stack_align: Alignment,
    stretch: bool,
) -> Vec<Vec2> {

    if columns == 0 { 
        return items.into_iter().map(|_| size.offset).collect() 
    }

    let row = match DIR.into() {
        FlexDir::LeftToRight => |v: Vec2| v.x,
        FlexDir::RightToLeft => |v: Vec2| v.x,
        FlexDir::BottomToTop => |v: Vec2| v.y,
        FlexDir::TopToBottom => |v: Vec2| v.y,
    };
    let mut result: Vec<f32> = vec![0.0; columns];
    let mut cursor = 0;
    let items: Vec<FlexItem> = items.into_iter().map(|item| {
        match &item.flex_control {
            FlexControl::Linebreak => {
                result[cursor] = result[cursor].max(row(item.dimension));
                cursor = (cursor + 1) % columns;
            },
            FlexControl::LinebreakMarker => {
                cursor = 0;
            },
            _ => {
                result[cursor] = result[cursor].max(row(item.dimension));
                cursor = (cursor + 1) % columns;
            }
        }
        item
    }).collect();

    let sum = result.iter().sum::<f32>();
    let margin = if stretch {
        ((row(size.dimension) - sum) / (result.len() - 1) as f32).max(row(size.margin))
    } else {
        row(size.margin)
    };
    let sum = sum + row(size.margin) * (result.len() - 1) as f32;

    let mut cursor = match line_align.into() {
        SpanAlign::Neg => 0.0,
        SpanAlign::Mid => (row(size.dimension) - sum) / 2.0,
        SpanAlign::Pos => row(size.dimension) - sum,
    };

    let mut ranges = Vec::new();
    for len in result {
        ranges.push((cursor, cursor + len));
        cursor += len + margin
    }

    table::<DIR, STACK>(size, items, &ranges, stack_align)
}

pub fn fixed_table<const DIR: u8, const STACK: u8>(
    size: BoxSize,
    items: impl IntoIterator<Item = FlexItem>,
    columns: &[f32],
    stack_align: Alignment,
) -> Vec<Vec2> {
    if columns.is_empty() { 
        return items.into_iter().map(|_| size.offset).collect() 
    }
    let row = match DIR.into() {
        FlexDir::LeftToRight => |v: Vec2| v.x,
        FlexDir::RightToLeft => |v: Vec2| v.x,
        FlexDir::BottomToTop => |v: Vec2| v.y,
        FlexDir::TopToBottom => |v: Vec2| v.y,
    };
    let dim = row(size.dimension);
    let margin = row(size.margin);
    let mut result = vec![(0.0, columns[0] * dim - margin)];
    for i in 0..columns.len() - 1 {
        result.push((columns[i] + margin, columns[i + 1] - margin));
    }
    result.push((columns[columns.len() - 1] + margin, dim - margin));
    table::<DIR, STACK>(size, items, &result, stack_align)
}

pub fn table<const DIR: u8, const STACK: u8>(
    size: BoxSize,
    items: impl IntoIterator<Item = FlexItem>,
    columns: &[(f32, f32)],
    stack_align: Alignment,
) -> Vec<Vec2> {
    let dir: FlexDir = DIR.into();
    let stack: Stacking = STACK.into();
    let axis: Axis = dir.into();

    let column_count = columns.len();

    let stack_dir = match axis {
        Axis::Horizontal => |a: Vec2| Vec2::new(0.0, a.y),
        Axis::Vertical => |a: Vec2| Vec2::new(a.x, 0.0),
    };

    let major = match dir {
        FlexDir::LeftToRight => |x: f32| Vec2::new(x, 0.0),
        FlexDir::RightToLeft => |x: f32| Vec2::new(-x, 0.0),
        FlexDir::BottomToTop => |x: f32| Vec2::new(0.0, x),
        FlexDir::TopToBottom => |x: f32| Vec2::new(0.0, -x),
    };

    let mut result = Vec::new();
    let mut buffer = Vec::new();

    let mut cursor = size.offset;

    
    let line_margin = match stack {
        Stacking::Lo => -stack_dir(size.margin),
        Stacking::Hi => stack_dir(size.margin),
    };

    for item in items {
        let flex_control = item.flex_control;
        if flex_control != FlexControl::LinebreakMarker {
            buffer.push(item);
        }
        if buffer.len() == column_count || flex_control.is_linebreak() {
            let line_height = maxlen_minor::<DIR>(buffer.iter());
            if stack == Stacking::Lo { cursor -= line_height }
            for (item, (col_start, col_end)) in buffer.drain(..).zip(columns) {
                let col_size = major(col_end - col_start) + line_height;
                result.push(cursor + major(*col_start) + col_size * (item.anchor.as_vec() + 0.5));
            }
            if stack == Stacking::Hi { cursor += line_height }
            cursor += line_margin;
        }
        if flex_control == FlexControl::LinebreakMarker {
            result.push(Vec2::ZERO);
        }
    }
    if !buffer.is_empty() {
        let line_height = maxlen_minor::<DIR>(buffer.iter());
        if stack == Stacking::Lo { cursor -= line_height }
        for (item, (col_start, col_end)) in buffer.drain(..).zip(columns) {
            let col_size = major(col_end - col_start) + line_height;
            result.push(cursor + major(*col_start) + col_size * (item.anchor.as_vec() + 0.5));
        }
        if stack == Stacking::Hi { cursor += line_height }
    } else {
        cursor -= line_margin;
    }

    let height = stack_dir(cursor - size.offset);
    let dim = stack_dir(size.dimension);

    match stack_align.into() {
        SpanAlign::Neg => {
            let dh = -height.min(Vec2::ZERO);
            result.iter_mut().for_each(|v| *v += dh);
        },
        SpanAlign::Mid => {
            let dh = dim / 2.0 - height / 2.0;
            result.iter_mut().for_each(|v| *v += dh)
        },
        SpanAlign::Pos => {
            let dh = dim - height.max(Vec2::ZERO);
            result.iter_mut().for_each(|v| *v += dh);
        },
    }
    result
}