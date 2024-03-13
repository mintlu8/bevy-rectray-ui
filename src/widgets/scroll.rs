use bevy::hierarchy::Parent;
use bevy::{hierarchy::Children, reflect::Reflect};
use bevy::ecs::{bundle::Bundle, entity::Entity};
use bevy::ecs::query::{Has, With};
use bevy::ecs::system::Commands;
use bevy::math::{Vec2, IVec2};
use bevy::ecs::{component::Component, query::Without};
use bevy::ecs::system::Query;
use bevy_defer::signals::{SignalId, SignalReceiver, SignalSender};
use crate::util::{Rem, WindowSize};
use crate::{Transform2D, anim::Attr, anim::Offset, DimensionData};
use crate::events::MouseWheelAction;
use crate::layout::Container;

use crate::events::MovementUnits;

use super::constraints::{constraint_system, listen_shared_position, Constraint, ConstraintBundle, ConstraintQuery, SharedPosition};

/// Propagate MouseWheelAction once to its children.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
#[component(storage="SparseSet")]
pub struct ScrollParent;

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
/// * [`Constraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`MouseWheelAction`]:
///     A signal that transfers the `being scrolled` status onto another entity.
///     This will trigger if either scrolled to the end or not scrollable to begin with.
/// * [`Scrolling`]:
///     When used as a signal id, receives `EvMouseWheel` on another scrollable sprite.
/// * [`SharedPosition`]: Signal for sharing relative position 
///     in its parent's bounds with another widget.
///     For example synchronizing a scrollbar with a textbox.
/// * [`PositionFac`](super::constraints::PositionFac): A signal that sends a value
///     in `0..=1` in its constraints when being scrolled.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Scrolling {
    pub pos_x: bool,
    pub neg_x: bool,
    pub pos_y: bool,
    pub neg_y: bool,
}

impl SignalId for Scrolling{
    type Data = MovementUnits;
}

impl Scrolling {
    pub const X: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: true,
            neg_x: true,
            pos_y: false,
            neg_y: false,
        },
        constraint: Constraint,
    };
    pub const Y: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: false,
            neg_x: false,
            pos_y: true,
            neg_y: true,
        },
        constraint: Constraint,
    };

    pub const BOTH: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: true,
            neg_x: true,
            pos_y: true,
            neg_y: true,
        },
        constraint: Constraint,
    };

    pub const NEG_X: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: false,
            neg_x: true,
            pos_y: false,
            neg_y: false,
        },
        constraint: Constraint,
    };
    pub const POS_X: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: true,
            neg_x: false,
            pos_y: false,
            neg_y: false,
        },
        constraint: Constraint,
    };
    pub const NEG_Y: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: false,
            neg_x: false,
            pos_y: false,
            neg_y: true,
        },
        constraint: Constraint,
    };
    pub const POS_Y: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            pos_x: false,
            neg_x: false,
            pos_y: true,
            neg_y: false,
        },
        constraint: Constraint,
    };

    pub fn x_scroll(&self) -> bool {
        self.neg_x || self.pos_x
    }

    pub fn y_scroll(&self) -> bool {
        self.neg_y || self.pos_y
    }

    pub fn with_constraints(self) -> impl Bundle {
        (self, Constraint)
    }
}

impl Default for Scrolling {
    fn default() -> Self {
        Self::BOTH.item
    }
}

pub(crate) fn scrolling_senders(
    sender: Query<(&MouseWheelAction, SignalSender<Scrolling>), Without<Scrolling>>,
) {
    for (action, signal) in sender.iter() {
        signal.send(action.get());
    }
}

pub(crate) fn propagate_mouse_wheel_action(
    mut commands: Commands,
    q: Query<(&MouseWheelAction, &Children), With<ScrollParent>>,
) {
    for (action, children) in q.iter() {
        for child in children {
            commands.entity(*child).insert(*action);
        }
    }
}

pub(crate) fn scrolling_system(
    window_size: WindowSize,
    rem: Rem,
    mut query: Query<(
        Entity, Option<&Parent>, &Scrolling, &DimensionData, Attr<Transform2D, Offset>,
        Option<&MouseWheelAction>, SignalReceiver<Scrolling>, Has<SharedPosition>,
    )>,
    mut constraints: Query<ConstraintQuery, With<Constraint>>,
    parent_query: Query<&DimensionData>,
) {
    let window_size = window_size.get();
    let rem = rem.get();
    for (entity, parent, scroll, dim, mut transform, action, recv, has_shared) in query.iter_mut() {
        let delta = if let Some(action) = action {
            action.0.pixels
        } else if let Some(action) = recv.poll_once() {
            action.pixels
        } else {
            if has_shared {
                if let Ok(constraints) = constraints.get_mut(entity) {
                    let parent = parent
                        .and_then(|x| parent_query.get(**x).ok())
                        .map(|x| x.size)
                        .unwrap_or(window_size);
                    listen_shared_position(constraints, &mut transform, scroll.x_scroll(), scroll.y_scroll(), parent, rem)
                }
            }
            continue;
        };
        let delta_scroll = match (scroll.x_scroll(), scroll.y_scroll()) {
            (true, true) => delta,
            (true, false) => Vec2::new(delta.x + delta.y, 0.0),
            (false, true) => Vec2::new(0.0, delta.x + delta.y),
            (false, false) => continue,
        };
        let parent = parent
            .and_then(|x| parent_query.get(**x).ok())
            .map(|x| x.size)
            .unwrap_or(window_size);
        transform.force_set_pixels(transform.get_pixels(parent, dim.em, rem) + delta_scroll);
        if let Ok(constraints) = constraints.get_mut(entity) {
            constraint_system(constraints, &mut transform, scroll.x_scroll(), scroll.y_scroll(), parent, rem)
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

pub(crate) fn scroll_discrete_system(
    mut query: Query<(&ScrollDiscrete, &mut Container, Option<&MouseWheelAction>, 
        SignalReceiver<Scrolling>, SignalSender<SharedPosition>, Option<&SharedPosition>
    )>,
) {
    for (scroll, mut container, action, recv, send, shared) in query.iter_mut() {
        let delta = if let Some(action) = action {
            action.0.lines
        } else if let Some(action) = recv.poll_once() {
            action.lines
        } else {
            if let Some(shared) = shared {
                if let Some(fac) = send.poll_sender() {
                    let fac = shared.transform(fac).dot(scroll.get().as_vec2());
                    container.set_fac(fac);
                }
            }
            continue;
        };
        let delta = delta.dot(scroll.get());
        match delta {
            ..=-1 => {
                container.decrement();
            }
            1.. => {
                container.increment();
            }
            0 => (),
        }
        if let Some(shared) = shared {
            let fac = scroll.get().signum().as_vec2() * container.get_fac();
            send.broadcast(shared.transform(fac));
        }
    }
}
