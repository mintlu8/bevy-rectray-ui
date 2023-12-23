use std::{fmt::Debug, any::TypeId};
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

/// A type erased nullable dynamic object.
#[derive(Debug)]
#[derive(Default)]
pub struct Object(Option<Box<dyn DataTransfer>>);

impl Clone for Object {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().map(|x| x.dyn_clone()))
    }
}



impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => a.dyn_eq(b.as_ref()),
            _ => false
        }
    }
}

impl Object {
    pub const NONE: Self = Self(None);

    pub fn unit() -> Self {
        Self(Some(Box::new(())))
    }

    pub fn new<T: DataTransfer>(v: T) -> Self {
        Self(Some(Box::new(v)))
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn get<T: DataTransfer>(&self) -> Option<T> {
        self.0.as_ref().and_then(|x| x.dyn_clone().downcast().ok().map(|x| *x))
    }

    pub fn clean(&mut self) {
        self.0.take();
    }


    pub fn take<T: DataTransfer>(&mut self) -> Option<T> {
        self.0.take().and_then(|x| x.downcast().ok().map(|x| *x))
    }

    pub fn set<T: DataTransfer>(&mut self, v: T) {
        self.0 = Some(Box::new(v))
    }

    pub fn swap<T: DataTransfer>(&mut self, v: T) -> Option<T>{
        let result = self.take();
        self.0 = Some(Box::new(v));
        result
    }
}
