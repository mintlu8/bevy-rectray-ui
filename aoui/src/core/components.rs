use bevy::{prelude::*, reflect::Reflect, math::Affine2};

use crate::{Size2, FontSize, Anchor, SizeUnit};

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
    pub set_em: FontSize,
    /// If set, always preserves the aspect ratio of the input sprite.
    /// 
    /// This will resize the dimension and affect children laid out against
    /// this sprite.
    pub preserve_aspect: bool,
    /// Evaluated size in pixels.
    ///     
    /// This value is computed every frame. 
    #[cfg_attr(feature="serde", serde(skip))]
    pub size: Vec2,
    /// A reliable min for '%' base dimensions.
    ///     
    /// This value is set by copied sources and dimension agnostic layouts.
    /// 
    /// If not set, the size would be 0 during the layout phase.
    #[cfg_attr(feature="serde", serde(skip))]
    pub reliable_size: Vec2,
    /// Aspect ratio of the sprite.
    /// 
    /// If paired with a sprite this will be copied.
    #[cfg_attr(feature="serde", serde(skip))]
    pub aspect: f32,
    /// Font size `em` on this sprite.
    /// 
    /// This value is computed every frame. 
    #[cfg_attr(feature="serde", serde(skip))]
    pub em: f32,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            dim: DimensionSize::Copied,
            set_em: FontSize::None,
            size: Vec2::ZERO,
            reliable_size: Vec2::ZERO,
            preserve_aspect: false,
            aspect: 1.0,
            em: 0.0,
        }
    }
}

impl Dimension {

    /// Dimension copied paired components.
    pub const COPIED: Self = Self {
        dim: DimensionSize::Copied,
        set_em: FontSize::None,
        size: Vec2::ZERO,
        reliable_size: Vec2::ZERO,
        preserve_aspect: false,
        aspect: 1.0,
        em: 0.0,
    };

    /// Dimension inherited from parent.
    pub const INHERIT: Self = Self {
        dim: DimensionSize::Owned(Size2::FULL),
        set_em: FontSize::None,
        size: Vec2::ZERO,
        reliable_size: Vec2::ZERO,
        preserve_aspect: false,
        aspect: 1.0,
        em: 0.0,
    };


