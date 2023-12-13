use bevy::{asset::Handle, sprite::{TextureAtlas, TextureAtlasSprite}, ecs::component::Component};
use bevy::math::{Vec2, Rect, UVec2};
use bevy::render::{texture::Image, color::Color};

use crate::{widget_extension, map_builder, dsl::builders::FrameBuilder};

use super::{Widget, DslFrom};

#[derive(Debug, Component)]
pub enum DeferredAtlasBuilder {
    Subdivide{
        image: Handle<Image>,
        slices: UVec2,
        padding: Option<Vec2>,
    },
    Grid {
        image: Handle<Image>,
        size: Vec2,
        count: UVec2,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
    },
    Images(Vec<Handle<Image>>)
}

pub struct AtlasLoader;

#[derive(Debug, Default)]
pub enum AtlasSprites {
    #[default]
    None,
    ImageName(String),
    ImageHandle(Handle<Image>),
    ImageNames(Vec<String>),
    ImageHandles(Vec<Handle<Image>>),
}

#[derive(Debug, Default)]
pub enum AtlasRectangles {
    #[default]
    None,
    AtlasFile(String),
    AtlasAsset(Handle<TextureAtlas>),
    Rectangles(Vec<Rect>),
    Subdivide(UVec2),
    Grid {
        size: Vec2,
        count: UVec2,
    },
}

widget_extension!(pub struct AtlasBuilder {
    pub atlas: AtlasRectangles,
    pub sprites: AtlasSprites,
    pub size: Option<Vec2>,
    pub color: Option<Color>,
    pub rect: Option<Rect>,
    pub flip: [bool; 2],
    pub index: usize,
    pub padding: Option<Vec2>,
    pub atlas_offset: Option<Vec2>,
    pub atlas_size: Option<Vec2>,
});

impl Widget for AtlasBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> bevy::prelude::Entity {
        let entity = map_builder!(self => FrameBuilder move (
            anchor, parent_anchor, center, opacity, visible,
            offset, rotation, scale, z, dimension, hitbox,
            layer, font_size, event
        )).spawn_with(commands, assets);
        let assets = ||assets.expect("Please pass in the AssetServer.");
        let [x, y] = self.flip;
        let sprite = TextureAtlasSprite{
            color: self.color.unwrap_or(Color::WHITE),
            index: self.index,
            flip_x: x,
            flip_y: y,
            custom_size: self.size,
            anchor: self.anchor.into(),
        };
        match self.atlas {
            AtlasRectangles::AtlasFile(file) => {
                let asset: Handle<TextureAtlas> = assets().load(file);
                commands.entity(entity).insert((
                    asset,
                    sprite
                ));
            },
            AtlasRectangles::AtlasAsset(asset) => {
                commands.entity(entity).insert((
                    asset,
                    sprite
                ));
            },
            AtlasRectangles::None => {
                let handles = match self.sprites {
                    AtlasSprites::ImageNames(names) => names.into_iter().map(|x| assets().load(x)).collect(),
                    AtlasSprites::ImageHandles(handles) => handles,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    DeferredAtlasBuilder::Images(handles)
                ));
            },
            AtlasRectangles::Rectangles(rectangles) => {
                let texture = match self.sprites {
                    AtlasSprites::ImageName(name) => assets().load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                let mut atlas = TextureAtlas::new_empty(texture, self.atlas_size
                    .expect("Must specify the size of the atlas image, since image loading is deferred."));
                for rect in rectangles {
                    atlas.add_texture(rect);
                }
                let atlas = assets().add(atlas);
                commands.entity(entity).insert((
                    sprite,
                    atlas,
                ));
            },
            AtlasRectangles::Grid { size, count } => {
                let image = match self.sprites {
                    AtlasSprites::ImageName(name) => assets().load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    DeferredAtlasBuilder::Grid { 
                        image, 
                        size, 
                        count, 
                        padding: self.padding, 
                        offset: self.atlas_offset, 
                    },
                ));
            },
            AtlasRectangles::Subdivide(slices) => {
                let image = match self.sprites {
                    AtlasSprites::ImageName(name) => assets().load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    DeferredAtlasBuilder::Subdivide { 
                        image,
                        padding: self.padding,
                        slices, 
                    },
                ));
            }
        }
        entity
    }
}

impl DslFrom<String> for AtlasSprites {
    fn dfrom(value: String) -> Self {
        Self::ImageName(value)
    }
}

impl DslFrom<&str> for AtlasSprites {
    fn dfrom(value: &str) -> Self {
        Self::ImageName(value.to_owned())
    }
}

impl DslFrom<Handle<Image>> for AtlasSprites {
    fn dfrom(value: Handle<Image>) -> Self {
        Self::ImageHandle(value)
    }
}

impl DslFrom<&Handle<Image>> for AtlasSprites {
    fn dfrom(value: &Handle<Image>) -> Self {
        Self::ImageHandle(value.clone())
    }
}

impl<const N: usize> DslFrom<[&str; N]> for AtlasSprites {
    fn dfrom(value: [&str; N]) -> Self {
        Self::ImageNames(value.into_iter().map(|x| x.to_owned()).collect())
    }
}

impl<const N: usize> DslFrom<[String; N]> for AtlasSprites {
    fn dfrom(value: [String; N]) -> Self {
        Self::ImageNames(value.into())
    }
}

impl DslFrom<&[&str]> for AtlasSprites {
    fn dfrom(value: &[&str]) -> Self {
        Self::ImageNames(value.iter().map(|x| (*x).to_owned()).collect())
    }
}

impl DslFrom<&[String]> for AtlasSprites {
    fn dfrom(value: &[String]) -> Self {
        Self::ImageNames(value.to_vec())
    }
}

impl DslFrom<Vec<&str>> for AtlasSprites {
    fn dfrom(value: Vec<&str>) -> Self {
        Self::ImageNames(value.into_iter().map(|x| x.to_owned()).collect())
    }
}

impl DslFrom<Vec<String>> for AtlasSprites {
    fn dfrom(value: Vec<String>) -> Self {
        Self::ImageNames(value)
    }
}

impl<const N: usize> DslFrom<[Handle<Image>; N]> for AtlasSprites {
    fn dfrom(value: [Handle<Image>; N]) -> Self {
        Self::ImageHandles(value.into())
    }
}

impl DslFrom<&[Handle<Image>]> for AtlasSprites {
    fn dfrom(value: &[Handle<Image>]) -> Self {
        Self::ImageHandles(value.to_vec())
    }
}

impl DslFrom<Vec<Handle<Image>>> for AtlasSprites {
    fn dfrom(value: Vec<Handle<Image>>) -> Self {
        Self::ImageHandles(value)
    }
}
