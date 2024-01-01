use bevy::{reflect::Reflect, ecs::{component::Component, query::WorldQuery}, math::Vec2};

use crate::{Size2, FontSize, SizeUnit};

/// Size of the sprite.
///
/// If `Copied` and paired with a component that has a dimension like [`Sprite`](bevy::sprite::Sprite),
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

/// Controls the dimension of the sprite.
#[derive(Debug, Clone, Component, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dimension {
    /// Input for dimension.
    pub dimension: DimensionSize,
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
    /// A reliable min for '%' base dimensions.
    ///     
    /// This value is set by copied sources and dimension agnostic layouts.
    /// 
    /// If not set, the size would be 0 during the layout phase.
    pub reliable_size: Vec2,
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
            dimension: DimensionSize::Copied,
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }
}

impl Dimension {

    /// Dimension copied paired components.
    pub const COPIED: Self = Self {
        dimension: DimensionSize::Copied,
        font_size: FontSize::None,
        preserve_aspect: false,
    };

    /// Dimension inherited from parent.
    pub const INHERIT: Self = Self {
        dimension: DimensionSize::Owned(Size2::FULL),
        font_size: FontSize::None,
        preserve_aspect: false,
    };


    /// Owned dimension in pixels.
    pub const fn pixels(size: Vec2) -> Self {
        Self {
            dimension: DimensionSize::Owned(Size2::pixels(size.x, size.y)),
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }

    /// Owned dimension in percentage.
    pub const fn percentage(size: Vec2) -> Self {
        Self {
            dimension: DimensionSize::Owned(Size2::percent(size.x, size.y)),
            font_size: FontSize::None,
            preserve_aspect: false,
        }
    }

    /// Owned dimension.
    pub const fn owned(size: Size2) -> Self {
        Self {
            dimension: DimensionSize::Owned(size),
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

    /// Edit a contextless underlying value.
    /// 
    /// Has no effect if dimension is not owned.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        match &mut self.dimension {
            DimensionSize::Copied => (),
            DimensionSize::Owned(v) => v.edit_raw(f),
        }
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.dimension, DimensionSize::Owned(..))
    }

    pub fn is_copied(&self) -> bool {
        matches!(self.dimension, DimensionSize::Copied)
    }
}

impl DimensionMutReadOnlyItem<'_> {

    pub fn size(&self) -> Vec2 {
        self.dynamic.size
    }

    /// Obtain a contextless underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.source.dimension {
            DimensionSize::Copied => self.dynamic.size,
            DimensionSize::Owned(v) => v.raw(),
        }
    }

    /// Run a function if dimension is owned.
    pub fn run_if_owned(&self, f: impl FnOnce(Vec2)) {
        match self.source.dimension {
            DimensionSize::Owned(_) => f(self.dynamic.size),
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
            DimensionSize::Copied => (data.size, data.em),
            DimensionSize::Owned(v) if self.source.preserve_aspect => {
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
            DimensionSize::Owned(v) => {
                data.size = v.as_pixels(parent, data.em, rem);
                (data.size, data.em)
            }
        }
    }

    /// Estimate size for dynamic layout, this notably does not use non-canon values.
    pub fn estimate(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        let data = &self.dynamic;
        let em = match self.source.font_size{
            FontSize::None => em,
            FontSize::Pixels(v) => v,
            FontSize::Ems(v) => em * v,
            FontSize::Rems(v) => rem * v,
        };
        match self.source.dimension {
            DimensionSize::Copied => data.size,
            DimensionSize::Owned(v) if self.source.preserve_aspect => {
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
                if v.units().0 == SizeUnit::Percent {
                    size.x = data.reliable_size.x;
                }
                if v.units().0 == SizeUnit::Percent {
                    size.x = data.reliable_size.x;
                }
                size
            }
            DimensionSize::Owned(v) => {
                let mut size = v.as_pixels(parent, em, rem);
                if v.units().0 == SizeUnit::Percent {
                    size.x = data.reliable_size.x;
                }
                if v.units().0 == SizeUnit::Percent {
                    size.x = data.reliable_size.x;
                }
                size
            }
        }
    }

    /// Obtain a contextless underlying value.
    pub fn raw(&self) -> Vec2 {
        match &self.source.dimension {
            DimensionSize::Copied => self.dynamic.size,
            DimensionSize::Owned(v) => v.raw(),
        }
    }

    /// Edit a contextless underlying value.
    /// 
    /// Has no effect if dimension is not owned.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        match &mut self.source.dimension {
            DimensionSize::Copied => (),
            DimensionSize::Owned(v) => v.edit_raw(f),
        }
    }

    /// Gain mutable access to the underlying owned vector.
    /// 
    /// # Panics
    /// 
    /// When dimension is copied.
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        match &mut self.source.dimension {
            DimensionSize::Copied => panic!("Cannot get raw of copied value."),
            DimensionSize::Owned(v) => v.raw_mut(),
        }
    }
    
    /// Run a function if dimension is owned.
    pub fn run_if_owned(&self, f: impl FnOnce(Vec2)) {
        match self.source.dimension {
            DimensionSize::Owned(_) => f(self.dynamic.size),
            _ => (),
        }
    }

    /// If `copied`, copy size. If `preserve_aspect`, copy aspect ratio.
    pub fn update_size(&mut self, value: impl FnOnce() -> Vec2) {
        match self.source.dimension {
            DimensionSize::Copied => {
                self.dynamic.size = value();
                self.dynamic.reliable_size = self.dynamic.size;
            },
            DimensionSize::Owned(_) if self.source.preserve_aspect => {
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
