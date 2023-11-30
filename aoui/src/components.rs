use bevy::{prelude::*, reflect::Reflect};

use crate::{Size2, SetEM, Anchor};

/// Marker component for the default schedules.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AoUI;

/// Size of the sprite.
///
/// If `Copied` and paired with a component that has a dimension like [`Sprite`],
/// this will be copied every frame,
/// useful when paired with a dynamic sized item like text or atlas.
/// 
/// If `Owned` we will try to edit the dimension of the paired sprite
#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DimensionSize {
    /// Copy `size` from sprite, rect, image, text, etc.
    Copied,
    /// Governs size of sprite, rect, image, text, etc.
    Owned(Size2)
}

/// Controls the dimension of the sprite
#[derive(Debug, Clone, Component, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dimension {
    /// Input for dimension.
    pub dim: DimensionSize,
    /// Modifies font size `em`.
    pub set_em: SetEM,
    /// Evaluated size in pixels.
    ///     
    /// This value is computed every frame. 
    #[cfg_attr(feature="serde", serde(default))]
    pub size: Vec2,
    /// Font size `em` on this sprite.
    /// 
    /// This value is computed every frame. 
    #[cfg_attr(feature="serde", serde(default))]
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

    /// Dimension copied paired components.
    pub const COPIED: Self = Self {
        dim: DimensionSize::Copied,
        set_em: SetEM::None,
        size: Vec2::ZERO,
        em: 16.0,
    };

    /// Dimension inherited from parent.
    pub const INHERIT: Self = Self {
        dim: DimensionSize::Owned(Size2::INHERIT),
        set_em: SetEM::None,
        size: Vec2::ZERO,
        em: 0.0,
    };


    /// Owned dimension in pixels.
    pub const fn pixels(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::pixels(size.x, size.y)),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 0.0,
        }
    }

    /// Owned dimension in percentage.
    pub const fn percentage(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::percent(size.x, size.y)),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 0.0,
        }
    }

    /// Owned dimension.
    pub const fn owned(size: Size2) -> Self {
        Self {
            dim: DimensionSize::Owned(size),
            set_em: SetEM::None,
            size: Vec2::ZERO,
            em: 0.0,
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
            DimensionSize::Owned(v) => {
                self.size = v.as_pixels(parent, self.em, rem);
                (self.size, self.em)
            }
        }
    }

    /// Obtain a contextless underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.dim {
            DimensionSize::Copied => self.size,
            DimensionSize::Owned(v) => v.raw(),
        }
    }

    /// Edit a contextless underlying value.
    /// 
    /// Has no effect if dimension is not owned.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        match &mut self.dim {
            DimensionSize::Copied => (),
            DimensionSize::Owned(v) => v.edit_raw(f),
        }
    }

    /// Gain mutable access to the underlying owned value.
    /// 
    /// For ease of use with egui.
    #[doc(hidden)]
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        match &mut self.dim {
            DimensionSize::Copied => panic!("Cannot get raw of copied value."),
            DimensionSize::Owned(v) => v.raw_mut(),
        }
    }
    
    /// Run a function if dimension is owned.
    pub fn run_if_owned(&self, f: impl FnOnce(Vec2)) {
        match self.dim {
            DimensionSize::Owned(_) => f(self.size),
            _ => (),
        }
    }

    /// Update by a copied value.
    pub fn update_copied(&mut self, value: impl FnOnce() -> Vec2) {
        match self.dim {
            DimensionSize::Copied => self.size = value(),
            _ => (),
        }
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.dim, DimensionSize::Owned(..))
    }

    pub fn is_copied(&self) -> bool {
        matches!(self.dim, DimensionSize::Copied)
    }
}

/// The 2D transform component for AoUI
#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct Transform2D{
    /// The sprite's offset, as well as
    /// parent rotation and parent scale
    /// are applied through this point.
    ///
    /// This always overwrites the `anchor` field in standard bevy components,
    /// and should ideally work the same way for third party implementations.
    pub anchor: Anchor,
    /// The anchor matched on the parent side.
    ///
    /// By default this is the same as `anchor`.
    /// 
    /// Unless doing skeletal animations,
    /// try avoid using this field in idiomatic usage of `AoUI`.
    pub parent_anchor: Anchor,
    /// Center of `rotation` and `scale`.
    ///
    /// By default this is the same as `anchor`.
    pub center: Anchor,
    /// Offset from parent's anchor.
    pub offset: Size2,
    /// Z depth, by default, this is `parent_z + z + eps * 8`
    pub z: f32,
    /// Rotation around `center`.
    pub rotation: f32,
    /// Scaling around `center`.
    pub scale: Vec2,
}

impl Transform2D {

    pub fn get_center(&self) -> Anchor{
        self.center.or(self.anchor)
    }

    pub fn get_parent_anchor(&self) -> Anchor{
        self.parent_anchor.or(self.anchor)
    }

    pub const UNIT: Self = Self {
        anchor: Anchor::Center,
        parent_anchor: Anchor::Inherit,
        center: Anchor::Inherit,
        offset: Size2::ZERO,
        rotation: 0.0,
        z: 0.0,
        scale: Vec2::ONE,
    };

    /// Set offset.
    pub fn with_offset(mut self, offset: impl Into<Size2>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Set rotation.
    pub fn with_rotation(mut self, rot: f32) -> Self {
        self.rotation = rot;
        self
    }

    /// Set scale.
    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    /// Set z offset.
    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Set anchor.
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Set parent anchor.
    ///  
    /// Discouraged in idiomatic use, unless doing skeletal animation.
    pub fn with_parent_anchor(mut self, anchor: Anchor) -> Self {
        self.parent_anchor = anchor;
        self
    }

    /// Set center.
    pub fn with_center(mut self, center: Anchor) -> Self {
        self.center = center;
        self
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::UNIT
    }
}

/// Builds a `GlobalTransform` on a `Anchor`, by default `Transform2d::anchor`.
#[derive(Debug, Clone, Component, Reflect)]
pub struct BuildGlobal(pub Anchor);

impl Default for BuildGlobal {
    fn default() -> Self {
        Self(Anchor::Inherit)
    }
}
/// Builds a `Transform` on a `Anchor`, by default `center`.
/// 
/// If `GlobalTransform` is present and [`BuildGlobal`] is not,
/// the output will be used for rendering.
/// 
/// If `BuildGlobal` is preset, this component 
/// have no effect on rendering.
#[derive(Debug, Clone, Component, Reflect)]
pub struct BuildTransform(pub Anchor);

impl Default for BuildTransform {
    fn default() -> Self {
        Self(Anchor::Inherit)
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Opacity {
    pub opactity: f32,
    pub computed: f32
}

impl Opacity {
    pub const FULL: Self = Self {
        opactity: 1.0,
        computed: 1.0,
    };
    pub const TRANSPARENT: Self = Self {
        opactity: 0.0,
        computed: 0.0,
    };
    pub const fn new(v: f32) -> Self {
        Self {
            opactity: v,
            computed: v,
        }
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Self::FULL
    }
}