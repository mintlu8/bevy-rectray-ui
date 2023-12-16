use std::{sync::{Arc, RwLock}, marker::PhantomData, fmt::Debug};
use bevy::ecs::{system::Query, component::Component};

use super::{dto::Object, DataTransfer};

/// Provides some checking against our chaotic namespace.
pub trait SignalSender: Send + Sync + 'static {}

pub trait SignalReceiver: Send + Sync + 'static {
    type Type: DataTransfer;
}

impl SignalSender for () {}
impl SignalReceiver for () {
    type Type = ();
}

/// A signal sender
#[derive(Component)]
pub struct Sender<T: SignalSender=()> {
    pub(super) signal: Signal,
    pub(super) map: Option<Box<dyn Fn(&mut Object) + Send + Sync + 'static>>,
    pub(super) p: PhantomData<T>,
}

impl<T: SignalSender> Debug for Sender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

/// A signal receiver
#[derive(Component)]
pub struct Receiver<T: SignalReceiver=()>{
    pub(super) signal: Signal,
    pub(super) map: Option<Box<dyn Fn(&mut Object) + Send + Sync + 'static>>,
    pub(super) p: PhantomData<T>,
}

impl<T: SignalReceiver> Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

#[derive(Debug, Clone)]
pub(super) struct Signal(pub(crate) Arc<RwLock<Object>>);

impl Signal {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Object::NONE)))
    }
    pub fn is_empty(&self) -> bool {
        self.0.read().unwrap().is_none()
    }

    pub fn clean(&self)  {
        self.0.write().unwrap().clean();
    }
}

impl<M: SignalSender> Sender<M> {
    pub fn mark<A: SignalSender>(self) -> Sender<A> {
        Sender { signal: self.signal, map: self.map, p: PhantomData  }
    }

    pub fn map<D, S>(self, f: impl Fn(D) -> S + Send + Sync + 'static) -> Self
        where M: Send + Sync+ 'static, D: DataTransfer, S: DataTransfer {
        Sender { 
            signal: self.signal,
            map: Some(Box::new(move |obj: &mut Object| {
                match obj.get::<D>() {
                    Some(o) => *obj = Object::new(f(o)),
                    None => (),
                }
            })),
            p: PhantomData
        }
    }

    pub fn send<T: DataTransfer>(&self, item: T) {
        let mut lock = self.signal.0.write().unwrap();
        lock.set(item);
    }

    pub fn send_object(&self, item: Object) {
        let mut lock = self.signal.0.write().unwrap();
        *lock = item;
    }

    /// Clone a signal without the its mapping function.
    pub fn fork(&self) -> Self {
        Self { 
            signal: self.signal.clone(), 
            map: None, 
            p: PhantomData 
        }
    }

    /// Gets a receiver of the underlying signal.
    pub fn get_receiver(&self) -> Receiver {
        Receiver { 
            signal: self.signal.clone(), 
            map: None, 
            p: PhantomData 
        }
    }

    /// Sends `()`
    pub(crate) fn send_empty(&self) {
        let mut lock = self.signal.0.write().unwrap();
        *lock = Object::unit();
    }
}

impl<M: SignalReceiver> Receiver<M> {
    pub fn mark<A: SignalReceiver>(self) -> Receiver<A> {
        Receiver { signal: self.signal, map: self.map, p: PhantomData }
    }
    
    pub fn map<D>(self, f: impl Fn(D) -> M::Type + Send + Sync + 'static) -> Self
        where M: Send + Sync + 'static, D: DataTransfer + Clone {
        Receiver { 
            signal: self.signal,
            map: Some(Box::new(move |obj: &mut Object| {
                match obj.get::<D>() {
                    Some(o) => *obj = Object::new(f(o)),
                    None => (),
                }
            })),
            p: PhantomData
        }
    }
    
    /// Receives data from a signal.
    pub fn poll(&self) -> Option<M::Type> {
        let read = self.signal.0.read().unwrap();
        match &self.map {
            Some(f) => {
                let mut obj = read.clone();
                f(&mut obj);
                obj.get()
            },
            None => read.get(),
        }
    }

    /// Receives anything regardless of type.
    pub fn poll_any(&self) -> bool {
        let read = self.signal.0.read().unwrap();
        read.is_some()
    }

    /// Clone, expect removes the mapping function.
    pub fn fork(&self) -> Self {
        Self { 
            signal: self.signal.clone(), 
            map: None, 
            p: PhantomData 
        }
    }
}

pub fn signal_cleanup<M: SignalSender>(mut query: Query<&Sender<M>>) {
    query.par_iter_mut().for_each(|x| x.signal.clean())
}

