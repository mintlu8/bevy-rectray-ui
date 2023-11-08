
use bevy::sprite::{Sprite, TextureAtlasSprite, Anchor};
use bevy::text::{TextLayoutInfo, Text2dBounds, update_text2d_layout};
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;

use crate::compute::compute_aoui_transforms;
use crate::{RotatedRect, ScreenSpaceTransform, BuildTransform, Dimension, AouiREM, DimensionSize, Transform2D};

/// Core plugin for AoUI Rendering.
pub struct AoUIPlugin;

impl bevy::prelude::Plugin for AoUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AouiREM>()
            .add_systems(PostUpdate, (
                copy_anchor, 
                copy_anchor_sprite, 
                copy_anchor_atlas,
                copy_dimension_sprite,
                copy_dimension_text,
                copy_dimension_atlas,
            ).before(compute_aoui_transforms).after(update_text2d_layout))
            .add_systems(PostUpdate, compute_aoui_transforms.before(build_transform))
            .add_systems(PostUpdate, 
                build_transform
                    .before(sync_dimension_atlas)
                    .before(sync_dimension_sprite)
                    .before(sync_dimension_text_bounds)
            )
            .add_systems(PostUpdate, (
                    sync_dimension_atlas,
                    sync_dimension_sprite,
                    sync_dimension_text_bounds,
                    sync_em_text,
                )
                .before(propagate_transforms)
                .before(sync_simple_transforms)
            )
            .add_systems(PostUpdate, 
                finalize
                    .after(propagate_transforms)
                    .after(sync_simple_transforms)
            );
    }
}

/// Copy our `anchor` component's value to the `Anchor` component
pub fn copy_anchor(mut query: Query<(&mut Anchor, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut a, anc)| *a = anc.anchor.clone())
}

/// Copy evaluated `TextLayoutInfo` value to our `Dimension::Copied` value
pub fn copy_dimension_text(mut query: Query<(&TextLayoutInfo, &mut Dimension)>) {
    //let scale_factor = window.get_single().map(|x| x.scale_factor() as f32).unwrap_or(1.0);
    query.par_iter_mut().for_each(|(text, mut dim)| {
        match dim.dim {
            DimensionSize::Copied => dim.size = text.logical_size,
            DimensionSize::Scaled(s) => dim.size = text.logical_size * s,
            _ => (),
        }
    })
}

/// Copy our `Anchors` component's value to the `Sprite` component
pub fn copy_anchor_sprite(mut query: Query<(&mut Sprite, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.clone();
    })
}

/// Synchonize size between `Sprite` and `Dimension`
pub fn copy_dimension_sprite(mut query: Query<(&Sprite, &Handle<Image>, &mut Dimension)>, assets: Res<Assets<Image>>) {
    query.par_iter_mut().for_each(|(sp, im, mut dimension)| {
        dimension.update_copied(|| {
            match sp.custom_size {
                Some(x) => x,
                None => match sp.rect {
                    Some(rect) => rect.max - rect.min,
                    None => assets.get(im).map(|x|x.size().as_vec2()).unwrap_or(Vec2::ZERO),
                },
            }
        });
    })
}

/// Copy anchor to the `TextureAtlasSprite` component
pub fn copy_anchor_atlas(mut query: Query<(&mut TextureAtlasSprite, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.clone();
    })
}

/// copy size between `TextureAtlasSprite` to `Dimension`
pub fn copy_dimension_atlas(mut query: Query<(&TextureAtlasSprite, &Handle<TextureAtlas>, &mut Dimension)>, assets: Res<Assets<TextureAtlas>>) {
    query.par_iter_mut().for_each(|(sp, im, mut dimension)| {
        dimension.update_copied(|| {
            match sp.custom_size {
                Some(size) => size,
                None => (|| -> Option<_> {
                   let rect = assets.get(im)?.textures.get(sp.index)?;
                   Some(rect.max - rect.min)
                })().unwrap_or(Vec2::ZERO)
            }
        });
    })
}

/// Synchonize size from `Dimension` to `Sprite`
pub fn sync_dimension_sprite(mut query: Query<(&mut Sprite, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.spawn_owned(|size| sp.custom_size = Some(size))
    })
}

/// Synchonize size from `Dimension` to `TextureAtlasSprite`
pub fn sync_dimension_atlas(mut query: Query<(&mut TextureAtlasSprite, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.spawn_owned(|size| sp.custom_size = Some(size))
    })
}

/// Copy owned dimension as text bounds. 
/// 
/// If this behavior is not desired, either remove this system
/// or do not use owned dimension on text.
pub fn sync_dimension_text_bounds(mut query: Query<(&mut Text2dBounds, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.spawn_owned(|size| sp.size = size)
    })
}


/// Copy owned dimension as text bounds. 
/// 
/// If this behavior is not desired, either remove this system
/// or do not use owned dimension on text.
pub fn sync_em_text(mut query: Query<(&mut Text, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        sp.sections.iter_mut().for_each(|x| x.style.font_size = dimension.em)
    })
}

/// Synchonize size between `Sprite` and `Dimension`
// pub fn sync_size_text(mut query: Query<(&mut TextLayoutInfo, &Dimension)>) {
//     query.par_iter_mut().for_each(|(mut sp, dimension)| {
//         match dimension.dim {
//             DimensionSize::Owned(_) => {
//                 sp. = Some(dimension.size)
//             },
//             _ => (),
//         }
//     })
// }

/// Build a transform using [`RotatedRect`].
pub fn build_transform(mut query: Query<(&RotatedRect, &BuildTransform, &mut Transform)>) {
    query.par_iter_mut().for_each(|(rect, BuildTransform(anchor), mut transform)| {
        transform.translation = rect.anchor(anchor).extend(rect.z);
        transform.rotation = Quat::from_rotation_z(rect.rotation);
        transform.scale = rect.scale.extend(1.0);
    });
}

/// The last system in the AoUI pipeline.
///
/// Currently does nothing in particular
/// other than copying [`ScreenSpaceTransform`] into [`GlobalTransform`].
pub fn finalize(
    mut query: Query<(&ScreenSpaceTransform, &mut GlobalTransform)>,
) {
    query.par_iter_mut().for_each(|(screen, mut scene)| {
        *scene = screen.0.into()
    });
}
