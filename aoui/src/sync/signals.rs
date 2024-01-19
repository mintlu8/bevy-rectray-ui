use std::{any::{Any, TypeId}, marker::PhantomData, sync::Arc};
use bevy::{utils::HashMap, ecs::{component::Component, entity::Entity, query::WorldQuery}};
use futures::Future;
use once_cell::sync::Lazy;
use crate::util::{Object, AsObject, ComponentCompose};
use super::{AsyncExecutor, AsyncSystemParam, Signal, SignalData, SignalInner, StateId, States, YieldNow};

pub trait SignalId: Any{
    type Data: AsObject;
}

#[derive(Debug, Clone)]
pub struct TypedSignal<T: AsObject> {
    inner: Arc<SignalData<Object>>,
    p: PhantomData<T>,
}

impl<T: AsObject> Default for TypedSignal<T> {
    fn default() -> Self {
        Self { inner: Default::default(), p: PhantomData }
    }
}

impl<T: AsObject> TypedSignal<T> {
    pub fn from_inner(inner: Arc<SignalData<Object>>) -> Self {
        Self {
            inner,
            p: PhantomData
        }
    }
    pub fn into_inner(self) -> Arc<SignalData<Object>> {
        self.inner
    }

    pub fn type_erase(self) -> TypedSignal<Object> {
        TypedSignal { 
            inner: self.inner, 
            p: PhantomData 
        }
    }

}

pub static DUMMY_SIGNALS: Lazy<Signals> = Lazy::new(||Signals::new());

#[derive(Debug, Component)]
pub struct Signals {
    pub senders: HashMap<TypeId, Signal<Object>>,
    pub receivers: HashMap<TypeId, Signal<Object>>,
}

impl ComponentCompose for Signals {
    fn compose(&mut self, other: Self) {
        self.senders.extend(other.senders);
        self.receivers.extend(other.receivers);
    }
}

impl Signals {
    pub fn new() -> Self {
        Self { senders: HashMap::new(), receivers: HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.senders.is_empty() && self.receivers.is_empty()
    }

    pub fn from_sender<T: SignalId>(signal: TypedSignal<T::Data>) -> Self {
        let mut this = Self::new();
        this.add_sender::<T>(signal);
        this
    }

    pub fn from_receiver<T: SignalId>(signal: TypedSignal<T::Data>) -> Self {
        let mut this = Self::new();
        this.add_receiver::<T>(signal);
        this
    }

    pub fn with_sender<T: SignalId>(mut self, signal: TypedSignal<T::Data>) -> Self {
        self.add_sender::<T>(signal);
        self
    }

    pub fn with_receiver<T: SignalId>(mut self, signal: TypedSignal<T::Data>) -> Self {
        self.add_receiver::<T>(signal);
        self
    }

    pub fn send<T: SignalId>(&self, item: T::Data) {
        match self.senders.get(&TypeId::of::<T>()){
            Some(x) => x.write(Object::new(item)),
            None => (),
        }
    }

    pub fn poll_once<T: SignalId>(&mut self) -> Option<T::Data>{
        match self.receivers.get_mut(&TypeId::of::<T>()){
            Some(sig) => sig.try_read().and_then(|x| x.get()),
            None => None,
        }
    }
    pub fn sender<T: SignalId>(&self) -> Option<Arc<SignalInner<Object>>> {
        self.senders.get(&TypeId::of::<T>()).map(|x| x.borrow_inner())
    }
    pub fn receiver<T: SignalId>(&self) ->  Option<Arc<SignalInner<Object>>> {
        self.receivers.get(&TypeId::of::<T>()).map(|x| x.borrow_inner())
    }
    pub fn add_sender<T: SignalId>(&mut self, signal: TypedSignal<T::Data>) {
        self.senders.insert(TypeId::of::<T>(), Signal::from_typed(signal));
    }
    pub fn add_receiver<T: SignalId>(&mut self, signal: TypedSignal<T::Data>) {
        self.receivers.insert(TypeId::of::<T>(), Signal::from_typed(signal));
    }
}

pub struct SigSend<T: SignalId>(Arc<SignalInner<Object>>, PhantomData<T>);

impl<T: SignalId> SigSend<T> {
    /// Send a value with a signal.
    pub fn send(self, item: T::Data) -> impl Fn() + Send + Sync + 'static  {
        let obj = Object::new(item);
        move ||self.0.write(obj.clone())
    }

