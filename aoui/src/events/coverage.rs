use bevy::{ecs::system::{Query, Res}, hierarchy::Children, math::Vec2};

use crate::{DimensionData, Transform2D, Anchor, AouiREM};

use super::{EventHandling, Handlers};


/// An signal sender that calculates how many pixels of the sprite's bounding
/// rectangle is covered by children's **offset** or **dimension**.
/// 
/// This calculates a min bound and a max bound of all children's 
/// anchor, min bound, and max bound,
/// ignores rotation and scaling.
#[derive(Debug, Clone)]
pub enum FetchCoveragePx {}


/// An signal sender that calculates how many percentage of the sprite's bounding
/// rectangle is covered by children's **offset** or **dimension**.
/// 
/// This calculates a min bound and a max bound of all children's 
/// anchor, min bound, and max bound,
/// ignores rotation and scaling.
#[derive(Debug, Clone)]
pub enum FetchCoveragePercent {}


impl EventHandling for FetchCoveragePx {
    type Data = Vec2;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for FetchCoveragePercent {
    type Data = Vec2;
    type Context = ();
    fn new_context() -> Self::Context {}
}

pub fn calculate_coverage(
    rem: Res<AouiREM>,
    query: Query<(&Transform2D, &DimensionData, Option<&Children>,
        Option<&Handlers<FetchCoveragePercent>>, 
        Option<&Handlers<FetchCoveragePx>>)>
) {
    
    let rem = rem.get();
    for (_, dimension, children, percent, px) in query.iter() {
        if percent.is_none() && px.is_none() { continue; }
        let Some(children) = children else {
            if let Some(handler) = percent {
                handler.send_signal(Vec2::ZERO);
            }
            if let Some(handler) = percent {
                handler.send_signal(Vec2::ZERO);
            }
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
            let bl = center + offset + dimension.size * Anchor::BottomLeft;
            let tr = center + offset + dimension.size * Anchor::TopRight;
            min = min.min(bl).min(anchor);
            max = max.max(tr).max(anchor);
        }
        let mut pixels = max - min;
        if pixels.is_nan() {
            pixels = Vec2::ZERO;
        }
        if let Some(handler) = px {
            handler.send_signal(pixels);
        }
        if let Some(handler) = percent {
            handler.send_signal(pixels / size);
        }
    }
}