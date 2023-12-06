#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub struct Submit;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub struct Change;

/// A serde based data transfer object.
#[derive(Debug)]
pub(crate) struct Dto(Vec<u8>);

impl Dto {
    pub(crate) fn new(value: &(impl serde::Serialize + ?Sized)) ->  Result<Self, postcard::Error>{
        postcard::to_stdvec(value).map(|x| Self(x))
    }

    pub(crate) fn set(&mut self, value: &(impl serde::Serialize + ?Sized)) -> Result<(), postcard::Error>{
        match postcard::to_stdvec(value) {
            Ok(vec) => { 
                self.0 = vec;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) fn get<'de, T: serde::Deserialize<'de>>(&'de self) -> Result<T, postcard::Error>{
        postcard::from_bytes(&self.0)
    }

    pub(crate) fn is_empty(& self) -> bool{
        self.0.is_empty()
    }

    pub(crate) fn clear(&mut self){
        self.0.clear()
    }
}

