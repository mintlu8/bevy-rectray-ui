use std::mem;

use bevy::{render::texture::Image, math::{Vec2, Rect}, sprite::{TextureAtlas, TextureAtlasBuilder}, log::warn};
use bevy::asset::{Handle, Assets, AssetServer};
use bevy::ecs::{component::Component, system::{Query, Commands, Res, ResMut}, entity::Entity};


#[derive(Debug, Component)]
pub enum DeferredAtlasBuilder {
    Subdivide {
        image: Handle<Image>,
        count: [usize; 2],
        padding: Option<Vec2>,
    },
    Images(Vec<Handle<Image>>),
    Rectangles {
        image: Handle<Image>,
        rectangles: Vec<Rect>,
    }
}

pub fn build_deferred_atlas(
    mut commands: Commands,
    mut atlas: Query<(Entity, &mut DeferredAtlasBuilder)>,
    server: Res<AssetServer>,
    mut image_assets: ResMut<Assets<Image>>,
){
    'main: for (entity, mut builder) in atlas.iter_mut(){
        let atlas = match builder.as_mut() {
            DeferredAtlasBuilder::Subdivide { image, count, padding } => {
                let [x, y] = *count;
                let Some(im) = image_assets.get(image.clone()) else {continue 'main;};
                let size = im.size().as_vec2() - 
                    padding.unwrap_or(Vec2::ZERO) * 
                    Vec2::new(x.saturating_sub(1) as f32, y.saturating_sub(1) as f32);
                let size = size / Vec2::new(x as f32, y as f32);
                TextureAtlas::from_grid(
                    image.clone(), 
                    size, 
                    y, x, 
                    *padding, 
                    None,
                )
            },
            DeferredAtlasBuilder::Images(images) => {
                let mut builder = TextureAtlasBuilder::default();
                for image in mem::take(images) {
                    let id = image.id();
                    let Some(im) = image_assets.get(image) else {continue 'main};
                    builder.add_texture(id, im);
                }
                match builder.finish(&mut image_assets) {
                    Ok(atlas) => atlas,
                    Err(e) => { 
                        warn!("Texture atlas building failed: {e}.");
                        continue 'main;
                    },
                }
            },
            DeferredAtlasBuilder::Rectangles { image, rectangles } => {
                let Some(im) = image_assets.get(image.clone()) else {return};
                let mut atlas = TextureAtlas::new_empty(image.clone(), im.size().as_vec2());
                atlas.textures = mem::take(rectangles);
                atlas
            },
        };
        commands.entity(entity)
            .remove::<DeferredAtlasBuilder>()
            .insert(server.add(dbg!(atlas)));
    }
}