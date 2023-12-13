
use bevy::{ecs::{system::{Query, Res, Resource}, component::Component, query::Without}, hierarchy::Children, math::Vec2, log::warn};
use crate::{Dimension, Transform2D, Anchor, AoUIREM, dsl::prelude::{Sender, Receiver, SigChange}, util::SigScroll};

use crate::events::MouseWheelAction;

/// Resource that determines the direction and magnitude of mousewheel scrolling.
#[derive(Debug, Clone, Copy, Resource)]
pub struct ScrollDirection(Vec2);

impl ScrollDirection {
    /// Normal scrolling.
    pub const UNIT: Self = Self(Vec2::ONE);
    /// Inverted scrolling, e.g. with a trackpad.
    pub const INVERTED: Self = Self(Vec2::new(1.0, -1.0));
    pub fn new(dir: Vec2) -> Self {
        Self(dir)
    }
    pub fn get(&self) -> Vec2 {
        self.0
    }
    pub fn set(&mut self, value: Vec2) {
        self.0 = value
    }
}

/// Add size relative scrolling support.
/// 
/// This component works out of the box for both smaller
/// and larger objects relative to the parent.
/// 
/// # Setup Requirements
/// 
/// * add a single child with the same dimension and
/// `Anchor::Center` to this widget.
/// * add children to that child.
#[derive(Debug, Clone, Copy, Component)]
pub struct Scrolling {
    x: bool,
    y: bool,
}

impl Scrolling {
    pub const X: Scrolling = Scrolling { x: true, y: false };
    pub const Y: Scrolling = Scrolling { x: false, y: true };
    pub const BOTH: Scrolling = Scrolling { x: true, y: true };
}

impl Default for Scrolling {

    fn default() -> Self {
        Self { x: true, y: true }
    }
}

pub fn drag_and_scroll(
    rem: Option<Res<AoUIREM>>,
    direction: Option<Res<ScrollDirection>>,
    scroll: Query<(&Scrolling, &Dimension, &Children, &MouseWheelAction, Option<&Sender<SigChange>>)>,
    sender: Query<(&MouseWheelAction, &Sender<SigScroll>), Without<Scrolling>>,
    receiver: Query<(&Scrolling, &Dimension, &Children, &Receiver<SigScroll>, Option<&Sender<SigChange>>), Without<MouseWheelAction>>,
    mut child_query: Query<(&Dimension, &mut Transform2D, Option<&Children>)>,
) {
    let rem = rem.map(|x|x.get()).unwrap_or(16.0);
    let direction = direction.map(|x|x.get()).unwrap_or(Vec2::ONE);
    for (action, signal) in sender.iter() {
        signal.send(action.get());
    }
    let iter = scroll.iter()
        .map(|(scroll, dimension, children, action, change)| 
            (scroll, dimension, children, action.get(), change))
        .chain(receiver.iter().filter_map(|(scroll, dimension, children, receiver, change)| 
            Some((scroll, dimension, children, receiver.poll()?, change)))
        );
    for (scroll, dimension, children, delta, change) in iter {
        let size = dimension.size;
        let delta_scroll = match (scroll.x, scroll.y) {
            (true, true) => delta,
            (true, false) => Vec2::new(delta.x + delta.y, 0.0),
            (false, true) => Vec2::new(0.0, delta.x + delta.y),
            (false, false) => continue,
        } * direction;
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
            let clamp_min = (size_min - min).min(size_max - max).min(Vec2::ZERO);
            let clamp_max = (size_max - max).max(size_min - min).max(Vec2::ZERO);
            if let Ok((_, mut transform, _)) = child_query.get_mut(container) {
                let offset = offset.clamp(clamp_min, clamp_max);
                transform.offset = offset.into();
                if let Some(signal) = change {
                    let frac = (offset - clamp_min) / (clamp_max - clamp_min);
                    let frac = match (scroll.x, scroll.y) {
                        (true, false) => frac.x.clamp(0.0, 1.0),
                        (false, true) => frac.y.clamp(0.0, 1.0),
                        _ => {
                            warn!("Failed sending 'Change<Scroll>', cannot compute fraction of 2D scrolling.");
                            continue;
                        },
                    };
                    signal.send(frac)
                }
            }
        }            
    }
}
