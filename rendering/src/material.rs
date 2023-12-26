use bevy::{asset::Handle, render::{texture::Image, color::Color}, math::{Rect, Vec2, Affine2}, sprite::{Sprite, TextureAtlas}};


pub trait AouiMaterial {
    fn set_color(&mut self);
    fn set_opacity(&mut self);
    fn set_clip(&mut self, clip: Affine2);
}

pub struct AouiSprite {
    pub texture: Handle<Image>,
    pub size: Option<Vec2>,
    pub rect: Rect,
    pub color: Color,
    pub opacity: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub struct AouiAtlas {
    pub atlas: Handle<TextureAtlas>,
    pub index: usize,
}
