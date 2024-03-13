use bevy::ecs::bundle::Bundle;
use bevy::ecs::query::{Has, With};
use bevy::hierarchy::Parent;
use bevy::math::Vec2;
use bevy::ecs::{component::Component, query::Without, entity::Entity};
use bevy::ecs::system::{Query, Res};
use bevy_defer::signals::{SignalId, SignalReceiver, SignalSender};
use crate::util::{Rem, WindowSize};
use crate::DimensionData;
use crate::{Transform2D, anim::Attr};
use serde::{Serialize, Deserialize};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::Offset};

use super::constraints::{constraint_system, listen_shared_position, Constraint, ConstraintBundle, ConstraintQuery};
use super::constraints::SharedPosition;

/// A component that enables dragging and dropping.
/// By default the sprite can be dragged anywhere with no restriction.
///
/// This works with all mouse buttons as long as
/// you add the corresponding `EventFlags`.
///
/// # Supporting components
///
/// * [`EventFlags`]: Requires `Drag` to be set.
/// * [`Constraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`DragSnapBack`]: Move the sprite back to its original position when dropped.
/// * [`Dragging`]: When used as a signal, 
///     receives `MouseDrag` on a draggable sprite with no event listener.
///     This is useful for creating a small draggable area, like a banner.
/// * [`SharedPosition`]: Shares relative position in its parent's bounds with another widget.
///     For example synchronizing scrollbar with a textbox.
/// * [`PositionFac`](super::constraints::PositionFac): 
///     A signal that sends a value in `0..=1` in its constraints when being dragged.

#[derive(Debug, Clone, Copy, Component)]
pub struct Dragging {
    pub x: bool,
    pub y: bool,
    pub drag_start: Vec2,
}

impl SignalId for Dragging {
    type Data = DragState;
}

impl Dragging {
    pub const X: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            x: true,
            y: false,
            drag_start: Vec2::ZERO
        },
        constraint: Constraint,
    };
    
    pub const Y: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            x: false,
            y: true,
            drag_start: Vec2::ZERO
        },
        constraint: Constraint,
    };

    pub const BOTH: ConstraintBundle<Self> = ConstraintBundle{
        item: Self {
            x: true,
            y: true,
            drag_start: Vec2::ZERO
        },
        constraint: Constraint,
    };
    
    pub fn last_drag_start(&self) -> Vec2 {
        self.drag_start
    }
    fn set(&mut self, value: Vec2) {
        self.drag_start = value
    }

    pub fn with_snap_back(self) -> impl Bundle {
        (self, DragSnapBack::DEFAULT)
    }

    pub fn with_constraints(self) -> impl Bundle {
        (self, Constraint)
    }

    pub fn with_snap_constraints(self) -> impl Bundle {
        (self, DragSnapBack::DEFAULT, Constraint)
    }
}

impl ConstraintBundle<Dragging> {
    pub fn with_snap_back(self) -> impl Bundle {
        (self, DragSnapBack::DEFAULT)
    }
}

impl Default for Dragging {
    fn default() -> Self {
        Self::BOTH.item
    }
}

/// Component that moves the sprite back to its original position if dropped.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct DragSnapBack {
    drag_start: Option<Vec2>,
}

impl DragSnapBack {
    pub const DEFAULT: Self = Self { drag_start: None };

    fn set(&mut self, value: Vec2) {
        self.drag_start = Some(value)
    }
}


pub(crate) fn drag_start(
    send: Query<(&CursorAction, SignalSender<Dragging>), Without<Dragging>>,
    mut receive: Query<(SignalReceiver<Dragging>, &mut Dragging, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut Dragging, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>)>,
) {
    for (focus, send) in send.iter() {
        if focus.intersects(EventFlags::AnyDown)  {
            send.send(DragState::Start);
        }
    }

    let iter = query.iter_mut()
        .filter_map(|(action, drag, transform, snap)| {
            action.intersects(EventFlags::AnyDown).then_some((drag, transform, snap))
        }).chain(receive.iter_mut()
        .filter_map(|(action, drag, transform, snap)|{
            (action.poll_once() == Some(DragState::Start)).then_some((drag, transform, snap))
        }));

    for (mut drag, mut transform, mut snap) in iter {
        match transform.component.offset.get_pixels() {
            Some(pixels) => {
                drag.set(pixels);
                if let Some(snap) = &mut snap {
                    snap.set(transform.take());

                }
            },
            None => panic!("Draggable sprites must have pixel units."),
        }
    }
}

/// State used to transfer the dragging event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragState {
    #[default]
    Start,
    Dragging,
    End,
}

pub(crate) fn dragging(
    window_size: WindowSize,
    rem: Rem,
    state: Res<CursorState>,
    send: Query<(&CursorFocus, SignalSender<Dragging>), Without<Dragging>>,
    mut query: Query<(
        Entity, Option<&Parent>, &Dragging, Attr<Transform2D, Offset>, 
        Option<&CursorFocus>, SignalReceiver<Dragging>, Has<SharedPosition>,
    )>,
    mut constraints: Query<ConstraintQuery, With<Constraint>>,
    parent_query: Query<&DimensionData>,
) {
    let window_size = window_size.get();
    let rem = rem.get();
    let delta = state.cursor_position() - state.down_position();
    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::AnyDrag)  {
            continue;
        }
        send.send(DragState::Dragging);
    }
    for (entity, parent, drag, mut transform, focus, recv, has_shared) in query.iter_mut() {
        if !(drag.x || drag.y) { continue; }
        if !focus.map(|x| x.intersects(EventFlags::AnyDrag)).unwrap_or(false) 
                && recv.poll_once() != Some(DragState::Dragging) {
            if has_shared {
                if let Ok(constraints) = constraints.get_mut(entity) {
                    let parent = parent
                        .and_then(|x| parent_query.get(**x).ok())
                        .map(|x| x.size)
                        .unwrap_or(window_size);
                    listen_shared_position(constraints, &mut transform, drag.x, drag.y, parent, rem)
                }
            }
            continue;
        }

        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0},
                if drag.y {delta.y} else {0.0},
            )
        };
        transform.force_set_pixels(pos);
        if let Ok(constraints) = constraints.get_mut(entity) {
            let parent = parent
                .and_then(|x| parent_query.get(**x).ok())
                .map(|x| x.size)
                .unwrap_or(window_size);
            constraint_system(constraints, &mut transform, drag.x, drag.y, parent, rem)
        }
    }
}



pub(crate) fn drag_end(
    send: Query<(&CursorAction, SignalSender<Dragging>), Without<Dragging>>,
    mut receive: Query<(&mut DragSnapBack, Attr<Transform2D, Offset>, SignalReceiver<Dragging>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut DragSnapBack, Attr<Transform2D, Offset>)>
) {
    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::DragEnd)  {
            continue;
        }
        send.send(DragState::End);
    }

    let iter = query.iter_mut()
        .filter_map(|(action, drag, transform)| {
            if action.intersects(EventFlags::DragEnd) {
                Some((drag, transform))
            } else {
                None
            }
        }).chain(receive.iter_mut()
        .filter_map(|(drag, transform, recv)|{
            if recv.poll_once() == Some(DragState::End) {
                Some((drag, transform))
            } else {
                None
            }
        }));

    for (mut snap, mut transform) in iter {
        if let Some(orig) = snap.drag_start.take() {
            transform.set(orig)
        }
    }
}
