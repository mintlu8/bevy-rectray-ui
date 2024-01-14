use bevy::{reflect::Reflect, ecs::{component::Component, query::WorldQuery}, math::Vec2};

use crate::{Size2, FontSize};

/// Size of the sprite.
///
/// If `Copied` and paired with a component that has a dimension like [`Sprite`](bevy::sprite::Sprite),
/// this will be copied every frame,
/// useful when paired with a dynamic sized item like text or atlas.
///
/// If `Owned` we will try to edit the dimension of the paired sprite
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DimensionType {
    #[default]
    /// Copy `size` from sprite, rect, image, text, etc.
    Copied,
    /// Generated from `Layout` and kept as reference for the next frame.
    Dynamic,
    /// Governs size of sprite, rect, image, text, etc.
    Owned(Size2)
}


/// Controls the dimension of the sprite.
#[derive(Debug, Clone, Component, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dimension {
    /// Input for dimension.
    pub dimension: DimensionType,
    /// Modifies font size `em`.
    pub font_size: FontSize,
    /// If set, always preserves the aspect ratio of the input sprite.
    ///
    /// This will resize the dimension and affect children laid out against
    /// this sprite.
    pub preserve_aspect: bool,
}

/// Runtime evaluated data of a widget's dimension.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DimensionData {
    /// Evaluated size in pixels.
    ///
    /// This value is computed every frame.
    pub size: Vec2,
    /// Aspect ratio of the sprite.
    ///
    /// If paired with a sprite this will be copied.
    pub aspect: f32,
    /// Font size `em` on this sprite.
    ///
    /// This value is computed every frame.
    pub em: f32,
}

/// A mutable query that obtains both [`Dimension`] and [`DimensionData`]
#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct DimensionMut {
    pub source: &'static mut Dimension,
    pub dynamic: &'static mut DimensionData,
}


impl Default for Dimension {
    fn default() -> Self {
        Self {
            dimension: DimensionType::Copied,
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }
}

impl Dimension {

    /// Dimension copied paired components.
    pub const COPIED: Self = Self {
        dimension: DimensionType::Copied,
        font_size: FontSize::None,
        preserve_aspect: false,
    };

    /// Dimension inherited from parent.
    pub const INHERIT: Self = Self {
        dimension: DimensionType::Owned(Size2::FULL),
        font_size: FontSize::None,
        preserve_aspect: false,
    };


    /// Owned dimension in pixels.
    pub const fn pixels(size: Vec2) -> Self {
        Self {
            dimension: DimensionType::Owned(Size2::pixels(size.x, size.y)),
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }

    /// Owned dimension in percentage.
    pub const fn percentage(size: Vec2) -> Self {
        Self {
            dimension: DimensionType::Owned(Size2::percent(size.x, size.y)),
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }

    /// Owned dimension.
    pub const fn owned(size: Size2) -> Self {
        Self {
            dimension: DimensionType::Owned(size),
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }

    /// Add a em modifier.
    pub const fn with_em(self, em: FontSize) -> Self {
        Self {
            dimension: self.dimension,
            font_size: em,
            preserve_aspect: false,
        }
    }

    /// Add preserve aspect.
    pub const fn with_preserve_aspect(self, preserve: bool) -> Self {
        Self {
            dimension: self.dimension,
            font_size: self.font_size,
            preserve_aspect: preserve,
        }
    }

    /// Run a closure with the underlying raw value.
    ///
    /// Has no effect if dimension is not owned.
    pub fn with_raw(&self, f: impl FnOnce(Vec2)) {
        match self.dimension {
            DimensionType::Copied => (),
            DimensionType::Dynamic => (),
            DimensionType::Owned(v) => f(v.raw()),
        }
    }
    /// Edit a contextless underlying value.
    ///
    /// Has no effect if dimension is not owned.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        match &mut self.dimension {
            DimensionType::Copied => (),
            DimensionType::Dynamic => (),
            DimensionType::Owned(v) => v.edit_raw(f),
        }
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.dimension, DimensionType::Owned(..))
    }

    pub fn is_copied(&self) -> bool {
        matches!(self.dimension, DimensionType::Copied)
    }
}

impl DimensionMutReadOnlyItem<'_> {

    pub fn size(&self) -> Vec2 {
        self.dynamic.size
    }

    /// Obtain a contextless underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.source.dimension {
            DimensionType::Copied => self.dynamic.size,
            DimensionType::Dynamic => self.dynamic.size,
            DimensionType::Owned(v) => v.raw(),
        }
    }

    /// Run a function if dimension is owned.
    pub fn run_if_owned(&self, f: impl FnOnce(Vec2)) {
        match self.source.dimension {
            DimensionType::Owned(_) => f(self.dynamic.size),
            _ => (),
        }
    }

    pub fn is_owned(&self) -> bool {
        self.source.is_owned()
    }

    pub fn is_copied(&self) -> bool {
        self.source.is_copied()
    }
}

