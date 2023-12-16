use std::sync::{OnceLock, Arc};

use bevy::ecs::{system::{SystemId, Query, Commands}, component::Component, removal_detection::RemovedComponents, query::{Without, With}};
use smallvec::SmallVec;

use crate::{events::{EventFlags, CursorAction, CursorFocus, ClickOutside, CursorClickOutside}, dsl::prelude::Sender};

use self::sealed::EventQuery;

/// Event handlers.
#[derive(Debug, Component)]
pub struct Handlers<T: EventHandling> {
    pub context: T::Context,
    pub handlers: SmallVec<[Handler;1]>,
}

#[derive(Debug)]
pub enum Handler {
    OneShotSystem(Arc<OnceLock<SystemId>>),
    Signal(Sender),
}

impl<T: EventHandling> Handlers<T> {
    pub fn oneshot(system_id: Arc<OnceLock<SystemId>>) -> Self{
        Self { context: T::new_context(), handlers: SmallVec::from_const([Handler::OneShotSystem(system_id)]) }
    }
    pub fn signal(signal: Sender) -> Self{
        Self { context: T::new_context(), handlers: SmallVec::from_const([Handler::Signal(signal)]) }
    }

    pub fn from_multi(handlers: impl IntoIterator<Item = Handler>) -> Self{
        Self { context: T::new_context(), handlers: handlers.into_iter().collect() }
    }

    pub fn handle(&self, commands: &mut Commands) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::OneShotSystem(system) => {
                    if let Some(system) = system.get() {
                        commands.run_system(system.clone())
                    }
                },
                Handler::Signal(signal) => {
                    signal.send_empty();
                },
            }
        }
    }
}

/// Trait for a handleable event.
pub trait EventHandling {
    type Context: Send + Sync + 'static;
    fn new_context() -> Self::Context;
}

/// Register a `type<T>` that can handle certain events.
pub fn event_handle<T: EventQuery + Send + Sync + 'static> (
    mut commands: Commands,
    query: Query<(&T::Component, &Handlers<T>)>,
) {
    for (action, system) in query.iter() {
        if T::validate(&system.context, action) {
            system.handle(&mut commands);
        }
    }
}

mod sealed {
    use bevy::ecs::component::Component;
    use super::{EventHandling, CursorAction, EventFlags};

    /// Check for associated event component.
    pub trait EventQuery: EventHandling {
        type Component: Component + Send + Sync;
        fn validate(ctx: &Self::Context, other: &Self::Component) -> bool;
    }

    macro_rules! impl_entity_query_for_mouse_active {
        ($($ident:ident)*) => {
            $(impl EventHandling for $crate::events::$ident {
                type Context = ();
                fn new_context() -> Self::Context {
                    ()
                }
            }
            
            impl EventQuery for $crate::events::$ident {
                type Component = CursorAction;
            
                fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                    EventFlags::$ident.contains(other.flags())
                }
            })*
        };
    }

    impl_entity_query_for_mouse_active!(
        Click Down DragEnd Drop RightClick
        RightDown MidClick MidDown DoubleClick
    );

}

impl EventHandling for ClickOutside {
    type Context = ();
    fn new_context() -> Self::Context {
        ()
    }
}

impl EventQuery for ClickOutside {
    type Component = CursorClickOutside;

    fn validate(_: &Self::Context, _: &Self::Component) -> bool {
        true
    }
}

macro_rules! impl_entity_query_for_mouse_state {
    ($($ident:ident)*) => {
        $(impl EventHandling for $crate::events::$ident {
            type Context = ();
            fn new_context() -> Self::Context {
                ()
            }
        }
        impl EventQuery for $crate::events::$ident {
            type Component = CursorFocus;
        
            fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                EventFlags::$ident.contains(other.flags())
            }
        })*
    };
}

impl_entity_query_for_mouse_state! (
    Hover Pressed MidPressed RightPressed
    Drag MidDrag RightDrag
);

/// Check if widget has lost focus (drag, hover, pressed).
#[derive(Debug)]
pub enum LoseFocus{}

impl EventHandling for LoseFocus {
    type Context = ();
    fn new_context() -> Self::Context {
        ()
    }
}

/// Check if widget has obtained focus (drag, hover, pressed).
#[derive(Debug)]
pub enum ObtainFocus{}

impl EventHandling for ObtainFocus {
    type Context = bool;
    fn new_context() -> Self::Context {
        false
    }
}

pub fn obtain_focus_detection(
    mut commands: Commands,
    mut focused: Query<&mut Handlers<ObtainFocus>, With<CursorFocus>>,
    mut unfocused: Query<&mut Handlers<ObtainFocus>, Without<CursorFocus>>,
) {
    for mut handlers in focused.iter_mut() {
        if handlers.context == true { continue; }
        handlers.context = true;
        handlers.handle(&mut commands);
    }
    for mut handlers in unfocused.iter_mut() {
        handlers.context = false;
    }
}

pub fn lose_focus_detection(
    mut commands: Commands,
    mut removed: RemovedComponents<CursorFocus>,
    actions: Query<&Handlers<LoseFocus>, Without<CursorFocus>>,
) {
    for handlers in actions.iter_many(removed.read()) {
        handlers.handle(&mut commands);
    }
}