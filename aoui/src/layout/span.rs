use std::{iter::repeat, mem};

use crate::{Anchor, layout::{Layout, CompactLayout, LayoutOutput, LayoutControl, SpanLayout, ParagraphLayout, DynamicSpanLayout}};

use super::{util::*, LayoutInfo};
use bevy::{prelude::Vec2, ecs::entity::Entity};

impl Layout for CompactLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let margin = parent.margin;
        match self.direction {
            LayoutDir::LeftToRight => compact(margin, entities, posx, posy),
            LayoutDir::RightToLeft => compact(margin, entities, negx, posy),
            LayoutDir::BottomToTop => compact(margin, entities, posy, posx),
            LayoutDir::TopToBottom => compact(margin, entities, negy, posx),
        }.normalized()
    }

    fn dimension_agnostic(&self) -> bool {
        true
    }
}

impl Layout for SpanLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let margin = parent.margin;
        let dimension = parent.dimension;
        let entity_anchors = match self.direction{
            LayoutDir::LeftToRight => span::<false>(dimension, margin, self.stretch, entities, hbucket, posx, posy),
            LayoutDir::RightToLeft => span::<true>(dimension, margin, self.stretch, entities, hbucket, posx, posy),
            LayoutDir::BottomToTop => span::<false>(dimension, margin, self.stretch, entities, vbucket, posy, posx),
            LayoutDir::TopToBottom => span::<true>(dimension, margin, self.stretch, entities, vbucket, posy, posx),
        };
        LayoutOutput { entity_anchors, dimension }.normalized()
    }
}


impl Layout for DynamicSpanLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let margin = parent.margin;
        let dimension = parent.dimension;
        let line_size = match self.direction.into() {
            Axis::Horizontal => {
                let line_height: f32 = entities.iter().map(|x| x.dimension.y).fold(0.0, |a, b| a.max(b));
                Vec2::new(dimension.x, line_height)
            },
            Axis::Vertical => {
                let line_height: f32 = entities.iter().map(|x| x.dimension.x).fold(0.0, |a, b| a.max(b));
                Vec2::new(line_height, dimension.y)
            },
        };
        let entity_anchors = match self.direction{
            LayoutDir::LeftToRight => span::<false>(line_size, margin, self.stretch, entities, hbucket, posx, posy),
            LayoutDir::RightToLeft => span::<true>(line_size, margin, self.stretch, entities, hbucket, posx, posy),
            LayoutDir::BottomToTop => span::<false>(line_size, margin, self.stretch, entities, vbucket, posy, posx),
            LayoutDir::TopToBottom => span::<true>(line_size, margin, self.stretch, entities, vbucket, posy, posx),
        };
        LayoutOutput { entity_anchors, dimension: line_size }.normalized()
    }
}

impl Layout for ParagraphLayout {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>) -> LayoutOutput {
        let margin = parent.margin;
        let dim = parent.dimension;
        const R: LayoutDir = LayoutDir::LeftToRight;
        const L: LayoutDir = LayoutDir::RightToLeft;
        const T: LayoutDir = LayoutDir::BottomToTop;
        const B: LayoutDir = LayoutDir::TopToBottom;
        let stretch = self.stretch;
        match (self.direction, self.stack) {
            (R, B) => paragraph::<false>(dim, margin, stretch, entities, hbucket, posx, negy),
            (L, B) => paragraph::<true >(dim, margin, stretch, entities, hbucket, posx, negy),
            (T, L) => paragraph::<false>(dim, margin, stretch, entities, vbucket, posy, negx),
            (B, L) => paragraph::<true >(dim, margin, stretch, entities, vbucket, posy, negx),
            (R, T) => paragraph::<false>(dim, margin, stretch, entities, hbucket, posx, posy),
            (L, T) => paragraph::<true >(dim, margin, stretch, entities, hbucket, posx, posy),
            (T, R) => paragraph::<false>(dim, margin, stretch, entities, vbucket, posy, posx),
            (B, R) => paragraph::<true >(dim, margin, stretch, entities, vbucket, posy, posx),
            _ => panic!("Direction and stack must be othogonal.")
        }.normalized()
    }
}

