
use bevy::{math::Vec2, reflect::Reflect, prelude::Component, sprite::Anchor};

/// A rotated 2D rectangle.
/// 
/// Note: `scale` is pre-multiplied into `dimension`.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[non_exhaustive]
pub struct RotatedRect{
    /// Center of the Rect.
    pub center: Vec2,
    /// Dimension of the Rect.
    pub dimension: Vec2,
    /// Rotation of the Rect.
    pub rotation: f32,
    /// Z depth of the Rect.
    pub z: f32,
    /// Scale of the rect, already baked in dimension.
    pub scale: Vec2,
}

/// Relevant info about an AoUI sprite's parent.
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ParentInfo {
    pub anchor: Vec2,
    pub dimension: Vec2,
    pub rotation: f32,
    pub z: f32,
    /// Scale is already baked in dimension.
    pub scale: Vec2,
    pub em: f32,
}

impl ParentInfo {
    pub fn new(rect: &RotatedRect, anc: Anchor, dimension: Vec2, em: f32) -> Self{
        ParentInfo{
            anchor: rect.anchor(anc),
            rotation: rect.rotation,
            scale: rect.scale,
            z: rect.z,
            dimension,
            em,
        }
    }

    pub fn from_anchor(rect: &RotatedRect, anc: Vec2, dimension: Vec2, em: f32) -> Self{
        ParentInfo{
            anchor: rect.anchor(Anchor::Custom(anc)),
            rotation: rect.rotation,
            scale: rect.scale,
            z: rect.z,
            dimension,
            em,
        }
    }
}

impl RotatedRect {
    /// Find the screen space position of an anchor.
    #[inline]
    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.center + Vec2::from_angle(self.rotation).rotate(self.dimension * anchor.as_vec())
    }

    /// convert a screen sapce point to local space, centered on `Center`.
    #[inline]
    pub fn local_space(&self, position: Vec2) -> Vec2 {
        Vec2::from_angle(-self.rotation).rotate(position - self.center)
    }

    /// convert a screen space point to local space, centered on `BottomLeft`.
    #[inline]
    pub fn local_space_bl(&self, position: Vec2) -> Vec2 {
        Vec2::from_angle(-self.rotation).rotate(position - self.center) + self.dimension / 2.0
    }

    /// Create an [`RotatedRect`] represeting the sprite's position on the screen space
    /// and an [`Affine3A`] that converts into the [`GlobalTransform`] suitable from the screen space
    #[allow(clippy::too_many_arguments)]
    pub fn construct(parent: &ParentInfo, anchor: Anchor, offset: Vec2, dim: Vec2,
            center: Anchor, rotation: f32, scale: Vec2, z: f32) -> Self{
        let parent_anchor = parent.anchor;
        // apply offset and dimension
        let self_center = offset + (center.as_vec() - anchor.as_vec()) * dim;
        let dir = (Anchor::Center.as_vec() - center.as_vec()) * dim;

        let out_center = Vec2::from_angle(parent.rotation).rotate(self_center * parent.scale) + parent_anchor;
        let rotation = parent.rotation + rotation;
        let scale = parent.scale * scale;
        let out_origin = out_center + Vec2::from_angle(rotation).rotate(dir * scale);

        Self {
            center: out_origin, z,
            dimension: dim * scale,
            rotation,
            scale,
        }
    }
}

#[cfg(feature="serde")]
const _: () = {
    use serde::{Serialize, Deserialize};
    impl Serialize for RotatedRect {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            (
                self.center.x, self.center.y, 
                self.dimension.x, self.dimension.y,
                self.rotation, self.z, 
                self.scale.x, self.scale.y
            ).serialize(serializer)
        }
    }


    impl<'de> Deserialize<'de> for RotatedRect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            let (cx, cy, dx, dy, r, z, sx, sy) = <_>::deserialize(deserializer)?; 
            Ok(Self { 
                center: Vec2::new(cx, cy),
                dimension: Vec2::new(dx, dy),
                rotation: r, z,
                scale: Vec2::new(sx, sy),
            })
        }
    }
};