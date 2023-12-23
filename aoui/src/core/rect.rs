
use std::ops::Mul;

use bevy::{math::{Vec2, Affine2, Rect}, reflect::Reflect, prelude::Component, ecs::entity::Entity, };

/// Anchor of a sprite, this is a more concise implementation than bevy's.
/// 
/// If a field is `Inherit` it will use `anchor` if possible.
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

    pub const fn new(v: Vec2) -> Self {
        Self(v)
    }

    pub const fn custom(x: f32, y: f32) -> Self {
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

impl From<Anchor> for bevy::sprite::Anchor {
    fn from(val: Anchor) -> Self {
        bevy::sprite::Anchor::Custom(val.0)
    }
}

impl From<&Anchor> for bevy::sprite::Anchor {
    fn from(val: &Anchor) -> Self {
        bevy::sprite::Anchor::Custom(val.0)
    }
}

/// A rotated 2D rectangle.
/// 
/// Note: `scale` is pre-multiplied into `dimension`.
#[derive(Debug, Clone, Copy, Component, PartialEq, Default, Reflect)]
#[non_exhaustive]
pub struct RotatedRect{
    /// Affine of the rect.
    pub affine: Affine2,
    /// Rotation of the Rect.
    pub rotation: f32,
    /// Z depth of the Rect.
    pub z: f32,
    /// Scale of the rect.
    pub scale: Vec2,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub enum PointOrRect {
    Point(Vec2),
    Rect(Affine2)
}

/// Relevant info about an Aoui sprite's parent.
#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct ParentInfo {
    pub entity: Option<Entity>,
    pub rect: RotatedRect,
    pub anchor: Option<Vec2>,
    pub dimension: Vec2,
    pub em: f32,
    pub opacity: f32,
    pub clip: Option<Affine2>,
    pub disabled: bool,
}

impl ParentInfo {
    pub fn with_anchor(mut self, anc: Vec2) -> Self {
        self.anchor = Some(self.rect.anchor(Anchor(anc)));
        self
    }
}

impl RotatedRect {

    pub fn rect(&self) -> Rect {
        Rect{
            min: self.affine.transform_point2(Vec2::new(-0.5, -0.5)),
            max: self.affine.transform_point2(Vec2::new(0.5, 0.5)),
        }
    }

    /// Find the screen space position of an anchor.
    #[inline]
    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.affine.transform_point2(anchor.as_vec())
    }

    #[inline]
    pub fn center(&self) -> Vec2 {
        self.affine.translation
    }

    /// convert a screen space point to local space, centered on `Center`.
    #[inline]
    pub fn local_space(&self, position: Vec2) -> Vec2 {
        Vec2::from_angle(-self.rotation).rotate(position - self.center())
    }

    /// Create an [`RotatedRect`] representing the sprite's position on the screen space
    /// and an `Affine3A` that converts into the `GlobalTransform` suitable from the screen space
    pub fn construct(parent: &ParentInfo, parent_anchor: Anchor, anchor: Anchor, offset: Vec2, dim: Vec2,
            center: Anchor, rotation: f32, scale: Vec2, z: f32) -> Self{
        let parent_anchor = parent.anchor.unwrap_or_else(|| 
            parent.rect.affine.transform_point2(parent_anchor.or(anchor).as_vec())
        );
        // apply offset and dimension
        let self_center = offset + (center.as_vec() - anchor.as_vec()) * dim;
        let dir = (Anchor::Center.as_vec() - center.as_vec()) * dim;

        let out_center = Vec2::from_angle(parent.rect.rotation).rotate(self_center * parent.rect.scale) + parent_anchor;
        let rotation = parent.rect.rotation + rotation;
        let scale = parent.rect.scale * scale;
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
            let [a, b, c, d, e, f] = self.affine.to_cols_array();
            [a, b, c, d, e, f, 
                self.rotation, self.z, 
                self.scale.x, self.scale.y
            ].serialize(serializer)
        }
    }


    impl<'de> Deserialize<'de> for RotatedRect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
            let [a,b,c,d,e,f,r,z,sx,sy] = <_>::deserialize(deserializer)?; 
            Ok(Self { 
                affine: Affine2::from_cols_array(&[a, b, c, d, e, f]),
                rotation: r, z,
                scale: Vec2::new(sx, sy),
            })
        }
    }
};
