use bevy::{math::Vec2, ecs::{system::{Query, Res, ResMut, Commands}, component::Component, query::{Without, With}, entity::Entity}, hierarchy::Parent, log::warn};
use crate::{Transform2D, signals::{types::SigDrag, KeyStorage}, Dimension, Anchor, events::{Handlers, EvMouseDrag, EvPositionFactor}};
use serde::{Serialize, Deserialize};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::{Interpolate, Offset}};
use crate::signals::Receiver;


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

impl Default for Draggable {
    fn default() -> Self {
        Self::BOTH
    }
}

/// Component that moves the sprite back to its original position if dropped. 
#[derive(Debug, Clone, Copy, Component)]
pub struct DragSnapBack {
    drag_start: Option<Vec2>,
}

impl DragSnapBack {
    pub const DEFAULT: Self = Self { drag_start: None };

    fn set(&mut self, value: Vec2) {
        self.drag_start = Some(value)
    }

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


/// This component prevents `Draggable` from going over its parent bounds,
/// giving it similar property to `Scrolling`.
/// 
/// If not specified, dragging is unbounded.
#[derive(Debug, Clone, Copy, Component)]
pub struct DragConstraint;


pub fn drag_start(
    mut commands: Commands,
    mut storage: ResMut<KeyStorage>,
    send: Query<(&CursorAction, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut receive: Query<(&Receiver<SigDrag>, &mut Draggable, &Transform2D, Option<&mut DragSnapBack>, Option<&mut Interpolate<Offset>>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut Draggable, &Transform2D, Option<&mut DragSnapBack>, Option<&mut Interpolate<Offset>>)>,
    ) {

    for (focus, send) in send.iter() {
        if focus.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown)  {
            send.handle(&mut commands, &mut storage, DragState::Start);
        }
    }

    let iter = query.iter_mut()
        .filter_map(|(action, drag, transform, snap, interpolate)| {
            if action.intersects(EventFlags::LeftDown | EventFlags::MidDown | EventFlags:: RightDown) {
                Some((drag, transform, snap, interpolate))
            } else {
                None
            }
        }).chain(receive.iter_mut()
        .filter_map(|(action, drag, transform, snap, interpolate)|{
            if action.poll() == Some(DragState::Start) {
                Some((drag, transform, snap, interpolate))
            } else {
                None
            }
        }));

    for (mut drag, transform, mut snap, mut interpolate) in iter {
        match transform.offset.get_pixels() {
            Some(pixels) => {
                drag.set(pixels);
                if let Some(snap) = &mut snap {
                    if let Some(inter) = &mut interpolate {
                        snap.set(inter.take_target());
                    } else {
                        snap.set(pixels);
                    }
                    
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
    mut storage: ResMut<KeyStorage>,
    state: Res<CursorState>,
    send: Query<(&CursorFocus, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut receive: Query<(Entity, &Draggable, &mut Transform2D, Option<&mut Interpolate<Offset>>, &Receiver<SigDrag>), Without<CursorFocus>>,
    mut query: Query<(Entity, &CursorFocus, &Draggable, &mut Transform2D, Option<&mut Interpolate<Offset>>)>,
    parent_query: Query<&Dimension, (Without<Draggable>, Without<DragConstraint>)>,
    constraint_query: Query<(&Parent, &Dimension, Option<&Handlers<EvPositionFactor>>), With<DragConstraint>>
) {
    let delta = state.cursor_position() - state.down_position();

    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag)  {
            continue;
        }
        send.handle(&mut commands, &mut storage, DragState::Dragging);
    }

    let iter = query.iter_mut()
        .filter_map(|(entity, focus, drag, transform, interpolate)| {
            if focus.intersects(EventFlags::LeftDrag | EventFlags::MidDrag | EventFlags:: RightDrag) {
                Some((entity, drag, transform, interpolate))
            } else {
                None
            }
        }).chain(receive.iter_mut()
        .filter_map(|(entity, drag, transform, interpolate, recv)|{
            if recv.poll() == Some(DragState::Dragging) {
                Some((entity, drag, transform, interpolate))
            } else {
                None
            }
        }));

    for (entity, drag, mut transform, interpolate) in iter {
        if !(drag.x || drag.y) { continue; }
        let mut pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0}, 
                if drag.y {delta.y} else {0.0}, 
            )
        };
        if let Ok((parent, dim, signal)) = constraint_query.get(entity) {
            if let Ok(dimension) = parent_query.get(parent.get()) {
                let min = dimension.size * Anchor::BottomLeft;
                let max = dimension.size * Anchor::TopRight;
                let origin = dimension.size * transform.get_parent_anchor() 
                    - dim.size * transform.anchor;
                let min = min + dim.size / 2.0 - origin;
                let max = max - dim.size / 2.0 - origin;
                let (min, max) = (min.min(max), min.max(max));

                if drag.x && max.x >= min.x {
                    pos.x = pos.x.clamp(min.x, max.x);
                }
                if drag.y && max.y >= min.y {
                    pos.y = pos.y.clamp(min.y, max.y);
                }
                
                if let Some(signal) = signal {
                    match (drag.x, drag.y) {
                        (true, false) => signal.handle(&mut commands, &mut storage, (pos.x - min.x) / (max.x - min.x)),
                        (false, true) => signal.handle(&mut commands, &mut storage, (pos.y - min.y) / (max.y - min.y)),
                        _ => warn!("Cannot send `Changed` signal from 2d dragging."),
                    }
                }
            } else {
                warn!("Drag constraints require a non-draggable parent to function.");
            }
        }

        //let pos = drag.last_drag_start() + pos;
        transform.offset.edit_raw(|x| *x = pos);
        if let Some(mut interpolate) = interpolate {
            interpolate.set(pos)
        }
    }
}


pub fn drag_end(
    mut commands: Commands,
    mut storage: ResMut<KeyStorage>,
    send: Query<(&CursorAction, &Handlers<EvMouseDrag>), Without<Draggable>>,
    mut receive: Query<(&mut DragSnapBack, &mut Transform2D, Option<&mut Interpolate<Offset>>, &Receiver<SigDrag>), Without<CursorAction>>,
    mut query: Query<(&CursorAction, &mut DragSnapBack, &mut Transform2D, Option<&mut Interpolate<Offset>>)>
) {
    for (focus, send) in send.iter() {
        if !focus.intersects(EventFlags::DragEnd)  {
            continue;
        }
        send.handle(&mut commands, &mut storage, DragState::End);
    }
    
    let iter = query.iter_mut()
        .filter_map(|(action, drag, transform, interpolate)| {
            if action.intersects(EventFlags::DragEnd) {
                Some((drag, transform, interpolate))
            } else {
                None
            }
        }).chain(receive.iter_mut()
        .filter_map(|(drag, transform, interpolate, recv)|{
            if recv.poll() == Some(DragState::End) {
                Some((drag, transform, interpolate))
            } else {
                None
            }
        }));

    for (mut snap, mut transform, mut interpolate) in iter {
        if let Some(orig) = snap.drag_start.take() {
            if let Some(inter) = &mut interpolate {
                inter.interpolate_to(orig)
            } else {
                transform.offset.edit_raw(|x| *x = orig)
            }
        }
    }
}
