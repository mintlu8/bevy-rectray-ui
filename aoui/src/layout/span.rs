use crate::LayoutControl;

use super::util::*;
use bevy::{prelude::Vec2, sprite::Anchor};

pub fn compact(
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
    advance: impl Fn(Vec2) -> Vec2,
    height: impl Fn(Vec2) -> Vec2
) -> (Vec<Vec2>, Vec2) {
    let mut result = Vec::new();
    let margin = advance(margin);
    let mut cursor = -margin;
    let line_height = height(Vec2::ONE);
    let mut max_len = Vec2::ZERO;
    for item in items {
        cursor += margin;

        let width = advance(item.dimension);
        let size = width + line_height;
        max_len = max_len.max(item.dimension);
        
        let anchor = cursor + (size / 2.0) + item.anchor.as_vec() * size.abs();
        result.push(anchor);
        cursor += width;
    }

    let height_mult = height(max_len) + advance(Vec2::ONE).abs();
    result.iter_mut().for_each(|x| *x *= height_mult);

    if cursor.cmplt(Vec2::ZERO).any(){
        let roll = cursor.min(Vec2::ZERO);
        result.iter_mut().for_each(|x| *x -= roll);
    }

    (result, cursor.abs() + height_mult)
}

pub fn span(
    size: Vec2,
    margin: Vec2,
    stretch: bool,
    items: impl IntoIterator<Item = LayoutItem>,
    buckets: impl Fn(&Anchor) -> Trinary,
    major_dir: impl Fn(Vec2) -> Vec2,
    minor_dir: impl Fn(Vec2) -> Vec2,
) -> Vec<Vec2>{
    let mut result = Vec::new();
    let mut categories = Vec::new();


    let major_dim = major_dir(size);    
    let minor_dim = minor_dir(size);

    let mut neg_len = 0usize;
    let mut mid_len = 0usize;
    let mut pos_len = 0usize;

    let mut neg_cursor = Vec2::ZERO;
    let mut mid_cursor = Vec2::ZERO;
    let mut pos_cursor = Vec2::ZERO;

    for item in items {
        let bucket = buckets(&item.anchor);
        let cursor = match buckets(&item.anchor) {
            Trinary::Neg => { neg_len += 1; &mut neg_cursor },
            Trinary::Mid => { mid_len += 1; &mut mid_cursor },
            Trinary::Pos => { pos_len += 1; &mut pos_cursor },
        };
        let cell_size = major_dir(item.dimension) + minor_dim;
        result.push(*cursor + cell_size * (item.anchor.as_vec() + 0.5));
        categories.push(bucket);
        *cursor += major_dir(item.dimension)
    }
    let margin = if stretch {
        if result.len() <= 1 {
            Vec2::ZERO
        } else {
            let remaining = major_dim - neg_cursor - mid_cursor - pos_cursor;
            remaining / (result.len() - 1) as f32
        }
    } else {
        major_dir(margin)
    };

    neg_cursor += margin * neg_len.saturating_sub(1) as f32;
    mid_cursor += margin * mid_len.saturating_sub(1) as f32;
    pos_cursor += margin * pos_len.saturating_sub(1) as f32;

    let mut neg_index = 0.0;
    let mut mid_index = 0.0;
    let mut pos_index = 0.0;

    let neg_len = neg_cursor.max(Vec2::ZERO);
    let pos_len = pos_cursor.max(Vec2::ZERO);
    let pos_roll = major_dim - pos_len;
    let mid_roll = (major_dim + neg_len - pos_len - mid_cursor) / 2.0;
    for (pos, category) in result.iter_mut().zip(categories) {
        match category {
            Trinary::Neg => {
                *pos += margin * neg_index;
                neg_index += 1.0;
            },
            Trinary::Mid => {
                *pos += margin * mid_index + mid_roll;
                mid_index += 1.0;
            },
            Trinary::Pos => {
                *pos += margin * pos_index + pos_roll;
                pos_index += 1.0;
            },
        }
    }
    result
}

pub fn paragraph(
    size: Vec2,
    margin: Vec2,
    stretch: bool,
    items: impl IntoIterator<Item = LayoutItem>,
    buckets: impl Fn(&Anchor) -> Trinary,
    line_dir: impl Fn(Vec2) -> Vec2,
    stack_dir: impl Fn(Vec2) -> Vec2,
) -> (Vec<Vec2>, Vec2){

    let length = |v| line_dir(v).x + line_dir(v).y;
    let minor_dir = |v| stack_dir(v).abs();


    let margin_flat = length(margin);
    let total = length(size);

    let mut len = 0.0;
    let mut result = Vec::new();
    let mut buffer = Vec::new();

    let mut cursor = Vec2::ZERO;

    let mut last_linebreak = false;
    for item in items {
        if len + length(item.dimension) > total 
                || item.control == LayoutControl::LinebreakMarker
                || last_linebreak {
            last_linebreak = false;
            let line_height = buffer.iter()
                .map(|x: &LayoutItem| minor_dir(x.dimension))
                .fold(Vec2::ZERO, |a, b| a.max(b));
            let line_size = line_dir(size) + line_height;
            let mut span = span(line_size, margin, stretch, buffer.drain(..), &buckets, &line_dir, &minor_dir);
            let line_height = if item.control == LayoutControl::LinebreakMarker {
                stack_dir(line_height.max(item.dimension))
            } else {
                stack_dir(line_height)
            };
            cursor += line_height.min(Vec2::ZERO);
            span.iter_mut().for_each(|x| *x += cursor);
            cursor += line_height.max(Vec2::ZERO);
            cursor += stack_dir(margin);
            result.extend(span);
            len = length(item.dimension);
        } else {
            len += length(item.dimension) + margin_flat;
        }
        if item.control == LayoutControl::Linebreak {
            last_linebreak = true;
        }
        if item.control == LayoutControl::LinebreakMarker {
            result.push(Vec2::ZERO);
        } else {
            buffer.push(item)
        }
    }

    if buffer.len() > 0 {
        let line_height = buffer.iter()
            .map(|x: &LayoutItem| minor_dir(x.dimension))
            .fold(Vec2::ZERO, |a, b| a.max(b));
        let line_size = line_dir(size) + line_height;
        let mut span = span(line_size, margin, stretch, buffer.drain(..), &buckets, &line_dir, &minor_dir);
        cursor += stack_dir(line_height).min(Vec2::ZERO);
        span.iter_mut().for_each(|x| *x += cursor);
        cursor += stack_dir(line_height).max(Vec2::ZERO);
        result.extend(span);
    } else if cursor != Vec2::ZERO {
        cursor -= stack_dir(margin);
    }

    if cursor.cmplt(Vec2::ZERO).any() {
        result.iter_mut().for_each(|x| {
            *x -= cursor.min(Vec2::ZERO)
        })
    }

    (result, cursor.abs() + line_dir(size))    
}
