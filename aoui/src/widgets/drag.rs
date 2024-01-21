use bevy::ecs::bundle::Bundle;
use bevy::math::Vec2;
use bevy::ecs::{component::Component, query::Without, entity::Entity};
use bevy::ecs::system::{Query, Res};
use crate::sync::{SignalId, SignalReceiver, SignalSender};
use crate::{Transform2D, anim::Attr};
use serde::{Serialize, Deserialize};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::Offset};

pub use super::constraints::DragConstraint;


/// A component that enables dragging and dropping.
/// By default the sprite can be dragged anywhere with no restriction.
///
/// This works with all mouse buttons as long as
/// you add the corresponding `EventFlags`.
///
/// # Supporting components
///
/// * [`EventFlags`]: Requires `Drag` to be set.
/// * [`DragConstraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`DragSnapBack`]: Move the sprite back to its original position when dropped.
/// * [`Handlers<EvMouseDrag>`]: A signal that transfers the `being dragged` status onto another entity.
/// * [`Invoke<SigDrag>`]:
///     Receives `EvMouseDrag` on a draggable sprite with no event listener.
///     This is useful for creating a small draggable area, like a banner.
/// * [`SharedPosition`]: Shares relative position in its parent's bounds with another widget.
///     For example synchronizing scrollbar with a textbox.
/// * [`Handlers<EvPositionFac>`]: A signal that sends a value in `0..=1` in its constraints when being dragged.

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
    pub const X: Self = Self {
        x: true,
        y: false,
        drag_start: Vec2::ZERO
    };
    pub const Y: Self = Self {
        x: false,
        y: true,
        drag_start: Vec2::ZERO
    };
    pub const BOTH: Self = Self {
        x: true,
        y: true,
        drag_start: Vec2::ZERO
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
        (self, DragConstraint)
    }

    pub fn with_snap_constraints(self) -> impl Bundle {
        (self, DragSnapBack::DEFAULT, DragConstraint)
    }
}

impl Default for Dragging {
    fn default() -> Self {
        Self::BOTH
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
        if focus.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown)  {
            send.send(DragState::Start);
        }
    }

    let iter = query.iter_mut()
        .filter_map(|(action, drag, transform, snap)| {
            if action.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown) {
                Some((drag, transform, snap))
            } else {
                None
            }
        }).chain(receive.iter_mut()
        .filter_map(|(mut action, drag, transform, snap)|{
            if action.poll_once() == Some(DragState::Start) {
                Some((drag, transform, snap))
            } else {
                None
            }
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
    state: Res<CursorState>,
    send: Query<(&CursorFocus, SignalSender<Dragging>), Without<Dragging>>,
    mut query: Query<(Entity, &CursorFocus, &Dragging, Attr<Transform2D, Offset>)>,
    mut receive: Query<(Entity, &Dragging, Attr<Transform2D, Offset>, SignalReceiver<Dragging>), Without<CursorFocus>>,
) -> Vec<Entity>{
    let delta = state.cursor_position() - state.down_position();
    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)  {
            continue;
        }
        send.send(DragState::Dragging);
    }

    let iter = query.iter_mut()
        .filter_map(|(entity, focus, drag, transform)| {
            focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)
                .then_some((entity, drag, transform))
        }).chain(receive.iter_mut()
        .filter_map(|(entity, drag, transform, mut recv)|
            (recv.poll_once() == Some(DragState::Dragging)).then_some((entity, drag, transform))
        ));

    let mut out = Vec::new();

    for (entity, drag, mut transform) in iter {
        out.push(entity);
        if !(drag.x || drag.y) { continue; }
        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0},
                if drag.y {delta.y} else {0.0},
            )
        };
        transform.force_set(pos);
    }
    out
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
        .filter_map(|(drag, transform, mut recv)|{
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
