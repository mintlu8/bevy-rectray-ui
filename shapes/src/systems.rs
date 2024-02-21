use bevy::{prelude::{Component, Vec2}, reflect::Reflect, ecs::{query::Changed, system::Query}};
use bevy_rectray::{Transform2D, Anchor, DimensionData};
use bevy_prototype_lyon::prelude::{GeometryBuilder, Path};
use bevy_prototype_lyon::shapes::*;

/// A shape managed by Aoui.
#[derive(Debug, Clone, Component, Reflect, Default)]
pub enum Shapes {
    Circle,
    #[default]
    Rectangle,
    RoundedRectangle(f32),
    Line(Vec2, Vec2),
    NGon(usize),
    Polyline(Vec<Vec2>),
    Polygon(Vec<Vec2>),
    Svg(String)
}

/// Aoui controlled aspects of Shape.
/// 
/// Provides intermediate change detection.
#[derive(Debug, Clone, Component, Reflect)]
pub struct ShapeDimension {
    pub size: Vec2,
    pub anchor: Anchor,
}

macro_rules! build_path {
    ($e: expr) => {
        GeometryBuilder::new()
            .add(&$e)
            .build()
    };
}

impl Shapes {
    pub fn build_path(&self, anchor: Anchor, dimension: Vec2) -> Path {
        let center = -dimension * anchor.as_vec();
        match self{
            Shapes::Circle => build_path!(Ellipse { 
                radii: dimension / 2.0, 
                center,
            }),
            Shapes::Rectangle => build_path!(Rectangle { 
                extents: dimension, 
                origin: RectangleOrigin::CustomCenter(center)
            }),
            Shapes::RoundedRectangle(radius) => {
                let half = dimension / 2.0;
                let conjugate = Vec2::new(half.x, -half.y);
                build_path!(RoundedPolygon{ 
                    points: vec![center - half, center - conjugate, center + half, center + conjugate], 
                    radius: *radius, 
                    closed: true 
                })
            },
            Shapes::Line(a, b) => build_path!(Line(*a, *b)),
            Shapes::NGon(sides) => build_path!(RegularPolygon { 
                sides: *sides,
                center,
                feature: RegularPolygonFeature::Radius(dimension.x.min(dimension.y) / 2.0),
            }),
            Shapes::Polyline(path) => build_path!(Polygon { 
                points: path.clone(), 
                closed: false,
            }),
            Shapes::Polygon(path) => build_path!(Polygon { 
                points: path.clone(),
                closed: true,
            }),
            Shapes::Svg(path) => build_path!(SvgPathShape {
                svg_path_string: path.clone(),
                svg_doc_size_in_px: Vec2::ONE,
            }),
        }
        
    }
}


pub fn sync_shape_size(mut query: Query<(&Transform2D, &DimensionData, &mut ShapeDimension)>) {
    for (transform, dimension, mut shape) in query.iter_mut() {
        if transform.anchor != shape.as_ref().anchor {
            shape.anchor = transform.anchor
        }

        if dimension.size != shape.as_ref().size {
            shape.size = dimension.size
        }
    }
}

pub fn rebuild_shapes(mut query: Query<(&Shapes, &ShapeDimension, &mut Path), Changed<ShapeDimension>>) {
    for (shape, cache, mut path) in query.iter_mut() {
        *path = shape.build_path(cache.anchor, cache.size)
    }
}