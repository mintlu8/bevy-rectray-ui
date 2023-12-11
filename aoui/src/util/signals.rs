use std::{sync::{Arc, RwLock}, marker::PhantomData, fmt::Debug};
use bevy::ecs::{system::Query, component::Component};

use super::{dto::Object, DataTransfer};

use self::sealed::SignalCreate;

/// A signal sender
#[derive(Component)]
pub struct Sender<T=()> {
    signal: Signal,
    map: Option<Box<dyn Fn(&mut Object) + Send + Sync + 'static>>,
    p: PhantomData<T>,
}

impl<T> Debug for Sender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

/// A signal receiver
#[derive(Component)]
pub struct Receiver<T=()>{
    signal: Signal,
    map: Option<Box<dyn Fn(&mut Object) + Send + Sync + 'static>>,
    p: PhantomData<T>,
}

impl<T> Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

#[derive(Debug, Clone)]
struct Signal(pub(crate) Arc<RwLock<Object>>);

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

impl Sender {
    pub fn mark<M>(self) -> Sender<M> {
        Sender { signal: self.signal, map: self.map, p: PhantomData  }
    }
}

impl<M> Sender<M> {
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

    /// Sends `()`
    pub(crate) fn send_empty(&self) {
        let mut lock = self.signal.0.write().unwrap();
        *lock = Object::unit();
    }
}

impl Receiver {
    pub fn mark<M>(self) -> Receiver<M> {
        Receiver { signal: self.signal, map: self.map, p: PhantomData }
    }
}


impl<M> Receiver<M> {
    pub fn map<D, S>(self, f: impl Fn(D) -> S + Send + Sync + 'static) -> Self
        where M: Send + Sync + 'static, D: DataTransfer + Clone, S: DataTransfer + Clone{
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
    pub fn poll<T: DataTransfer>(&self) -> Option<T> {
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

    /// Receives a `()`, usually sent by buttons without payloads.
    pub fn poll_empty(&self) -> bool {
        let read = self.signal.0.read().unwrap();
        read.get::<()>().is_some()
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

mod sealed {
    use std::marker::PhantomData;

    use super::{Sender, Receiver, Signal};

    pub trait SignalCreate {
        fn new() -> Self;
    }

    macro_rules! signal_create {
        ($sender: ident, $first: ident) => {
            impl SignalCreate for ($sender, $first) {
                fn new() -> Self {
                    let signal = Signal::new();
                    (
                        $sender{
                            signal: signal.clone(), 
                            map: None,
                            p: PhantomData
                        },
                        $first{
                            signal: signal, 
                            map: None,
                            p: PhantomData
                        }, 
                    )
                }
            }
        };
        ($sender: ident, $first: ident, $($receivers: ident),*) => {
            impl
                SignalCreate for ($sender, $($receivers),* , $first) {
                fn new() -> Self {
                    let signal = Signal::new();
                    (
                        $sender{
                            signal: signal.clone(), 
                            map: None,
                            p: PhantomData
                        }, 
                        $($receivers{
                            signal: signal.clone(), 
                            map: None,
                            p: PhantomData
                        },)*
                        $first{
                            signal: signal, 
                            map: None,
                            p: PhantomData
                        },
                    )
                }
            }

            signal_create!($sender, $($receivers),*);
        };
    }

    signal_create!(Sender, 
        Receiver, Receiver, Receiver, Receiver,
        Receiver, Receiver, Receiver, Receiver,
        Receiver, Receiver, Receiver, Receiver
    );   
}

/// Create a spmc signal that can be polled.
/// 
/// ```
/// # /*
/// let (sender, recv_a, recv_b, ...) = signal();
/// # */
/// ```
/// 
/// To have multiple senders or receiver on the same entity,
/// mark them.
/// 
/// ```
/// # /*
/// let sender = sender.mark::<ButtonClick>()
/// # */
/// ```
/// 
/// If registered, this signal is cleared at the end of the frame.
/// 
/// ```
/// # /*
/// app.register_aoui_signal::<ButtonClick>()
/// # */
/// ```
pub fn signal<S: SignalCreate>() -> S {
    S::new()
}

pub fn signal_cleanup<T: Send + Sync + 'static>(mut query: Query<&Sender<T>>) {
    query.par_iter_mut().for_each(|x| x.signal.clean())
}
