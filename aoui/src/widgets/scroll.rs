
use bevy::{hierarchy::Children, math::{Vec2, IVec2}, log::warn};
use bevy::ecs::{component::Component, query::Without};
use bevy::ecs::system::{Query, Res, Commands, ResMut};
use crate::{Dimension, Transform2D, Anchor, AouiREM, signals::types::SigScroll};
use crate::layout::{Container, LayoutControl};
use crate::events::{EvMouseWheel, Handlers, EvPositionFactor};
use crate::signals::{Receiver, KeyStorage};

use crate::events::MouseWheelAction;


/// Add size relative scrolling support.
/// 
/// This component works out of the box for both smaller
/// and larger objects relative to the parent.
/// 
/// # Setup Requirements
/// 
/// * add a single child with the `Size2::FULL` and
/// `Anchor::Center`, which acts as a container.
/// * add children to that child.
/// 
/// # Events and Signals
/// 
/// * `EvMouseWheel`: If scrolling has no effect on the sprite's position, send it to another recipient.
/// * `SigScroll`: Receives `EvScrollWheel` and scrolls this widget.
/// * `EvPositionFactor`: Sends a value between `0..=1` corresponding to the entity's location.
/// When received, act if this entity is being scrolled upon.
/// 
/// # Limitations
/// 
/// * Does not detect rotation, will always use local space orientation.
/// * Does not support `Interpolate`.
#[derive(Debug, Clone, Copy, Component)]
pub struct Scrolling {
    pos_x: bool,
    neg_x: bool,
    pos_y: bool,
    neg_y: bool,
}

impl Scrolling {
    pub const X: Scrolling = Scrolling { 
        pos_x: true, 
        neg_x: true, 
        pos_y: false, 
        neg_y: false,
    };
    pub const Y: Scrolling = Scrolling { 
        pos_x: false, 
        neg_x: false, 
        pos_y: true, 
        neg_y: true,
    };
    pub const NEG_X: Scrolling = Scrolling { 
        pos_x: false, 
        neg_x: true, 
        pos_y: false, 
        neg_y: false,
    };
    pub const NEG_Y: Scrolling = Scrolling { 
        pos_x: false, 
        neg_x: false, 
        pos_y: false, 
        neg_y: true,
    };
    pub const POS_X: Scrolling = Scrolling { 
        pos_x: true, 
        neg_x: false, 
        pos_y: false, 
        neg_y: false,
    };
    pub const POS_Y: Scrolling = Scrolling { 
        pos_x: false, 
        neg_x: false, 
        pos_y: true, 
        neg_y: false,
    };
    pub const BOTH: Scrolling = Scrolling { 
        pos_x: true, 
        neg_x: true, 
        pos_y: true, 
        neg_y: true,
    };

    pub fn x_scroll(&self) -> bool {
        self.neg_x || self.pos_x
    }

    pub fn y_scroll(&self) -> bool {
        self.neg_y || self.pos_y
    }
}

impl Default for Scrolling {
    fn default() -> Self {
        Self::BOTH
    }
}

