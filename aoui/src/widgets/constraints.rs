use std::sync::Arc;
use atomic::{Atomic, Ordering};
use bevy::ecs::entity::Entity;
use bevy::ecs::query::Has;
use bevy::{reflect::Reflect, log::warn, ecs::query::With};
use bevy::window::{Window, PrimaryWindow};
use bevy::math::{Vec2, IVec2};
use bevy::hierarchy::{Children, Parent};
use bevy::ecs::{component::Component, system::{Commands, Res, Query}};
use crate::{signals::KeyStorage, AouiREM, Dimension, Transform2D, Anchor, anim::Attr, layout::Container};
use crate::anim::Offset;
use crate::events::{Handlers, EvMouseWheel, MouseWheelAction, EvPositionFactor};
use self::sealed::BuildSharedPosition;

use super::{scroll::{Scrolling, ScrollDiscrete}, drag::Dragging};

fn filter_nan(v: Vec2) -> Vec2 {
    Vec2::new(
        if v.x.is_nan() {0.0} else {v.x}, 
        if v.y.is_nan() {0.0} else {v.y},
    )
}

fn flip_vec(v: Vec2, [x, y]: &[bool; 2]) -> Vec2 {
    Vec2::new(
        if *x {v.x} else {1.0 - v.x}, 
        if *y {v.y} else {1.0 - v.y}, 
    )
}

/// A marker component for denoting position changed via dragging or scrolling this frame.
#[derive(Debug, Clone, Component, Reflect)]
#[component(storage="SparseSet")]
pub struct PositionChanged;

/// Remove [`PositionChanged`].
pub fn remove_position_changed(mut commands: Commands, 
    query: Query<Entity, With<PositionChanged>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<PositionChanged>();
    }
}
/// A shared percentage based position.
#[derive(Debug, Clone, Component, Reflect)]
pub struct SharedPosition{
    #[reflect(ignore)]
    pub position: Arc<Atomic<Vec2>>,
    pub flip: [bool; 2],
}

impl SharedPosition {
    pub fn flip(mut self, x: bool, y: bool) -> Self {
        self.flip = [x, y];
        self
    }
}

impl Default for SharedPosition {
    fn default() -> Self {
        Self {
            position: Arc::new(Atomic::new(Vec2::NAN)),
            flip: [false; 2],
        }
    }

}

mod sealed {
    use super::SharedPosition;

    pub trait BuildSharedPosition: Sized  {
        fn build() -> Self;
    }

    impl<const N: usize> BuildSharedPosition for [SharedPosition; N] {
        fn build() -> Self {
            let base = SharedPosition::default();
            core::array::from_fn(|_| base.clone())
        }
    }

    macro_rules! impl_build_pos {
        ($name:ident) => {
            impl BuildSharedPosition for ($name,) {
                fn build() -> Self {
                    ($name::default(),)
                }
            }
        };
        ($name:ident $(,$rest: ident)*) => {
            impl BuildSharedPosition for ($name $(,$rest)*) {
                fn build() -> Self {
                    let first = $name::default();
                    ($({let val: $rest = first.clone(); val},)* first)
                }
            }
            impl_build_pos!($($rest),*);
        };
    }

    impl_build_pos!(
        SharedPosition,SharedPosition,SharedPosition,SharedPosition,
        SharedPosition,SharedPosition,SharedPosition,SharedPosition,
        SharedPosition,SharedPosition,SharedPosition,SharedPosition
    );
}


impl SharedPosition {

    pub fn new() -> Self {
        Self {
            position: Arc::new(Atomic::new(Vec2::NAN)),
            flip: [false; 2],
        }
    }

    pub fn many<T: BuildSharedPosition>() -> T {
        T::build()
    }
}

/// Constraints this based on its parent.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub struct DragConstraint;

/// Constraints children based on this entity.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub struct ScrollConstraint;


