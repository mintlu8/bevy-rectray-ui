use std::{collections::BTreeMap, mem};

use ab_glyph::{Font as FontTrait, ScaleFont};
use bevy::math::Vec2;
use bevy::ecs::component::Component;
use bevy::ecs::query::{With, Changed};
use bevy::ecs::system::{Query, Res, ResMut};
use bevy::render::mesh::Mesh;
use bevy::sprite::{Mesh2d, Mesh2dHandle};
use bevy::{ecs::system::Resource, math::Rect, text::Font};
use bevy::asset::{Handle, AssetServer, Assets};
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::utils::{HashMap, hashbrown::HashSet};
use bevy::render::{texture::Image, color::Color, render_resource::Extent3d};
use ordered_float::OrderedFloat;

#[derive(Debug, Resource)]
pub struct GlyphCache {
    maps: HashMap<(Handle<Font>, OrderedFloat<f32>), GlyphAtlas>
}

#[derive(Debug)]
pub struct GlyphAtlas {
    pub image: Handle<Image>,
    pub glyphs: HashMap<char, Rect>,
    pub ignored: HashSet<char>,
    pub queue: Vec<char>,
    pub margin: u32,
    pub dimension: u32,
    pub row_height: u32,
    pub cursors: Vec<u32>,
}

impl GlyphAtlas {
    pub fn new(asset_server: &AssetServer, font: &Font, size: f32, margin: u32, text: &str) -> Self{
        let font = font.font.as_scaled(size);
        let line_height = font.height().ceil() as u32;
        let image = Image::new(Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }, TextureDimension::D2, vec![1,1], TextureFormat::R8Unorm);
        GlyphAtlas{
            image: asset_server.add(image),
            glyphs: HashMap::new(),
            ignored: HashSet::new(),
            queue: text.chars().collect(),
            margin,
            dimension: 0,
            row_height: line_height,
            cursors: vec![],
        }
    }

    pub fn build_queued(&mut self, im: &mut Image, font: &Font, size: f32) {
        if self.queue.is_empty() || size <= 0.0 {
            return;
        }
        let font = font.font.as_scaled(size);
        let line_height = self.row_height;
        let margin = self.margin;
        let total_len: u32 = self.queue.iter().map(|c| {
            let glyph = font.scaled_glyph(*c);
            font.glyph_bounds(&glyph).width().ceil() as u32 + margin
        }).sum::<u32>() + self.cursors.iter().sum::<u32>();
        // default 64 x 64
        let mut dimension = self.dimension.max(64);
        loop {
            let len = dimension + margin;
            let lines = len / (line_height + margin);
            if lines * len >= total_len {
                break;
            }
            dimension *= 2;
        }

        self.expand(im, dimension);

        let buffer = &mut im.data;
        let mut glyphs = HashMap::new();
        let mut ignored = HashSet::new();

        for char in self.queue.iter().copied() {
            if glyphs.contains_key(&char) || ignored.contains(&char) {
                continue;
            }
            let glyph = font.scaled_glyph(char);
            let bounds = font.glyph_bounds(&glyph);
            for (i, row) in self.cursors.iter_mut().enumerate() {
                if *row + bounds.width().ceil() as u32 > dimension as u32 {
                    continue;
                }
                let (x0, y0) = (*row, i as u32 * line_height + i.saturating_sub(1) as u32 * margin);
                if let Some(glyph) = font.outline_glyph(glyph){
                    glyph.draw(|x, y, c|{
                        buffer[(x + x0 + (y + y0) * dimension) as usize] = (c * 255.0) as u8;
                    });
                    glyphs.insert(char, Rect::new(
                        x0 as f32, y0 as f32, 
                        x0 as f32 + bounds.width(), y0 as f32 + bounds.height()
                    ));
                } else {
                    ignored.insert(char);
                }
                *row += bounds.width().ceil() as u32 + self.margin;
                break;
            }
        }
    }

    pub fn queue(&mut self, s: &str) {
        self.queue.extend(s.chars())
    }

    pub fn expand(&mut self, im: &mut Image, dimension: u32){
        if dimension == self.dimension { return; }
        let mut new_buffer = vec![0u8; dimension as usize * dimension as usize];
        let d0 = self.dimension as usize;
        let d1 = dimension as usize;
        for i in 0..d0 {
            new_buffer[i * d1..i * d1 + d0].copy_from_slice(&im.data[i * d0..i * d0 + d0]);
        }
        im.texture_descriptor.size = Extent3d{
            width: dimension,
            height: dimension,
            depth_or_array_layers: 1,
        };
        im.data = new_buffer;
        self.dimension = dimension;
        let lines = (self.dimension + self.margin) / (self.row_height + self.margin);
        self.cursors.resize(lines as usize, 0);
    }

    pub fn expand_margin(&mut self, margin: u32) {
        self.queue = mem::take(&mut self.glyphs).into_keys().collect();
        self.margin = margin
    }
}

#[derive(Debug, Component)]
pub struct TextSegment {
    pub text: String,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub color: Color,
    pub opacity: f32,
    pub wrap: bool,
}

pub struct ComputedTextSize(Vec2);
pub struct CharOffsets(Vec<f32>);

pub fn build_text_mesh(
    assets: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    fonts: Res<Assets<Font>>,
    cache: ResMut<GlyphCache>,
    query: Query<(&TextSegment, &Mesh2dHandle), Changed<TextSegment>>
) {
    for (seg, mesh) in query.iter() {
        let Some(font) = fonts.get(seg.font.clone()) else { continue; };
        let font = font.font.as_scaled(seg.font_size);
        let cache = cache.maps.get(&(seg.font.clone(), OrderedFloat(seg.font_size)));
        let mut origin = 0.0;
        let mut size = Vec2::ZERO;
        let mut glyphs = Vec::new();
        let mut mesh = meshes.get(mesh.0);
        for char in seg.text.chars() {
            let glyph = font.scaled_glyph(char);
            origin += font.h_advance(glyph.id);
            size += font.glyph_bounds(&glyph).max.into();
            
        }
    }

}