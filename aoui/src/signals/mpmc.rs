use std::{marker::PhantomData, fmt::Debug};
use bevy::ecs::component::Component;

use crate::dsl::DslFrom;

use super::{dto::Object, DataTransfer, sig::Signal};

/// Marker trait for `Receiver` compatible signals.
/// 
/// This The parent type denotes behavior and the associated type denotes type.
pub trait SignalReceiver: Send + Sync + 'static {
    /// A hint for function marking.
    type Type: DataTransfer;
}

impl SignalReceiver for () {
    type Type = ();
}

#[derive(Default)]
pub enum SignalMapper {
    #[default]
    None,
    Function(Box<dyn SignalMapperFn>)
}

impl SignalMapper {
    pub fn map(&self, mut obj: Object) -> Object {
        match self {
            SignalMapper::None => obj,
            SignalMapper::Function(f) => {
                f.call(&mut obj);
                obj
            },
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! signal_mapper {
    ($in_ty: ty, $f: expr) => {
        $crate::signals::SignalMapper::Function(
            Box::new(move |obj: &mut $crate::signals::Object| {
                let Some(a) = obj.get::<$in_ty>() else {return};
                *obj = Object::new(($f)(a));
            })
        )
    };
    (|$var: ident: $ty: ty| $expr: expr) => {
        $crate::signals::SignalMapper::Function(
            Box::new(move |obj: &mut $crate::signals::Object| {
                let Some(a) = obj.get::<$in_ty>() else {return};
                *obj = Object::new((|$var: $ty| $expr)(a));
            })
        )
    };
    (|mut $var: ident: $ty: ty| $expr: expr) => {
        $crate::signals::SignalMapper::Function(
            Box::new(move |obj: &mut $crate::signals::Object| {
                let Some(a) = obj.get::<$in_ty>() else {return};
                *obj = Object::new((|mut $var: $ty| $expr)(a));
            })
        )
    };
}

impl Signal {
    pub fn get_mapped<T: DataTransfer>(&self, mapper: &SignalMapper) -> Option<T> {
        match mapper {
            SignalMapper::None => self.read(),
            SignalMapper::Function(f) => {
                let mut obj = self.read_dyn();
                f.call(&mut obj);
                obj.get()
            },
        }
    }
}

impl Debug for SignalMapper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Function(_) => write!(f, "Function"),
        }
    }
}

impl Clone for SignalMapper {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Function(arg0) => Self::Function(arg0.dyn_clone()),
        }
    }
}


pub trait SignalMapperFn: Send + Sync + 'static {
    fn call(&self, obj: &mut Object);
    fn dyn_clone(&self) -> Box<dyn SignalMapperFn>;
}

impl<F> SignalMapperFn for F where F: Fn(&mut Object) + Clone + Send + Sync + 'static{
    fn call(&self, obj: &mut Object) {
        self(obj)
    }

    fn dyn_clone(&self) -> Box<dyn SignalMapperFn> {
        Box::new(self.clone())
    }
}

/// A signal sender
#[derive(Component)]
pub struct SenderBuilder<T: DataTransfer> {
    pub(super) signal: Signal,
    p: PhantomData<T>,
}

impl<T: DataTransfer> Clone for SenderBuilder<T> {
    fn clone(&self) -> Self {
        SenderBuilder { signal: self.signal.clone(), p: PhantomData }
    }
}

impl<T: DataTransfer> SenderBuilder<T> {

    pub(super) fn new(signal: Signal) -> Self {
        Self {
            signal,
            p: PhantomData,
        }
    }

    pub fn build(self) -> Sender<T>{
        Sender { 
            signal: self.signal, 
            map: SignalMapper::None, 
            p: PhantomData,
        }
    }

    pub fn map<In: DataTransfer>(self, f: impl Fn(In) -> T + Clone + Send + Sync + 'static) -> Sender<In> {
        Sender {
            signal: self.signal,
            map: signal_mapper!(In, f),
            p: PhantomData,
        }
    }

    pub fn dynamic(self) -> DynamicSender {
        DynamicSender {
            signal: self.signal,
            map: SignalMapper::None,
        }
    }
}

/// A signal sender, unlike receiver this is not a component.
/// 
/// Use event handler instead.
#[derive(Clone)]
pub struct Sender<T: DataTransfer> {
    pub(super) signal: Signal,
    pub(super) map: SignalMapper,
    p: PhantomData<T>,
}

impl<T: DataTransfer> Debug for Sender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

