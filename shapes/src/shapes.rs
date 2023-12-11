use bevy::{math::Vec2, sprite::{Mesh2dHandle, ColorMaterial}, prelude::Color};
use crate::bundles::BuildTransformBundle;
use bevy_prototype_lyon::prelude::*;

use crate::{dsl::prelude::*, widgets::shape::{Shapes, ShapeDimension}, widget_extension};

use super::convert::DslInto;

impl DslInto<Option<Fill>> for Color{
    fn dinto(self) -> Option<Fill> {
        Some(Fill::color(self))
    }
}

impl DslInto<Option<Stroke>> for (Color, i32){
    fn dinto(self) -> Option<Stroke> {
        let (color, size) = self;
        Some(Stroke { 
            color, 
            options: StrokeOptions::DEFAULT
                .with_line_width(size as f32)
                .with_start_cap(LineCap::Round)
                .with_end_cap(LineCap::Round)
        })
    }
}

impl DslInto<Option<Stroke>> for (Color, f32){
    fn dinto(self) -> Option<Stroke> {
        let (color, size) = self;
        Some(Stroke { 
            color, 
            options: StrokeOptions::DEFAULT
                .with_line_width(size)
                .with_start_cap(LineCap::Round)
                .with_end_cap(LineCap::Round)
        })
    }
}

widget_extension! {
    pub struct ShapeBuilder {
        pub size: Option<Vec2>,
        pub shape: Shapes,
        pub fill: Option<Fill>,
        pub stroke: Option<Stroke>,
        pub stroke_size: f32,
        /// Unlike the default behavior of `Lyon`,
        /// 
        /// The default is `Round`.
        pub caps: Option<OneOrTwo<[LineCap; 2]>>,
    },
    this, commands, assets,    
    components: (
        BuildTransformBundle::default(),
        this.shape.build_path(this.anchor, this.size.unwrap_or(Vec2::ONE)),
        this.shape,
        ShapeDimension { 
            size: this.size.unwrap_or(Vec2::ONE), 
            anchor: this.anchor,
        },
        Mesh2dHandle::default(),
        assets.expect("Please pass in the AssetServer").add(ColorMaterial::default()),
        Some(fill) = this.fill => fill,
        Some(stroke) = this.stroke => {
            let mut stroke = stroke;
            if let Some(OneOrTwo([l ,r])) = this.caps.dinto() {
                stroke.options = stroke.options.with_start_cap(l).with_end_cap(r)
            }
            stroke
        }
    ),
}


/// Construct a shape with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! shape {
    ($ctx: tt {$($tt:tt)*}) => {
            $crate::meta_dsl!($ctx [$crate::dsl::builders::ShapeBuilder] {
            $($tt)*
        })
    };
}

/// Construct a rectangle with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! rectangle {
    ($ctx: tt {$($tt:tt)*}) => {
        $crate::meta_dsl!($ctx [$crate::dsl::builders::ShapeBuilder] {
            shape: $crate::widgets::Shapes::Rectangle,
            $($tt)*
        })
    };
}

/// Construct a circle with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! circle {
    ($ctx: tt {$($tt:tt)*}) => {
        $crate::meta_dsl!($ctx [$crate::dsl::builders::ShapeBuilder] {
            shape: $crate::widgets::Shapes::Circle,
            $($tt)*
        })
    };
}
