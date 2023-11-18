use bevy::{log::error, ecs::component::Component};

/// Submit something that should be handled with serde.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct Submit(Dto);

impl Submit {
    pub fn new(value: &(impl serde::Serialize + ?Sized)) -> Self{
        Self(Dto::new(value))
    }

    pub fn get<'de, T: serde::Deserialize<'de> + Default>(&'de self) -> T{
        self.0.get()
    }

    pub fn try_get<'de, T: serde::Deserialize<'de> + Default>(&'de self) -> Option<T>{
        self.0.try_get()
    }
}

/// A serde based data transfer object.
#[derive(Debug)]
pub(crate) struct Dto(Vec<u8>);

impl Dto {
    pub(crate) fn new(value: &(impl serde::Serialize + ?Sized)) -> Self{
        match postcard::to_stdvec(value) {
            Ok(vec) => Self(vec),
            Err(e) => {
                error!("Serialization failed: {}", e);
                Self(Vec::new())
            },
        }
    }

    pub(crate) fn get<'de, T: serde::Deserialize<'de> + Default>(&'de self) -> T{
        match postcard::from_bytes(&self.0) {
            Ok(value) => value, 
            Err(e) => {
                error!("Deserialization failed: {}", e);
                T::default()
            }
        }
    }


    pub(crate) fn try_get<'de, T: serde::Deserialize<'de> + Default>(&'de self) -> Option<T>{
        postcard::from_bytes(&self.0).ok()
    }
}