fn trim<T>(slice: &[T], mut f: impl FnMut(&T) -> bool) -> &[T]{
    let mut min = 0;
    let mut max = slice.len();
    for (i, v) in slice.iter().enumerate() {
        min = i;
        if !f(v) { break }
    }
    for (i, v) in slice.iter().enumerate().skip(min).rev() {
        if !f(v) { break }
        max = i;
    }
    &slice[min..max]
}

pub(crate) fn compact(
    margin: Vec2,
    items: Vec<LayoutItem>,
    advance: impl Fn(Vec2) -> Vec2,
    height: impl Fn(Vec2) -> Vec2
) -> LayoutOutput {
    let mut result = Vec::new();
    let margin = advance(margin);
    let mut cursor = -margin;
    let line_height = height(Vec2::ONE);
    let mut max_len = Vec2::ZERO;
    let items = trim(&items, |x| x.control == LayoutControl::WhiteSpace);
    for item in items {
        cursor += margin;

        let width = advance(item.dimension);
        let size = width + line_height;
        max_len = max_len.max(item.dimension);
        
        let anchor = cursor + (size / 2.0) + item.anchor.as_vec() * size.abs();
        result.push((item.entity, anchor));
        cursor += width;
    }

    let height_mult = height(max_len) + advance(Vec2::ONE).abs();
    result.iter_mut().for_each(|(_, x)| *x *= height_mult);

    if cursor.cmplt(Vec2::ZERO).any(){
        let roll = cursor.min(Vec2::ZERO);
        result.iter_mut().for_each(|(_, x)| *x -= roll);
    }
    LayoutOutput {
        entity_anchors: result,
        dimension: cursor.abs() + height_mult,
    }
}

