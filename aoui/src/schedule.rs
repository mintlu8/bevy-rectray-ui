
use bevy::sprite::{Sprite, TextureAtlasSprite, Anchor};
use bevy::text::{TextLayoutInfo, update_text2d_layout, Text2dBounds};
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;

use crate::compute::compute_aoui_root;
use crate::{Anchors, RotatedRect, ScreenSpaceTransform, BuildTransform, Dimension, AorsREM, DimensionSize};

/// Core plugin for AoUI Rendering.
pub struct AoUIPlugin;

impl bevy::prelude::Plugin for AoUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AorsREM>()
            .add_systems(PostUpdate, (
                copy_anchor, 
                copy_anchor_sprite, 
                copy_dimension_sprite,
                copy_dimension_text,
                copy_anchor_atlas,
                copy_dimension_atlas,
            ).before(compute_aoui_root).after(update_text2d_layout))
            .add_systems(PostUpdate, compute_aoui_root.before(build_transform))
            .add_systems(PostUpdate, 
                build_transform
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

/// Copy our `Anchors` component's value to the `Anchor` component
pub fn copy_anchor(mut query: Query<(&mut Anchor, &Anchors)>) {
    query.par_iter_mut().for_each_mut(|(mut a, anc)| *a = anc.anchor.clone())
}

/// Copy evaluated `TextLayoutInfo` value to our `Dimension::Copied` value
pub fn copy_dimension_text(mut query: Query<(&TextLayoutInfo, &mut Dimension)>) {
    query.par_iter_mut().for_each_mut(|(text, mut dim)| {
        if let DimensionSize::Copied = &mut dim.as_mut().dim {
            dim.size = text.size / 2.0;
        }
    })
}

/// Copy our `Anchors` component's value to the `Sprite` component
pub fn copy_anchor_sprite(mut query: Query<(&mut Sprite, &Anchors)>) {
    query.par_iter_mut().for_each_mut(|(mut sp, anc)| {
        sp.anchor = anc.anchor.clone();
    })
}

/// Synchonize size between `Sprite` and `Dimension`
pub fn copy_dimension_sprite(mut query: Query<(&mut Sprite, &Handle<Image>, &mut Dimension)>, assets: Res<Assets<Image>>) {
    query.par_iter_mut().for_each_mut(|(mut sp, im, mut dimension)| {
        match &mut dimension.as_mut().dim {
            DimensionSize::Copied => {
                dimension.size = match sp.custom_size {
                    Some(x) => x,
                    None => match sp.rect {
                        Some(rect) => rect.max - rect.min,
                        None => assets.get(im).map(|x|x.size()).unwrap_or(Vec2::ZERO),
                    },
                }
            },
            DimensionSize::Owned(_) => {
                sp.custom_size = Some(dimension.size)
            },
        }
    })
}

/// Copy dimension to text bounds. 
/// 
/// This is not the standard behavior.
/// Insert this manually if desired.
pub fn copy_dimension_text_bounds(mut query: Query<(&mut Text2dBounds, &mut Dimension)>) {
    query.par_iter_mut().for_each_mut(|(mut sp, mut dimension)| {
        match &mut dimension.as_mut().dim {
            DimensionSize::Owned(_) => {
                sp.size = dimension.size
            },
            _ => (),
        }
    })
}

/// Copy our `Anchors` component's value to the `TextureAtlasSprite` component
pub fn copy_anchor_atlas(mut query: Query<(&mut TextureAtlasSprite, &Anchors)>) {
    query.par_iter_mut().for_each_mut(|(mut sp, anc)| {
        sp.anchor = anc.anchor.clone();
    })
}

/// Synchonize size between `TextureAtlasSprite` and `Dimension`
pub fn copy_dimension_atlas(mut query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut Dimension)>, assets: Res<Assets<TextureAtlas>>) {
    query.par_iter_mut().for_each_mut(|(mut sp, im, mut dimension)| {
        if let DimensionSize::Copied = &mut dimension.as_mut().dim {
            dimension.size = match sp.custom_size {
                Some(size) => size,
                None => (|| -> Option<_> {
                   let rect = assets.get(im)?.textures.get(sp.index)?;
                   Some(rect.max - rect.min)
                })().unwrap_or(Vec2::ZERO)
            }
        } else {
            sp.custom_size = Some(dimension.size);
        }
    })
}

/// Build a transform with [`Anchors::center`] and [`RotatedRect`]
pub fn build_transform(mut query: Query<(&Anchors, &RotatedRect, &mut Transform), With<BuildTransform>>) {
    for (aoui, quad, mut transform) in query.iter_mut() {
        transform.translation = quad.anchor(aoui.get_center()).extend(quad.z);
        transform.rotation = Quat::from_rotation_z(quad.rotation);
        transform.scale = quad.scale.extend(1.0);
    }
}


/// Currently does nothing in particular
/// other than copying [`ScreenSpaceTransform`] into [`GlobalTransform`].
pub fn finalize(
    mut query: Query<(&ScreenSpaceTransform, &mut GlobalTransform)>,
) {
    for (screen, mut scene) in query.iter_mut() {
        *scene = screen.0.into()
    }
}