pub fn scroll_constraint(
    mut commands: Commands,
    storage: Res<KeyStorage>,
    rem: Option<Res<AouiREM>>,
    query: Query<(&Scrolling, &Dimension, Option<&SharedPosition>, &Children, 
        Option<&Handlers<EvMouseWheel>>,
        Option<&Handlers<EvPositionFactor>>,
        Has<PositionChanged>,
    ), With<ScrollConstraint>>,
    mut child_query: Query<(&Dimension, Attr<Transform2D, Offset>, Option<&Children>)>,
) {
    let rem = rem.map(|x|x.get()).unwrap_or(16.0);
    for (scroll, dimension, shared, children, scroll_handler, fac_handler, changed) in query.iter() {
        let size = dimension.size;
        
        if children.len() != 1 {
            warn!("Component 'Scrolling' requires exactly one child as a buffer.");
            continue;
        }
        let container = children[0];
        if let Ok((_, transform, Some(children))) = child_query.get(container){
            if transform.component.anchor != Anchor::Center {
                warn!("Component 'Scrolling' requires its child to have Anchor::Center.");
                continue;
            }
            let offset = transform.get();
            let size_min = size * Anchor::BottomLeft;
            let size_max = size * Anchor::TopRight;
            let mut min = Vec2::ZERO;
            let mut max = Vec2::ZERO;
            for (dimension, transform, ..) in child_query.iter_many(children) {
                let anc = size * transform.component.get_parent_anchor();
                let offset = transform.get_pixels(size, dimension.em, rem);
                let center = anc + offset - dimension.size * transform.component.anchor;
                let bl = center + dimension.size * Anchor::BottomLeft;
                let tr = center + dimension.size * Anchor::TopRight;
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
            let Ok(mut transform) = child_query.get_mut(container).map(|(_, t, _)| t) else {continue};
            let offset = offset.clamp(min, max);
            transform.force_set(offset);
            match shared {
                None if changed => {
                    let fac = filter_nan((offset - min) / (max - min));
                    match (scroll.x_scroll(), scroll.y_scroll()) {
                        (true, false) => {
                            let value = fac.x.clamp(0.0, 1.0);
                            if let Some(signal) = fac_handler {
                                signal.handle(&mut commands, &storage, value)
                            }
                        },
                        (false, true) => {
                            let value = fac.y.clamp(0.0, 1.0);
                            if let Some(signal) = fac_handler {
                                signal.handle(&mut commands, &storage, value)
                            }
                        },
                        (true, true) if fac_handler.is_some() => {
                            warn!("Warning: Cannot Send `SigPositionFactor` with 2d scrolling.")
                        }
                        _ => (),
                    }
                },
                None => (),
                Some(SharedPosition{ position, flip }) if changed => {
                    // If scrolled to the end pipe the scroll event to the parent.
                    if let Some(piping) = scroll_handler {
                        let delta = offset - transform.get();
                        if delta != Vec2::ZERO {
                            let action = MouseWheelAction {
                                lines: IVec2::ZERO,
                                pixels: delta,
                            };
                            piping.handle(&mut commands, &storage, action);
                        }
                    }
                    let fac = filter_nan((offset - min) / (max - min));
                    position.store(flip_vec(fac, flip), Ordering::Relaxed);
    
                    match (scroll.x_scroll(), scroll.y_scroll()) {
                        (true, false) => {
                            let value = fac.x.clamp(0.0, 1.0);
                            if let Some(signal) = fac_handler {
                                signal.handle(&mut commands, &storage, value)
                            }
                        },
                        (false, true) => {
                            let value = fac.y.clamp(0.0, 1.0);
                            if let Some(signal) = fac_handler {
                                signal.handle(&mut commands, &storage, value)
                            }
                        },
                        (true, true) if fac_handler.is_some() => {
                            warn!("Warning: Cannot Send `SigPositionFactor` with 2d scrolling.")
                        }
                        _ => (),
                    }
                },
                Some(SharedPosition{ position, flip }) => {
                    let fac = flip_vec(position.load(Ordering::Relaxed), flip);
                    if fac.is_nan() { continue; }
                    transform.force_set((max - min) * fac + min);
                },
            }
        }            
    }
}

pub fn drag_constraint(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    storage: Res<KeyStorage>,
    rem: Option<Res<AouiREM>>,
    mut query: Query<(&Dragging, Attr<Transform2D, Offset>, &Dimension, 
        Option<&SharedPosition>,
        Option<&Parent>, 
        Option<&Handlers<EvPositionFactor>>,
        Has<PositionChanged>,
    ), With<DragConstraint>>,
    parent_query: Query<&Dimension>,
) {
    let window_size = window.get_single().map(|x| Vec2::new(x.width(), x.height())).ok();
    let rem = rem.map(|x| x.get()).unwrap_or(16.0);

    for (drag, mut transform, dim, shared, parent, fac_handler, changed) in query.iter_mut() {
        let Some(dimension) = parent
            .and_then(|p| parent_query.get(p.get()).ok())
            .map(|x| x.size)
            .or(window_size)
            else {continue};
            
        let min = dimension * Anchor::BottomLeft;
        let max = dimension * Anchor::TopRight;
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
        match shared {
            None if changed => {
                match (drag.x, drag.y) {
                    (true, false) => {
                        let value = fac.x.clamp(0.0, 1.0);
                        if let Some(signal) = fac_handler {
                            signal.handle(&mut commands, &storage, value)
                        }
                    },
                    (false, true) => {
                        let value = fac.y.clamp(0.0, 1.0);
                        if let Some(signal) = fac_handler {
                            signal.handle(&mut commands, &storage, value)
                        }
                    },
                    (true, true) if fac_handler.is_some() => {
                        warn!("Warning: Cannot Send `SigPositionFactor` with 2d dragging.")
                    }
                    _ => (),
                }
            },
            None => (),
            Some(SharedPosition { position, flip }) if changed => {
                position.store(flip_vec(fac, flip), Ordering::Relaxed);
                match (drag.x, drag.y) {
                    (true, false) => {
                        let value = fac.x.clamp(0.0, 1.0);
                        if let Some(signal) = fac_handler {
                            signal.handle(&mut commands, &storage, value)
                        }
                    },
                    (false, true) => {
                        let value = fac.y.clamp(0.0, 1.0);
                        if let Some(signal) = fac_handler {
                            signal.handle(&mut commands, &storage, value)
                        }
                    },
                    (true, true) if fac_handler.is_some() => {
                        warn!("Warning: Cannot Send `SigPositionFactor` with 2d dragging.")
                    }
                    _ => (),
                }
            },
            Some(SharedPosition { position, flip }) => {
                let fac = flip_vec(position.load(Ordering::Relaxed), flip);
                if fac.is_nan() { continue; }
                if drag.x {
                    pos.x = (max.x - min.x) * fac.x + min.x;
                }
                if drag.y {
                    pos.y = (max.y - min.y) * fac.y + min.y;
                }
                transform.force_set(pos)
            },
        }
    }
}


pub fn discrete_scroll_sync(
    mut commands: Commands,
    storage: Res<KeyStorage>,
    mut query: Query<(&ScrollDiscrete, &mut Container, &Children,
        Option<&SharedPosition>,
        Option<&Handlers<EvPositionFactor>>,
        Has<PositionChanged>,
    )>,
) {
    for (scroll, mut container, children, shared, fac_handler, changed) in query.iter_mut() {
        let Some(range) = container.range.as_mut() else {continue};
        let len = children.len() - range.end;
        let fac = if len == 0 {0.0} else {range.start as f32 / len as f32};
        match shared {
            Some(SharedPosition{ position, flip }) if changed => {
                let mut fac2 = fac * scroll.get().as_vec2();
                if fac2.x < 0.0 || fac2.y < 0.0 {
                    fac2 += Vec2::ONE;
                }
                position.store(flip_vec(fac2, flip), Ordering::Relaxed);
                if let Some(signal) = fac_handler {
                    signal.handle(&mut commands, &storage, fac)
                }
            },
            Some(SharedPosition{ position, flip }) => {
                let fac = flip_vec(position.load(Ordering::Relaxed), flip);
                if fac.is_nan() { continue; }
                let mut pos = (fac / scroll.get().as_vec2()).dot(Vec2::ONE);
                if pos < 0.0 {
                    pos += 1.0;
                }
                range.start = ((pos * len as f32).round() as usize).max(len);
            },
            None => (),
        }
    }
}
