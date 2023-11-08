use bevy::{prelude::*, sprite::Anchor, reflect::Reflect, math::Affine3A};

use crate::{Size2, SetEM};

/// Marker component for our rendering.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AoUI;

/// Size of the sprite.
///
/// If `Copied` and paired with a component that has a dimension like [`Sprite`],
/// this will be copied every frame.
/// 
/// This is useful when paired with a dynamic sized item like text or atlas.
/// 
/// If `Scaled`, acts as `Copied` but scales the copied dimension without 
/// modifying the scale of the sprite.
/// 
/// If `Owned` we will try to edit the dimension of the paired sprite
#[derive(Debug, Clone, Reflect)]
pub enum DimensionSize {
    /// Copy `size` from sprite, rect, image, text, etc.
    Copied,
    /// Copy and scale `size` from sprite, rect, image, text, etc.
    Scaled(Vec2),
    /// Governs size of sprite, rect, image, text, etc.
    Owned(Size2)
}

/// Controls the dimension, absolute or relative, of the sprite
#[derive(Debug, Clone, Component, Reflect)]
pub struct Dimension {
    /// Input for dimension.
    pub dim: DimensionSize,
    /// Modifies the relative size `em`.
    pub set_em: SetEM,
    /// Evaluated size in pixels.
    ///     
    /// This value is computed every frame. 
    pub size: Vec2,
    /// Relative size `em` on this sprite.
    /// 
    /// This value is computed every frame. 
    /// 
    /// By default `16`.
    pub em: f32,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            dim: DimensionSize::Copied,
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 16.0,
        }
    }
}

impl Dimension {

    /// Dimension copied from the likes of [`Sprite`], [`Image`] or [`TextLayoutInfo`](bevy::text::TextLayoutInfo).
    pub const COPIED: Self = Self {
        dim: DimensionSize::Copied,
        set_em: SetEM::None,
        size: Vec2::ZERO,
        em: 16.0,
    };

    pub const INHERIT: Self = Self {
        dim: DimensionSize::Owned(Size2::INHERIT),
        set_em: SetEM::None,
        size: Vec2::ZERO,
        em: 16.0,
    };


    /// Owned dimension in pixels.
    pub const fn pixels(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::pixels(size.x, size.y)),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 16.0,
        }
    }

    /// Owned dimension in pixels.
    pub const fn percentage(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::percent(size.x, size.y)),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 16.0,
        }
    }

    /// Owned dimension in relative size.
    pub const fn owned(size: Size2) -> Self {
        Self {
            dim: DimensionSize::Owned(size),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 16.0,
        }
    }

    /// Add a em modifier.
    pub const fn with_em(self, em: SetEM) -> Self {
        Self {
            dim: self.dim,
            set_em: em,
            size: self.size,
            em: self.em,
        }
    }

    /// Updates dimension and returns size and em
    pub fn update(&mut self, parent: Vec2, em: f32, rem: f32) -> (Vec2, f32) {
        self.em = match self.set_em{
            SetEM::None => em,
            SetEM::Pixels(v) => v,
            SetEM::Ems(v) => em * v,
            SetEM::Rems(v) => rem * v,
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

    /// Obtain a context-less underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.dim {
            DimensionSize::Copied => self.size,
            DimensionSize::Scaled(_) => self.size,
            DimensionSize::Owned(v) => v.raw(),
        }
    }

    /// Get mutable access to the underlying owned value.
    /// 
    /// For ease of use with egui.
    #[doc(hidden)]
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        match &mut self.dim {
            DimensionSize::Copied => panic!("Cannot get raw of copied value."),
            DimensionSize::Scaled(_) => panic!("Cannot get raw of copied value."),
            DimensionSize::Owned(v) => v.raw_mut(),
        }
    }
    
    /// Run a function if dimension is owned.
    pub fn spawn_owned(&self, f: impl FnOnce(Vec2)) {
        match self.dim {
            DimensionSize::Owned(_) => f(self.size),
            _ => (),
        }
    }

    /// Update by a copied value.
    pub fn update_copied(&mut self, value: impl FnOnce() -> Vec2) {
        match self.dim {
            DimensionSize::Copied => self.size = value(),
            DimensionSize::Scaled(scale) => self.size = value() * scale,
            _ => (),
        }
    }
}

/// The 2D transform component for AoUI
#[derive(Debug, Clone, Component, Reflect)]
pub struct Transform2D{
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
    /// Offset from parent's anchor.
    pub offset: Size2,
    /// Z depth, by default, this is `parent_z + z + eps * 8`
    pub z: f32,
    /// Rotation around [`center`].
    pub rotation: f32,
    /// Scaling around [`center`].
    pub scale: Vec2,
}

/// An intermediate screen space transform output
/// centered on `(0,0)` and scaled to window size.
/// 
/// ScreenSpaceTransform will build a `GlobalTransform` on `anchor`.
/// If **unset** and `BuildTransform` is present, a transform is built
/// on `center` by bevy's own pipeline. This can be useful for integrating with
/// [`Mesh`](bevy::prelude::Mesh) or third party crates like
/// [`bevy_prototype_lyon::Geometry`](https://docs.rs/bevy_prototype_lyon/latest/bevy_prototype_lyon/geometry/trait.Geometry.html)
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ScreenSpaceTransform(pub Affine3A);

impl Transform2D {

    pub fn get_center(&self) -> &Anchor{
        match &self.center {
            Some(center) => center,
            None => &self.anchor,
        }
    }

    pub const DEFAULT: Self = Self {
        anchor: Anchor::Center,
        center: None, 
        offset: Size2::ZERO,
        rotation: 0.0,
        z: 0.0,
        scale: Vec2::ONE,
    };

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

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_center(mut self, center: Anchor) -> Self {
        self.center = Some(center);
        self
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Generate a [`Transform`] component at a custom anchor for intergration with bevy.
/// 
/// If [`GlobalTransform`] is present and [`ScreenSpaceTransform`] is not,
/// the output will be used for self rendering.
/// 
/// If `ScreenSpaceTransform` is preset, this component will have no effect
/// on self rendering.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct BuildTransform(pub Anchor);

