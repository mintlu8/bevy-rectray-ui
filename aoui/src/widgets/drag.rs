use bevy::math::Vec2;
use bevy::ecs::{component::Component, query::Without, bundle::Bundle, entity::Entity};
use bevy::ecs::system::{Query, Res, Commands};
use crate::dsl::DslInto;
use crate::{Transform2D, anim::Attr};
use crate::events::{Handlers, EvMouseDrag, EvPositionFactor};
use crate::signals::{Invoke, ReceiveInvoke};
use serde::{Serialize, Deserialize};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::Offset};

use super::{SharedPosition, constraints::PositionChanged};
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
/// * [`Receiver<SigDrag>`]: 
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


pub fn drag_start(
    mut commands: Commands,
    send: Query<(Entity, &CursorAction, &Handlers<EvMouseDrag>), Without<Dragging>>,
    mut receive: Query<(&Invoke<Dragging>, &mut Dragging, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut Dragging, Attr<Transform2D, Offset>, Option<&mut DragSnapBack>)>,
) {
    for (entity, focus, send) in send.iter() {
        let mut commands = commands.entity(entity);
        if focus.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown)  {
            send.handle(&mut commands, DragState::Start);
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

/// State used to transfer the dragging event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragState {
    #[default]
    Start,
    Dragging,
    End,
}

impl ReceiveInvoke for Dragging {
    type Type = DragState;
}

pub fn dragging(
    mut commands: Commands,
    state: Res<CursorState>,
    send: Query<(Entity, &CursorFocus, &Handlers<EvMouseDrag>), Without<Dragging>>,
    mut query: Query<(Entity, &CursorFocus, &Dragging, Attr<Transform2D, Offset>)>,
    mut receive: Query<(Entity, &Dragging, Attr<Transform2D, Offset>, &Invoke<Dragging>), Without<CursorFocus>>,
) {
    let delta = state.cursor_position() - state.down_position();

    for (entity, focus, send) in send.iter() {
        let mut commands = commands.entity(entity);
        if !focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)  {
            continue;
        }
        send.handle(&mut commands, DragState::Dragging);
    }

    let iter = query.iter_mut()
        .filter_map(|(entity, focus, drag, transform)| {
            focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)
                .then_some((entity, drag, transform))
        }).chain(receive.iter_mut()
        .filter_map(|(entity, drag, transform, recv)|
            (recv.poll() == Some(DragState::Dragging)).then_some((entity, drag, transform))
        ));

    for (entity, drag, mut transform) in iter {
        if !(drag.x || drag.y) { continue; }
        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0}, 
                if drag.y {delta.y} else {0.0}, 
            )
        };
        transform.force_set(pos);
        commands.entity(entity).insert(PositionChanged);
    }
}



pub fn drag_end(
    mut commands: Commands,
    send: Query<(Entity, &CursorAction, &Handlers<EvMouseDrag>), Without<Dragging>>,
    mut receive: Query<(&mut DragSnapBack, Attr<Transform2D, Offset>, &Invoke<Dragging>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut DragSnapBack, Attr<Transform2D, Offset>)>
) {
    for (entity, focus, send) in send.iter() {
        let mut commands = commands.entity(entity);
        if !focus.intersects(EventFlags::DragEnd)  {
            continue;
        }
        send.handle(&mut commands, DragState::End);
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

/// Builder trait for a draggable widget.
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

    fn with_invoke(self, handler: impl DslInto<Handlers<EvMouseDrag>>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_recv(self, handler: impl DslInto<Invoke<Dragging>>) -> impl IntoDraggingBuilder {
        (self.with_constraints(), handler.dinto())
    }
}

impl IntoDraggingBuilder for Dragging {}

impl<T> IntoDraggingBuilder for (DragSnapBack, T) where T: IntoDraggingBuilder {
    fn with_constraints(self) -> impl IntoDraggingBuilder { 
        (self.0, T::with_constraints(self.1)) 
    }
}

impl<T, A> IntoDraggingBuilder for (T, A) where T: IntoDraggingBuilder, A: Bundle + Default {
    fn with_constraints(self) -> impl IntoDraggingBuilder { self }
}
