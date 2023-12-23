use std::ops::{Add, Mul};

use bevy::{render::color::Color, time::Time, text::Text};
use bevy::sprite::{Sprite, TextureAtlasSprite};
use bevy::ecs::{component::Component, system::{Query, Res}};
use bevy::math::{Vec2, Vec4};
use crate::{Transform2D, Dimension, Opacity};
use interpolation::EaseFunction;
use smallvec::SmallVec;

use super::{Easing, Playback};

#[derive(Debug, Clone, Component)]
#[component(storage="SparseSet")]

/// A smart tweener that manages an external value.
pub struct Interpolate<T: Interpolation>{
    /// Easing function of the tweener.
    curve: Easing,
    /// Interpolates through these keyframes.
    /// 
    /// Invariant: this field must have at least 1 value.
    range: SmallVec<[(T::Data, f32); 1]>,
    /// Current time.
    current: f32,
    time: f32,
    default_time: f32,
    playback: Playback,
}

pub trait IntoInterpolate<T: Interpolation> {
    fn into_interpolate(self) -> SmallVec<[(T::Data, f32); 1]>;
}

impl<T: Interpolation> IntoInterpolate<T> for (T::FrontEnd, T::FrontEnd) {
    fn into_interpolate(self) -> SmallVec<[(T::Data, f32); 1]> {
        let (start, end) = self;
        [(T::into_data(start), 0.0), (T::into_data(end), 1.0)].into_iter().collect()
    }
}

impl<T: Interpolation, const N: usize> IntoInterpolate<T> for [(T::FrontEnd, f32); N] {
    fn into_interpolate(self) -> SmallVec<[(T::Data, f32); 1]> {
        self.into_iter().map(|(a, b)| (T::into_data(a), b)).collect()
    }
}

impl<T: Interpolation> IntoInterpolate<T> for &[(T::FrontEnd, f32)] {
    fn into_interpolate(self) -> SmallVec<[(T::Data, f32); 1]> {
        self.iter().map(|(a, b)| (T::into_data(*a), *b)).collect()
    }
}

impl<T: Interpolation> IntoInterpolate<T> for SmallVec<[(T::Data, f32); 1]> {
    fn into_interpolate(self) -> SmallVec<[(T::Data, f32); 1]> {
        self
    }
}

impl<T: Interpolation> Interpolate<T> {

    pub const fn const_new(curve: Easing, position: T::Data, time: f32) -> Self {
        Interpolate {
            curve,
            time: 0.0,
            default_time: time,
            range: SmallVec::from_const([(position, 0.0)]),
            current: 0.0,
            playback: Playback::Once,
        }
    }


    pub fn new(curve: Easing, position: T::FrontEnd, time: f32) -> Self {
        Interpolate {
            curve,
            time: 0.0,
            default_time: time,
            range: SmallVec::from_const([(T::into_data(position), 0.0)]),
            current: 0.0,
            playback: Playback::Once,
        }
    }

    pub fn ease(curve: EaseFunction, position: T::FrontEnd, time: f32) -> Self {
        Interpolate {
            curve: Easing::Ease(curve),
            time: 0.0,
            default_time: time,
            range: SmallVec::from_const([(T::into_data(position), 0.0)]),
            current: 0.0,
            playback: Playback::Once,
        }
    }

    pub fn init(curve: Easing, positions: impl IntoInterpolate<T>, time: f32) -> Self {
        Interpolate {
            curve,
            time,
            default_time: time,
            range: positions.into_interpolate(),
            current: 0.0,
            playback: Playback::Once,
        }
    }

    pub fn looping(curve: Easing, positions: impl IntoInterpolate<T>, time: f32) -> Self {
        Interpolate {
            curve,
            time,
            default_time: time,
            range: positions.into_interpolate(),
            current: 0.0,
            playback: Playback::Loop,
        }
    }

    pub fn repeat(curve: Easing, positions: impl IntoInterpolate<T>, time: f32) -> Self {
        Interpolate {
            curve,
            time,
            default_time: time,
            range: positions.into_interpolate(),
            current: 0.0,
            playback: Playback::Repeat,
        }
    }

