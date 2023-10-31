
use bevy::{math::{Vec2, Affine3A}, reflect::Reflect, prelude::{Component, Quat}, sprite::Anchor};

/// A rotated 2D rectangle.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[non_exhaustive]
pub struct RotatedRect{
    pub center: Vec2,
    pub dimension: Vec2,
    pub rotation: f32,
    pub z: f32,
    pub scale: Vec2,
}

/// Relevant info about an AoUI sprite's parent.
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ParentInfo {
    pub anchor: Vec2,
    pub dimension: Vec2,
    pub em: Vec2,
    pub rotation: f32,
    pub z: f32,
    pub scale: Vec2,
}

impl ParentInfo {
    pub fn new(quad: &RotatedRect, anc: &Anchor, dimension: Vec2, em: Vec2) -> Self{
        ParentInfo{
            anchor: quad.anchor(anc),
            rotation: quad.rotation,
            scale: quad.scale,
            z: quad.z,
            dimension,
            em,
        }
    }

    pub fn from_anchor(quad: &RotatedRect, anc: Vec2, dimension: Vec2, em: Vec2) -> Self{
        ParentInfo{
            anchor: anc,
            rotation: quad.rotation,
            scale: quad.scale,
            z: quad.z,
            dimension,
            em,
        }
    }
}

impl RotatedRect {
    /// Find the screen space position of an anchor.
    #[inline]
    pub fn anchor(&self, anchor: &Anchor) -> Vec2 {
        self.center + Vec2::from_angle(self.rotation).rotate(self.dimension * anchor.as_vec())
    }

    /// Find the screen space position of an anchor, assuming no rotation.
    #[inline]
    pub fn anchor_fast(&self, anchor: &Anchor) -> Vec2 {
        self.center + self.dimension * anchor.as_vec()
    }

    /// Create an [`RotatedRect`] represeting the sprite's position on the screen space
    /// and an [`Affine3A`] that converts into the [`GlobalTransform`] suitable from the screen space
    pub fn construct(parent: &ParentInfo, anchor: &Anchor, offset: Vec2, dim: Vec2,
            center: &Anchor, rotation: f32, scale: Vec2, z: f32) -> (Self, Affine3A){

        let parent_anchor = parent.anchor;
        // apply offset and dimension
        let self_center = offset + (center.as_vec() - anchor.as_vec()) * dim;
        let self_origin = offset + (Anchor::Center.as_vec() - anchor.as_vec()) * dim;
        // pass1 applies rotation and scale inherited from parent
        let pass1_center = Vec2::from_angle(parent.rotation).rotate(self_center * parent.scale) + parent_anchor;
        let pass1_origin = Vec2::from_angle(parent.rotation).rotate(self_origin * parent.scale) + parent_anchor;
        // pass2 applies the sprites own rotation and scale
        let pass2_origin = pass1_center + Vec2::from_angle(rotation).rotate((pass1_origin - pass1_center) * scale);
        let rect = Self {
            center: pass2_origin, z,
            dimension: dim * parent.scale * scale,
            rotation: parent.rotation + rotation,
            scale: parent.scale * scale,
        };
        (rect,
        Affine3A::from_scale_rotation_translation(
            (parent.scale * scale).extend(1.0),
            Quat::from_rotation_z(parent.rotation + rotation),
            rect.anchor(anchor).extend(z)
        ))
    }

    /// Create an [`RotatedRect`] represeting the sprite's position on the screen space
    /// and an [`Affine3A`] that converts into the [`GlobalTransform`] suitable from the screen space
    /// This only considers parent's scale.
    pub fn construct_no_rot_scale(parent: &ParentInfo, anchor: &Anchor, offset: Vec2, dim: Vec2, z: f32) -> (Self, Affine3A){
        let dim = dim * parent.scale;
        let origin = parent.anchor + offset * parent.scale + (Anchor::Center.as_vec() - anchor.as_vec()) * dim;
        let rect = Self {
            center: origin, z,
            dimension: dim, 
            rotation: 0.0,
            scale: Vec2::ONE,
        };
        (rect,
        Affine3A::from_translation(
            rect.anchor_fast(anchor).extend(z)
        ))
    }
}
