use std::fmt::Debug;

use bevy::ecs::{component::Component, removal_detection::RemovedComponents};
use bevy::ecs::query::{Without, With};
use bevy::ecs::system::{Query, Commands, EntityCommands};
use smallvec::SmallVec;

use crate::dsl::{DslFrom, DslInto};
use crate::events::*;
use crate::signals::{SignalSender, SignalMapper, SignalBuilder, KeyStorage, Object, AsObject};
use crate::widgets::drag::DragState;

use self::sealed::EventQuery;

use super::mutation::{Mutation, IntoMutationCommand};
use super::oneshot::OneShot;
use super::{EvLoseFocus, EvObtainFocus, EvButtonClick, EvTextSubmit, EvTextChange, EvToggleChange, EvMouseDrag, EvPositionFactor};

/// Event handlers.
#[derive(Debug, Component)]
pub struct Handlers<T: EventHandling> {
    pub context: T::Context,
    pub handlers: SmallVec<[Handler<T>;1]>,
}

impl<T: EventHandling> Default for Handlers<T> {
    fn default() -> Self {
        Self { 
            context: Default::default(), 
            handlers: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Handler<T: EventHandling> {
    /// Run a oneshot system, currently characterized by being agnostic to the caller.
    OneShotSystem(OneShot),
    /// Mutate components associated with this entity.
    Mutation(Mutation<T::Data>),
    /// Send a signal with the associated data.
    Signal(SignalSender<T::Data>),
    /// Set a key-value pair in a [storage](crate::signals::KeyStorage).
    GlobalKey(String, SignalMapper),
}

impl<T: EventHandling> DslFrom<OneShot> for Handler<T> {
    fn dfrom(value: OneShot) -> Self {
        Handler::OneShotSystem(value)
    }
}

impl<T: EventHandling> DslFrom<SignalBuilder<T::Data>> for Handler<T> {
    fn dfrom(value: SignalBuilder<T::Data>) -> Self {
        Handler::Signal(value.send())
    }
}

impl<T: EventHandling> DslFrom<SignalSender<T::Data>> for Handler<T> {
    fn dfrom(value: SignalSender<T::Data>) -> Self {
        Handler::Signal(value)
    }
}

impl<T: EventHandling> DslFrom<String> for Handler<T> {
    fn dfrom(value: String) -> Self {
        Handler::GlobalKey(value, SignalMapper::None)
    }
}

impl<T: EventHandling> DslFrom<&str> for Handler<T> {
    fn dfrom(value: &str) -> Self {
        Handler::GlobalKey(value.to_owned(), SignalMapper::None)
    }
}

impl<T: EventHandling> DslFrom<Mutation<T::Data>> for Handler<T> {
    fn dfrom(value: Mutation<T::Data>) -> Self {
        Handler::Mutation(value)
    }
}

impl<T: EventHandling> DslFrom<OneShot> for Handlers<T> {
    fn dfrom(value: OneShot) -> Self {
        Handlers::new(value)
    }
}

impl<T: EventHandling> DslFrom<SignalBuilder<T::Data>> for Handlers<T> {
    fn dfrom(value: SignalBuilder<T::Data>) -> Self {
        Handlers::new(value.send())
    }
}

impl<T: EventHandling> DslFrom<SignalSender<T::Data>> for Handlers<T> {
    fn dfrom(value: SignalSender<T::Data>) -> Self {
        Handlers::new(value)
    }
}

impl<T: EventHandling> DslFrom<Mutation<T::Data>> for Handlers<T> {
    fn dfrom(value: Mutation<T::Data>) -> Self {
        Handlers::new(value)
    }
}

impl<T: EventHandling> Handlers<T> {

    pub fn new_empty() -> Self {
        Self { context: T::new_context(), handlers: SmallVec::new_const() }
    }

    pub fn new(handler: impl DslInto<Handler<T>>) -> Self {
        Self { context: T::new_context(), handlers: SmallVec::from_const([handler.dinto()]) }
    }

    /// Chain another handler in a builder pattern.
    pub fn and(mut self, handler: impl DslInto<Handler<T>>) -> Self {
        self.handlers.push(handler.dinto());
        self
    }

    pub fn oneshot<M: Send + Sync + 'static>(
        commands: &mut Commands, 
        handler: impl IntoSystem<(), (), M> + Send + Sync + 'static
    ) -> Self {
        Self {
            context: T::new_context(), handlers: SmallVec::from_const([
                OneShot::new(commands, handler).dinto()
            ])
        }
    }

    pub fn and_oneshot<M: Send + Sync + 'static>(
        mut self,
        commands: &mut Commands, 
        handler: impl IntoSystem<(), (), M> + Send + Sync + 'static
    ) -> Self {
        self.handlers.push(OneShot::new(commands, handler).dinto());
        self
    }


    pub fn and_mutate<M, N>(
        mut self,
        handler: impl IntoMutationCommand<T::Data, M, N>
    ) -> Self {
        self.handlers.push(Mutation::new(handler).dinto());
        self
    }


    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    pub fn send_signal(&self, data: T::Data) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::Signal(signal) => {
                    signal.send(data.clone());
                },
                _ => {
                    warn!("Fetch only supports sending signals.")
                }
            }
        }
    }

    pub fn handle(&self, commands: &mut EntityCommands, keys: &KeyStorage, data: T::Data) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::OneShotSystem(system) => {
                    if let Some(system) = system.get() {
                        commands.commands().run_system(system)
                    }
                },
                Handler::Mutation(mutation) => {
                    mutation.exec(commands, data.clone());
                }
                Handler::Signal(signal) => {
                    signal.send(data.clone());
                },
                Handler::GlobalKey(name, mapper) => {
                    keys.set_dyn(name, mapper.map(Object::new(data.clone())));
                },
            }
        }
    }

    pub fn handle_dyn(&self, commands: &mut EntityCommands, keys: &KeyStorage, data: Object) {
        for handler in self.handlers.iter() {
            match handler {
                Handler::OneShotSystem(system) => {
                    if let Some(system) = system.get() {
                        commands.commands().run_system(system)
                    }
                },
                Handler::Mutation(mutation) => {
                    if let Some(data) = data.get() {
                        mutation.exec(commands, data);
                    }
                }
                Handler::Signal(signal) => {
                    signal.send_dyn(data.clone())
                },
                Handler::GlobalKey(name, mapper) => {
                    keys.set_dyn(name, mapper.map(data.clone()));
                },
            }
        }
    }

    pub fn cleanup(&self, drop_flag: u8){ 
        self.handlers.iter().for_each(|x| match x {
            Handler::OneShotSystem(_) => (),
            Handler::Mutation(_) => (),
            Handler::Signal(sig) => sig.try_cleanup(drop_flag),
            Handler::GlobalKey(_, _) => (),
        })
    }
}