pub(crate) fn span<const REV: bool>(
    size: Vec2,
    margin: Vec2,
    stretch: bool,
    mut items: Vec<LayoutItem>,
    buckets: impl Fn(&Anchor) -> Trinary,
    major_dir: impl Fn(Vec2) -> Vec2,
    minor_dir: impl Fn(Vec2) -> Vec2,
) -> Vec<(Entity, Vec2)>{
    let mut result = Vec::new();

    let major_dim = major_dir(size);    
    let minor_dim = minor_dir(size);

    let mut neg_len = 0usize;
    let mut mid_len = 0usize;
    let mut pos_len = 0usize;

    if REV { items.reverse(); }

    items.iter().for_each(|x| {
        match buckets(&x.anchor) {
            Trinary::Neg => neg_len += 1,
            Trinary::Mid => mid_len += 1,
            Trinary::Pos => pos_len += 1,
        }
    });

    // This in fact does not get called when len is 1.
    items.sort_by_cached_key(|x| {
        match buckets(&x.anchor) {
            Trinary::Neg => 0,
            Trinary::Mid => 1,
            Trinary::Pos => 2,
        }
    });

    let neg = trim(&items[0..neg_len], |x| x.control == LayoutControl::WhiteSpace);
    let mid = trim(&items[neg_len..neg_len + mid_len], |x| x.control == LayoutControl::WhiteSpace);
    let pos = trim(&items[neg_len + mid_len..neg_len + mid_len + pos_len], |x| x.control == LayoutControl::WhiteSpace);

    let mut neg_cursor = Vec2::ZERO;
    let mut mid_cursor = Vec2::ZERO;
    let mut pos_cursor = Vec2::ZERO;

    for item in neg{
        let cell_size = major_dir(item.dimension) + minor_dim;
        result.push((item.entity, neg_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        neg_cursor += major_dir(item.dimension)
    }

    for item in mid{
        let cell_size = major_dir(item.dimension) + minor_dim;
        result.push((item.entity, mid_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        mid_cursor += major_dir(item.dimension)
    }

    for item in pos{
        let cell_size = major_dir(item.dimension) + minor_dim;
        result.push((item.entity, pos_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        pos_cursor += major_dir(item.dimension)
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

    neg_cursor += margin * neg.len().saturating_sub(1) as f32;
    mid_cursor += margin * mid.len().saturating_sub(1) as f32;
    pos_cursor += margin * pos.len().saturating_sub(1) as f32;

    let mut neg_index = 0.0;
    let mut mid_index = 0.0;
    let mut pos_index = 0.0;

    let neg_len = neg_cursor.max(Vec2::ZERO);
    let pos_len = pos_cursor.max(Vec2::ZERO);
    let pos_offset = major_dim - pos_len;
    let mid_offset = (major_dim + neg_len - pos_len - mid_cursor) / 2.0;

    let categories = repeat(Trinary::Neg).take(neg.len())
        .chain(repeat(Trinary::Mid).take(mid.len()))
        .chain(repeat(Trinary::Pos).take(pos.len()));

    for ((_, pos), category) in result.iter_mut().zip(categories) {
        match category {
            Trinary::Neg => {
                *pos += margin * neg_index;
                neg_index += 1.0;
            },
            Trinary::Mid => {
                *pos += margin * mid_index + mid_offset;
                mid_index += 1.0;
            },
            Trinary::Pos => {
                *pos += margin * pos_index + pos_offset;
                pos_index += 1.0;
            },
        }
    }
    result
}

pub(crate) fn paragraph<const REV: bool>(
    size: Vec2,
    margin: Vec2,
    stretch: bool,
    items: impl IntoIterator<Item = LayoutItem>,
    buckets: impl Fn(&Anchor) -> Trinary,
    line_dir: impl Fn(Vec2) -> Vec2,
    stack_dir: impl Fn(Vec2) -> Vec2,
) -> LayoutOutput{

    let length = |v| line_dir(v).x.abs() + line_dir(v).y.abs();
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
                .map(|x: &LayoutItem| minor_dir(x.dimension).abs())
                .fold(Vec2::ZERO, |a, b| a.max(b));
            let line_size = line_dir(size) + line_height;
            let mut span = span::<REV>(line_size, margin, stretch, mem::take(&mut buffer), &buckets, &line_dir, &minor_dir);
            let line_height = if item.control == LayoutControl::LinebreakMarker {
                stack_dir(line_height.max(item.dimension))
            } else {
                stack_dir(line_height)
            };
            cursor += line_height.min(Vec2::ZERO);
            span.iter_mut().for_each(|(_, x)| *x += cursor);
            cursor += line_height.max(Vec2::ZERO);
            cursor += stack_dir(margin);
            result.extend(span);
            len = length(item.dimension) + margin_flat;
        } else {
            len += length(item.dimension) + margin_flat;
        }
        if item.control == LayoutControl::Linebreak {
            last_linebreak = true;
        }
        if item.control != LayoutControl::LinebreakMarker {
            buffer.push(item)
        }
    }

    if !buffer.is_empty() {
        let line_height = buffer.iter()
            .map(|x: &LayoutItem| minor_dir(x.dimension).abs())
            .fold(Vec2::ZERO, |a, b| a.max(b));
        let line_size = line_dir(size) + line_height;            
        let mut span = span::<REV>(line_size, margin, stretch, buffer, &buckets, &line_dir, &minor_dir);
        cursor += stack_dir(line_height).min(Vec2::ZERO);
        span.iter_mut().for_each(|(_, x)| *x += cursor);
        cursor += stack_dir(line_height).max(Vec2::ZERO);
        result.extend(span);
    } else if cursor != Vec2::ZERO {
        cursor -= stack_dir(margin);
    }

    if cursor.cmplt(Vec2::ZERO).any() {
        result.iter_mut().for_each(|(_, x)| {
            *x -= cursor.min(Vec2::ZERO)
        })
    }

    LayoutOutput {
        entity_anchors: result,
        dimension: cursor.abs() + line_dir(size),
    }
}
