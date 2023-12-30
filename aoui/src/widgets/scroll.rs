use bevy::{hierarchy::Children, math::{Vec2, IVec2}, log::warn, reflect::Reflect, ecs::{query::With, system::Res, bundle::Bundle, entity::Entity}};
use bevy::ecs::{component::Component, query::Without};
use bevy::ecs::system::{Query, Commands};
use crate::{Transform2D, signals::types::SigScroll, anim::Attr, anim::Offset, events::EvPositionFactor, Dimension, AouiREM};
use crate::layout::{Container, LayoutControl};
use crate::events::{EvMouseWheel, Handlers};
use crate::signals::{Receiver, KeyStorage};
use crate::dsl::DslInto;

use crate::events::MouseWheelAction;
pub use super::constraints::ScrollConstraint;

use super::constraints::{SharedPosition, PositionChanged};

/// Add mouse wheel scrolling support.
/// 
/// This component moves children in this sprites
/// bounding area.
/// 
/// # Setup Requirements
/// 
/// * add a single child with the `Size2::FULL` and
/// `Anchor::Center`, which acts as a container.
/// * add children to that child.
/// 
/// # Supporting components
/// 
/// * [`EventFlags`](crate::events::EventFlags): Requires `MouseWheel` to be set.
/// * [`ScrollConstraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`Handlers<EvMouseWheel>`]: 
///     A signal that transfers the `being scrolled` status onto another entity.
///     This will trigger if either scrolled to the end or not scrollable to begin with.
/// * [`Receiver<SigScroll>`]: 
///     Receives `EvMouseWheel` on another scrollable sprite.
/// * [`SharedPosition`]: Shares relative position in its parent's bounds with another widget. 
///     For example synchronizing a scrollbar with a textbox.
/// * [`Handlers<EvPositionFac>`]: A signal that sends a value 
///     in `0..=1` in its constraints when being scrolled.
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
    mut scroll: Query<(Entity, &Scrolling, &Dimension, &Children, &MouseWheelAction)>,
    sender: Query<(&MouseWheelAction, &Handlers<EvMouseWheel>), Without<Scrolling>>,
    mut receiver: Query<(Entity, &Scrolling, &Dimension, &Children, &Receiver<SigScroll>), Without<MouseWheelAction>>,
    mut child_query: Query<Attr<Transform2D, Offset>, With<Children>>,
) {
    let rem = rem.map(|x| x.get()).unwrap_or(16.0);
    for (action, signal) in sender.iter() {
        signal.handle(&mut commands, &storage, *action);
    }
    let iter = scroll.iter_mut()
        .map(|(entity, scroll, dim, children, action)| 
            (entity, scroll, dim, children, *action))
        .chain(receiver.iter_mut().filter_map(|(entity, scroll, dim, children, receiver)| 
            Some((entity, scroll, dim, children, receiver.poll()?))));
    for (entity, scroll, dim, children, delta) in iter {
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
        commands.entity(entity).insert(PositionChanged);
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
    mut commands: Commands,
    mut scroll: Query<(Entity, &ScrollDiscrete, &mut Container, &Children, &MouseWheelAction)>,
    mut receiver: Query<(Entity, &ScrollDiscrete, &mut Container, &Children, &Receiver<SigScroll>), Without<MouseWheelAction>>,
    child_query: Query<&LayoutControl>,
) {
    let iter = scroll.iter_mut()
        .map(|(entity, scroll, container, children, action)| 
            (entity, scroll, container, children, *action))
        .chain(receiver.iter_mut().filter_map(|(entity, scroll, container, children, receiver)| 
            Some((entity, scroll, container, children, receiver.poll()?))));
    
    for (entity, scroll, mut container, children, delta) in iter {
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
        commands.entity(entity).insert(PositionChanged);
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