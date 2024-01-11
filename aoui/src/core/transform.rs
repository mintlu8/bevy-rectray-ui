use bevy::{reflect::Reflect, ecs::component::Component, math::Vec2};

use crate::{Anchor, Size2};

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
    /// If not set, this is `parent_z + 0.01`.
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
        anchor: Anchor::CENTER,
        parent_anchor: Anchor::CENTER,
        center: Anchor::INHERIT,
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
        Self(Anchor::INHERIT)
    }
}


/// Builds a `GlobalTransform` for `Mesh2d`,
/// this always uses `Anchor::Center` and converts dimension to scale.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct BuildMeshTransform;
