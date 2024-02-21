use bevy::sprite::{Sprite, TextureAtlasLayout};
use bevy::{asset::Handle, sprite::TextureAtlas, math::UVec2, ecs::entity::Entity};
use bevy::math::{Vec2, Rect};
use bevy::render::{texture::Image, color::Color};

use crate::{frame_extension, widgets::DeferredAtlasBuilder, bundles::BuildTransformBundle, build_frame};

use crate::util::{Widget, DslFrom, RCommands};

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
    AtlasHandle(Handle<TextureAtlasLayout>),
    AtlasStruct(TextureAtlasLayout),
    Rectangles(Vec<Rect>),
    Subdivide([usize; 2]),
    Grid {
        offset: Vec2,
        size: Vec2,
        count: [usize; 2],
    },
}

frame_extension!(pub struct AtlasBuilder {
    /// Either the atlas or the rectangle of the atlas.
    ///
    /// # Accepts
    ///
    /// * File name: `String` or `&str`
    /// (requires an importer for `TextureAtlas`)
    /// * Handle: `Handle<TextureAtlas>`
    /// * Struct: `TextureAtlas`
    /// * Rectangles: `Vec<Rect>`, `[Rect; N]`, `[[f32; 4]; N]`
    /// * Subdivision: `UVec2`, `[u32; 2]`
    /// * Grid: `AtlasGrid { .. }`
    /// * Unspecified: Builds a atlas with `sprites`
    pub atlas: AtlasRectangles,
    /// Sprites supporting the atlas
    ///
    /// # Accepts
    ///
    /// * File name: `String`
    /// * Handle: `Handle<Image>`
    /// * File names: `Vec<String>`
    /// * File handles: `Vec<Handle<Image>>`
    pub sprites: AtlasSprites,
    /// Size of the sprite
    pub size: Option<Vec2>,
    /// Flips the sprite.
    pub flip: [bool; 2],
    /// Index of the atlas.
    pub index: usize,
    /// Padding of the atlas.
    pub atlas_padding: Option<Vec2>,
});

impl Widget for AtlasBuilder {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let entity = build_frame!(commands, self).insert(BuildTransformBundle::default()).id();
        let [x, y] = self.flip;
        let sprite = Sprite {
            color: self.color.unwrap_or(Color::WHITE),
            flip_x: x,
            flip_y: y,
            custom_size: self.size,
            anchor: self.anchor.into(),
            rect: None,
        };
        match self.atlas {
            AtlasRectangles::AtlasFile(file) => {
                let texture = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load::<Image>(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                let layout: Handle<TextureAtlasLayout> = commands.load(file);
                commands.entity(entity).insert((
                    sprite,
                    texture,
                    TextureAtlas {
                        index: self.index,
                        layout
                    }
                ));
            },
            AtlasRectangles::AtlasStruct(atlas) => {
                let texture = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load::<Image>(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                let layout: Handle<TextureAtlasLayout> = commands.add_asset(atlas);
                commands.entity(entity).insert((
                    sprite,
                    texture,
                    TextureAtlas {
                        index: self.index,
                        layout
                    }
                ));
            },
            AtlasRectangles::AtlasHandle(layout) => {
                let texture = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load::<Image>(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    texture,
                    TextureAtlas {
                        index: self.index,
                        layout
                    }
                ));
            },
            AtlasRectangles::None => {
                let handles = match self.sprites {
                    AtlasSprites::ImageNames(names) => names.into_iter().map(|x| commands.load(x)).collect(),
                    AtlasSprites::ImageHandles(handles) => handles,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    DeferredAtlasBuilder::Images{
                        index: self.index,
                        images: handles,
                    }
                ));
            },
            AtlasRectangles::Rectangles(rectangles) => {
                let image = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    image,
                    DeferredAtlasBuilder::Rectangles{
                        rectangles,
                        index: self.index,
                    },
                ));
            },
            AtlasRectangles::Grid { size, count, offset } => {
                let image = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                let [x, y] = count;
                let atlas = TextureAtlasLayout::from_grid(size, y, x, self.atlas_padding, Some(offset));
                let layout = commands.add_asset(atlas);
                commands.entity(entity).insert((
                    sprite,
                    image,
                    TextureAtlas {
                        layout,
                        index: self.index,
                    },
                ));
            },
            AtlasRectangles::Subdivide(slices) => {
                let image = match self.sprites {
                    AtlasSprites::ImageName(name) => commands.load(name),
                    AtlasSprites::ImageHandle(handle) => handle,
                    _ => panic!("Invalid atlas build mode. Either supply images or rectangles on an image.")
                };
                commands.entity(entity).insert((
                    sprite,
                    image,
                    DeferredAtlasBuilder::Subdivide {
                        padding: self.atlas_padding,
                        count: slices,
                        index: self.index,
                    },
                ));
            }
        }
        (entity, entity)
    }
}

/// Construct a texture atlas sprite. The underlying struct is [`AtlasBuilder`].
#[macro_export]
macro_rules! atlas {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::AtlasBuilder] {
            $($tt)*
        })
    };
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

impl DslFrom<String> for AtlasRectangles {
    fn dfrom(value: String) -> Self {
        AtlasRectangles::AtlasFile(value)
    }
}

impl DslFrom<&str> for AtlasRectangles {
    fn dfrom(value: &str) -> Self {
        AtlasRectangles::AtlasFile(value.to_owned())
    }
}

impl DslFrom<TextureAtlasLayout> for AtlasRectangles {
    fn dfrom(value: TextureAtlasLayout) -> Self {
        AtlasRectangles::AtlasStruct(value)
    }
}

impl DslFrom<Handle<TextureAtlasLayout>> for AtlasRectangles {
    fn dfrom(value: Handle<TextureAtlasLayout>) -> Self {
        AtlasRectangles::AtlasHandle(value)
    }
}

impl DslFrom<Vec<Rect>> for AtlasRectangles {
    fn dfrom(value: Vec<Rect>) -> Self {
        AtlasRectangles::Rectangles(value)
    }
}

impl DslFrom<&[Rect]> for AtlasRectangles {
    fn dfrom(value: &[Rect]) -> Self {
        AtlasRectangles::Rectangles(value.to_vec())
    }
}

impl<const N: usize> DslFrom<[Rect; N]> for AtlasRectangles {
    fn dfrom(value: [Rect; N]) -> Self {
        AtlasRectangles::Rectangles(value.into())
    }
}

impl DslFrom<[usize; 2]> for AtlasRectangles {
    fn dfrom(value: [usize; 2]) -> Self {
        AtlasRectangles::Subdivide(value)
    }
}

impl DslFrom<UVec2> for AtlasRectangles {
    fn dfrom(value: UVec2) -> Self {
        AtlasRectangles::Subdivide([value.x as usize, value.y as usize])
    }
}


impl<const N: usize> DslFrom<[[i32; 4]; N]> for AtlasRectangles {
    fn dfrom(value: [[i32; 4]; N]) -> Self {
        AtlasRectangles::Rectangles(value.into_iter()
            .map(|[x, y, w, h]| Rect {
                min: Vec2::new(x as f32, y as f32),
                max: Vec2::new((x + w) as f32, (y + h) as f32),
            })
            .collect()
        )
    }
}

impl<const N: usize> DslFrom<[[f32; 4]; N]> for AtlasRectangles {
    fn dfrom(value: [[f32; 4]; N]) -> Self {
        AtlasRectangles::Rectangles(value.into_iter()
            .map(|[x, y, w, h]| Rect {
                min: Vec2::new(x, y),
                max: Vec2::new(x + w, y + h),
            })
            .collect()
        )
    }
}
