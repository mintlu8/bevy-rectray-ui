use bevy::ecs::entity::Entity;
use bevy::ecs::system::In;
use bevy::{reflect::Reflect, log::warn, ecs::query::With};
use bevy::window::{Window, PrimaryWindow};
use bevy::math::{Vec2, IVec2};
use bevy::hierarchy::{Children, Parent};
use bevy::ecs::{component::Component, system::{Res, Query}};
use crate::dsl::prelude::Signals;
use crate::util::ChildIter;
use crate::DimensionData;
use crate::events::MovementUnits;
use crate::sync::SignalId;
use crate::{AouiREM, Transform2D, Anchor, anim::Attr, layout::Container};
use crate::anim::Offset;

use super::{scroll::{Scrolling, ScrollDiscrete}, drag::Dragging};

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
}

impl SignalId for SharedPosition {
    type Data = Vec2;
}

/// Constraints this based on its parent.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Default, Reflect)]
pub struct DragConstraint;

/// Constraints children based on this entity.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Default, Reflect)]
pub struct ScrollConstraint;


pub fn scroll_constraint(
    input: In<Vec<Entity>>,
    rem: Option<Res<AouiREM>>,
    mut query: Query<(&Scrolling, &DimensionData, Option<&SharedPosition>, 
        ChildIter, Option<&mut Signals>,
    ), With<ScrollConstraint>>,
    mut child_query: Query<(&DimensionData, Attr<Transform2D, Offset>, Option<&Children>)>,
) {
    let rem = rem.map(|x|x.get()).unwrap_or(16.0);
    let mut iter = query.iter_many_mut(input.0);
    while let Some((scroll, dimension, shared, children, mut signals)) = iter.fetch_next() {
        let size = dimension.size;
        let Some(container) = children.get_single() else {
            warn!("Component 'Scrolling' requires exactly one child as a buffer.");
            continue;
        };
        if let Ok((_, transform, Some(children))) = child_query.get(container){
            if transform.component.anchor != Anchor::CENTER {
                warn!("Component 'Scrolling' requires its child to have Anchor::Center.");
                continue;
            }
            let offset = transform.get();
            let size_min = size * Anchor::BOTTOM_LEFT;
            let size_max = size * Anchor::TOP_RIGHT;
            let mut min = Vec2::ZERO;
            let mut max = Vec2::ZERO;
            for (dimension, transform, ..) in child_query.iter_many(children) {
                let anc = size * transform.component.get_parent_anchor();
                let offset = transform.get_pixels(size, dimension.em, rem);
                let center = anc + offset - dimension.size * transform.component.anchor;
                let bl = center + dimension.size * Anchor::BOTTOM_LEFT;
                let tr = center + dimension.size * Anchor::TOP_RIGHT;
                min = min.min(bl);
                max = max.max(tr);
            }
            let constraint_min = Vec2::new(
                if scroll.neg_x {f32::MIN} else {0.0},
                if scroll.neg_y {f32::MIN} else {0.0},
            );
            let constraint_max = Vec2::new(
                if scroll.pos_x {f32::MAX} else {0.0},
                if scroll.pos_y {f32::MAX} else {0.0},
            );
            let (min, max) = (
                (size_min - min).min(size_max - max).min(Vec2::ZERO).max(constraint_min),
                (size_max - max).max(size_min - min).max(Vec2::ZERO).min(constraint_max),
            );
            let flip = match shared {
                Some(SharedPosition { flip }) => *flip,
                None => [false, false],
            };
            match signals.as_mut().and_then(|s| s.poll_senders_once::<SharedPosition>()) {
                Some(position) => {
                    let fac = flip_vec(position, flip);
                    if fac.is_nan() { continue; }
                    if let Ok((_, mut transform, _)) = child_query.get_mut(container){
                        transform.force_set((max - min) * fac + min);
                    }
                },
                None => {
                    let Ok(mut transform) = child_query.get_mut(container).map(|(_, t, _)| t) else {continue};
                    transform.force_set(offset.clamp(min, max));

                    let delta = offset - transform.get();
                    if delta != Vec2::ZERO {
                        let action = MovementUnits {
                            lines: IVec2::ZERO,
                            pixels: delta,
                        };
                        if let Some(signals) = &signals {
                            signals.send::<Scrolling>(action);
                        }
                    }
                    let fac = filter_nan((offset - min) / (max - min));
                    let Some(signals) = signals else {continue};
                    signals.broadcast::<SharedPosition>(flip_vec(fac, flip));

                    match (scroll.x_scroll(), scroll.y_scroll()) {
                        (true, false) => {
                            let value = fac.x.clamp(0.0, 1.0);
                            signals.send::<PositionFac>(value);
                        },
                        (false, true) => {
                            let value = fac.y.clamp(0.0, 1.0);
                            signals.send::<PositionFac>(value);
                        },
                        (true, true) if signals.has_receiver::<PositionFac>() => {
                            warn!("Warning: Cannot Send `SigPositionFactor` with 2d scrolling.")
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn drag_constraint(
    input: In<Vec<Entity>>,
    window: Query<&Window, With<PrimaryWindow>>,
    rem: Option<Res<AouiREM>>,
    mut query: Query<(&Dragging, Attr<Transform2D, Offset>, &DimensionData,
        Option<&SharedPosition>,
        Option<&Parent>,
        Option<&mut Signals>,
    ), With<DragConstraint>>,
    parent_query: Query<&DimensionData>,
) {
    let window_size = window.get_single().map(|x| Vec2::new(x.width(), x.height())).ok();
    let rem = rem.map(|x| x.get()).unwrap_or(16.0);

    let mut iter = query.iter_many_mut(input.0);
    while let Some((drag, mut transform, dim, shared, parent, mut signals)) = iter.fetch_next(){
        let Some(dimension) = parent
            .and_then(|p| parent_query.get(p.get()).ok())
            .map(|x| x.size)
            .or(window_size)
            else {continue};

        let min = dimension * Anchor::BOTTOM_LEFT;
        let max = dimension * Anchor::TOP_RIGHT;
        let origin = dimension * transform.component.get_parent_anchor()
            - dim.size * transform.component.anchor;
        let min = min + dim.size / 2.0 - origin;
        let max = max - dim.size / 2.0 - origin;
        let (min, max) = (min.min(max), min.max(max));

        let mut pos = transform.get_pixels(dimension, dim.em, rem);

        if drag.x && max.x >= min.x {
            pos.x = pos.x.clamp(min.x, max.x);
        }
        if drag.y && max.y >= min.y {
            pos.y = pos.y.clamp(min.y, max.y);
        }
        let fac = filter_nan((pos - min) / (max - min));
        transform.force_set(pos);
        let flip = match shared {
            Some(SharedPosition { flip, .. }) => *flip,
            None => [false, false],
        };
        match signals.as_mut().and_then(|s| s.poll_senders_once::<SharedPosition>()) {
            Some(position) => {
                let fac = flip_vec(position, flip);
                if fac.is_nan() { continue; }
                if drag.x {
                    pos.x = (max.x - min.x) * fac.x + min.x;
                }
                if drag.y {
                    pos.y = (max.y - min.y) * fac.y + min.y;
                }
                transform.force_set(pos)
            }
            None => {
                let Some(signals) = signals else {continue};
                signals.broadcast::<SharedPosition>(flip_vec(fac, flip));
                match (drag.x, drag.y) {
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
        }
    }
}


pub fn discrete_scroll_sync(
    input: In<Vec<Entity>>,
    mut query: Query<(&ScrollDiscrete, Option<&SharedPosition>, &mut Container, Option<&mut Signals>)>,
) {
    let mut iter = query.iter_many_mut(input.0);
    while let Some((scroll, shared, mut container, mut signals)) = iter.fetch_next() {
        let flip = match shared {
            Some(SharedPosition { flip, .. }) => *flip,
            None => [false, false],
        };
        let fac = container.get_fac();
        match signals.as_mut().and_then(|s| s.poll_once::<SharedPosition>()) {
            Some(position) => {
                let fac = flip_vec(position, flip);
                if fac.is_nan() { continue; }
                container.set_fac(fac.x + fac.y);
            },
            None => {
                let Some(signals) = signals else {continue};
                let mut fac2 = fac * scroll.get().as_vec2();
                if fac2.x < 0.0 || fac2.y < 0.0 {
                    fac2 += Vec2::ONE;
                }
                signals.broadcast::<SharedPosition>(flip_vec(fac2, flip));
                signals.send::<PositionFac>(fac)
            }
        }
    }
}