impl DimensionMutItem<'_> {

    pub fn size(&self) -> Vec2 {
        self.dynamic.size
    }

    /// Updates dimension and returns size and em
    pub fn update(&mut self, parent: Vec2, em: f32, rem: f32) -> (Vec2, f32) {
        let data = &mut self.dynamic;
        data.em = match self.source.font_size{
            FontSize::None => em,
            FontSize::Pixels(v) => v,
            FontSize::Ems(v) => em * v,
            FontSize::Rems(v) => rem * v,
        };
        match self.source.dimension {
            DimensionType::Copied => (data.size, data.em),
            DimensionType::Dynamic => (data.size, data.em),
            DimensionType::Owned(v) if self.source.preserve_aspect => {
                let mut size = v.as_pixels(parent, data.em, rem);
                let current_aspect = size.x / size.y;
                if current_aspect > data.aspect {
                    size.x = size.y * data.aspect
                } else {
                    size.y = size.x / data.aspect
                }
                if !size.is_nan() {
                    data.size = size;
                }
                (data.size, data.em)
            }
            DimensionType::Owned(v) => {
                data.size = v.as_pixels(parent, data.em, rem);
                (data.size, data.em)
            }
        }
    }

    /// Estimate size for a dynamic layout, this notably uses 0 for percentage size.
    pub fn estimate(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        let data = &self.dynamic;
        let em = match self.source.font_size{
            FontSize::None => em,
            FontSize::Pixels(v) => v,
            FontSize::Ems(v) => em * v,
            FontSize::Rems(v) => rem * v,
        };
        match self.source.dimension {
            DimensionType::Copied => data.size,
            DimensionType::Dynamic => data.size,
            DimensionType::Owned(v) if self.source.preserve_aspect => {
                let mut size = v.as_pixels(parent, em, rem);
                let current_aspect = size.x / size.y;
                if current_aspect > data.aspect {
                    size.x = size.y * data.aspect
                } else {
                    size.y = size.x / data.aspect
                }
                if size.is_nan() {
                    return Vec2::ZERO;
                }
                if v.units().0.is_relative() {
                    size.x = 0.0;
                }
                if v.units().0.is_relative() {
                    size.x = 0.0;
                }
                size
            }
            DimensionType::Owned(v) => {
                let mut size = v.as_pixels(parent, em, rem);
                if v.units().0.is_relative() {
                    size.x = 0.0;
                }
                if v.units().0.is_relative() {
                    size.x = 0.0;
                }
                size
            }
        }
    }

    /// Obtain a contextless underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.source.dimension {
            DimensionType::Copied => self.dynamic.size,
            DimensionType::Dynamic => self.dynamic.size,
            DimensionType::Owned(v) => v.raw(),
        }
    }

    /// Edit a contextless underlying value.
    ///
    /// Has no effect if dimension is not owned.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        match &mut self.source.dimension {
            DimensionType::Copied => (),
            DimensionType::Dynamic => (),
            DimensionType::Owned(v) => v.edit_raw(f),
        }
    }

    /// Gain mutable access to the underlying owned vector.
    ///
    /// # Panics
    ///
    /// When dimension is copied.
    #[doc(hidden)]
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        match &mut self.source.dimension {
            DimensionType::Copied => panic!("Cannot get raw of copied value."),
            DimensionType::Dynamic => panic!("Cannot get raw of dynamic value."),
            DimensionType::Owned(v) => v.raw_mut(),
        }
    }

    /// Run a function if dimension is owned.
    pub fn run_if_owned(&self, f: impl FnOnce(Vec2)) {
        match self.source.dimension {
            DimensionType::Owned(_) => f(self.dynamic.size),
            _ => (),
        }
    }

    /// Update size based on a foreign source.
    ///
    /// If `copied`, copy size. If `preserve_aspect`, copy aspect ratio.
    pub fn update_size(&mut self, value: impl FnOnce() -> Vec2) {
        match self.source.dimension {
            DimensionType::Copied => {
                self.dynamic.size = value();
            },
            DimensionType::Owned(_) if self.source.preserve_aspect => {
                let value = value();
                self.dynamic.aspect = value.y / value.x;
            }
            _ => (),
        }
    }

    pub fn is_owned(&self) -> bool {
        self.source.is_owned()
    }

    pub fn is_copied(&self) -> bool {
        self.source.is_copied()
    }
}
