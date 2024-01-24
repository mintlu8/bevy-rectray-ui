
mod converters;
mod core_convert;
mod impl_convert;
mod vec_convert;
pub use converters::*;

pub(crate) mod sealed {
    pub enum SealToken{}
}

pub trait DslConvertReadOnly<> {
    
}
pub trait DslConvert<B, const C: char> {
    fn parse(self) -> B;
    fn sealed(seal: SealToken);
}

pub(crate) use sealed::SealToken;

/// The `From` trait for `bevy_aoui`'s DSL.
pub trait DslFrom<T> {
    fn dfrom(value: T) -> Self;
}

/// The `Into` trait for `bevy_aoui`'s DSL.
pub trait DslInto<T> {
    fn dinto(self) -> T;
}


