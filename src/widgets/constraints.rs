use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::ecs::query::WorldQuery;
use bevy::log::warn;
use bevy::math::Vec2;
use bevy::reflect::Reflect;
use crate::dsl::prelude::Signals;
use crate::util::convert::DslConvert;
use crate::DimensionData;
use bevy_defer::signals::SignalId;
use crate::{Transform2D, Anchor, anim::Attr};
use crate::anim::Offset;

#[derive(Debug, Clone, Copy, Default, Bundle)]
pub struct ConstraintBundle<T: Component + Copy> {
    pub item: T,
    pub constraint: Constraint,
}

impl<T: Component + Copy> ConstraintBundle<T> {
    pub const fn without_constraint(self) -> T {
        self.item
    }
}

impl<T: Component + Copy> DslConvert<T, 'ç'> for ConstraintBundle<T> {
    fn parse(self) -> T {
        self.item
    }
    fn sealed(_: crate::util::convert::SealToken) {}
}

fn filter_nan(v: Vec2) -> Vec2 {
    Vec2::new(
        if v.x.is_nan() {0.0} else {v.x},
        if v.y.is_nan() {0.0} else {v.y},
    )
}

fn flip_vec(v: Vec2, [x, y]: [bool; 2]) -> Vec2 {
    Vec2::new(
        if x {1.0 - v.x} else {v.x},
        if y {1.0 - v.y} else {v.y},
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionFac {}

impl SignalId for PositionFac {
    type Data = f32;
}

/// A shared percentage based position.
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct SharedPosition{
    pub flip: [bool; 2],
}

impl SharedPosition {
    pub fn new(x: bool, y: bool) -> Self {
        Self { flip: [x, y] }
    }

    pub fn transform(&self, v: Vec2) -> Vec2 {
        let [x, y] = self.flip;
        Vec2::new(
            if x {-v.x} else {v.x}, 
            if y {-v.y} else {v.y}, 
        )
    }
}

impl SignalId for SharedPosition {
    type Data = Vec2;
}

/// Constraints this based on its parent.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Default, Reflect)]
pub struct Constraint;

pub(crate) type ConstraintQuery = (
    &'static DimensionData,
    Option<&'static SharedPosition>,
    Option<&'static Signals>,
);

pub fn constraint_system(
    query: <ConstraintQuery as WorldQuery>::Item<'_>,
    transform: &mut <Attr<Transform2D, Offset> as WorldQuery>::Item<'_>, 
    dir_x: bool,
    dir_y: bool,
    dimension: Vec2,
    rem: f32,
) {
    let (dim, shared, signals) = query;

    let min = dimension * Anchor::BOTTOM_LEFT;
    let max = dimension * Anchor::TOP_RIGHT;
    let origin = dimension * transform.component.get_parent_anchor()
        - dim.size * transform.component.anchor;
    let min = min + dim.size / 2.0 - origin;
    let max = max - dim.size / 2.0 - origin;
    let (min, max) = (min.min(max), min.max(max));

    let mut pos = transform.get_pixels(dimension, dim.em, rem);

    if dir_x && max.x >= min.x {
        pos.x = pos.x.clamp(min.x, max.x);
    }
    if dir_y && max.y >= min.y {
        pos.y = pos.y.clamp(min.y, max.y);
    }
    let fac = filter_nan((pos - min) / (max - min));
    transform.force_set(pos);
    let flip = match shared {
        Some(SharedPosition { flip, .. }) => *flip,
        None => [false, false],
    };
    let Some(signals) = signals else {return};
    // broadcast bypasses poll_senders_once.
    signals.broadcast::<SharedPosition>(flip_vec(fac, flip));
    match (dir_x, dir_y) {
        (true, false) => {
            let value = fac.x.clamp(0.0, 1.0);
            signals.send::<PositionFac>(value);
        },
        (false, true) => {
            let value = fac.y.clamp(0.0, 1.0);
            signals.send::<PositionFac>(value);
        },
        (true, true) if signals.has_sender::<PositionFac>() => {
            warn!("Warning: Cannot Send `PositionFactor` with 2d dragging.")
        }
        _ => (),
    }
}


pub fn listen_shared_position(
    query: <ConstraintQuery as WorldQuery>::Item<'_>,
    transform: &mut <Attr<Transform2D, Offset> as WorldQuery>::Item<'_>, 
    dir_x: bool,
    dir_y: bool,
    dimension: Vec2,
    rem: f32,
) {
    let (dim, shared, Some(signals)) = query else {return};

    if let Some(position) = signals.poll_sender_once::<SharedPosition>() {
        let min = dimension * Anchor::BOTTOM_LEFT;
        let max = dimension * Anchor::TOP_RIGHT;
        let origin = dimension * transform.component.get_parent_anchor()
            - dim.size * transform.component.anchor;
        let min = min + dim.size / 2.0 - origin;
        let max = max - dim.size / 2.0 - origin;
        let (min, max) = (min.min(max), min.max(max));

        let mut pos = transform.get_pixels(dimension, dim.em, rem);
        let flip = match shared {
            Some(SharedPosition { flip, .. }) => *flip,
            None => [false, false],
        };
        let fac = flip_vec(position, flip);
        if fac.is_nan() { return; }
        if dir_x {
            pos.x = (max.x - min.x) * fac.x + min.x;
        }
        if dir_y {
            pos.y = (max.y - min.y) * fac.y + min.y;
        }
        transform.force_set(pos)
    }
}
