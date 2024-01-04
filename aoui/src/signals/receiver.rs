use std::{marker::PhantomData, fmt::Debug};

use bevy::ecs::{system::{EntityCommands, Query, Commands}, component::Component, entity::Entity};

use crate::{events::mutation::IntoMutationCommand, dsl::DslFrom};

use super::{Object, SignalBuilder, dto::AsObject, sig::Signal};

#[derive(Component)]
pub struct SignalReceiver<const SIGNAL_ID: u8> {
    signal: Signal,
    func: Box<dyn Fn(&mut EntityCommands, Object) + Send + Sync + 'static>,
}

impl<const S: u8> Debug for SignalReceiver<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalReceiver").finish()
    }
}

impl<const A: u8> SignalReceiver<A> {
    pub fn with_slot<const S: u8>(self) -> SignalReceiver<S> {
        SignalReceiver {
            signal: self.signal,
            func: self.func,
        }
    }
}

impl<T: AsObject> SignalBuilder<T> {

    /// A [`Mutation`](crate::events::Mutation) that gets run on receiving an event.
    /// This requires the correct type for the `Mutation` to be called. 
    /// 
    /// If receiving type erased input, see
    /// [`recv_filter`](Self::recv_filter) or [`recv_select`](Self::recv_select).
    pub fn recv<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<0> {
        SignalReceiver{
            signal: self.signal,
            func: Box::new(move |commands: &mut EntityCommands, obj| {
                let entity = commands.id();
                if let Some(item) = obj.get() {
                    commands.commands().add(f.clone().into_command(entity, item));
                }
            }),
        }
    }

    /// Run `then` if typing is correct, run `or_else` if type mismatch.
    /// Does not run if value is `None`.
    pub fn recv_filter<A, B, C, D>(self, then: impl IntoMutationCommand<T, A, B>, or_else: impl IntoMutationCommand<(), C, D>) -> SignalReceiver<0>{
        SignalReceiver{
            signal: self.signal,
            func: Box::new(move |commands: &mut EntityCommands, obj: Object| {
                let entity = commands.id();
                if let Some(item) = obj.get() {
                    commands.commands().add(then.clone().into_command(entity, item));
                } else if obj.is_some() {
                    commands.commands().add(or_else.clone().into_command(entity, ()));
                }
            })
        }
    }

    /// Run `then` if value equals `if_eq`, run `or_else` otherwise.
    /// Does not run if value is `None`.
    pub fn recv_select<A, B, C, D>(self, if_eq: impl AsObject, then: impl IntoMutationCommand<(), A, B>, or_else: impl IntoMutationCommand<(), C, D>) -> SignalReceiver<0>{
        let if_eq = if_eq.into_object();
        SignalReceiver{
            signal: self.signal,
            func: Box::new(move |commands: &mut EntityCommands, obj: Object| {
                let entity = commands.id();
                if obj.equal_to(&if_eq) {
                    commands.commands().add(then.clone().into_command(entity, ()));
                } else {
                    commands.commands().add(or_else.clone().into_command(entity, ()));
                }
            })
        }
    }

    /// Receives a signal at the `0` slot.
    pub fn recv0<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<0> {
        self.recv(f).with_slot()
    }

    /// Receives a signal at the `1` slot.
    pub fn recv1<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<1> {
        self.recv(f).with_slot()
    }

    /// Receives a signal at the `2` slot.
    pub fn recv2<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<2> {
        self.recv(f).with_slot()
    }

    /// Receives a signal at the `3` slot.
    pub fn recv3<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<3> {
        self.recv(f).with_slot()
    }

    /// Receives a signal at the `4` slot.
    pub fn recv4<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<4> {
        self.recv(f).with_slot()
    }

    /// Receives a signal at the `5` slot.
    pub fn recv5<A, B>(self, f: impl IntoMutationCommand<T, A, B>) -> SignalReceiver<5> {
        self.recv(f).with_slot()
    }

    pub fn invoke<A: ReceiveInvoke>(self) -> Invoke<A> {
        Invoke { signal: self.signal, p: PhantomData }
    }
}


pub fn signal_receive<const S: u8>(
    mut commands: Commands,
    query: Query<(Entity, &SignalReceiver<S>)>) {
    query.iter().for_each(|(entity, recv)| {
        if let Some(obj) = recv.signal.read::<Object>(){
            let mut commands = commands.entity(entity);
            (recv.func)(&mut commands, obj)
        }
    })
}

#[derive(Component)]
pub struct Invoke<T: ReceiveInvoke>{
    signal: Signal,
    p: PhantomData<T>
}

impl<T: ReceiveInvoke> Debug for Invoke<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Invoke").finish()
    }
}

impl<T: ReceiveInvoke> Default for Invoke<T> {
    fn default() -> Self {
        Invoke { signal: Signal::new(), p: PhantomData }
    }
}

pub trait ReceiveInvoke {
    type Type: AsObject;
}

impl<T: ReceiveInvoke> Invoke<T> {
    pub fn poll_any(&self) -> bool {
        self.signal.read_any()
    }
    pub fn poll(&self) -> Option<T::Type> {
        self.signal.read()
    }
}

impl<T: ReceiveInvoke> DslFrom<SignalBuilder<T::Type>> for Invoke<T> {
    fn dfrom(value: SignalBuilder<T::Type>) -> Self {
        value.invoke()
    }
}

impl<T: ReceiveInvoke> DslFrom<SignalBuilder<T::Type>> for Option<Invoke<T>> {
    fn dfrom(value: SignalBuilder<T::Type>) -> Self {
        Some(value.invoke())
    }
}