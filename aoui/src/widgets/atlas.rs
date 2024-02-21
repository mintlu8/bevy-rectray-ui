use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::ecs::{component::Component, entity::Entity};
use bevy::reflect::Reflect;
use bevy::sprite::{TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout};
use bevy::{
    log::warn,
    math::{Rect, Vec2},
    render::texture::Image,
};
use std::mem;

/// A deferred [`TextureAtlas`] builder that waits for all its sprites to be loaded.
#[derive(Debug, Component, Reflect)]
pub enum DeferredAtlasBuilder {
    Subdivide {
        index: usize,
        count: [usize; 2],
        padding: Option<Vec2>,
    },
    Images {
        index: usize,
        images: Vec<Handle<Image>>,
    },
    Rectangles {
        index: usize,
        rectangles: Vec<Rect>,
    },
}

pub(crate) fn build_deferred_atlas(
    mut commands: Commands,
    mut atlas: Query<(Entity, &mut DeferredAtlasBuilder, Option<&Handle<Image>>)>,
    server: Res<AssetServer>,
    image_assets: ResMut<Assets<Image>>,
) {
    'main: for (entity, mut builder, image) in atlas.iter_mut() {
        match builder.as_mut() {
            DeferredAtlasBuilder::Subdivide {
                index,
                count,
                padding,
            } => {
                let Some(image) = image else {continue};
                let [x, y] = *count;
                let Some(im) = image_assets.get(image) else {
                    continue 'main;
                };
                let size = im.size().as_vec2()
                    - padding.unwrap_or(Vec2::ZERO)
                        * Vec2::new(x.saturating_sub(1) as f32, y.saturating_sub(1) as f32);
                let size = size / Vec2::new(x as f32, y as f32);
                let atlas = TextureAtlasLayout::from_grid(size, y, x, *padding, None);
                commands
                    .entity(entity)
                    .remove::<DeferredAtlasBuilder>()
                    .insert(TextureAtlas {
                        layout: server.add(atlas),
                        index: *index,
                    });
            }
            DeferredAtlasBuilder::Images{ images, index } => {
                let mut builder = TextureAtlasBuilder::default();
                for image in images {
                    let id = image.id();
                    let Some(im) = image_assets.get(image.id()) else {
                        continue 'main;
                    };
                    builder.add_texture(Some(id), im);
                }
                match builder.finish() {
                    Err(e) => {
                        warn!("Texture atlas building failed: {e}.");
                        continue 'main;
                    }
                    Ok((atlas, image)) => {
                        commands
                            .entity(entity)
                            .remove::<DeferredAtlasBuilder>()
                            .insert(server.add(image))
                            .insert(TextureAtlas {
                                layout: server.add(atlas),
                                index: *index,
                            });
                    },

                }
            }
            DeferredAtlasBuilder::Rectangles { rectangles, index } => {
                let Some(image) = image else {continue};
                let Some(image) = image_assets.get(image) else {continue};
                let mut atlas = TextureAtlasLayout::new_empty(image.size().as_vec2());
                atlas.textures = mem::take(rectangles);
                commands
                    .entity(entity)
                    .remove::<DeferredAtlasBuilder>()
                    .insert(TextureAtlas {
                        layout: server.add(atlas),
                        index: *index,
                    });
            }
        };
    }
}
