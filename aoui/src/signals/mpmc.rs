use std::{marker::PhantomData, fmt::Debug};

use crate::dsl::CloneSplit;

use super::{dto::{Object, AsObject}, sig::Signal};

/// A function that maps the value of a signal.
#[derive(Default)]
pub enum SignalMapper {
    #[default]
    None,
    Function(Box<dyn SignalMapperFn>),
    If(Object, Object, Object),
}

impl SignalMapper {
    pub fn map(&self, mut obj: Object) -> Object {
        match self {
            SignalMapper::None => obj,
            SignalMapper::Function(f) => {
                f.call(&mut obj);
                obj
            },
            SignalMapper::If(cond, then, el) => {
                if obj.equal_to(cond) {then.clone()} else {el.clone()}
            }
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
    pub fn get_mapped<T: AsObject>(&self, mapper: &SignalMapper) -> Option<T> {
        match mapper {
            SignalMapper::None => self.read(),
            SignalMapper::Function(f) => {
                self.read::<Object>()
                    .and_then(|mut x| {
                        f.call(&mut x);
                        x.get()
                    })
            },
            SignalMapper::If(cond, a, b) => {
                self.read::<Object>()
                    .and_then(|obj| {
                        if obj.equal_to(cond) {
                            a.get()
                        } else {
                            b.get()
                        }
                    })
            },
        }
    }
}

impl Debug for SignalMapper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Function(_) => write!(f, "Function"),
            Self::If(..) => write!(f, "If"),
        }
    }
}

impl Clone for SignalMapper {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Function(arg0) => Self::Function(arg0.dyn_clone()),
            Self::If(a,b,c) => Self::If(a.clone(), b.clone(), c.clone()),
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

#[derive(Debug)]
pub struct SignalBuilder<T: AsObject> {
    pub(super) signal: Signal,
    p: PhantomData<T>,
}

impl<T: AsObject> Clone for SignalBuilder<T> {
    fn clone(&self) -> Self {
        SignalBuilder { signal: self.signal.clone(), p: PhantomData }
    }
}

impl<T: AsObject> SignalBuilder<T> {

    pub(super) fn new(signal: Signal) -> Self {
        Self {
            signal,
            p: PhantomData,
        }
    }

    pub fn send(self) -> SignalSender<T>{
        SignalSender { 
            signal: self.signal, 
            map: SignalMapper::None, 
            p: PhantomData,
        }
    }

    pub fn map_send<In: AsObject>(self, f: impl Fn(In) -> T + Clone + Send + Sync + 'static) -> SignalSender<In> {
        SignalSender {
            signal: self.signal,
            map: signal_mapper!(In, f),
            p: PhantomData,
        }
    }

    /// Send if equals, `map_send` does not work with multiple types.
    pub fn cond_send<In: AsObject>(self, if_eq: impl AsObject, then: T, or_else: T) -> SignalSender<In> {
        SignalSender {
            signal: self.signal,
            map: SignalMapper::If(Object::new(if_eq), Object::new(then), Object::new(or_else)),
            p: PhantomData,
        }
    }

    /// Erase the type of a sender, necessary with `Payload`.
    pub fn type_erase(self) -> SignalSender<Object> {
        SignalSender {
            signal: self.signal,
            map: SignalMapper::None,
            p: PhantomData
        }
    }

    // pub fn recv<Out: SignalReceiver<Type = T>>(self) -> Receiver<Out>{
    //     Receiver { 
    //         signal: self.signal, 
    //         map: SignalMapper::None, 
    //         p: PhantomData,
    //     }
    // }

    // pub fn map_recv<Out: SignalReceiver>(self, f: impl Fn(T) -> Out::Type + Clone + Send + Sync + 'static) -> Receiver<Out> {
    //     Receiver {
    //         signal: self.signal,
    //         map: signal_mapper!(T, f),
    //         p: PhantomData,
    //     }
    // }

    // pub fn cond_recv<Out: SignalReceiver>(self, if_eq: impl DataTransfer, then: Out::Type, or_else: Out::Type) -> Receiver<Out> {
    //     Receiver {
    //         signal: self.signal,
    //         map: SignalMapper::If(Object::new(if_eq), Object::new(then), Object::new(or_else)),
    //         p: PhantomData,
    //     }
    // }


    // /// Special receiver that maps `Some(_) => ()` regardless of type.
    // pub fn recv_any(self) -> Receiver<()> {
    //     Receiver {
    //         signal: self.signal,
    //         map: SignalMapper::Function(Box::new(|obj: &mut Object| *obj = Object::new(()))),
    //         p: PhantomData,
    //     }
    // }

    pub fn clone_split<S: CloneSplit<SignalBuilder<T>>>(&self) -> S {
        S::clone_split(SignalBuilder::new(self.signal.clone()))
    }
}

/// A signal sender, unlike receiver this is not a component.
/// 
/// Use event handler instead.
#[derive(Clone)]
pub struct SignalSender<T: AsObject> {
    pub(super) signal: Signal,
    pub(super) map: SignalMapper,
    p: PhantomData<T>,
}

impl<T: AsObject> Debug for SignalSender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Signal as Debug>::fmt(&self.signal, f)
    }
}

impl<T: AsObject> SignalSender<T> {
    pub fn send(&self, item: T) {
        let obj = self.map.map(Object::new(item));
        self.signal.write_dyn(obj);
    }

    pub fn send_dyn(&self, item: Object) {
        let obj = self.map.map(item);
        self.signal.write_dyn(obj);
    }

    /// Create a new receiver of the underlying signal.
    pub fn new_receiver(&self) -> SignalBuilder<T> {
        SignalBuilder { 
            signal: self.signal.clone(), 
            p: PhantomData 
        }
    }

    /// Try remove the underlying item if polled.
    /// 
    /// This simulates bevy's double buffered events.
    /// 
    /// Drop flag is updated every frame to achieve a 'double_buffered' effect.
    pub fn try_cleanup(&self, drop_flag: u8) {
        self.signal.try_clean(drop_flag);
    }

    pub fn clone_split<S: CloneSplit<SignalBuilder<T>>>(&self) -> S {
        S::clone_split(SignalBuilder::new(self.signal.clone()))
    }
}

impl SignalSender<Object> {
    /// Create a new receiver of the underlying signal.
    pub fn specialize_receiver<T: AsObject>(&self) -> SignalBuilder<T> {
        SignalBuilder { 
            signal: self.signal.clone(), 
            p: PhantomData 
        }
    }
}

// impl<T: SignalReceiver> DslFrom<SignalBuilder<T::Type>> for Option<Receiver<T>> {
//     fn dfrom(value: SignalBuilder<T::Type>) -> Self {
//         Some(value.recv())
//     }
// }

// impl<T: SignalReceiver> DslFrom<SignalBuilder<T::Type>> for Receiver<T> {
//     fn dfrom(value: SignalBuilder<T::Type>) -> Self {
//         value.recv()
//     }
// }