    /// Owned dimension in pixels.
    pub const fn pixels(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::pixels(size.x, size.y)),
            set_em: FontSize::None,
            size: Vec2::ZERO,
            reliable_size: Vec2::ZERO,
            preserve_aspect: false,
            aspect: 1.0,
            em: 0.0,
        }
    }

    /// Owned dimension in percentage.
    pub const fn percentage(size: Vec2) -> Self {
        Self {
            dim: DimensionSize::Owned(Size2::percent(size.x, size.y)),
            set_em: FontSize::None,
            size: Vec2::ZERO,
            reliable_size: Vec2::ZERO,
            preserve_aspect: false,
            aspect: 1.0,
            em: 0.0,
        }
    }

    /// Owned dimension.
    pub const fn owned(size: Size2) -> Self {
        Self {
            dim: DimensionSize::Owned(size),
            set_em: FontSize::None,
            size: Vec2::ZERO,
            reliable_size: Vec2::ZERO,
            preserve_aspect: false,
            aspect: 1.0,
            em: 0.0,
        }
    }

    /// Add a em modifier.
    pub const fn with_em(self, em: FontSize) -> Self {
        Self {
            dim: self.dim,
            set_em: em,
            size: self.size,
            reliable_size: Vec2::ZERO,
            preserve_aspect: false,
            aspect: 1.0,
            em: self.em,
        }
    }

    /// Set aspect, and set preserve_aspect to true.
    pub const fn with_aspect(self, aspect: f32) -> Self {
        Self {
            dim: self.dim,
            set_em: self.set_em,
            size: self.size,
            reliable_size: Vec2::ZERO,
            preserve_aspect: true,
            aspect,
            em: self.em,
        }
    }

    /// Add preserve aspect.
    pub const fn with_preserve_aspect(self, preserve: bool) -> Self {
        Self {
            dim: self.dim,
            set_em: self.set_em,
            size: self.size,
            reliable_size: Vec2::ZERO,
            preserve_aspect: preserve,
            aspect: 1.0,
            em: self.em,
        }
    }

    /// Updates dimension and returns size and em
    pub fn update(&mut self, parent: Vec2, em: f32, rem: f32) -> (Vec2, f32) {
        self.em = match self.set_em{
            FontSize::None => em,
            FontSize::Pixels(v) => v,
            FontSize::Ems(v) => em * v,
            FontSize::Rems(v) => rem * v,
        };
        match self.dim {
            DimensionSize::Copied => (self.size, self.em),
            DimensionSize::Owned(v) if self.preserve_aspect => {
                let mut size = v.as_pixels(parent, self.em, rem);
                let current_aspect = size.x / size.y;
                if current_aspect > self.aspect {
                    size.x = size.y * self.aspect
                } else {
                    size.y = size.x / self.aspect
                }
                if !size.is_nan() {
                    self.size = size;
                }
                (self.size, self.em)
            }
            DimensionSize::Owned(v) => {
                self.size = v.as_pixels(parent, self.em, rem);
                (self.size, self.em)
            }
        }
    }

    /// Estimate size for dynamic layout, this notably does not use non-canon values.
    pub fn estimate(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        let em = match self.set_em{
            FontSize::None => em,
            FontSize::Pixels(v) => v,
            FontSize::Ems(v) => em * v,
            FontSize::Rems(v) => rem * v,
        };
        match self.dim {
            DimensionSize::Copied => self.size,
            DimensionSize::Owned(v) if self.preserve_aspect => {
                let mut size = v.as_pixels(parent, em, rem);
                let current_aspect = size.x / size.y;
                if current_aspect > self.aspect {
                    size.x = size.y * self.aspect
                } else {
                    size.y = size.x / self.aspect
                }
                if size.is_nan() {
                    return Vec2::ZERO;
                }
                if v.units().0 == SizeUnit::Percent {
                    size.x = self.reliable_size.x;
                }
                if v.units().0 == SizeUnit::Percent {
                    size.x = self.reliable_size.x;
                }
                size
            }
            DimensionSize::Owned(v) => {
                let mut size = v.as_pixels(parent, em, rem);
                if v.units().0 == SizeUnit::Percent {
                    size.x = self.reliable_size.x;
                }
                if v.units().0 == SizeUnit::Percent {
                    size.x = self.reliable_size.x;
                }
                size
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

    /// If `copied`, copy size. If `preserve_aspect`, copy aspect ratio.
    pub fn update_size(&mut self, value: impl FnOnce() -> Vec2) {
        match self.dim {
            DimensionSize::Copied => {
                self.size = value();
                self.reliable_size = self.size;
            },
            DimensionSize::Owned(_) if self.preserve_aspect => {
                let value = value();
                self.aspect = value.y / value.x;
            }
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

/// The 2D transform component for Aoui
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
    /// By default and in idiomatic use, this is the same as `anchor`.
    /// 
    /// Unless doing skeletal animations,
    /// try avoid using this field if possible.
    pub parent_anchor: Anchor,
    /// Center of `rotation` and `scale`.
    ///
    /// By default this is `Center`, 
    /// If set to `Inherit`, would be the same as `anchor`.
    pub center: Anchor,
    /// Offset from parent's anchor.
    pub offset: Size2,
    /// Z depth, if set, this is `parent_z + z`.
    /// If not set, this is `parent_z.next_after()`.
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
        parent_anchor: Anchor::Center,
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
    /// This is discouraged in idiomatic use.
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

/// Builds a `GlobalTransform` on a `Anchor`, by default `Transform2D::anchor`.
#[derive(Debug, Clone, Component, Reflect)]
pub struct BuildTransform(pub Anchor);

impl Default for BuildTransform {
    fn default() -> Self {
        Self(Anchor::Inherit)
    }
}


/// Builds a `GlobalTransform` for `Mesh2d`, 
/// this always uses `Anchor::Center` and converts dimension to scale.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct BuildMeshTransform;


/// Stores opacity of the widget, not used by default but
/// can be used by implementors.
#[derive(Debug, Clone, Component, Reflect)]
pub struct Opacity {
    /// User specified opacity of the widget.
    pub opacity: f32,
    /// Computed opacity of the widget.
    pub computed_opacity: f32,
    /// Occluded
    pub occluded: bool,
    /// Disabled
    pub disabled: bool,
    /// Propagated disabled value.
    pub computed_disabled: bool,
}

impl Opacity {
    /// Fully opaque.
    pub const OPAQUE: Self = Self {
        opacity: 1.0,
        computed_opacity: 1.0,
        disabled: false,
        occluded: false,
        computed_disabled: false,
    };
    /// Fully transparent.
    pub const TRANSPARENT: Self = Self {
        opacity: 0.0,
        computed_opacity: 0.0,
        disabled: false,
        occluded: false,
        computed_disabled: false,
    };
    /// Create opacity from a value.
    pub const fn new(v: f32) -> Self {
        Self {
            opacity: v,
            computed_opacity: v,
            disabled: false,
            occluded: false,
            computed_disabled: false,
        }
    }

    /// If not, the event pipeline will ignore this entity.
    pub fn is_active(&self) -> bool {
        !self.computed_disabled && !self.occluded && !self.disabled && self.computed_opacity > 0.0
    }

    pub fn is_disabled(&self) -> bool {
        self.computed_disabled
    }

    pub fn get(&self) -> f32 {
        if self.occluded {
            0.0
        } else {
            self.computed_opacity
        }
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Self::OPAQUE
    }
}

/// Writes opacity to the associated alpha value of sprite, text, etc.
/// 
/// This behavior is opt-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
pub struct SetAlpha;

/// Data related to clipping.
#[derive(Debug, Component, Default)]
pub struct Clipping {
    /// If set, use this sprite's bounding rectangle to clip its children.
    /// 
    /// This currently only affect events, you need `clipping_layer` for
    /// render clipping. This might change in the future.
    pub clip: bool,
    /// Global space clipping, is the inverse of some parent's `RotatedRect`.
    /// 
    /// This occludes cursor events.
    pub global: Option<Affine2>,
    /// Local space clipping, between `0..=1`.
    /// 
    /// Experimental, unused currently.
    pub local: Option<Rect>,
}

impl Clipping {
    pub fn new(clip: bool) -> Self {
        Clipping {
            clip,
            global: None,
            local: None,
        }
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        match self.global {
            Some(affine) => {
                let vec = affine.transform_point2(pos);
                vec.x.abs() <= 0.5 && vec.y.abs() <= 0.5
            }
            None => true,
        }
    }
}
