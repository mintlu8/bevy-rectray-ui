use std::{cmp::Ordering, marker::PhantomData, ops::BitAnd};

use bevy::prelude::*;

use crate::{AoUI, RotatedRect, Hitbox};

/// A system builder that propagates events.
/// 
/// Given a user generated mouse event, 
/// we find a target entity with hitbox in range and 
/// highest aggregate Z.
/// 
/// You need to manually put `EventPipe::system_both` or 
/// `EventPipe::system_ok_only` into your event pipeline
#[derive(Debug)]
pub struct EventPipe<T: CursorEvent>(PhantomData<T>);

/// A cursor event used by our event pump.
pub trait CursorEvent: Event {
    /// Type of FLAG, default should be `u32`
    type FlagTy: BitAnd<Self::FlagTy, Output = Self::FlagTy> + Copy + Default + Eq + Send + Sync;
    /// Determines which hitboxes receive this event based on a bitflag.
    const FLAG: Self::FlagTy;
    /// Event output if an entity receives the event.
    type WithEntity;
    /// Event output if no entity receives the event.
    type WithoutEntity;
    /// Position of the cursor for detection.
    fn position(&self) -> Vec2;
    /// Append the discovered entity to the event.
    fn with_entity(&self, entity: Entity) -> Self::WithEntity;
    /// If no entities found, return another event.
    fn without_entity(&self) -> Self::WithoutEntity;
}

impl<T: CursorEvent> EventPipe<T> {
    /// A system that converts a cursor input event into different events depand on
    /// if an entity receives it or not.
    pub fn system_both(
        mut reader: EventReader<T>,
        mut writer_with: EventWriter<T::WithEntity>,
        mut writer_without: EventWriter<T::WithoutEntity>,
        entity_query: Query<(Entity, &RotatedRect, &Hitbox<T::FlagTy>), With<AoUI>>,
    ) where T::WithEntity: Event, T::WithoutEntity: Event {

        for event in reader.iter() {
            let point = event.position();
            let max = entity_query
                        .iter()
                        .filter_map(|(entity, rect, hitbox)|{
                    if T::FLAG == Default::default() || (hitbox.flags & T::FLAG) != Default::default() {
                        if hitbox.contains(rect, point) {
                            Some((entity.clone(), rect.z))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }).max_by(|a, b|a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                .map(|x| x.0);
            match max {
                Some(entity) => writer_with.send(event.with_entity(entity)),
                None => writer_without.send(event.without_entity()),
            }
        }
    }

    /// A system that converts a cursor input event
    /// if an entity receives the event.
    pub fn system_ok_only(
        mut reader: EventReader<T>,
        mut writer_with: EventWriter<T::WithEntity>,
        entity_query: Query<(Entity, &RotatedRect, &Hitbox<T::FlagTy>), With<AoUI>>,
    ) where T::WithEntity: Event {

        for event in reader.iter() {
            let point = event.position();
            let max = entity_query
                        .iter()
                        .filter_map(|(entity, rect, hitbox)|{
                    if T::FLAG == Default::default() || (hitbox.flags & T::FLAG) != Default::default() {
                        if hitbox.contains(rect, point) {
                            Some((entity.clone(), rect.z))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }).max_by(|a, b|a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
                .map(|x| x.0);
            match max {
                Some(entity) => writer_with.send(event.with_entity(entity)),
                None => (),
            }
        }
    }
}
