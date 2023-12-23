
use bevy::{reflect::TypePath, sprite::Material2d, ecs::system::{Query, ResMut}};
use bevy::asset::{Asset, Handle, Assets};
use bevy::math::{Vec2, Vec4};
use bevy::render::{color::Color, texture::Image};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, Shader};
use bevy_aoui::{Dimension, anim::Interpolate, dsl::DslInto};

use crate::builders::Stroke;

pub const CAPSULE_SHADER: Handle<Shader> =          Handle::weak_from_u128(270839355282343875567970925758141260060);
pub const CAPSULE_SHADOW_SHADER: Handle<Shader> =   Handle::weak_from_u128(270839355282343875567970925758141260061);

pub const ROUNDED_RECTANGLE_SHADER: Handle<Shader> =       Handle::weak_from_u128(270839355282343875567970925758141260070);
pub const ROUNDED_SHADOW_SHADER: Handle<Shader> =          Handle::weak_from_u128(270839355282343875567970925758141260071);

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
#[non_exhaustive]
pub struct CapsuleShadowMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub color: Color,
    /// The background color of the material
    #[uniform(1)]
    pub shadow_size: f32,
    /// The size of the material on screen in pixels
    #[uniform(2)]
    pub size: Vec2,
    #[uniform(3)]
    pub darken: f32,
}
impl CapsuleShadowMaterial {
    pub fn new(color: Color, shadow_size: f32) -> Self {
        Self { color, shadow_size, size: Vec2::ZERO, darken: 1.0 }
    }
    /// The default setting is tuned for light theme, set it between `0..=1` in dark theme.
    pub fn darken(mut self, darken: f32) -> Self {
        self.darken = darken;
        self
    }
}

impl Material2d for CapsuleShadowMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        ShaderRef::Handle(CAPSULE_SHADOW_SHADER)
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
#[non_exhaustive]
pub struct CapsuleMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub color: Color,
    /// The size of the material on screen in pixels
    #[uniform(1)]
    pub size: Vec2,
    /// The background color of the material
    #[uniform(2)]
    pub stroke_color: Color,
    /// The size of the material on screen in pixels
    #[uniform(3)]
    pub stroke_size: f32,
    #[texture(4)]
    #[sampler(5)]
    pub image: Option<Handle<Image>>
}

impl CapsuleMaterial {
    pub fn new(color: Color) -> Self {
        Self { color, image: None, size: Vec2::ZERO, stroke_color: Color::NONE, stroke_size: 0.0 }
    }

    pub fn from_image(image: Handle<Image>, color: Color, ) -> Self {
        Self { color, image: Some(image), size: Vec2::ZERO, stroke_color: Color::NONE, stroke_size: 0.0  }
    }

    pub fn with_stroke(mut self, stroke: impl DslInto<Stroke>) -> Self {
        let stroke = stroke.dinto();
        self.stroke_color = stroke.color;
        self.stroke_size = stroke.size;
        self
    }
}

impl Material2d for CapsuleMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        ShaderRef::Handle(CAPSULE_SHADER)
    }
}

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
    pub corners: Vec4,
    #[uniform(4)]
    pub darken: f32,
}
impl RoundedShadowMaterial {
    pub fn new(color: Color, corner: f32, size: f32) -> Self {
        Self { color, shadow_size: size, size: Vec2::ZERO, corners: Vec4::splat(corner),
            darken: 1.0,
        }
    }

    /// The default setting is tuned for light theme, set it between `0..=1` in dark theme.
    pub fn darken(mut self, darken: f32) -> Self {
        self.darken = darken;
        self
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
    #[uniform(4)]
    pub corners: Vec4,
    #[texture(5)]
    #[sampler(6)]
    pub image: Option<Handle<Image>>
}

impl RoundedRectangleMaterial {

    pub fn new(color: Color, corner: f32) -> Self {
        Self { color, image: None, corners: Vec4::splat(corner), size: Vec2::ZERO,
        stroke_color: Color::NONE, stroke_size: 0.0 }
    }

    pub fn from_image(image: Handle<Image>, color: Color, corner: f32) -> Self {
        Self { color, image: Some(image), corners: Vec4::splat(corner), size: Vec2::ZERO,
            stroke_color: Color::NONE, stroke_size: 0.0 }
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

pub fn sync_capsule(
    query: Query<(&Handle<CapsuleMaterial>, &Dimension)>, 
    mut assets: ResMut<Assets<CapsuleMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn sync_capsule_shadow(
    query: Query<(&Handle<CapsuleShadowMaterial>, &Dimension)>, 
    mut assets: ResMut<Assets<CapsuleShadowMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn sync_rounded_rect(
    query: Query<(&Handle<RoundedRectangleMaterial>, &Dimension)>,
    mut assets: ResMut<Assets<RoundedRectangleMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn sync_rounded_shadow(
    query: Query<(&Handle<RoundedShadowMaterial>, &Dimension)>, 
    mut assets: ResMut<Assets<RoundedShadowMaterial>>
){
    for (handle, dimension) in query.iter() {
        let Some(asset) = assets.get_mut(handle) else {return};
        asset.size = dimension.size;
    }
}

pub fn interpolate_capsule_color(
    query: Query<(&Interpolate<Color>, &Handle<CapsuleMaterial>)>, 
    mut assets: ResMut<Assets<CapsuleMaterial>> 
){
    for (interpolate, material) in query.iter() {
        let Some(asset) = assets.get_mut(material) else {return};
        asset.color = interpolate.get()
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