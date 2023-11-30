
use bevy::math::Affine3A;
use bevy::sprite::{Sprite, TextureAtlasSprite, Anchor};
use bevy::text::{TextLayoutInfo, Text2dBounds, update_text2d_layout};
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::compute::*;
use crate::{RotatedRect, BuildGlobal, BuildTransform, Dimension, AoUIREM, DimensionSize, Transform2D};

/// Fetch info for the tree, happens before `AoUITreeUpdate`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUILoadInput;

/// Update the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUITreeUpdate;

/// Update data with the tree, happens after `AoUITreeUpdate` and before bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUIStoreOutput;

/// Write to `GlobalTransform`, after bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUIFinalize;

/// Core plugin for AoUI Rendering.
pub struct AoUIPlugin;

impl bevy::prelude::Plugin for AoUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AoUIREM>()
            .configure_sets(PostUpdate, AoUILoadInput
                .before(AoUITreeUpdate)
                .after(update_text2d_layout))
            .configure_sets(PostUpdate, AoUITreeUpdate
                .before(AoUIStoreOutput))
            .configure_sets(PostUpdate, AoUIStoreOutput
                .before(propagate_transforms)
                .before(sync_simple_transforms)
            )
            .configure_sets(PostUpdate, AoUIFinalize
                .after(propagate_transforms)
                .after(sync_simple_transforms)
            )
            .add_systems(PostUpdate, (
                copy_anchor, 
                copy_anchor_sprite, 
                copy_anchor_atlas,
                copy_dimension_sprite,
                copy_dimension_text,
                copy_dimension_atlas,
            ).in_set(AoUILoadInput))
            .add_systems(PostUpdate,
                compute_aoui_transforms::<PrimaryWindow, TRoot, TAll>
            .in_set(AoUITreeUpdate))
            .add_systems(PostUpdate, (
                    build_transform,
                sync_dimension_atlas,
                sync_dimension_sprite,
                sync_dimension_text_bounds,
                sync_em_text,
            ).in_set(AoUIStoreOutput))
            .add_systems(PostUpdate, 
                finalize.in_set(AoUIFinalize)
            );
    }
}

/// Copy our `anchor` component's value to the `Anchor` component
pub fn copy_anchor(mut query: Query<(&mut Anchor, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut a, anc)| *a = anc.anchor.into())
}

/// Copy evaluated `TextLayoutInfo` value to our `Dimension::Copied` value
pub fn copy_dimension_text(mut query: Query<(&TextLayoutInfo, &mut Dimension)>) {
    //let scale_factor = window.get_single().map(|x| x.scale_factor() as f32).unwrap_or(1.0);
    query.par_iter_mut().for_each(|(text, mut dim)| {
        match dim.dim {
            DimensionSize::Copied => dim.size = text.logical_size,
            _ => (),
        }
    })
}

/// Copy our `Anchors` component's value to the `Sprite` component
pub fn copy_anchor_sprite(mut query: Query<(&mut Sprite, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.into();
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
        sp.anchor = anc.anchor.into();
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
        dimension.run_if_owned(|size| sp.custom_size = Some(size))
    })
}

/// Synchonize size from `Dimension` to `TextureAtlasSprite`
pub fn sync_dimension_atlas(mut query: Query<(&mut TextureAtlasSprite, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.run_if_owned(|size| sp.custom_size = Some(size))
    })
}

/// Opts out of synchronizing text bounds.
#[derive(Debug, Component)]
pub struct OptOutTextBoundsSync;

/// Opts out of synchronizing font size.
#[derive(Debug, Component)]
pub struct OptOutFontSizeSync;


/// Copy owned dimension as text bounds. 
pub fn sync_dimension_text_bounds(mut query: Query<(&mut Text2dBounds, &Dimension), Without<OptOutTextBoundsSync>>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.run_if_owned(|size| sp.size = size)
    })
}


/// Copy em as text size.
pub fn sync_em_text(mut query: Query<(&mut Text, &Dimension), Without<OptOutFontSizeSync>>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        sp.sections.iter_mut().for_each(|x| x.style.font_size = dimension.em)
    })
}

/// Build a transform using [`RotatedRect`].
pub fn build_transform(mut query: Query<(&RotatedRect, &BuildTransform, &mut Transform)>) {
    query.par_iter_mut().for_each(|(rect, BuildTransform(anchor), mut transform)| {
        transform.translation = rect.anchor(*anchor).extend(rect.z);
        transform.rotation = Quat::from_rotation_z(rect.rotation);
        transform.scale = rect.scale.extend(1.0);
    });
}

/// Generate [`GlobalTransform`] with  [`BuildGlobal`].
pub fn finalize(
    mut query: Query<(&BuildGlobal, &Transform2D, &RotatedRect, &mut GlobalTransform)>,
) {
    query.par_iter_mut().for_each(|(build, transform, rect, mut scene)| {
        *scene = Affine3A::from_scale_rotation_translation(
            rect.scale.extend(1.0), 
            Quat::from_rotation_z(rect.rotation), 
            rect.anchor(build.0.or(transform.anchor)).extend(rect.z)
        ).into()
    });
}