pub fn scrolling_system(
    mut commands: Commands,
    mut key_storage: ResMut<KeyStorage>,
    rem: Option<Res<AouiREM>>,
    scroll: Query<(&Scrolling, &Dimension, &Children, &MouseWheelAction, Option<&Handlers<EvMouseWheel>>, Option<&Handlers<EvPositionFactor>>)>,
    sender: Query<(&MouseWheelAction, &Handlers<EvMouseWheel>), Without<Scrolling>>,
    receiver: Query<(&Scrolling, &Dimension, &Children, &Receiver<SigScroll>, Option<&Handlers<EvMouseWheel>>, Option<&Handlers<EvPositionFactor>>), Without<MouseWheelAction>>,
    mut child_query: Query<(&Dimension, &mut Transform2D, Option<&Children>)>,
) {
    let rem = rem.map(|x|x.get()).unwrap_or(16.0);
    for (action, signal) in sender.iter() {
        signal.handle(&mut commands, &mut key_storage, *action);
    }
    let iter = scroll.iter()
        .map(|(scroll, dimension, children, action, handler, fac)| 
            (scroll, dimension, children, *action, handler, fac))
        .chain(receiver.iter().filter_map(|(scroll, dimension, children, receiver, handler, fac)| 
            Some((scroll, dimension, children, receiver.poll()?, handler, fac))));
    for (scroll, dimension, children, delta, handler, fac) in iter {
        let size = dimension.size;
        let delta_scroll = match (scroll.x_scroll(), scroll.y_scroll()) {
            (true, true) => delta.pixels,
            (true, false) => Vec2::new(delta.pixels.x + delta.pixels.y, 0.0),
            (false, true) => Vec2::new(0.0, delta.pixels.x + delta.pixels.y),
            (false, false) => continue,
        };
        if children.len() != 1 {
            warn!("Component 'Scrolling' requires exactly one child as a buffer.");
            continue;
        }
        let container = children[0];
        if let Ok((_, transform, Some(children))) = child_query.get(container){
            if transform.anchor != Anchor::Center {
                warn!("Component 'Scrolling' requires its child to have Anchor::Center.");
                continue;
            }
            let offset = transform.offset.raw() + delta_scroll;
            let size_min = size * Anchor::BottomLeft;
            let size_max = size * Anchor::TopRight;
            let mut min = Vec2::ZERO;
            let mut max = Vec2::ZERO;
            for (dimension, transform, _) in child_query.iter_many(children) {
                let anc = size * transform.get_parent_anchor();
                let offset = transform.offset.as_pixels(size, dimension.em, rem);
                let center = anc + offset - dimension.size * transform.anchor;
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
            let clamp_min = (size_min - min).min(size_max - max).min(Vec2::ZERO);
            let clamp_max = (size_max - max).max(size_min - min).max(Vec2::ZERO);
            if let Ok((_, mut transform, _)) = child_query.get_mut(container) {
                let offset = offset.clamp(clamp_min, clamp_max);
                let offset = offset.clamp(constraint_min, constraint_max);
                // If scrolled to the end pipe the scroll event to the parent.
                if transform.offset == offset.into() {
                    if let Some(piping) = handler {
                        piping.handle(&mut commands, &mut key_storage, delta);
                    }
                }
                transform.offset = offset.into();
                if let Some(fac_handler) = fac {
                    let frac = (offset - clamp_min) / (clamp_max - clamp_min);
                    let frac = match (scroll.x_scroll(), scroll.y_scroll()) {
                        (true, false) => frac.x.clamp(0.0, 1.0),
                        (false, true) => frac.y.clamp(0.0, 1.0),
                        _ => {
                            warn!("Failed sending 'Change<Scroll>', cannot compute fraction of 2D scrolling.");
                            continue;
                        },
                    };
                    fac_handler.handle(&mut commands, &mut key_storage, frac);
                }
            }
        }            
    }
}

/// Marker component for making scrolling affect 
/// the `range` value on a layout,
/// 
/// This implementation has the benefit of not requiring scrolling.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct ScrollDiscrete(IVec2);

impl ScrollDiscrete {
    pub fn new() -> Self {
        Self(IVec2::ONE)
    }

    pub fn from_direction(direction: IVec2) -> Self {
        Self(direction)
    }
}


pub fn scrolling_discrete(
    mut commands: Commands,
    mut key_storage: ResMut<KeyStorage>,
    mut scroll: Query<(&ScrollDiscrete, &mut Container, &Children, &MouseWheelAction, Option<&Handlers<EvPositionFactor>>)>,
    mut receiver: Query<(&ScrollDiscrete, &mut Container, &Children, &Receiver<SigScroll>, Option<&Handlers<EvPositionFactor>>), Without<MouseWheelAction>>,
    child_query: Query<&LayoutControl>,
) {
    let iter = scroll.iter_mut()
        .map(|(scroll, container, children, action, fac)| 
            (scroll, container, children, *action, fac))
        .chain(receiver.iter_mut().filter_map(|(scroll, container, children, receiver, fac)| 
            Some((scroll, container, children, receiver.poll()?, fac))));
    
    for (scroll, mut container, children, delta, fac) in iter {
        let Some(range) = container.range.clone() else {
            warn!("ScrollDiscrete requires a range in `Container`.");
            continue;
        };
        let delta = delta.lines.dot(scroll.0);
        let count = children.len() - child_query.iter_many(children).filter(|x| 
             x == &&LayoutControl::IgnoreLayout
        ).count();
        let len = range.len();
        if len > count {
            container.range = Some(0..len);
            continue;
        }
        let max = count - len;
        let start = match delta {
            ..=-1 => {
                let start = range.start.saturating_sub((-delta) as usize).clamp(0, max);
                container.range = Some(start..start + len);
                start
            }
            1.. => {
                let start = range.start.saturating_add(delta as usize).clamp(0, max);
                container.range = Some(start..start + len);
                start
            } 
            0 => continue,
        };
        if let Some(fac_handler) = fac {
            let frac = start as f32 / max as f32;
            fac_handler.handle(&mut commands, &mut key_storage, frac);
        }
    }
}