/// Trait for a handleable event.
pub trait EventHandling: 'static {
    type Data: AsObject;
    type Context: Default + Send + Sync + 'static;
    fn new_context() -> Self::Context;
}

/// Register a `type<T>` that can handle certain events.
pub fn handle_event<T: EventQuery + Send + Sync + 'static> (
    mut commands: Commands,
    keys: Res<KeyStorage>,
    query: Query<(Entity, &T::Component, &Handlers<T>)>,
) {
    for (entity, action, system) in query.iter() {
        let mut commands = commands.entity(entity);
        if T::validate(&system.context, action) {
            system.handle(&mut commands, &keys, T::get_data(&system.context, action));
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
        fn get_data(ctx: &Self::Context, other: &Self::Component) -> Self::Data;
    }

    macro_rules! impl_entity_query_for_mouse_active {
        ($($ident:ident)*) => {
            $(impl EventHandling for $crate::events::sealed::$ident {
                type Data = ();
                type Context = ();
                fn new_context() -> Self::Context {}
            }
            
            impl EventQuery for $crate::events::sealed::$ident {
                type Component = CursorAction;
            
                fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                    EventFlags::$ident.contains(other.flags())
                }

                fn get_data(_: &Self::Context, _: &Self::Component) -> () {}
            })*
        };
    }

    impl_entity_query_for_mouse_active!(
        LeftClick LeftDown DragEnd Drop RightClick
        RightDown MidClick MidDown DoubleClick
    );
}

impl EventHandling for EvMouseWheel {
    type Data = MouseWheelAction;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvMouseDrag {
    type Data = DragState;
    type Context = DragState;
    fn new_context() -> Self::Context {
        DragState::Start
    }
}

impl EventHandling for EvClickOutside {
    type Data = ();
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventQuery for EvClickOutside {
    type Component = CursorClickOutside;

    fn validate(_: &Self::Context, _: &Self::Component) -> bool {
        true
    }
    fn get_data(_: &Self::Context, _: &Self::Component) -> Self::Data {}
}

macro_rules! impl_entity_query_for_mouse_state {
    ($($ident:ident)*) => {
        $(impl EventHandling for $crate::events::sealed::$ident {
            type Data = ();
            type Context = ();
            fn new_context() -> Self::Context {}
        }
        impl EventQuery for $crate::events::sealed::$ident {
            type Component = CursorFocus;
        
            fn validate(_: &Self::Context, other: &Self::Component) -> bool {
                EventFlags::$ident.contains(other.flags())
            }
            fn get_data(_: &Self::Context, _: &Self::Component) -> Self::Data {}
        })*
    };
}

impl_entity_query_for_mouse_state! (
    Hover LeftPressed MidPressed RightPressed
    LeftDrag MidDrag RightDrag
);

impl EventHandling for EvLoseFocus {
    type Data = ();
    type Context = ();
    fn new_context() -> Self::Context {}
}


impl EventHandling for EvObtainFocus {
    type Data = ();
    type Context = bool;
    fn new_context() -> Self::Context { false }
}

impl EventHandling for EvButtonClick {
    type Data = Object;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvToggleChange {
    type Data = bool;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvTextChange {
    type Data = String;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvTextSubmit {
    type Data = String;
    type Context = ();
    fn new_context() -> Self::Context {}
}

impl EventHandling for EvPositionFactor {
    type Data = f32;
    type Context = ();
    fn new_context() -> Self::Context {}
}

pub fn obtain_focus_detection(
    mut commands: Commands,
    keys: Res<KeyStorage>,
    mut focused: Query<(Entity, &mut Handlers<EvObtainFocus>), With<CursorFocus>>,
    mut unfocused: Query<&mut Handlers<EvObtainFocus>, Without<CursorFocus>>,
) {
    for (entity, mut handlers) in focused.iter_mut() {
        if handlers.context { continue; }
        handlers.context = true;
        let mut commands = commands.entity(entity);
        handlers.handle(&mut commands, &keys, ());
    }
    for mut handlers in unfocused.iter_mut() {
        handlers.context = false;
    }
}

pub fn lose_focus_detection(
    mut commands: Commands,
    keys: Res<KeyStorage>,
    mut removed: RemovedComponents<CursorFocus>,
    actions: Query<(Entity, &Handlers<EvLoseFocus>), Without<CursorFocus>>,
) {
    for (entity, handlers) in actions.iter_many(removed.read()) {
        let mut commands = commands.entity(entity);
        handlers.handle(&mut commands, &keys, ());
    }
}