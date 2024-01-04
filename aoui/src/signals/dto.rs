use std::{fmt::Debug, any::TypeId, mem};
use downcast_rs::{impl_downcast, Downcast};

const _: Option<Box<dyn DataTransfer>> = None;

pub trait DataTransfer: Downcast + Debug + Send + Sync + 'static {
    fn dyn_clone(&self) -> Box<dyn DataTransfer>;
    fn dyn_eq(&self, other: &dyn DataTransfer) -> bool;
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl_downcast!(DataTransfer);
impl<T> DataTransfer for T where T: Debug + Clone + PartialEq + Send + Sync + 'static{
    fn dyn_clone(&self) -> Box<dyn DataTransfer> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &dyn DataTransfer) -> bool {
        match other.downcast_ref::<T>() {
            Some(some) => some == self,
            None => false,
        }
    }
}

pub trait AsObject: Sized + Debug + Clone + Send + Sync + 'static {
    fn get(obj: &Object) -> Option<Self>;
    fn get_ref(obj: &Object) -> Option<&Self>;
    fn from_object(obj: Object) -> Option<Self>;
    fn into_object(self) -> Object;
}

impl<T> AsObject for T where T: DataTransfer + Clone {
    fn get(obj: &Object) -> Option<Self> {
        obj.0.as_ref().and_then(|x| x.dyn_clone().downcast().ok().map(|x| *x))
    }

    fn get_ref(obj: &Object) -> Option<&Self> {
        obj.0.as_ref().and_then(|x| x.downcast_ref())
    }
    
    fn from_object(obj: Object) -> Option<Self> {
        obj.0.and_then(|x| x.downcast().map(|x| *x).ok())
    }

    fn into_object(self) -> Object {
        Object(Some(Box::new(self)))
    }
}

impl AsObject for Object  {
    fn get(obj: &Object) -> Option<Self> {
        if obj.is_some(){
            Some(obj.clone())
        } else {
            None
        }
    }

    fn get_ref(obj: &Object) -> Option<&Self> {
        if obj.is_some(){
            Some(obj)
        } else {
            None
        }
    }

    fn from_object(obj: Object) -> Option<Self> {
        if obj.is_some(){
            Some(obj)
        } else {
            None
        }
    }

    fn into_object(self) -> Object {
        self
    }
}

/// A type erased nullable dynamic object.
#[derive(Debug)]
#[derive(Default)]
pub struct Object(Option<Box<dyn DataTransfer>>);

impl Clone for Object {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().map(|x| x.dyn_clone()))
    }
}


impl Object {
    pub const NONE: Self = Self(None);

    pub fn unit() -> Self {
        Self(Some(Box::new(())))
    }

    /// Create a unnameable object that is not equal to anything.
    pub fn unnameable() -> Self {
        #[derive(Debug, Clone)]
        struct UnnameableUnequal;

        impl PartialEq for UnnameableUnequal{
            fn eq(&self, _: &Self) -> bool {
                false
            }
        }
        Self(Some(Box::new(UnnameableUnequal)))
    }


    pub fn new<T: AsObject>(v: T) -> Self {
        AsObject::into_object(v)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn get<T: AsObject>(&self) -> Option<T> {
        AsObject::get(self)
    }

    pub fn get_ref<T: AsObject>(&self) -> Option<&T> {
        AsObject::get_ref(self)
    }

    pub fn clean(&mut self) {
        self.0.take();
    }

    pub fn take<T: AsObject>(&mut self) -> Option<T> {
        AsObject::from_object(mem::take(self))
    }

    pub fn set<T: AsObject>(&mut self, v: T) {
        *self = AsObject::into_object(v)
    }

    pub fn swap<T: AsObject>(&mut self, v: T) -> Option<T>{
        let result = self.take();
        *self = AsObject::into_object(v);
        result
    }

    /// Object does not impl `PartialEq` for specialization.
    pub fn equal_to(&self, other: &Object) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => a.dyn_eq(b.as_ref()),
            (None, None) => true,
            _ => false
        }
    }
}
