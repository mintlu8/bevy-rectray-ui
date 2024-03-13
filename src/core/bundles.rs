//! Bundles mapping the features of `bevy_2d`.
//!
//! The implementations here mimics the behavior of bevy
//! and not necessarily the same of their corresponding widget builder.
#![allow(missing_docs)]
use bevy::{
    prelude::*,
    sprite::{Mesh2dHandle, Material2d},
    text::{Text2dBounds, TextLayoutInfo}
};

use crate::{Transform2D, RotatedRect, BuildTransform, Hitbox, layout::LayoutControl, Size2, Opacity, Anchor, Clipping, DimensionData, Dimension, Coloring};


/// The minimal bundle required for bevy_rectray's pipeline to function.
///
/// Provides transform propagation but no rendering support.
#[derive(Debug, Default, Bundle)]
pub struct RectrayBundle {
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub dimension_data: DimensionData,
    pub control: LayoutControl,
    pub rect: RotatedRect,
    pub clipping: Clipping,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
}

impl RectrayBundle {
    pub fn empty(anchor: Anchor, size: impl Into<Size2>) -> Self{
        RectrayBundle {
            transform: Transform2D::UNIT.with_anchor(anchor),
            dimension: Dimension::owned(size.into()),
            ..Default::default()
        }
    }
}

/// A bundle generating a [`GlobalTransform`] with Aoui.
#[derive(Debug, Default, Bundle)]
pub struct BuildTransformBundle {
    pub builder: BuildTransform,
    pub global: GlobalTransform,
}

impl BuildTransformBundle {
    pub fn new(anchor: Anchor) -> Self{
        Self {
            builder: BuildTransform(anchor),
            ..Default::default()
        }
    }
}

/// A bundle that breaks a multiline [`Container`](crate::layout::Container)
/// in place without taking up space.
#[derive(Debug, Bundle)]
pub struct LinebreakBundle {
    bundle: RectrayBundle,
    control: LayoutControl,
}


impl LinebreakBundle {
    pub fn new(size: impl Into<Size2>) -> Self{
        Self {
            bundle: RectrayBundle {
                dimension: Dimension {
                    dimension: crate::DimensionType::Owned(size.into()),
                    ..Default::default()
                },
                ..Default::default()
            },
            control: LayoutControl::LinebreakMarker,
        }
    }

    pub fn ems(size: Vec2) -> Self{
        Self {
            bundle: RectrayBundle {
                dimension: Dimension {
                    dimension: crate::DimensionType::Owned(Size2::em(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            },
            control: LayoutControl::LinebreakMarker,
        }
    }
}

impl Default for LinebreakBundle {
    fn default() -> Self {
        Self::new(Vec2::default())
    }
}

/// The Rectray version of [`SpriteBundle`](bevy::sprite::SpriteBundle)
#[derive(Debug, Default, Bundle)]
pub struct RSpriteBundle {
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub dimension_data: DimensionData,
    pub control: LayoutControl,
    pub rect: RotatedRect,
    pub build: BuildTransform,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub clipping: Clipping,
    pub color: Coloring,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}

impl RSpriteBundle {
    fn with_atlas(self, atlas: TextureAtlas) -> impl Bundle {
        (self, atlas)
    }

    fn with_slice(self, slice: TextureSlicer) -> impl Bundle {
        (self, ImageScaleMode::Sliced(slice))
    }

    fn with_scale_mode(self, scale: ImageScaleMode) -> impl Bundle {
        (self, scale)
    }

    fn with_atlas_scale(self, atlas: TextureAtlas, slice: ImageScaleMode) -> impl Bundle {
        (self, atlas, slice)
    }
}


/// The Aoui version of [`Text2dBundle`](bevy::text::Text2dBundle)
#[derive(Debug, Default, Bundle)]
pub struct RTextBundle {
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub dimension_data: DimensionData,
    pub control: LayoutControl,
    pub rect: RotatedRect,
    pub build: BuildTransform,
    pub hitbox: Hitbox,
    pub text: Text,
    pub text_anchor: bevy::sprite::Anchor,
    pub text_bounds: Text2dBounds,
    pub text_layout: TextLayoutInfo,
    pub clipping: Clipping,
    pub color: Coloring,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}


/// The Aoui version of [`MaterialMesh2dBundle`](bevy::sprite::MaterialMesh2dBundle)
#[derive(Debug, Default, Bundle)]
pub struct RMesh2dBundle<M: Material2d>{
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub dimension_data: DimensionData,
    pub control: LayoutControl,
    pub rect: RotatedRect,
    pub build: BuildTransform,
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
    pub clipping: Clipping,
    pub color: Coloring,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}
