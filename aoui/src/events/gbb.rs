use bevy::{ecs::{component::Component, system::Query}, hierarchy::Parent, math::Vec2};

use crate::{sync::{SignalId, SignalSender}, util::{Rem, WindowSize}, Anchor, DimensionData, Transform2D};

/// An signal that calculates the sized of the sprite's bounding
/// rectangle of **offset** or **dimension**.
#[derive(Debug, Clone)]
pub enum GreaterBoundingBoxPx {}

impl SignalId for GreaterBoundingBoxPx {
    type Data = Vec2;
}


/// An signal that calculates the sized of the sprite's bounding
/// rectangle of **offset** or **dimension** in relation to the parent's dimension.
#[derive(Debug, Clone)]
pub enum GreaterBoundingBoxPercent {}

impl SignalId for GreaterBoundingBoxPercent {
    type Data = Vec2;
}

/// Runs signals [`GreaterBoundingBoxPx`] and [`GreaterBoundingBoxPercent`]
/// that calculates the size of a sprites greater bounding box, including anchor.
#[derive(Debug, Clone, Component)]
pub struct GreaterBoundingBox {
    pixels: Vec2,
    percent: Vec2,
}
impl GreaterBoundingBox {
    pub const fn new() -> Self {
        Self { pixels: Vec2::NAN, percent: Vec2::NAN }
    }
}

impl Default for GreaterBoundingBox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_greater_bounding_box(
    window_size: WindowSize,
    rem: Rem,
    mut query: Query<(&mut GreaterBoundingBox, &Transform2D, &DimensionData, Option<&Parent>,
        SignalSender<GreaterBoundingBoxPx>,
        SignalSender<GreaterBoundingBoxPercent>)>,
    parent_query: Query<&DimensionData>,
) {
    let window_size = window_size.get();
    let rem = rem.get();
    for (mut bounds, transform, dimension, parent, px, pct) in query.iter_mut() {
        let parent = parent.and_then(|p| parent_query.get(**p).ok())
            .map(|x| x.size)
            .unwrap_or(window_size);
        let anchor = parent * transform.get_parent_anchor();
        let center = anchor - dimension.size * transform.anchor;
        let offset = transform.offset.as_pixels(parent, dimension.em, rem);
        let bl = center + offset + dimension.size * Anchor::BOTTOM_LEFT;
        let tr = center + offset + dimension.size * Anchor::TOP_RIGHT;
        let min = bl.min(anchor);
        let max = tr.max(anchor);
        let dim = max - min;

        if bounds.pixels != dim {
            bounds.pixels = dim;
            px.send(dim);
        }
        let mut percent = dim / parent;
        if percent.is_nan() {
            percent = Vec2::ZERO;
        }

        if bounds.percent != percent {
            bounds.percent = percent;
            pct.send(percent);
        }
    }
}
