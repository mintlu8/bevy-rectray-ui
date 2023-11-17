use std::{sync::OnceLock, borrow::Cow};

use bevy::{input::keyboard::KeyCode, ecs::{system::{SystemId, Query, Commands}, component::Component}};

use crate::events::{EventFlags, CursorAction};

pub trait EventContains {
    type Inst;
    fn contains(&self, other: &Self::Inst) -> bool;
}

impl EventContains for EventFlags {
    type Inst = Self;

    fn contains(&self, other: &Self::Inst) -> bool {
        self.contains(*other)
    }
}


#[derive(Component)]
pub struct OneShot<T> {
    event: T,
    cell: &'static OnceLock<SystemId>,
}

impl<T> OneShot<T> {
    pub fn new(event: T, cell: &'static OnceLock<SystemId>) -> Self{
        Self { event, cell }
    }
    pub fn get(&self) -> Option<SystemId>{
        self.cell.get().copied()
    }
}

pub fn call_oneshot_mouse(
    mut commands: Commands,
    query: Query<(&CursorAction, &OneShot<EventFlags>)>,
) {
    for (action, system) in query.iter() {
        if system.event.contains(action.flags()) {
            if let Some(system) = system.get() {
                commands.run_system(system)
            }
        }
    }
}