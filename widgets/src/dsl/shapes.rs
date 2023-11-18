use bevy::{math::Vec2, sprite::{Mesh2dHandle, ColorMaterial}, prelude::{Color, Handle}};
use bevy_aoui::bundles::BuildGlobalBundle;
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
        /// pub material: Option<Handle<Material2d>>,
        pub default_material: Handle<ColorMaterial>,
        /// Unlike the default behavior of `Lyon`,
        /// 
        /// The default is `Round`.
        pub caps: Option<OneOrTwo<[LineCap; 2]>>,
    },
    this, commands,
    components: (
        BuildGlobalBundle::default(),
        this.shape.build_path(this.anchor, this.size.unwrap_or(Vec2::ONE)),
        this.shape,
        ShapeDimension { 
            size: this.size.unwrap_or(Vec2::ONE), 
            anchor: this.anchor,
        },
        Mesh2dHandle::default(),
        this.default_material,
    ),
    pattern: (
        Some(fill) = this.fill => fill,
        Some(mut stroke) = this.stroke => {
            if let Some(OneOrTwo([l ,r])) = this.caps.dinto() {
                stroke.options = stroke.options.with_start_cap(l).with_end_cap(r)
            }
            stroke
        }
    )
}


/// Construct a shape with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! shape {
    (($commands: expr, $server: expr $(, $ctx: expr)*) {$($tt:tt)*}) => {
            $crate::meta_dsl!(($commands, $server $(, $ctx)*) [$crate::dsl::builders::ShapeBuilder] {
            default_material: $server.add(::bevy::prelude::ColorMaterial::default()),
            $($tt)*
        })
    };
}

/// Construct a rectangle with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! rectangle {
    (($commands: expr, $server: expr $(, $ctx: expr)*) {$($tt:tt)*}) => {
        $crate::meta_dsl!(($commands, $server $(, $ctx)*) [$crate::dsl::builders::ShapeBuilder] {
            default_material: $server.add(::bevy::prelude::ColorMaterial::default()),
            shape: $crate::widgets::Shapes::Rectangle,
            $($tt)*
        })
    };
}

/// Construct a circle with `bevy_prototype_lyon`.
#[macro_export]
macro_rules! circle {
    (($commands: expr, $server: expr $(, $ctx: expr)*) {$($tt:tt)*}) => {
        $crate::meta_dsl!(($commands, $server $(, $ctx)*) [$crate::dsl::builders::ShapeBuilder] {
            default_material: $server.add(::bevy::prelude::ColorMaterial::default()),
            shape: $crate::widgets::Shapes::Circle,
            $($tt)*
        })
    };
}
