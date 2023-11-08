use std::{cmp::Ordering, marker::PhantomData};

use bevy::{prelude::*, ecs::{system::SystemId, schedule::ScheduleLabel}};

use crate::{RotatedRect, Hitbox};

#[derive(Debug, Clone, Component)]
pub struct CursorEventHandler<T: CursorEvent>{ 
    system: SystemId,
    p: PhantomData<T>
}

/// A event that contains positions and support click detection.
pub trait CursorEvent: Event + Sized{
    /// Output of positions.
    type Points: IntoIterator<Item = Vec2> + Copy;
    /// Positions for detection.
    fn positions(&self) -> Self::Points;

    fn run_at<L: ScheduleLabel + Clone>(label: L) -> CursorEventPlugin<Self, L> {
        CursorEventPlugin { 
            label, 
            p: PhantomData 
        }
    }
}

/// Plugin for registering a AoUI cursor event.
/// 
/// Created using [`CursorEvent::run_at`]
pub struct CursorEventPlugin<T, L>{
    label: L,
    p: PhantomData<T>
}

impl<T: CursorEvent, L: ScheduleLabel + Clone> Plugin for CursorEventPlugin<T, L> {
    fn build(&self, app: &mut App) {
        app.add_event::<T>()
            .add_systems(self.label.clone(), query_hitbox_event::<T>);
    }
}

/// Find hitboxes that containing some points
pub fn query_hitbox<'t>(entity_query: impl IntoIterator<Item = (Entity, &'t RotatedRect, &'t Hitbox)>, points: impl IntoIterator<Item = Vec2> + Copy) -> Option<Entity> {    entity_query
        .into_iter()
        .filter_map(|(entity, rect, hitbox)|{
            if points.into_iter().all(|pt| hitbox.contains(rect, pt)) {
                Some((entity.clone(), rect.z))
            } else {
                None
            }
    }).max_by(|a, b|a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
    .map(|x| x.0)
}

/// Find hitboxes that containing some points
fn query_hitbox_event<T: CursorEvent>(
    mut commands: Commands,
    entity_query: Query<(&RotatedRect, &Hitbox, &CursorEventHandler<T>)>, 
    mut event: EventReader<T>,
) {
    for event in event.read() {
        let handler = entity_query
            .iter()
            .filter_map(|(rect, hitbox, handler)|{
                if event.positions().into_iter().all(|pt| hitbox.contains(rect, pt)) {
                    Some((handler, rect.z))
                } else {
                    None
                }
            }).max_by(|a, b|a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        if let Some((handler, _)) = handler {
            commands.run_system(handler.system)
        }
    }
    
}