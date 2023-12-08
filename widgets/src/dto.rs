use bevy::ecs::component::Component;

use crate::Sender;

pub type DtoError = postcard::Error;

/// The `Submit` signal, i.e. checkbox press, textbox press enter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub enum Submit {}

/// The `Change` signal, i.e. radio button change, textbox change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub enum Change {}

/// A serde based data transfer object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Dto(pub(crate) Vec<u8>);

impl Dto {
    pub(crate) fn new(value: &(impl serde::Serialize + ?Sized)) ->  Result<Self, DtoError>{
        postcard::to_stdvec(value).map(Self)
    }

    pub(crate) fn set(&mut self, value: &(impl serde::Serialize + ?Sized)) -> Result<(), DtoError>{
        match postcard::to_stdvec(value) {
            Ok(vec) => { 
                self.0 = vec;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) fn get<'de, T: serde::Deserialize<'de>>(&'de self) -> Result<T, DtoError>{
        postcard::from_bytes(&self.0)
    }

    pub(crate) fn is_empty(& self) -> bool{
        self.0.is_empty()
    }

    pub(crate) fn clear(&mut self){
        self.0.clear()
    }
}

/// When attached to a widget with no inherent data, e.g. a button,
/// the submit signal will send the containing data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)] 
pub struct Payload(Dto);

impl Payload {
    pub const fn empty() -> Self {
        Self(Dto(Vec::new()))
    }

    pub fn new(value: &impl serde::Serialize) -> Result<Self, postcard::Error> {
        Ok(Self(Dto::new(value)?))
    }

    pub fn send<M>(&self, sender: &Sender<M>) {
        sender.send_bytes(&self.0.0)
    }
}