
use std::ops::Mul;

use bevy::{math::{Vec2, Affine2}, reflect::Reflect, prelude::Component, ecs::entity::Entity, };


#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Anchor(Vec2);

#[allow(non_upper_case_globals)]
impl Anchor {
    pub const Inherit: Self = Self(Vec2::NAN);
    pub const BottomLeft: Self = Self(Vec2::new(-0.5, -0.5));
    pub const BottomCenter: Self = Self(Vec2::new(0.0, -0.5));
    pub const BottomRight: Self = Self(Vec2::new(0.5, -0.5));
    pub const CenterLeft: Self = Self(Vec2::new(-0.5, 0.0));
    pub const Center: Self = Self(Vec2::ZERO);
    pub const CenterRight: Self = Self(Vec2::new(0.5, 0.0));
    pub const TopLeft: Self = Self(Vec2::new(-0.5, 0.5));
    pub const TopCenter: Self = Self(Vec2::new(0.0, 0.5));
    pub const TopRight: Self = Self(Vec2::new(0.5, 0.5));

    pub fn new(v: Vec2) -> Self {
        Self(v)
    }

    pub fn custom(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn is_inherit(&self) -> bool {
        self.0.is_nan()
    }

    pub fn as_vec(&self) -> Vec2 {
        self.0
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn or(self, other: Self) -> Self {
        if self.is_inherit() {
            other
        } else {
            self
        }
    }

    pub fn str_name(&self) -> &'static str {
        match (self.0.x, self.0.y) {
            (x, y) if x < -0.16 && y < -0.16 => "BottomLeft",
            (x, y) if x < -0.16 && y > 0.16 => "TopLeft",
            (x, _) if x < -0.16 => "CenterLeft",
            (x, y) if x > 0.16 && y < -0.16 => "BottomRight",
            (x, y) if x > 0.16 && y > 0.16 => "TopRight",
            (x, _) if x > 0.16 => "CenterRight",
            (_, y) if y < -0.16 => "BottomCenter",
            (_, y) if y > 0.16 => "TopCenter",
            _ => "Center",
        }
    }
}

impl Mul<Vec2> for Anchor {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<Anchor> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Anchor) -> Self::Output {
        self * rhs.0
    }
}

impl Into<bevy::sprite::Anchor> for Anchor {
    fn into(self) -> bevy::sprite::Anchor {
        bevy::sprite::Anchor::Custom(self.0)
    }
}

impl Into<bevy::sprite::Anchor> for &Anchor {
    fn into(self) -> bevy::sprite::Anchor {
        bevy::sprite::Anchor::Custom(self.0)
    }
}

/// A rotated 2D rectangle.
/// 
/// Note: `scale` is pre-multiplied into `dimension`.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[non_exhaustive]
pub struct RotatedRect{
    /// Affine of the rect.
    pub affine: Affine2,
    /// Rotation of the Rect.
    pub rotation: f32,
    /// Z depth of the Rect.
    pub z: f32,
    /// Scale of the rect, already baked in dimension.
    pub scale: Vec2,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub enum PointOrRect {
    Point(Vec2),
    Rect(Affine2)
}

/// Relevant info about an AoUI sprite's parent.
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ParentInfo {
    pub entity: Option<Entity>,
    pub position: PointOrRect,
    pub rotation: f32,
    pub z: f32,
    pub dimension: Vec2,
    pub scale: Vec2,
    pub em: f32,
    pub opacity: f32,
}

impl ParentInfo {
    pub fn new(entity: Option<Entity>, rect: &RotatedRect, dimension: Vec2, em: f32, opacity: f32) -> Self{
        ParentInfo{
            entity,
            position: PointOrRect::Rect(rect.affine),
            rotation: rect.rotation,
            scale: rect.scale,
            z: rect.z,
            dimension,
            em,
            opacity,
        }
    }

    pub fn from_anchor(entity: Option<Entity>, rect: &RotatedRect, anc: Vec2, dimension: Vec2, em: f32, opacity: f32) -> Self{
        ParentInfo{
            entity,
            position: PointOrRect::Point(rect.anchor(Anchor::new(anc))),
            rotation: rect.rotation,
            scale: rect.scale,
            z: rect.z,
            dimension,
            em,
            opacity,
        }
    }
}

impl RotatedRect {
    /// Find the screen space position of an anchor.
    #[inline]
    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.affine.transform_point2(anchor.as_vec())
    }

    #[inline]
    pub fn center(&self) -> Vec2 {
        self.affine.translation
    }

    /// convert a screen sapce point to local space, centered on `Center`.
    #[inline]
    pub fn local_space(&self, position: Vec2) -> Vec2 {
        Vec2::from_angle(-self.rotation).rotate(position - self.center())
    }

    /// Create an [`RotatedRect`] represeting the sprite's position on the screen space
    /// and an [`Affine3A`] that converts into the [`GlobalTransform`] suitable from the screen space
    #[allow(clippy::too_many_arguments)]
    pub fn construct(parent: &ParentInfo, parent_anchor: Anchor, anchor: Anchor, offset: Vec2, dim: Vec2,
            center: Anchor, rotation: f32, scale: Vec2, z: f32) -> Self{
        let parent_anchor = match parent.position {
            PointOrRect::Point(p) => p,
            PointOrRect::Rect(r) => r.transform_point2(parent_anchor.or(anchor).as_vec()),
        };
        // apply offset and dimension
        let self_center = offset + (center.as_vec() - anchor.as_vec()) * dim;
        let dir = (Anchor::Center.as_vec() - center.as_vec()) * dim;

        let out_center = Vec2::from_angle(parent.rotation).rotate(self_center * parent.scale) + parent_anchor;
        let rotation = parent.rotation + rotation;
        let scale = parent.scale * scale;
        let out_origin = out_center + Vec2::from_angle(rotation).rotate(dir * scale);

        Self {
            affine: Affine2::from_scale_angle_translation(
                dim * scale, 
                rotation, 
                out_origin
            ),
            z,
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
                self.affine,
                self.rotation, self.z, 
                self.scale.x, self.scale.y
            ).serialize(serializer)
        }
    }


    impl<'de> Deserialize<'de> for RotatedRect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            let (af, r, z, sx, sy) = <_>::deserialize(deserializer)?; 
            Ok(Self { 
                affine: af,
                rotation: r, z,
                scale: Vec2::new(sx, sy),
            })
        }
    }
};