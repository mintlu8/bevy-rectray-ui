use bevy::{math::Vec2, ecs::{system::{Query, Res}, component::Component, query::{Without, With}}, hierarchy::Parent};
use bevy_aoui::{Size2, Transform2D};

use crate::{events::{CursorAction, CursorState, EventFlags, CursorFocus}, anim::{Interpolate, Offset}};


/// A component that enables dragging and dropping. 
/// By default the sprite can be dragged anywhere with no restriction.
/// 
/// This works with all mouse buttons as long as
/// you add the corresponding event flags.
/// 
/// # Supporting components
/// 
/// * [`EventFlags`]: Requires `Drag` to be set.
/// * [`Constraint`]: This stops dragging outside of a bound and provides a linear value if applicable.
/// * [`DragSnapBack`]: Move the sprite back to its original position if dropped. 
/// Uses `Transition` if applicable.
/// * [`DragFactor`]: Yields a value when dragged.
/// * [`Sender<Changed>`](crate::Sender): A signal that sends the [`DragFactor`] when changed.
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


/// This component stops `offset` from going over this range. 
/// 
/// Min does not need to be less than max, instead
/// position close to min produces lower `DragFactor`.
/// 
/// The units have to match offset, otherwise this will panic.
/// This behavior might change in the future.
#[derive(Debug, Clone, Copy, Component)]
pub struct Constraint{
    pub min: Size2,
    pub max: Size2,
}

/// Records the distance from `Constraint::min` compared to `Constraint::max`, in range `0.0..=1.0`
/// 
/// This should only be used on single axis dragging.
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DragFactor(f32);

/// Allows this sprite to serve as a drag hitbox
/// for its parent.
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct DragParent;

impl DragFactor {
    pub fn get(&self) -> f32 {
        self.0
    }

    fn set(&mut self, value: f32) {
        self.0 = value.clamp(0.0, 1.0)
    }
}

#[allow(clippy::type_complexity)]
pub fn drag_start(
    mut query: Query<(&CursorAction, &Transform2D, &mut Draggable, Option<&mut DragSnapBack>, Option<&mut Interpolate<Offset>>)>,
    mut inactive: Query<(&Transform2D, &mut Draggable, Option<&mut DragSnapBack>, Option<&mut Interpolate<Offset>>), Without<CursorAction>>,
    child: Query<(&Parent, &CursorAction), (With<DragParent>, Without<Draggable>)>,
) {
    for (action, transform, mut drag, mut snap, mut interpolate) in query.iter_mut() {
        if !action.intersects(EventFlags::Down | EventFlags::MidDown | EventFlags:: RightDown)  {
            continue;
        }
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
    for (parent, action) in child.iter() {
        if !action.intersects(EventFlags::Down | EventFlags::MidDown | EventFlags:: RightDown)  {
            continue;
        }
        let Ok((transform, mut drag, mut snap, mut interpolate)) = inactive.get_mut(parent.get()) else {continue};
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

pub fn dragging(
    state: Res<CursorState>,
    mut query: Query<(&CursorFocus, &Draggable, &mut Transform2D, Option<&mut Interpolate<Offset>>)>,
    mut inactive: Query<(&Draggable, &mut Transform2D, Option<&mut Interpolate<Offset>>), Without<CursorFocus>>,
    child: Query<(&Parent, &CursorFocus), (With<DragParent>, Without<Draggable>)>,
) {
    let delta = state.cursor_position() - state.down_position();
    for (focus, drag, mut transform, interpolate) in query.iter_mut() {
        if !focus.intersects(EventFlags::Drag | EventFlags::MidDrag | EventFlags:: RightDrag) {
            continue;
        }
        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0}, 
                if drag.y {delta.y} else {0.0}, 
            )
        };
        transform.offset.edit_raw(|x| *x = pos);
        if let Some(mut interpolate) = interpolate {
            interpolate.set(pos)
        }
    }

    for (parent, focus) in child.iter() {
        if !focus.intersects(EventFlags::Drag | EventFlags::MidDrag | EventFlags:: RightDrag)  {
            continue;
        }
        let Ok((drag, mut transform, interpolate)) = inactive.get_mut(parent.get()) else {continue};
        let pos = drag.last_drag_start() + {
            Vec2::new(
                if drag.x {delta.x} else {0.0}, 
                if drag.y {delta.y} else {0.0}, 
            )
        };
        transform.offset.edit_raw(|x| *x = pos);
        if let Some(mut interpolate) = interpolate {
            interpolate.set(pos)
        }
    }
}


pub fn drag_end(
    mut query: Query<(&CursorAction, &mut DragSnapBack, &mut Transform2D, Option<&mut Interpolate<Offset>>)>,
    mut inactive: Query<(&mut DragSnapBack, &mut Transform2D, Option<&mut Interpolate<Offset>>), Without<CursorAction>>,
    child: Query<(&Parent, &CursorAction), (With<DragParent>, Without<Draggable>)>,
) {
    for (action, mut snap, mut transform, mut interpolate) in query.iter_mut() {
        if !action.is(EventFlags::DragEnd) {
            continue;
        }
        if let Some(orig) = snap.drag_start.take() {
            if let Some(inter) = &mut interpolate {
                inter.interpolate_to(orig)
            } else {
                transform.offset.edit_raw(|x| *x = orig)
            }
        }
    }
    for (parent, action) in child.iter() {
        if !action.is(EventFlags::DragEnd) {
            continue;
        }
        let Ok((mut snap, mut transform, mut interpolate)) = inactive.get_mut(parent.get()) else {continue};
        if let Some(orig) = snap.drag_start.take() {
            if let Some(inter) = &mut interpolate {
                inter.interpolate_to(orig)
            } else {
                transform.offset.edit_raw(|x| *x = orig)
            }
        }
    }
}

pub fn apply_constraints(
    mut query: Query<(&Constraint, &mut Transform2D, Option<&mut DragFactor>)>
) {
    for (constraint, mut transform, mut factor) in query.iter_mut() {
        if constraint.min.units() == constraint.max.units() || constraint.max.units() == transform.offset.units() {
            let start = constraint.min.raw();
            let end = constraint.max.raw();
            let min = Vec2::min(start, end);
            let max = Vec2::max(start, end);
            transform.offset.edit_raw(|x| *x = x.clamp(min, max));
            if let Some(factor) = &mut factor {
                let curr = (transform.offset.raw() - start).length();
                let max = (end - start).length();
                factor.set(curr / max);
            }
        } else {
            panic!("Units mismatch in constraints: {:?}, {:?} and {:?}", 
                constraint.min.units(), 
                constraint.max.units(), 
                transform.offset.units()
            )
        }
    }
}