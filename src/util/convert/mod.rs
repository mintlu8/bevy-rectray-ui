
mod converters;
mod core_convert;
mod impl_convert;
mod vec_convert;
pub use converters::*;

pub(crate) mod sealed {
    pub enum SealToken{}
}

/// The core conversion trait for DSL conversion. 
/// See [`DslFrom`] and [`DslInto`] for the main implementors.
/// 
/// This trait uses a marker type for extra flexibility
/// compared to its counterpart [`Into`]. 
/// Since any ambiguity can break `bevy_rectray`'s DSL,
/// this trait is sealed and cannot be implemented downstream.
pub trait DslConvert<B, const C: char> {
    fn parse(self) -> B;
    fn sealed(seal: SealToken);
}

pub(crate) use sealed::SealToken;

/// The `From` trait for `bevy_rectray`'s DSL.
pub trait DslFrom<T> {
    fn dfrom(value: T) -> Self;
}

/// The `Into` trait for `bevy_rectray`'s DSL.
pub trait DslInto<T> {
    fn dinto(self) -> T;
}


