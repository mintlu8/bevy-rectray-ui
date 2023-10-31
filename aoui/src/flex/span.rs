use crate::FlexControl;

use super::util::*;
use bevy::prelude::Vec2;

/// DIR is a u8 for monomorphization,
pub fn span<const DIR: u8, const STRETCH: bool>(size: BoxSize, items: impl IntoIterator<Item = FlexItem>) -> Vec<Vec2> {
    let dir: FlexDir = DIR.into();
    let axis: Axis = dir.into();
    let mut neg = Vec::new();
    let mut mid = Vec::new();
    let mut pos = Vec::new();
    let mut len = 0usize;

    let mut sum = Vec2::ZERO;
    for (index, item) in items.into_iter().enumerate() {
        len += 1;
        let align = axis.span_align(&item.anchor);
        match align {
            SpanAlign::Neg => &mut neg,
            SpanAlign::Mid => &mut mid,
            SpanAlign::Pos => &mut pos,
        }.push((index, item.dimension, axis.rev().span_align(&item.anchor)));
        sum += item.dimension;
    }
    let mut margin = match axis {
        Axis::Horizontal => Vec2::new(size.margin.x, 0.0),
        Axis::Vertical => Vec2::new(0.0, size.margin.y),
    };
    if STRETCH && len > 1 {
        let new_margin = margin.max((size.dimension - sum) / (len - 1) as f32);
        match axis {
            Axis::Horizontal => margin = Vec2::new(new_margin.x, 0.0),
            Axis::Vertical => margin = Vec2::new(0.0, new_margin.x),
        }
    }
    let mut result = vec![Vec2::ZERO; len];
    let major = match axis {
        Axis::Horizontal => |a: Vec2| Vec2::new(a.x, 0.0),
        Axis::Vertical => |a: Vec2| Vec2::new(0.0, a.y),
    };
    let mut left_line = match axis {
        Axis::Horizontal => size.offset,
        Axis::Vertical => size.offset,
    };
    let mut right_line = match axis {
        Axis::Horizontal => size.offset + Vec2::new(size.dimension.x, 0.0),
        Axis::Vertical => size.offset + Vec2::new(0.0, size.dimension.y),
    };
    let height = match axis {
        Axis::Horizontal => Vec2::new(0.0, size.dimension.y),
        Axis::Vertical => Vec2::new(size.dimension.x, 0.0),
    };

    if dir.is_reversed() {
        neg.reverse();
        mid.reverse();
        pos.reverse();
    }

    for (index, size, align) in neg {
        result[index] = match align {
            SpanAlign::Neg => left_line,
            SpanAlign::Mid => left_line + height / 2.0,
            SpanAlign::Pos => left_line + height,
        };
        left_line += major(size) + margin;
    }

    for (index, size, align) in pos.into_iter().rev() {
        result[index] = match align {
            SpanAlign::Neg => right_line,
            SpanAlign::Mid => right_line + height / 2.0,
            SpanAlign::Pos => right_line + height,
        };
        right_line -= major(size) + margin;
    }

    let mid_len = mid.iter().map(|(_, size, _)| size).sum::<Vec2>() + margin * mid.len().saturating_sub(1) as f32;
    let mut mid_line = (right_line + left_line - major(mid_len)) / 2.0;
    for (index, size, align) in mid {
        mid_line += major(size) / 2.0;
        result[index] = match align {
            SpanAlign::Neg => mid_line,
            SpanAlign::Mid => mid_line + height / 2.0,
            SpanAlign::Pos => mid_line + height,
        };
        mid_line += major(size) / 2.0 + margin;
    }
    result
}


/// Lay out a wrapping span.
pub fn paragraph<const DIR: u8, const STACK: u8, const STRETCH: bool>(mut size: BoxSize, items: impl IntoIterator<Item = FlexItem>, align: &Alignment) -> Vec<Vec2> {
    let dir: FlexDir = DIR.into();
    let stack: Stacking = STACK.into();
    let axis: Axis = dir.into();
    let mut curr = Vec::new();
    let mut result = Vec::new();
    let original = size.offset;
    let line_dir = match axis {
        Axis::Horizontal => |a: Vec2| a.x,
        Axis::Vertical => |a: Vec2| a.y,
    };
    let stack_dir = match axis {
        Axis::Horizontal => |a: Vec2| Vec2::new(0.0, a.y),
        Axis::Vertical => |a: Vec2| Vec2::new(a.x, 0.0),
    };
    let inf = match axis {
        Axis::Horizontal => Vec2::new(f32::INFINITY, 0.0),
        Axis::Vertical =>  Vec2::new(0.0, f32::INFINITY),
    };
    let margin = line_dir(size.margin);
    let line_margin = stack_dir(size.margin) * stack.signum();
    let mut len = 0.0;
    let max = line_dir(size.dimension);
    for item in items {
        len += line_dir(item.dimension);
        if len > max || item.flex_control == FlexControl::LinebreakMarker {
            let mut line_height = maxlen_minor::<DIR>(curr.iter());
            if item.flex_control == FlexControl::LinebreakMarker {
                line_height = line_height.max(stack_dir(item.dimension));
            }
            if stack == Stacking::Lo { size.offset -= line_height; }
            result.extend(span::<DIR, STRETCH>(size.with_max_dim(line_height + inf), curr.drain(..)));
            if stack == Stacking::Hi { size.offset += line_height; }
            size.offset += line_margin;
            if item.flex_control != FlexControl::LinebreakMarker {
                len = line_dir(item.dimension);
                curr.push(item)
            } else {
                len = 0.0;
                // Insert has to happen because of the number of children must match,
                // but using this is unspecified behavior.
                result.push(Vec2::ZERO)
            }
        } else if item.flex_control == FlexControl::Linebreak {
            curr.push(item);
            len = 0.0;
            let line_height = maxlen_minor::<DIR>(curr.iter());
            if stack == Stacking::Lo { size.offset -= line_height; }
            result.extend(span::<DIR, STRETCH>(size.with_max_dim(line_height + inf), curr.drain(..)));
            if stack == Stacking::Hi { size.offset += line_height; }
            size.offset += line_margin;
        } else {
            if len > 0.0 {
                len += margin;
            }
            curr.push(item)
        }
    }
    if !curr.is_empty() {
        let line_height = maxlen_minor::<DIR>(curr.iter());
        if stack == Stacking::Lo { size.offset -= line_height; }
        result.extend(span::<DIR, STRETCH>(size.with_max_dim(line_height + inf), curr.drain(..)));
        if stack == Stacking::Hi { size.offset += line_height; }
    }
    let height = stack_dir(size.offset - original);
    let dim = stack_dir(size.dimension);

    match align.into() {
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
