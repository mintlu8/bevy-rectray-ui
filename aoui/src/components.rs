use bevy::{prelude::*, sprite::Anchor, reflect::Reflect, math::Affine3A};

use crate::{Size2, SetEM};

/// Marker component for our rendering.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AoUI;

/// Properties usually automatically synchronized from bevy's native components.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct Anchors{
    /// Governs the rotation and scale specified in [`Transform2D`].
    /// This also serves as the [`Transform`] center is needed.
    ///
    /// By default this is the same as [`anchor`].
    pub center: Option<Anchor>,
    /// Where the sprite is parented to.
    /// Offset, parent rotation and parent scale
    /// are applied through this point.
    ///
    /// This always overwrites the `anchor` field in standard bevy components.
    pub anchor: Anchor,
}

/// Size of the sprite.
///
/// If `Copied` and paired with a component that has a dimension like [`Sprite`],
/// this will be copied every frame.
/// 
/// This is useful when paired with a dynamic sized item like text or atlas.
/// 
/// If `Scaled`, this acts as `Copied` but scales the copied dimension without 
/// modifying the scale of the sprite.
/// 
/// If `Owned` we will try to edit the dimension of the paired sprite
#[derive(Debug, Clone, Reflect)]
pub enum DimensionSize {
    Copied,
    Scaled(Vec2),
    Owned(Size2)
}

/// Marker component for multiplying dimension to scale.
/// 
/// This is useful in rendering 2D mesh.
#[derive(Debug, Clone, Component, Default, Reflect)]
#[non_exhaustive]
pub struct DimensionAsScale;

/// Controls the dimension, absolute or relative, of the sprite
#[derive(Debug, Clone, Component, Reflect)]
pub struct Dimension {
    /// Input for dimension.
    pub dim: DimensionSize,
    /// Modifies the relative size em.
    pub set_em: SetEM,
    /// Evaluated size.
    /// 
    /// Should be initialized if known, otherwise init with zero.
    pub size: Vec2,
    /// Relative size, computed every frame. 
    /// 
    /// By default `(16, 16)`
    pub em: Vec2,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            dim: DimensionSize::Copied,
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: Vec2::new(16.0, 16.0),
        }
    }
}

impl Dimension {

    pub fn pixels(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(size.into()),
            size,
            ..Default::default()
        }
    }

    pub const fn copied() -> Self {
        Self {
            dim: DimensionSize::Copied,
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: Vec2::ZERO,
        }
    }

    pub const fn owned(size: Size2) -> Self {
        Self {
            dim: DimensionSize::Owned(size),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: Vec2::ZERO,
        }
    }

    /// Updates dimension and returns size and em
    pub fn update(&mut self, parent: Vec2, em: Vec2, rem: Vec2) -> (Vec2, Vec2) {
        self.em = match self.set_em{
            SetEM::None => em,
            SetEM::Pixels(v) => v,
            SetEM::Scale(v) => em * v,
            SetEM::ScaleRem(v) => rem * v,
        };
        match self.dim {
            DimensionSize::Copied => (self.size, self.em),
            DimensionSize::Scaled(_) => (self.size, self.em),
            DimensionSize::Owned(v) => {
                self.size = v.as_pixels(parent, em, rem);
                (self.size, self.em)
            }
        }
    }

    pub fn raw(&self) -> Vec2 {
        match &self.dim {
            DimensionSize::Copied => self.size,
            DimensionSize::Scaled(_) => self.size,
            DimensionSize::Owned(v) => v.raw(),
        }
    }
}

impl Anchors {
    pub fn get_center(&self) -> &Anchor{
        match &self.center {
            Some(center) => center,
            None => &self.anchor,
        }
    }
}

/// A 2D transform component for AoUI
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Transform2D{
    /// Offset from parent's anchor.
    pub offset: Size2,
    /// Z depth, by default, this is `parent_z + z + eps`
    pub z: f32,
    /// Rotation around [`center`].
    pub rotation: f32,
    /// Scaling around [`center`].
    pub scale: Vec2,
}

/// An intermediate screen space transform output
/// centered on (0,0) and scaled to window size.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ScreenSpaceTransform(pub Affine3A);

impl Transform2D {

    pub const DEFAULT: Self = Self {
        offset: Size2::ZERO,
        rotation: 0.0,
        z: 0.0,
        scale: Vec2::ONE,
    };

    pub fn new(offset: impl Into<Size2>) -> Self {
        Self { offset: offset.into(), rotation: 0.0, scale: Vec2::ONE, z: 0.0 }
    }

    pub fn with_offset(mut self, offset: impl Into<Size2>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn with_rotation(mut self, rot: f32) -> Self {
        self.rotation = rot;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self { offset: Size2::ZERO, rotation: 0.0, scale: Vec2::ONE, z: 0.0 }
    }
}

impl Anchors {
    pub const fn inherit(anchor: Anchor) -> Self{
        Self {
            center: None,
            anchor,
        }
    }

    pub const fn new(anchor: Anchor, center: Anchor) -> Self{
        Self {
            center: Some(center),
            anchor,
        }
    }
}


/// Generate a transform component for intergration with bevy.
///
/// The generated transform uses computed [`center`](Anchors::center) as its translation
/// and does not affect the actual transform of the sprite.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct BuildTransform;

