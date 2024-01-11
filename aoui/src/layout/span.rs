use std::{iter::repeat, mem};

use crate::layout::{Layout, StackLayout, LayoutOutput, LayoutControl, SpanLayout, ParagraphLayout};

use super::{util::*, LayoutInfo, LayoutRange};
use bevy::{prelude::Vec2, ecs::entity::Entity};

impl<D: Direction> Layout for StackLayout<D> {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>, range: &mut LayoutRange) -> LayoutOutput {
        let margin = parent.margin;
        range.resolve(entities.len());
        stack::<D>(margin, &entities[range.to_range(entities.len())]).normalized().with_max(entities.len())
    }
}

impl<D: StretchDir> Layout for SpanLayout<D> {
    fn place(&self, parent: &LayoutInfo, mut entities: Vec<LayoutItem>, range: &mut LayoutRange) -> LayoutOutput {
        let margin = parent.margin;
        let dimension = parent.dimension;
        range.resolve(entities.len());
        let len = entities.len();
        let entity_anchors = span::<D>(dimension, margin, &mut entities[range.to_range(len)]);
        LayoutOutput { entity_anchors, dimension, max_count: entities.len() }.normalized().with_max(entities.len())
    }
}

impl<D1: StretchDir, D2: Direction> Layout for ParagraphLayout<D1, D2> where (D1, D2): DirectionPair {
    fn place(&self, parent: &LayoutInfo, entities: Vec<LayoutItem>, _:  &mut LayoutRange) -> LayoutOutput {
        let margin = parent.margin;
        let dim = parent.dimension;
        paragraph::<D1, D2>(dim, margin, entities).normalized()
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

pub(crate) fn stack<D: Direction>(
    margin: Vec2,
    items: &[LayoutItem]
) -> LayoutOutput {
    let mut result = Vec::new();
    let margin = D::main(margin);
    let mut cursor = -margin;
    let line_height = D::side_vec(1.0);
    let mut max_len = Vec2::ZERO;
    let items = trim(items, |x| x.control == LayoutControl::WhiteSpace);
    for item in items {
        cursor += margin;

        let width = D::main(item.dimension);
        let size = width + line_height;
        max_len = max_len.max(item.dimension);

        let anchor = cursor + (size / 2.0) + item.anchor.as_vec() * size.abs();
        result.push((item.entity, anchor));
        cursor += width;
    }

    let height_mult = D::side(max_len) + D::main(Vec2::ONE).abs();
    result.iter_mut().for_each(|(_, x)| *x *= height_mult);

    if cursor.cmplt(Vec2::ZERO).any(){
        let roll = cursor.min(Vec2::ZERO);
        result.iter_mut().for_each(|(_, x)| *x -= roll);
    }
    LayoutOutput {
        entity_anchors: result,
        dimension: cursor.abs() + height_mult,
        max_count: items.len()
    }
}

pub(crate) fn span<D: StretchDir>(
    size: Vec2,
    margin: Vec2,
    items: &mut [LayoutItem],
) -> Vec<(Entity, Vec2)>{
    let mut result = Vec::new();

    let major_dim = D::Pos::main(size);
    let minor_dim = D::Pos::side(size);

    let mut neg_len = 0usize;
    let mut mid_len = 0usize;
    let mut pos_len = 0usize;

    if D::reversed() { items.reverse(); }

    items.iter().for_each(|x| {
        match D::bucket(x.anchor) {
            Trinary::Neg => neg_len += 1,
            Trinary::Mid => mid_len += 1,
            Trinary::Pos => pos_len += 1,
        }
    });

    // This in fact does not get called when len is 1.
    items.sort_by_cached_key(|x| {
        match D::bucket(x.anchor) {
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
        let cell_size = D::Pos::main(item.dimension) + minor_dim;
        result.push((item.entity, neg_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        neg_cursor += D::Pos::main(item.dimension)
    }

    for item in mid{
        let cell_size = D::Pos::main(item.dimension) + minor_dim;
        result.push((item.entity, mid_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        mid_cursor += D::Pos::main(item.dimension)
    }

    for item in pos{
        let cell_size = D::Pos::main(item.dimension) + minor_dim;
        result.push((item.entity, pos_cursor + cell_size * (item.anchor.as_vec() + 0.5)));
        pos_cursor += D::Pos::main(item.dimension)
    }

    let margin = if D::STRETCH {
        if result.len() <= 1 {
            Vec2::ZERO
        } else {
            let remaining = major_dim - neg_cursor - mid_cursor - pos_cursor;
            remaining / (result.len() - 1) as f32
        }
    } else {
        D::Pos::main(margin)
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

pub(crate) fn paragraph<D1: StretchDir, D2: Direction>(
    size: Vec2,
    margin: Vec2,
    items: impl IntoIterator<Item = LayoutItem>,
) -> LayoutOutput{

    let margin_flat = D1::len(margin);
    let total = D1::len(size);

    let mut len = 0.0;
    let mut result = Vec::new();
    let mut buffer = Vec::new();

    let mut cursor = Vec2::ZERO;

    let mut last_linebreak = false;
    let mut lines = 0;
    for item in items {
        if len + D1::len(item.dimension) > total
                || item.control == LayoutControl::LinebreakMarker
                || last_linebreak {
            last_linebreak = false;
            let line_height = buffer.iter()
                .map(|x: &LayoutItem| D2::main(x.dimension).abs())
                .fold(Vec2::ZERO, |a, b| a.max(b));
            let line_size = D1::main(size) + line_height;
            let mut span = span::<D1>(line_size, margin, &mut mem::take(&mut buffer));
            let line_height = if item.control == LayoutControl::LinebreakMarker {
                D2::main(line_height.max(item.dimension))
            } else {
                D2::main(line_height)
            };
            cursor += line_height.min(Vec2::ZERO);
            span.iter_mut().for_each(|(_, x)| *x += cursor);
            cursor += line_height.max(Vec2::ZERO);
            cursor += D2::main(margin);
            result.extend(span);
            len = D1::len(item.dimension) + margin_flat;
            lines += 1;
        } else {
            len += D1::len(item.dimension) + margin_flat;
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
            .map(|x: &LayoutItem| D2::main(x.dimension).abs())
            .fold(Vec2::ZERO, |a, b| a.max(b));
        let line_size = D1::main(size) + line_height;
        let mut span = span::<D1>(line_size, margin, &mut buffer);
        cursor += D2::main(line_height).min(Vec2::ZERO);
        span.iter_mut().for_each(|(_, x)| *x += cursor);
        cursor += D2::main(line_height).max(Vec2::ZERO);
        result.extend(span);
        lines += 1;
    } else if cursor != Vec2::ZERO {
        cursor -= D2::main(margin);
    }

    if cursor.cmplt(Vec2::ZERO).any() {
        result.iter_mut().for_each(|(_, x)| {
            *x -= cursor.min(Vec2::ZERO)
        })
    }

    LayoutOutput {
        entity_anchors: result,
        dimension: cursor.abs() + D1::main(size),
        max_count: lines
    }
}