    /// Send a value with a signal.
    /// 
    /// Unlike `send` this is guaranteed to not be polled by the same sender.
    pub fn broadcast(self, item: T::Data) -> impl Fn() + Send + Sync + 'static  {
        let obj = Object::new(item);
        move ||self.0.broadcast(obj.clone())
    }

    /// Receive a value from the sender, 
    /// can receive value `send` sent from this signal,
    /// but not from `broadcast`.
    pub fn recv(self) -> impl Future<Output = T::Data> + Send + Sync + 'static {
        async move {
            loop {
                let signal = self.0.clone();
                let obj = signal.async_read().await;
                if let Some(data) = obj.get() {
                    return data;
                } else {
                    YieldNow::new().await
                }
            }
        }
    }
}

impl <T: SignalId> AsyncSystemParam for SigSend<T>  {
    fn from_async_context(
            _: Entity,
            _: &Arc<AsyncExecutor>,
            signals: &Signals,
            _: &States,
        ) -> Self {
        SigSend(
            signals.sender::<T>()
                .expect(&format!("Signal sender of type <{}> missing", stringify!(T))),
            PhantomData
        )
    }
}

pub struct SigRecv<T: SignalId>(Arc<SignalInner<Object>>, PhantomData<T>);

impl<T: SignalId> SigRecv<T> {
    pub fn recv(self) -> impl Future<Output = T::Data> + Send + Sync + 'static {
        async move {
            loop {
                let signal = self.0.clone();
                let obj = signal.async_read().await;
                if let Some(data) = obj.get() {
                    return data;
                } else {
                    YieldNow::new().await
                }
            }
        }
    }
}

impl<T: SignalId<Data = Object>> SigRecv<T> {
    pub fn recv_as<A: AsObject>(self) -> impl Future<Output = A> + Send + Sync + 'static {
        async move {
            loop {
                let signal = self.0.clone();
                let obj = signal.async_read().await;
                if let Some(data) = obj.get() {
                    return data;
                } else {
                    YieldNow::new().await
                }
            }
        }
    }
}


impl <T: SignalId> AsyncSystemParam for SigRecv<T>  {
    fn from_async_context(
            _: Entity,
            _: &Arc<AsyncExecutor>,
            signals: &Signals,
            _: &States,
        ) -> Self {
        SigRecv(
            signals.receiver::<T>()
                .expect(&format!("Signal receiver of type <{}> missing", stringify!(T))),
            PhantomData
        )
    }
}

#[derive(Debug, WorldQuery)]
pub struct SignalSender<T: SignalId>{
    signals: Option<&'static Signals>,
    p: PhantomData<T>,
}

impl<T: SignalId> SignalSenderItem<'_, T> {
    pub fn send(&self, item: T::Data) {
        if let Some(signals) = self.signals {
            signals.send::<T>(item);
        }
    }
}

#[derive(Debug, WorldQuery)]
pub struct SignalState<T: SignalId + StateId>{
    signals: Option<&'static Signals>,
    states: Option<&'static States>,
    p: PhantomData<T>,
}

impl<T: SignalId + StateId<Data = <T as SignalId>::Data>> SignalStateItem<'_, T> {
    pub fn exists(&self) -> bool {
        self.signals.is_some() || self.states.is_some()
    }

    pub fn send(&self, item: <T as SignalId>::Data) {
        if let Some(signals) = self.signals {
            signals.send::<T>(item.clone());
        }
        if let Some(states) = self.states {
            states.set::<T>(item);
        }
    }
}

#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct SignalReceiver<T: SignalId>{
    signals: Option<&'static mut Signals>,
    p: PhantomData<T>,
}

impl<T: SignalId> SignalReceiverItem<'_, T> {
    pub fn poll_once(&mut self) -> Option<T::Data> {
        self.signals.as_mut().and_then(|sig| sig.poll_once::<T>())
    }

    pub fn poll_any(&mut self) -> bool {
        self.signals.as_mut().and_then(|sig| sig.poll_once::<T>()).is_some()
    }
}


pub enum RoleSignal<T: SignalId>{
    Sender(TypedSignal<T::Data>),
    Receiver(TypedSignal<T::Data>),
}

impl<T: SignalId> RoleSignal<T> {
    pub fn and<A: SignalId>(self, other: RoleSignal<A>) -> Signals {
        let base = match self {
            RoleSignal::Sender(s) => Signals::from_sender::<T>(s),
            RoleSignal::Receiver(r) => Signals::from_receiver::<T>(r),
        };
        base.and(other)
    }

    pub fn into_signals(self) -> Signals {
        match self {
            RoleSignal::Sender(s) => Signals::from_sender::<T>(s),
            RoleSignal::Receiver(r) => Signals::from_receiver::<T>(r),
        }
    }
}

impl Signals {
    pub fn and<A: SignalId>(self, other: RoleSignal<A>) -> Signals {
        match other {
            RoleSignal::Sender(s) => self.with_sender::<A>(s),
            RoleSignal::Receiver(r) => self.with_receiver::<A>(r),
        }
    }

    pub fn into_signals(self) -> Signals {
        self
    }
}