use bevy::{hierarchy::Children, math::{Vec2, IVec2}, log::warn, reflect::Reflect, ecs::{query::With, system::Res, bundle::Bundle}};
use bevy::ecs::{component::Component, query::Without};
use bevy::ecs::system::{Query, Commands};
use crate::{Transform2D, signals::types::SigScroll, anim::Attr, dsl::{prelude::{Offset, EvPositionFactor}, DslInto}, Dimension, AouiREM, events::Handler};
use crate::layout::{Container, LayoutControl};
use crate::events::{EvMouseWheel, Handlers};
use crate::signals::{Receiver, KeyStorage};

use crate::events::MouseWheelAction;
pub use super::constraints::ScrollConstraint;

use super::constraints::SharedPosition;

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
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Scrolling {
    pub pos_x: bool,
    pub neg_x: bool,
    pub pos_y: bool,
    pub neg_y: bool,
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
    rem: Option<Res<AouiREM>>,
    storage: Res<KeyStorage>,
    mut scroll: Query<(&Scrolling, &Dimension, &Children, &MouseWheelAction, Option<&mut SharedPosition>)>,
    sender: Query<(&MouseWheelAction, &Handlers<EvMouseWheel>), Without<Scrolling>>,
    mut receiver: Query<( &Scrolling, &Dimension, &Children, &Receiver<SigScroll>, Option<&mut SharedPosition>), Without<MouseWheelAction>>,
    mut child_query: Query<Attr<Transform2D, Offset>, With<Children>>,
) {
    let rem = rem.map(|x| x.get()).unwrap_or(16.0);
    for (action, signal) in sender.iter() {
        signal.handle(&mut commands, &storage, *action);
    }
    let iter = scroll.iter_mut()
        .map(|(scroll, dim, children, action, shared)| 
            (scroll, dim, children, *action, shared))
        .chain(receiver.iter_mut().filter_map(|(scroll, dim, children, receiver, shared)| 
            Some((scroll, dim, children, receiver.poll()?, shared))));
    for (scroll, dim, children, delta, shared) in iter {
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
        if let Ok(mut transform) = child_query.get_mut(container){
            transform.force_set_pixels(transform.get_pixels(dim.size, dim.em, rem) + delta_scroll);
        }
        if let Some(mut shared) = shared {
            shared.updated = true;
        }
    }
}

/// Marker component for making scrolling affect 
/// the `range` value on a layout.
/// 
/// This implementation has the benefit of not requiring clipping.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub enum ScrollDiscrete {
    XPos,
    XNeg,
    YPos,
    #[default]
    YNeg,
}

impl ScrollDiscrete {
    pub fn new() -> Self {
        Self::YNeg
    }

    pub fn get(&self) -> IVec2 {
        match self {
            ScrollDiscrete::XPos => IVec2::new(1, 0),
            ScrollDiscrete::XNeg => IVec2::new(-1, 0),
            ScrollDiscrete::YPos => IVec2::new(0, 1),
            ScrollDiscrete::YNeg => IVec2::new(0, -1),
        }
    }
}


pub fn scrolling_discrete(
    mut scroll: Query<(&ScrollDiscrete, &mut Container, &Children, &MouseWheelAction, Option<&mut SharedPosition>)>,
    mut receiver: Query<(&ScrollDiscrete, &mut Container, &Children, &Receiver<SigScroll>, Option<&mut SharedPosition>), Without<MouseWheelAction>>,
    child_query: Query<&LayoutControl>,
) {
    let iter = scroll.iter_mut()
        .map(|(scroll, container, children, action, shared)| 
            (scroll, container, children, *action, shared))
        .chain(receiver.iter_mut().filter_map(|(scroll, container, children, receiver, shared)| 
            Some((scroll, container, children, receiver.poll()?, shared))));
    
    for (scroll, mut container, children, delta, shared) in iter {
        let Some(range) = container.range.clone() else {
            warn!("ScrollDiscrete requires a range in `Container`.");
            continue;
        };
        let delta = delta.lines.dot(scroll.get());
        let count = children.len() - child_query.iter_many(children).filter(|x| 
             x == &&LayoutControl::IgnoreLayout
        ).count();
        let len = range.len();
        if len > count {
            container.range = Some(0..len);
            continue;
        }
        let max = count - len;
        match delta {
            ..=-1 => {
                let start = range.start.saturating_sub((-delta) as usize).clamp(0, max);
                container.range = Some(start..start + len);
            }
            1.. => {
                let start = range.start.saturating_add(delta as usize).clamp(0, max);
                container.range = Some(start..start + len);
            } 
            0 => continue,
        };
        if let Some(mut shared) = shared {
            shared.updated = true
        }
    }
}


pub trait IntoScrollingBuilder: Bundle + Default {

    fn with_constraints(self) -> impl IntoScrollingBuilder {
        (ScrollConstraint, self)
    }

    fn with_shared_position(self, position: impl DslInto<SharedPosition>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), position.dinto())
    }

    fn with_handler(self, handler: impl DslInto<Handlers<EvPositionFactor>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_send(self, handler: impl DslInto<Handlers<EvMouseWheel>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_recv(self, handler: impl DslInto<Receiver<SigScroll>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }
}

impl IntoScrollingBuilder for Scrolling {}

impl<T, A> IntoScrollingBuilder for (T, A) where T: IntoScrollingBuilder, A: Bundle + Default {
    fn with_constraints(self) -> impl IntoScrollingBuilder { 
        (T::with_constraints(self.0), self.1)
    }
}

impl<T> IntoScrollingBuilder for (ScrollConstraint, T) where T: IntoScrollingBuilder {
    fn with_constraints(self) -> impl IntoScrollingBuilder { self }
}