use bevy::{
    prelude::*, 
    sprite::{Sprite, Mesh2dHandle, Material2d}, 
    text::{Text2dBounds, TextLayoutInfo}
};

use crate::{Transform2D, RotatedRect, BuildGlobal, Hitbox, AoUI, BuildTransform, LayoutControl, Dimension, Size2, Opacity, Anchor};


/// The minimal bundle required for AoUI to function.
///
/// Provides DOM propagation but no rendering support.
#[derive(Debug, Default, Bundle)]
pub struct AoUIBundle {
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
}

/// A bundle generating a [`GlobalTransform`] with AoUI.
#[derive(Debug, Default, Bundle)]
pub struct BuildGlobalBundle {
    pub builder: BuildGlobal,
    pub global: GlobalTransform,
}

impl BuildGlobalBundle {
    pub fn new(anchor: Anchor) -> Self{
        Self { 
            builder: BuildGlobal(anchor),
            ..Default::default() 
        }
    }
}

/// A bundle generating a [`Transform`] with AoUI.
/// 
/// Use [`BuildSpacialBundle`] if you need a [`GlobalTransform`].
#[derive(Debug, Default, Bundle)]
pub struct BuildTransformBundle {
    pub builder: BuildTransform,
    pub transform: Transform,
}

impl BuildTransformBundle {
    pub fn at_anchor(anchor: Anchor) -> Self{
        Self { 
            builder: BuildTransform(anchor),
            ..Default::default() 
        }
    }
}

/// A bundle generating a [`Transform`] and a [`GlobalTransform`] by proxy.
#[derive(Debug, Default, Bundle)]
pub struct BuildSpacialBundle {
    pub builder: BuildTransform,
    pub transform: Transform,
    pub global: GlobalTransform,
}

impl BuildSpacialBundle {
    pub fn at_anchor(anchor: Anchor) -> Self{
        Self { 
            builder: BuildTransform(anchor),
            ..Default::default() 
        }
    }
}


/// A bundle that breaks a multiline [`Container`](crate::Container) 
/// in place without taking up space.
#[derive(Debug, Bundle)]
pub struct LinebreakBundle {
    bundle: AoUIBundle,
    control: LayoutControl,
}


impl LinebreakBundle {
    pub fn new(size: impl Into<Size2>) -> Self{
        Self {
            bundle: AoUIBundle { 
                dimension: Dimension {
                    dim: crate::DimensionSize::Owned(size.into()),
                    ..Default::default()
                }, 
                ..Default::default()
            },
            control: LayoutControl::LinebreakMarker,
        }
    }

    pub fn ems(size: Vec2) -> Self{
        Self {
            bundle: AoUIBundle { 
                dimension: Dimension {
                    dim: crate::DimensionSize::Owned(Size2::em(size.x, size.y)),
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

/// The AoUI version of [`SpriteBundle`](https://docs.rs/bevy/latest/bevy/sprite/struct.SpriteBundle.html)
#[cfg(feature="bundles")]
#[derive(Debug, Default, Bundle)]
pub struct AoUISpriteBundle {
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub build: BuildGlobal,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}

/// The AoUI version of [`Text2dBundle`](https://docs.rs/bevy/latest/bevy/prelude/struct.Text2dBundle.html)
#[cfg(feature="bundles")]
#[derive(Debug, Default, Bundle)]
pub struct AoUITextBundle {
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub build: BuildGlobal,
    pub hitbox: Hitbox,
    pub text: Text,
    pub text_anchor: bevy::sprite::Anchor,
    pub text_bounds: Text2dBounds,
    pub text_layout: TextLayoutInfo,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}


/// The AoUI version of [`MaterialMesh2dBundle`](https://docs.rs/bevy/latest/bevy/prelude/struct.Text2dBundle.html)
#[cfg(feature="bundles")]
#[derive(Debug, Default, Bundle)]
pub struct AoUIMaterialMesh2dBundle<M: Material2d>{
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub build: BuildTransform,
    pub screen: BuildGlobal,
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
    pub opacity: Opacity,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
}