use std::sync::OnceLock;

use bevy::ecs::{system::{SystemId, Query, Commands}, component::Component, removal_detection::RemovedComponents, query::Without};

use crate::{events::{EventFlags, CursorAction, CursorFocus, ClickOutside, CursorClickOutside}, dto::Submit};

/// Event handler though a oneshot system.
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

/// Register a `type<T>` that can handle certain events.
pub fn call_oneshot<T: EventQuery + Send + Sync + 'static> (
    mut commands: Commands,
    query: Query<(&T::Component, &OneShot<T>)>,
) {
    for (action, system) in query.iter() {
        if system.event.validate(action) {
            if let Some(system) = system.get() {
                commands.run_system(system)
            }
        }
    }
}

/// Check for associated event component.
pub trait EventQuery {
    type Component: Component + Send + Sync;
    fn validate(&self, other: &Self::Component) -> bool;
}

impl EventQuery for EventFlags {
    type Component = CursorAction;

    fn validate(&self, other: &Self::Component) -> bool {
        self.contains(other.flags())
    }
}

macro_rules! impl_entity_query_for_mouse_active {
    ($($ident:ident)*) => {
        $(impl EventQuery for $crate::events::$ident {
            type Component = CursorAction;
        
            fn validate(&self, other: &Self::Component) -> bool {
                EventFlags::$ident.contains(other.flags())
            }
        })*
    };
}

impl_entity_query_for_mouse_active!(
    Click Down DragEnd Drop RightClick
    RightDown MidClick MidDown DoubleClick
);

impl EventQuery for ClickOutside {
    type Component = CursorClickOutside;

    fn validate(&self, _: &Self::Component) -> bool {
        true
    }
}

macro_rules! impl_entity_query_for_mouse_state {
    ($($ident:ident)*) => {
        $(impl EventQuery for $crate::events::$ident {
            type Component = CursorFocus;
        
            fn validate(&self, other: &Self::Component) -> bool {
                EventFlags::$ident.contains(other.flags())
            }
        })*
    };
}

impl_entity_query_for_mouse_state! (
    Hover Pressed MidPressed RightPressed
    Drag MidDrag RightDrag
);

/// One-shot system for checking the sumbit event.
pub struct OnSubmit;

impl EventQuery for OnSubmit {
    type Component = Submit;

    fn validate(&self, _: &Self::Component) -> bool {
        true
    }
}

/// Check if widget has lost focus (drag, hover, pressed).
pub struct LoseFocus;

pub fn lose_focus_detection(
    mut commands: Commands,
    mut removed: RemovedComponents<CursorFocus>,
    actions: Query<&OneShot<LoseFocus>, Without<CursorFocus>>,
) {
    for action in actions.iter_many(removed.read()) {
        if let Some(system) = action.cell.get() {
            commands.run_system(*system)
        }
    }
}