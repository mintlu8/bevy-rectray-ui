
mod converters;
mod core_convert;
mod impl_convert;
mod vec_convert;
pub use converters::*;

mod sealed {
    pub trait DslConvert<B, const C: char> {
        fn parse(self) -> B;
    }
}

pub(crate) use sealed::DslConvert;

/// The `From` trait for `bevy_aoui`'s DSL.
pub trait DslFrom<T> {
    fn dfrom(value: T) -> Self;
}

/// The `Into` trait for `bevy_aoui`'s DSL.
pub trait DslInto<T> {
    fn dinto(self) -> T;
}
