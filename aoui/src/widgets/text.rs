use bevy::{reflect::Reflect, asset::{Handle, Assets}};
use bevy::render::texture::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::text::{Font, Text, TextStyle};
use bevy::ecs::{component::Component, query::Changed, world::Mut};
use bevy::ecs::system::{Query, Res, ResMut};

use crate::{DimensionData, util::ScalingFactor, Coloring};

use ab_glyph::{Font as _, point};
use ab_glyph::ScaleFont as _;

/// A simple text manager representing a fragment of text,
/// fragment as in no explicit support for linebreaks.
/// When paired with [`Sprite`](bevy::sprite::Sprite), this will render it with `ab_glyph`,
/// when paired when [`Text`] this will update its contents.
///
/// This struct is designed with change detection in mind to maximize performance.
#[derive(Debug, Clone, Default, PartialEq, Component, Reflect)]
#[non_exhaustive]
pub struct TextFragment {
    pub text: String,
    pub font: Handle<Font>,
    pub size: f32,
}

impl TextFragment {

    pub fn new(text: impl Into<String>) -> Self {
        TextFragment {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn with_font(mut self, font: Handle<Font>) -> Self{
        self.font = font;
        self
    }

    /// Does not change if value is not changed
    pub fn set_text(s: &mut Mut<Self>, value: &str) {
        if s.text != value {
            s.text = value.to_owned()
        }
    }

    /// Does not change if value is not changed
    pub fn set_font(s: &mut Mut<Self>, value: &Handle<Font>) {
        if &s.font != value {
            s.font = value.clone()
        }
    }

    /// Does not change if value is not changed
    pub fn set_size(s: &mut Mut<Self>, em: f32) {
        if s.size != em {
            s.size = em
        }
    }
}

pub fn sync_em_text_fragment(
    mut query: Query<(&DimensionData, &mut TextFragment), Changed<TextFragment>, >
) {
    query.iter_mut().for_each(|(dim, mut frag)| {
        TextFragment::set_size(&mut frag, dim.em)
    })
}


pub fn sync_text_text_fragment(
    mut query: Query<(&mut Text, &Coloring, &TextFragment), Changed<TextFragment>, >
) {
    query.iter_mut().for_each(|(mut text, color, frag)| {
        if frag.size <= 0.0 {return}
        text.sections.clear();
        text.sections.push(bevy::text::TextSection {
            value: frag.text.clone(),
            style: TextStyle {
                font: frag.font.clone(),
                font_size: frag.size,
                color: color.color,
            }
        })
    })
}

pub fn measure_string<F: ab_glyph::Font>(
    font: &impl ab_glyph::ScaleFont<F>,
    string: &str,
) -> f32 {
    let mut cursor = 0.0;
    let mut last = '\0';
    for c in string.chars() {
        cursor += font.kern(font.glyph_id(last), font.glyph_id(c));
        cursor += font.h_advance(font.glyph_id(c));
        last = c
    }
    cursor
}

pub fn sync_sprite_text_fragment(
    scale_factor: ScalingFactor,
    mut images: ResMut<Assets<Image>>,
    fonts: Res<Assets<Font>>,
    mut query: Query<(&TextFragment, &Handle<Image>), Changed<TextFragment>>
) {
    let scale_factor = scale_factor.get();
    for (fragment, handle) in query.iter_mut() {
        if fragment.size <= 0.0 {continue;}
        let font = match fonts.get(&fragment.font) {
            Some(font) => font.font.as_scaled(fragment.size * scale_factor),
            None => continue,
        };
        let Some(image) = images.get_mut(handle) else {continue};
        let dimension = measure_string(&font, &fragment.text);
        let height = font.height().ceil();
        let width = (dimension.ceil() as usize).max(1);
        let height = (height.ceil() as usize).max(1);
        let mut buffer = vec![0u8; width * height * 4];

        let mut cursor = 0.0;
        let mut last = '\0';
        for c in fragment.text.chars() {
            let mut glyph = font.scaled_glyph(c);
            glyph.position = point(cursor, 0.0 + font.ascent());
            cursor += font.kern(font.glyph_id(last), font.glyph_id(c));
            cursor += font.h_advance(font.glyph_id(c));
            last = c;
            if let Some(glyph) = font.outline_glyph(glyph) {
                let bounds = glyph.px_bounds();
                glyph.draw(|x, y, v| {
                    let x = x as usize + bounds.min.x as usize;
                    let y = y as usize + bounds.min.y as usize;
                    if x < width && y < height {
                        buffer[(x + y * width) * 4] = 255;
                        buffer[(x + y * width) * 4 + 1] = 255;
                        buffer[(x + y * width) * 4 + 2] =  255;
                        buffer[(x + y * width) * 4 + 3] += (v * 255.0) as u8;
                    }
                })
            }
        }

        *image = Image::new(Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        }, TextureDimension::D2, buffer, TextureFormat::Rgba8Unorm)
    }
}
