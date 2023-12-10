pub type DtoError = postcard::Error;

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
}