    fn get_data(&self) -> T::Data {
        if self.range.len() == 1 || self.time <= 0.0 || (self.playback.is_once() && self.current >= self.time) {
            return self.range.last().expect("Interpolate has no value, this is a bug.").0;
        }
        if self.current <= 0.0 {
            return self.range.first().unwrap().0;
        }
        let t = match self.playback {
            Playback::Once => (self.current / self.time).clamp(0.0, 1.0),
            Playback::Loop => {
                let v = (self.current / self.time).rem_euclid(2.0);
                1.0 - (1.0 - v).abs()
            },
            Playback::Repeat => (self.current / self.time).rem_euclid(1.0),
        };
        let p = self.curve.get(t);
        let (mut i0, mut f0) = self.range[0];
        for (i1, f1) in self.range.iter().skip(1).copied() {
            if p < f1 {
                let a = p - f0;
                let len = f1 - f0;
                return i0 * ((len - a) / len) + i1 * (a / len);
            }
            (i0, f0) = (i1, f1);
        }
        self.range.last().unwrap().0
    }

    pub fn get(&self) -> T::FrontEnd {
        T::into_front_end(self.get_data())
    }

    /// Get source of this interpolation
    pub fn source(&self) -> T::Data {
        self.range.first().expect("Interpolate has no value, this is a bug.").0
    }

    /// Get target of this interpolation
    pub fn target(&self) -> T::FrontEnd {
        T::into_front_end(self.range.last().expect("Interpolate has no value, this is a bug.").0)
    }

    /// End animation and obtain the target.
    pub fn take_target(&mut self) -> T::FrontEnd {
        let pos = self.get_data();
        let result = self.target();
        self.range = SmallVec::from_const([(pos, 0.0)]);
        result
    }

    /// Update the timer
    pub fn update(&mut self, time: f32) {
        self.current = self.current + time;
    }

    /// Set position and stop interpolation.
    pub fn set(&mut self, pos: T::FrontEnd) {
        self.range = SmallVec::from_const([(T::into_data(pos), 0.0)]);
        self.current = 0.0;
        self.time = self.default_time;
    }

    /// If `to` is the current target, ignore.
    /// If `to` is the current source, reverse.
    /// Otherwise interpolate to the target.
    pub fn interpolate_to(&mut self, to: T::FrontEnd) {
        if self.range.len() > 1 && T::into_front_end(self.range[0].0) == to {
            self.reverse()
        } else if self.target() != to {
            self.range = [(self.get_data(), 0.0), (T::into_data(to), 1.0)].into_iter().collect();
            self.current = 0.0;
            self.time = self.default_time;
        }
    }

    /// Reverse the current curve.
    pub fn reverse(&mut self) {
        self.range.reverse();
        self.range.iter_mut().for_each(|(_, x)| *x = 1.0 - *x);
        self.current = (self.time - self.current).clamp(0.0, self.time);
    }

    /// If end of `range` is the current target, ignore.
    /// If end of `range` is the current source, reverse.
    /// Otherwise interpolate with range, using the current position as range[0].
    /// 
    /// Write directly if this behavior is not desired.
    pub fn interpolate(&mut self, range: impl IntoInterpolate<T>) {
        let mut range = range.into_interpolate();
        if self.range.len() > 1 && opt_eq::<T>(range.last(), self.range.first()) {
            self.reverse()
        } else if !opt_eq::<T>(self.range.last(), range.last()) {
            let pos = self.get_data();
            range[0] = (pos, 0.0);
            self.range = range;
            self.current = 0.0;
            self.time = self.default_time;
        }
    }
    
    /// Interpolate to a target, overwriting default time, 
    /// and using the current position as range[0].
    pub fn interpolate_with_time(&mut self, range: impl IntoInterpolate<T>, time: f32) {
        let mut range = range.into_interpolate();
        let pos = self.get_data();
        range[0] = (pos, 0.0);
        self.range = range;
        self.current = 0.0;
        self.time = time;
    }
}

fn opt_eq<T: Interpolation>(left: Option<&(T::Data, f32)>, right: Option<&(T::Data, f32)>) -> bool {
    match (left, right) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(a), Some(b)) => T::into_front_end(a.0) == T::into_front_end(b.0),
    }
}

/// Trait for a marker type representing a target of interpolation.
pub trait Interpolation {
    type FrontEnd: PartialEq + Copy;
    // We don't compare data.
    type Data: Add<Self::Data, Output = Self::Data> + Mul<f32, Output = Self::Data> + Copy;

