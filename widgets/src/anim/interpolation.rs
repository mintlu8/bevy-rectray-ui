use std::ops::{Add, Mul};

use bevy::{math::Vec2, render::color::Color, ecs::{component::Component, system::{Query, Res}}, time::Time, sprite::Sprite};
use bevy_aoui::{Transform2D, Dimension, Opacity};
use interpolation::{EaseFunction, Ease};

#[derive(Debug, Component)]
#[component(storage="SparseSet")]
/// A CSS like easing animation executor. 
/// 
/// To use, instead of directy editing `Transform2D`, call `register`.
pub struct Interpolate<T: Interpolation>{
    curve: EaseFunction,
    range: Option<(T::Data, T::Data)>,
    current: f32,
    time: f32,
}

impl<T: Interpolation> Default for Interpolate<T> {
    fn default() -> Self {
        Self { 
            curve: EaseFunction::QuarticInOut, 
            range: None, 
            current: 0.0, 
            time: 0.0,
        }
    }
}


impl<T: Interpolation> Interpolate<T> {

    pub fn new(curve: EaseFunction, time: f32) -> Self {
        Interpolate {
            curve,
            time,
            range: None,
            current: 0.0
        }
    }

    pub fn get(&self) -> Option<T::Data> {
        let (from, to) = self.range?;
        if self.time <= 0.0 {
            return Some(to);
        }
        let p = (self.current / self.time).clamp(0.0, 1.0).calc(self.curve);
        Some(from * (1.0 - p) + to * p)
    }

    /// End animation and obtain the target.
    pub fn take_target(&mut self) -> Option<T::Data> {
        self.range.take().map(|(_, x)| x)
    }

    /// Returns true on the last frame.
    pub fn step(&mut self, time: f32) -> bool {
        if self.range.is_some() {
            self.current += time;
            if self.current > self.time {
                self.range = None;
                return true
            }
        }
        false
    }

    pub fn register(&mut self, from: T::Data, to: T::Data) {
        self.range = Some((from, to));
        self.current = 0.0;
    }

    pub fn register_with_time(&mut self, from: T::Data, to: T::Data, time: f32) {
        self.range = Some((from, to));
        self.current = 0.0;
        self.time = time;
    }
}

pub trait Interpolation {
    type Data: Add<Self::Data, Output = Self::Data> + Mul<f32, Output = Self::Data> + Copy;
}

macro_rules! impl_interpolation {
    ($($name: ident: $ty: ty);* $(;)?) => {
        $(pub struct $name;

        impl Interpolation for $name {
            type Data = $ty;
        })*
    };
}

impl_interpolation!(
    Offset: Vec2; Rotation: f32; Scale: Vec2;
);

impl Interpolation for Dimension {
    type Data = Vec2;
}

impl Interpolation for Color {
    type Data = Color;
}

impl Interpolation for Opacity {
    type Data = f32;
}

pub fn interpolate_offset(
    mut query: Query<(&mut Transform2D, &Interpolate<Offset>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        let Some(v) = interpolate.get() else {continue};
        transform.offset.edit_raw(|x| *x = v);
    }
}

pub fn interpolate_rotation(
    mut query: Query<(&mut Transform2D, &Interpolate<Rotation>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        let Some(v) = interpolate.get() else {continue};
        transform.rotation = v;
    }
}

pub fn interpolate_scale(
    mut query: Query<(&mut Transform2D, &Interpolate<Scale>)>
) {
    for (mut transform, interpolate) in query.iter_mut() {
        let Some(v) = interpolate.get() else {continue};
        transform.scale = v;
    }
}

pub fn interpolate_dimension(
    mut query: Query<(&mut Dimension, &Interpolate<Dimension>)>
) {
    for (mut dimension, interpolate) in query.iter_mut() {
        if dimension.is_owned() {
            let Some(v) = interpolate.get() else {continue};
            dimension.edit_raw(|x| *x = v);
        }
    }
}

pub fn interpolate_color(
    mut query: Query<(&mut Sprite, &Interpolate<Color>)>
) {
    for (mut sprite, interpolate) in query.iter_mut() {
        let Some(v) = interpolate.get() else {continue};
        sprite.color = v;
    }
}

pub fn interpolate_opacity(
    mut query: Query<(&mut Opacity, &Interpolate<Opacity>)>
) {
    for (mut opacity, interpolate) in query.iter_mut() {
        let Some(v) = interpolate.get() else {continue};
        opacity.opactity = v;
    }
}

pub fn update_interpolate(
    time: Res<Time>,
    mut query0: Query<&mut Interpolate<Offset>>,
    mut query1: Query<&mut Interpolate<Rotation>>,
    mut query2: Query<&mut Interpolate<Scale>>,
    mut query3: Query<&mut Interpolate<Dimension>>,
    mut query4: Query<&mut Interpolate<Color>>,
    mut query5: Query<&mut Interpolate<Opacity>>,
) {
    let time = time.delta_seconds();
    for mut item in query0.iter_mut() {
        item.step(time);
    }
    for mut item in query1.iter_mut() {
        item.step(time);
    }
    for mut item in query2.iter_mut() {
        item.step(time);
    }
    for mut item in query3.iter_mut() {
        item.step(time);
    }
    for mut item in query4.iter_mut() {
        item.step(time);
    }
    for mut item in query5.iter_mut() {
        item.step(time);
    }
}
