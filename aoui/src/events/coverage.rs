use bevy::{ecs::{component::Component, query::With, system::{Query, Res}}, hierarchy::Children, math::Vec2};

use crate::{DimensionData, Transform2D, Anchor, AouiREM, sync::{SignalId, SignalSender}};

/// An signal sender that calculates how many pixels of the sprite's bounding
/// rectangle is covered by children's **offset** or **dimension**.
///
/// This calculates a min bound and a max bound of all children's
/// anchor, min bound, and max bound,
/// ignores rotation and scaling.
#[derive(Debug, Clone)]
pub enum CoveragePx {}

impl SignalId for CoveragePx {
    type Data = Vec2;
}


/// An signal sender that calculates how many percentage of the sprite's bounding
/// rectangle is covered by children's **offset** or **dimension**.
///
/// This calculates a min bound and a max bound of all children's
/// anchor, min bound, and max bound,
/// ignores rotation and scaling.
#[derive(Debug, Clone)]
pub enum CoveragePercent {}

impl SignalId for CoveragePercent {
    type Data = Vec2;
}

#[derive(Debug, Clone, Component)]
pub enum CalculateDimensionCoverage {}

pub fn calculate_coverage(
    rem: Res<AouiREM>,
    query: Query<(&Transform2D, &DimensionData, Option<&Children>,
        SignalSender<CoveragePx>,
        SignalSender<CoveragePercent>), With<CalculateDimensionCoverage>>
) {

    let rem = rem.get();
    for (_, dimension, children, percent, px) in query.iter() {
        if !percent.exists() && !px.exists() { continue; }
        let Some(children) = children else {
            percent.send(Vec2::ZERO);
            px.send(Vec2::ZERO);
            continue;
        };
        let em = dimension.em;
        let size = dimension.size;
        let mut min = Vec2::NAN;
        let mut max = Vec2::NAN;
        for child in children {
            let Ok((transform, dimension, ..)) = query.get(*child) else {continue};
            let anchor = size * transform.get_parent_anchor();
            let center = anchor - dimension.size * transform.anchor;
            let offset = transform.offset.as_pixels(size, em, rem);
            let bl = center + offset + dimension.size * Anchor::BOTTOM_LEFT;
            let tr = center + offset + dimension.size * Anchor::TOP_RIGHT;
            min = min.min(bl).min(anchor);
            max = max.max(tr).max(anchor);
        }
        let mut pixels = max - min;
        if pixels.is_nan() {
            pixels = Vec2::ZERO;
        }
        px.send(pixels);
        percent.send(pixels / size);
    }
}
