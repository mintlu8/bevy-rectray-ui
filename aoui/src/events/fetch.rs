use std::marker::PhantomData;
use bevy::ecs::system::{Query, Res};
use crate::{anim::{Interpolation, Padding, Margin}, Dimension,  DimensionData, dsl::prelude::{Rotation, Scale, Offset}, Transform2D, Opacity, layout::Container, AouiREM};
use super::{EventHandling, Handlers};

mod sealed {
    #[derive(Debug, Clone)]
    pub struct Sealed;
}

/// An event that fetches a piece of data through a channel.
/// 
/// Unlike normal events, these should not be registered and
/// can only send signals.
/// 
/// i.e. `Fetch<Dimension>`
/// 
/// Note this fetch the `raw` size if underlying value is a `Size2`,
/// use `Fetch<Evaluated<T>>` to get the evaluated pixel value.
#[derive(Debug, Clone)]
pub enum Fetch<T> {
    _Phantom(sealed::Sealed, PhantomData<T>)
}

/// Fetch evaluated pixel size instead of raw size.
/// 
/// i.e. `Fetch<Evaluated<Dimension>>`
#[derive(Debug, Clone)]
pub enum Evaluated<T> {
    _Phantom(sealed::Sealed, PhantomData<T>)
}

impl<T: Interpolation> EventHandling for Fetch<T> where T::Data: PartialEq {
    type Data = T::Data;

    type Context = ();

    fn new_context() -> Self::Context {}
}

impl<T: Interpolation> EventHandling for Fetch<Evaluated<T>> where T::Data: PartialEq {
    type Data = T::Data;

    type Context = ();

    fn new_context() -> Self::Context {}
}

pub fn transfer_dimension(
    query: Query<(&Dimension, &Handlers<Fetch<Dimension>>)>
) {
    query.iter().for_each(|(dim, sig)| {
        dim.with_raw(|x| sig.send_signal(x))
    })
}

pub fn transfer_dimension_evaluated(
    query: Query<(&DimensionData, &Handlers<Fetch<Evaluated<Dimension>>>)>
) {
    query.iter().for_each(|(dim, sig)| {
        sig.send_signal(dim.size)
    })
}

pub fn transfer_offset(
    query: Query<(&Transform2D, &Handlers<Fetch<Offset>>)>
) {
    query.iter().for_each(|(transform, sig)| {
        sig.send_signal(transform.offset.raw())
    })
}

pub fn transfer_offset_evaluated(
    rem: Res<AouiREM>,
    query: Query<(&Transform2D, &DimensionData, &Handlers<Fetch<Evaluated<Offset>>>)>
) {
    query.iter().for_each(|(transform, dim, sig)| {
        sig.send_signal(transform.offset.as_pixels(dim.size, dim.em, rem.get()))
    })
}

pub fn transfer_rotation(
    query: Query<(&Transform2D, &Handlers<Fetch<Rotation>>)>
) {    
    query.iter().for_each(|(transform, sig)| {
        sig.send_signal(transform.rotation)
    })
}

pub fn transfer_scale(
    query: Query<(&Transform2D, &Handlers<Fetch<Scale>>)>
) {    
    query.iter().for_each(|(transform, sig)| {
        sig.send_signal(transform.scale)
    })
}


pub fn transfer_opacity(
    query: Query<(&Opacity, &Handlers<Fetch<Opacity>>)>
) {
    query.iter().for_each(|(op, sig)| {
        sig.send_signal(op.opacity)
    })
}

pub fn transfer_margin(
    query: Query<(&Container, &Handlers<Fetch<Margin>>)>
) {
    query.iter().for_each(|(container, sig)| {
        sig.send_signal(container.margin.raw())
    })
}

pub fn transfer_padding(
    query: Query<(&Container, &Handlers<Fetch<Padding>>)>
) {
    query.iter().for_each(|(container, sig)| {
        sig.send_signal(container.padding.raw())
    })
}


pub fn transfer_margin_evaluated(
    rem: Res<AouiREM>,
    query: Query<(&DimensionData, &Container, &Handlers<Fetch<Evaluated<Margin>>>)>
) {
    query.iter().for_each(|(dim, container, sig)| {
        sig.send_signal(container.margin.as_pixels(dim.size, dim.em, rem.get()))
    })
}

pub fn transfer_padding_evaluated(
    rem: Res<AouiREM>,
    query: Query<(&DimensionData, &Container, &Handlers<Fetch<Evaluated<Padding>>>)>
) {
    query.iter().for_each(|(dim, container, sig)| {
        sig.send_signal(container.padding.as_pixels(dim.size, dim.em, rem.get()))
    })
}
    