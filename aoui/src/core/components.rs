use bevy::{prelude::*, reflect::Reflect, math::Affine2};

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

/// Ignores writing opacity to the associated alpha value of sprite, text, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
pub struct IgnoreAlpha;

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

/// If specified, breaks hierarchy, making the sprite window space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
pub struct Detach;