    fn into_data(data: Self::FrontEnd) -> Self::Data;
    fn into_front_end(data: Self::Data) -> Self::FrontEnd;
}

#[derive(Debug)]
pub enum Offset{}
#[derive(Debug)]
pub enum Rotation{}
#[derive(Debug)]
pub enum Scale{}
#[derive(Debug)]
pub enum Index{}



impl Interpolation for Offset {
    type FrontEnd = Vec2;
    type Data = Vec2;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data }
}

impl Interpolation for Rotation {
    type FrontEnd = f32;
    type Data = f32;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data }
}

impl Interpolation for Scale {
    type FrontEnd = Vec2;
    type Data = Vec2;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data }
}

impl Interpolation for Index {
    type FrontEnd = usize;
    type Data = f32;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data as f32 }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data.round() as usize }
}

impl Interpolation for Dimension {
    type FrontEnd = Vec2;
    type Data = Vec2;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data }
}

impl Interpolation for Color {
    type FrontEnd = Color;
    type Data = Vec4;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data.into() }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data.into() }
}

impl Interpolation for Opacity {
    type FrontEnd = f32;
    type Data = f32;
    fn into_data(data: Self::FrontEnd) -> Self::Data { data }
    fn into_front_end(data: Self::Data) -> Self::FrontEnd { data }
}

pub fn interpolate_offset(
    mut query: Query<(&mut Transform2D, &Interpolate<Offset>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        transform.offset.edit_raw(|x| *x = interpolate.get());
    }
}

pub fn interpolate_rotation(
    mut query: Query<(&mut Transform2D, &Interpolate<Rotation>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        transform.rotation = interpolate.get();
    }
}

pub fn interpolate_scale(
    mut query: Query<(&mut Transform2D, &Interpolate<Scale>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        transform.scale = interpolate.get();
    }
}

pub fn interpolate_dimension(
    mut query: Query<(&mut Dimension, &Interpolate<Dimension>)>
) {
    for (mut dimension, interpolate) in query.iter_mut() {
        if dimension.is_owned() {
            dimension.edit_raw(|x| *x = interpolate.get());
        }
    }
}

pub fn interpolate_index(
    mut query: Query<(&mut TextureAtlasSprite, &Interpolate<Index>)>
) {
    for (mut atlas, interpolate) in query.iter_mut() {
        atlas.index = interpolate.get()
    }
}

pub fn interpolate_color(
    mut sp_query: Query<(&mut Sprite, &Interpolate<Color>)>,
    mut text_query: Query<(&mut Text, &Interpolate<Color>)>
) {
    for (mut sprite, interpolate) in sp_query.iter_mut() {
        sprite.color = interpolate.get();
    }
    for (mut text, interpolate) in text_query.iter_mut() {
        let color = interpolate.get();
        for section in text.sections.iter_mut() {
            section.style.color = color;
        }
    }
}

pub fn interpolate_opacity(
    mut query: Query<(&mut Opacity, &Interpolate<Opacity>)>
) {
    for (mut opacity, interpolate) in query.iter_mut() {
        opacity.opacity = interpolate.get();
    }
}

pub fn update_interpolate(
    time: Res<Time>,
    mut query0: Query<&mut Interpolate<Offset>>,
    mut query1: Query<&mut Interpolate<Rotation>>,
    mut query2: Query<&mut Interpolate<Scale>>,
    mut query3: Query<&mut Interpolate<Dimension>>,
    mut query4: Query<&mut Interpolate<Index>>,
    mut query5: Query<&mut Interpolate<Color>>,
    mut query6: Query<&mut Interpolate<Opacity>>,
) {
    let time = time.delta_seconds();
    for mut item in query0.iter_mut() {
        item.update(time);
    }
    for mut item in query1.iter_mut() {
        item.update(time);
    }
    for mut item in query2.iter_mut() {
        item.update(time);
    }
    for mut item in query3.iter_mut() {
        item.update(time);
    }
    for mut item in query4.iter_mut() {
        item.update(time);
    }
    for mut item in query5.iter_mut() {
        item.update(time);
    }
    for mut item in query6.iter_mut() {
        item.update(time);
    }
}
