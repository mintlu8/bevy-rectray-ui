
use bevy::{reflect::TypePath, sprite::Material2d, ecs::system::{Query, ResMut}};
use bevy::asset::{Asset, Handle, Assets};
use bevy::math::{Vec2, Vec4};
use bevy::render::{color::Color, texture::Image};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, Shader};
use bevy_aoui::{anim::{Interpolate, Interpolation}, dsl::DslInto, DimensionData};

use crate::builders::Stroke;

pub const ROUNDED_RECTANGLE_SHADER: Handle<Shader> =       Handle::weak_from_u128(270839355282343875567970925758141260070);
pub const ROUNDED_SHADOW_SHADER: Handle<Shader> =          Handle::weak_from_u128(270839355282343875567970925758141260071);

/// If you 
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
#[non_exhaustive]
pub struct RoundedShadowMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub color: Color,
    /// The size of the material on screen in pixels
    #[uniform(1)]
    pub shadow_size: f32,
    #[uniform(2)]
    pub size: Vec2,
    #[uniform(3)]
    pub capsule: f32,
    #[uniform(4)]
    pub corners: Vec4,
}
impl RoundedShadowMaterial {
    pub fn new(color: Color, corner: f32, size: f32) -> Self {
        Self { 
            color, 
            shadow_size: size, 
            size: Vec2::ZERO,
            capsule: 0.0,
            corners: Vec4::splat(corner),
        }
    }

    pub fn capsule(color: Color, size: f32) -> Self {
        Self { 
            color, 
            shadow_size: size, 
            size: Vec2::ZERO,
            capsule: 1.0,
            corners: Vec4::ZERO,
        }
    }
}

impl Material2d for RoundedShadowMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        ShaderRef::Handle(ROUNDED_SHADOW_SHADER)
    }
}


#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
#[non_exhaustive]
pub struct RoundedRectangleMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub color: Color,
    /// The size of the material on screen in pixels
    #[uniform(1)]
    pub size: Vec2,
    #[uniform(2)]
    pub stroke_color: Color,
    /// The size of the material on screen in pixels
    #[uniform(3)]
    pub stroke_size: f32,
    /// 0 means rounded rectangle, 1 means capsule, this can be interpolated for some nice animation.
    #[uniform(4)]
    pub capsule: f32,
    #[uniform(5)]
    pub corners: Vec4,
    #[texture(6)]
    #[sampler(7)]
    pub image: Option<Handle<Image>>
}

pub trait IntoCorners {
    fn into_corners(self) -> Vec4;
}

impl IntoCorners for f32 {
    fn into_corners(self) -> Vec4 {
        Vec4::splat(self)
    }
}

impl IntoCorners for [f32; 4] {
    fn into_corners(self) -> Vec4 {
        Vec4::from_array(self)
    }
}

impl IntoCorners for Vec4 {
    fn into_corners(self) -> Vec4 {
        self
    }
}

impl RoundedRectangleMaterial {

    pub fn new(color: Color, corner: impl IntoCorners) -> Self {
        Self { 
            color, image: None, corners: corner.into_corners(), size: Vec2::ZERO,
            capsule: 0.0,
            stroke_color: Color::NONE, stroke_size: 0.0 
        }
    }


    pub fn capsule(color: Color) -> Self {
        Self { 
            color, image: None, corners: Vec4::ZERO, size: Vec2::ZERO,
            capsule: 1.0,
            stroke_color: Color::NONE, stroke_size: 0.0 
        }
    }

    pub fn rect(color: Color) -> Self {
        Self { 
            color, image: None, corners: Vec4::ZERO, size: Vec2::ZERO,
            capsule: 0.0,
            stroke_color: Color::NONE, stroke_size: 0.0 
        }
    }
    

    pub fn from_image(image: Handle<Image>, color: Color, corner: impl IntoCorners) -> Self {
        Self { 
            color, image: Some(image), corners: corner.into_corners(), size: Vec2::ZERO,
            capsule: 0.0,
            stroke_color: Color::NONE, stroke_size: 0.0 
        }
    }

    pub fn capsule_image(image: Handle<Image>, color: Color) -> Self {
        Self { 
            color, image: Some(image), corners: Vec4::ZERO, size: Vec2::ZERO,
            capsule: 1.0,
            stroke_color: Color::NONE, stroke_size: 0.0 
        }
    }

    pub fn with_stroke(mut self, stroke: impl DslInto<Stroke>) -> Self {
        let stroke = stroke.dinto();
        self.stroke_color = stroke.color;
        self.stroke_size = stroke.size;
        self
    }
}

impl Material2d for RoundedRectangleMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        ShaderRef::Handle(ROUNDED_RECTANGLE_SHADER)
    }
}

pub fn sync_rounded_rect(
    query: Query<(&Handle<RoundedRectangleMaterial>, &DimensionData)>,
    mut assets: ResMut<Assets<RoundedRectangleMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn sync_rounded_shadow(
    query: Query<(&Handle<RoundedShadowMaterial>, &DimensionData)>, 
    mut assets: ResMut<Assets<RoundedShadowMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn interpolate_round_rect_color(
    query: Query<(&Interpolate<Color>, &Handle<RoundedRectangleMaterial>)>, 
    mut assets: ResMut<Assets<RoundedRectangleMaterial>> 
){
    for (interpolate, material) in query.iter() {
        let Some(asset) = assets.get_mut(material) else {return};
        asset.color = interpolate.get()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StrokeColor {}

impl Interpolation for StrokeColor {
    type FrontEnd = Color;
    type Data = Vec4;

    fn into_data(data: Self::FrontEnd) -> Self::Data {
        data.into()
    }

    fn into_front_end(data: Self::Data) -> Self::FrontEnd {
        data.into()
    }
}

pub fn interpolate_stroke_color(
    query: Query<(&Interpolate<StrokeColor>, &Handle<RoundedRectangleMaterial>)>, 
    mut assets: ResMut<Assets<RoundedRectangleMaterial>> 
){
    for (interpolate, material) in query.iter() {
        let Some(asset) = assets.get_mut(material) else {return};
        asset.stroke_color = interpolate.get()
    }
}
