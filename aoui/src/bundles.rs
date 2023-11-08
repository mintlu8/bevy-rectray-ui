use bevy::{
    prelude::*, 
    sprite::{Sprite, Anchor, Mesh2dHandle, Material2d}, 
    text::{Text2dBounds, TextLayoutInfo}
};

use crate::{Transform2D, RotatedRect, ScreenSpaceTransform, Hitbox, AoUI, BuildTransform, LayoutControl, Dimension, Size2};


/// The minimal bundle required for AoUI to function.
///
/// This provides propagation but no rendering support.
#[derive(Debug, Default, Bundle)]
pub struct AoUIBundle {
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub vis: VisibilityBundle,
}

/// A bundle that receives transform results produced by AoUI.
#[derive(Debug, Default, Bundle)]
pub struct AoUIOutputBundle {
    pub screen: ScreenSpaceTransform,
    pub global: GlobalTransform,
}

/// A bundle for integration with native bevy components that rely on [`Transform`].
///
/// Notably this is needed to have native bevy children.
#[derive(Debug, Default, Bundle)]
pub struct BuildTransformBundle {
    pub builder: BuildTransform,
    pub transform: Transform,
}

/// A bundle that breaks a flexbox in place without taking up space.
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
    pub screen: ScreenSpaceTransform,
    pub sprite: Sprite,
    pub vis: VisibilityBundle,
    pub global: GlobalTransform,
    pub texture: Handle<Image>,
}

/// The AoUI version of [`Text2dBundle`](https://docs.rs/bevy/latest/bevy/prelude/struct.Text2dBundle.html)
#[cfg(feature="bundles")]
#[derive(Debug, Default, Bundle)]
pub struct AoUITextBundle {
    pub core: AoUI,
    pub transform: Transform2D,
    pub dimension: Dimension,
    pub rect: RotatedRect,
    pub screen: ScreenSpaceTransform,
    pub hitbox: Hitbox,
    pub text: Text,
    pub text_anchor: Anchor,
    pub text_bounds: Text2dBounds,
    pub text_layout: TextLayoutInfo,
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
    pub screen: ScreenSpaceTransform,
    pub mesh: Mesh2dHandle,
    pub material: Handle<M>,
    pub vis: VisibilityBundle,
    pub global_transform: GlobalTransform,
}