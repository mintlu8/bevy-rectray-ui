use bevy::{math::Vec2, ecs::{system::{Query, Res, Commands}, component::Component, query::Without, bundle::Bundle}};
use crate::{Transform2D, signals::{types::SigDrag, KeyStorage},events::{Handlers, EvMouseDrag}, anim::Attr, dsl::{DslInto, prelude::EvPositionFactor}};
use serde::{Serialize, Deserialize};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::Offset};
use crate::signals::Receiver;

use super::SharedPosition;
pub use super::constraints::DragConstraint;


/// A component that enables dragging and dropping. 
/// By default the sprite can be dragged anywhere with no restriction.
/// 
/// This works with all mouse buttons as long as
/// you add the corresponding event flags.
/// 
/// # Supporting components
/// 
/// * [`EventFlags`]: Requires `Drag` to be set.
/// * [`DragConstraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`DragSnapBack`]: Move the sprite back to its original position if dropped. 
/// Uses `Transition` if applicable.
/// * [`Sender<Changed>`]: A signal that sends a value in `0..=1` in its constraints when being dragged.
/// * [`SigDrag`]: 
/// Sent by a non-draggable sprite with a drag event handler, 
/// and received by a draggable sprite without an event handler.
/// This is useful for creating a small draggable area, like a banner.

/// 
/// # Panics
/// 
/// If offset is not in `px`.
#[derive(Debug, Clone, Copy, Component)]
pub struct Draggable {
    pub x: bool,
    pub y: bool,
    drag_start: Vec2,
}

impl Draggable {
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
}

impl Default for Draggable {
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


pub fn drag_start(
    mut commands: Commands,
    storage: Res<KeyStorage>,
    send: Query<(&CursorAction, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut receive: Query<(&Receiver<SigDrag>, &mut Draggable, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut Draggable, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>)>,
) {

    for (focus, send) in send.iter() {
        if focus.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown)  {
            send.handle(&mut commands, &storage, DragState::Start);
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
        .filter_map(|(action, drag, transform, snap)|{
            if action.poll() == Some(DragState::Start) {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragState {
    #[default]
    Start,
    Dragging,
    End,
}

pub fn dragging(
    mut commands: Commands,
    storage: Res<KeyStorage>,
    state: Res<CursorState>,
    send: Query<(&CursorFocus, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut query: Query<(&CursorFocus, &Draggable, Attr<Transform2D, Offset>, Option<&mut SharedPosition>)>,
    mut receive: Query<(&Draggable, Attr<Transform2D, Offset>, &Receiver<SigDrag>, Option<&mut SharedPosition>), Without<CursorFocus>>,
) {
    let delta = state.cursor_position() - state.down_position();

    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)  {
            continue;
        }
        send.handle(&mut commands, &storage, DragState::Dragging);
    }

    let iter = query.iter_mut()
        .filter_map(|(focus, drag, transform, shared)| {
            focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)
                .then(||(drag, transform, shared))
        }).chain(receive.iter_mut()
        .filter_map(|(drag, transform, recv, shared)|
            (recv.poll() == Some(DragState::Dragging)).then(||(drag, transform, shared))
        ));

    for (drag, mut transform, shared) in iter {
        if !(drag.x || drag.y) { continue; }
        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0}, 
                if drag.y {delta.y} else {0.0}, 
            )
        };
        transform.force_set(pos);
        if let Some(mut shared) = shared {
            shared.updated = true
        }
    }
}



pub fn drag_end(
    mut commands: Commands,
    storage: Res<KeyStorage>,
    send: Query<(&CursorAction, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut receive: Query<(&mut DragSnapBack, Attr<Transform2D, Offset>, &Receiver<SigDrag>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut DragSnapBack, Attr<Transform2D, Offset>)>
) {
    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::DragEnd)  {
            continue;
        }
        send.handle(&mut commands, &storage, DragState::End);
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
            if recv.poll() == Some(DragState::End) {
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
pub trait IntoDraggingBuilder: Bundle + Default {

    fn with_constraints(self) -> impl IntoDraggingBuilder {
        (self, DragConstraint)
    }
    
    fn with_snap_back(self) -> impl IntoDraggingBuilder {
        (DragSnapBack::DEFAULT, self)
    }

    fn with_position(self, position: impl DslInto<SharedPosition>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), position.dinto())
    }

    fn with_handler(self, handler: impl DslInto<Handlers<EvPositionFactor>>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_send(self, handler: impl DslInto<Handlers<EvMouseDrag>>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_recv(self, handler: impl DslInto<Receiver<SigDrag>>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), handler.dinto())
    }
}

impl IntoDraggingBuilder for Draggable {}

impl<T> IntoDraggingBuilder for (DragSnapBack, T) where T: IntoDraggingBuilder {
    fn with_constraints(self) -> impl IntoDraggingBuilder { 
        (self.0, T::with_constraints(self.1)) 
    }
}

impl<T, A> IntoDraggingBuilder for (T, A) where T: IntoDraggingBuilder, A: Bundle + Default {
    fn with_constraints(self) -> impl IntoDraggingBuilder { self }
}
