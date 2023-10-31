use bevy::{prelude::*, sprite::Anchor, math::Affine2};

use crate::RotatedRect;

use super::layout::SparseLayout;

/// Child space position in a [`SparseContainer`]
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct SparsePosition(pub Vec2);

impl SparsePosition {
    pub fn load(&self) -> Vec2 {
        self.0
    }

    pub fn store(&mut self, value: Vec2) {
        self.0 = value
    }
}

/// A container accepting children with a 2D position [`SparsePosition`]
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct SparseContainer {
    /// Layout type of a sparse container
    pub layout: SparseLayout,

    /// Local space `(0, 0)` in child space.
    pub origin: Vec2,

    /// Transform the points in the layout without transforming
    /// its children or the `child_rect`.
    pub transform: Affine2,

    /// The parent rect where childrens are placed against
    ///
    /// by default, this produces `[(-x/2, 0), (x/2, y)]`
    pub child_rect: Option<Rect>,
}

/// Info for positioning an item in a [`SparseContainer`].
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SparseItem {
    pub anchor: Anchor,
    pub dimension: Vec2,
    pub position: Vec2,
}

impl SparseContainer {

    pub fn place_all(&self, rect: &RotatedRect, items: impl IntoIterator<Item = SparseItem>) -> Vec<Vec2>{
        let center = rect.center;
        match self.layout {
            SparseLayout::Rectangles { x, y, size } => {
                items.into_iter().map(|item| {
                    let coords = item.position - self.origin;
                    let coords = x.vec() * coords.x + y.vec() * coords.y;
                    let self_origin = self.transform.transform_point2(coords * size);
                    self_origin + center
                }).collect()
            },
            SparseLayout::Isometric { x, y, size } => {
                items.into_iter().map(|item| {
                    let coords = item.position - self.origin;
                    let coords = x.vec() * coords.x + y.vec() * coords.y;
                    let self_origin = self.transform.transform_point2(coords * size);
                    self_origin + center
                }).collect()
            },
            SparseLayout::HexGrid { x, y, size } => {
                items.into_iter().map(|item| {
                    let coords = item.position - self.origin;
                    let x = x.flat();
                    let xy = x.project_onto(y.flat());
                    let xx = x - xy;
                    let dx = coords.x.rem_euclid(2.0); 
                    let xy = if dx > 1.0 {
                        xy * (2.0 - dx)
                    } else {
                        xy * dx
                    };
                    let coords = xx * coords.x + xy + y.flat() * coords.y;
                    let self_origin = self.transform.transform_point2(coords * size);
                    self_origin + center
                }).collect()
            },
        }

    }
}