/// A signal sender
#[derive(Component)]
pub struct ReceiverBuilder<T: DataTransfer> {
    pub(super) signal: Signal,
    p: PhantomData<T>,
}

impl<T: DataTransfer> Clone for ReceiverBuilder<T> {
    fn clone(&self) -> Self {
        ReceiverBuilder { signal: self.signal.clone(), p: PhantomData }
    }
}


impl<T: DataTransfer> ReceiverBuilder<T> {

    pub(super) fn new(signal: Signal) -> Self {
        Self {
            signal,
            p: PhantomData,
        }
    }

    pub fn build<Out: SignalReceiver<Type = T>>(self) -> Receiver<Out>{
        Receiver { 
            signal: self.signal, 
            map: SignalMapper::None, 
            p: PhantomData,
        }
    }

    pub fn map<Out: SignalReceiver>(self, f: impl Fn(T) -> Out::Type + Clone + Send + Sync + 'static) -> Receiver<Out> {
        Receiver {
            signal: self.signal,
            map: signal_mapper!(T, f),
            p: PhantomData,
        }
    }

    /// Special receiver that maps `Some(_) => ()` regardless of type.
    pub fn any(self) -> Receiver<()> {
        Receiver {
            signal: self.signal,
            map: SignalMapper::Function(Box::new(|obj: &mut Object| *obj = Object::new(()))),
            p: PhantomData,
        }
    }
}

/// A signal receiver
#[derive(Component, Clone)]
pub struct Receiver<T: SignalReceiver>{
    pub(super) signal: Signal,
    pub(super) map: SignalMapper,
    pub(super) p: PhantomData<T>,
}

impl<T: SignalReceiver> Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

impl<T: DataTransfer> Sender<T> {
    pub fn send(&self, item: T) {
        let obj = self.map.map(Object::new(item));
        self.signal.write_dyn(obj);
    }

    pub fn send_dyn(&self, item: Object) {
        let obj = self.map.map(item);
        self.signal.write_dyn(obj);
    }

    /// Create a new receiver of the underlying signal.
    pub fn new_receiver(&self) -> ReceiverBuilder<T> {
        ReceiverBuilder { 
            signal: self.signal.clone(), 
            p: PhantomData 
        }
    }

    /// Try remove the underlying item if polled,
    /// if not, set it as polled.
    /// 
    /// This simulates bevy's double buffered events.
    pub fn try_cleanup(&self) {
        self.signal.try_clean();
    }

}


impl<M: SignalReceiver> Receiver<M> {
    
    /// Receives data from a signal.
    pub fn poll(&self) -> Option<M::Type> {
        self.signal.get_mapped(&self.map)
    }
}

impl<M: SignalReceiver<Type = ()>> Receiver<M> {
    
    /// Receives data from a signal.
    pub fn poll_any(&self) -> bool {
        self.signal.read_any()
    }
}

/// A signal sender with dynamic input, commonly paired with `Payload`.
/// 
/// This is created by type erasing the signal builder, meaning the result
/// is still typed, and type mismatches are ignored.
#[derive(Component, Clone)]
pub struct DynamicSender {
    pub(super) signal: Signal,
    pub(super) map: SignalMapper,
}

impl Debug for DynamicSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}
impl DynamicSender{
    pub fn send<T: DataTransfer>(&self, item: T) {
        let obj = self.map.map(Object::new(item));
        self.signal.write_dyn(obj);
    }

    pub fn send_dyn(&self, item: Object) {
        let obj = self.map.map(item);
        self.signal.write_dyn(obj);
    }

    /// Create a new receiver of the underlying signal.
    pub fn new_receiver<T: DataTransfer>(&self) -> ReceiverBuilder<T> {
        ReceiverBuilder { 
            signal: self.signal.clone(), 
            p: PhantomData 
        }
    }

    /// Try remove the underlying item if polled,
    /// if not, set it as polled.
    /// 
    /// This simulates bevy's double buffered events.
    pub fn try_cleanup(&self) {
        self.signal.try_clean();
    }
}

impl<T: SignalReceiver> DslFrom<ReceiverBuilder<T::Type>> for Option<Receiver<T>> {
    fn dfrom(value: ReceiverBuilder<T::Type>) -> Self {
        Some(value.build())
    }
}

impl<T: SignalReceiver> DslFrom<ReceiverBuilder<T::Type>> for Receiver<T> {
    fn dfrom(value: ReceiverBuilder<T::Type>) -> Self {
        value.build()
    }
}