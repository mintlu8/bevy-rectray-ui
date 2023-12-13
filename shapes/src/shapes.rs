use bevy::{math::Vec2, sprite::{Mesh2dHandle, ColorMaterial}, prelude::Color};
use bevy_aoui::{widget_extension, dsl::{prelude::OneOrTwo, DslFrom}, bundles::BuildTransformBundle};
use bevy_prototype_lyon::prelude::*;

use crate::systems::{ShapeDimension, Shapes};

#[derive(Debug, Default)]
pub enum OptionX<T> {
    Some(T),
    #[default]
    None
}

impl DslFrom<Color> for OptionX<Fill>{
    fn dfrom(value: Color) -> Self {
        OptionX::Some(Fill::color(value))
    }
}

impl DslFrom<(Color, i32)> for OptionX<Stroke>{
    fn dfrom((color, size): (Color, i32)) -> Self {
        OptionX::Some(Stroke { 
            color, 
            options: StrokeOptions::DEFAULT
                .with_line_width(size as f32)
                .with_start_cap(LineCap::Round)
                .with_end_cap(LineCap::Round)
        })
    }
}

impl DslFrom<(Color, f32)> for OptionX<Stroke>{
    fn dfrom((color, size): (Color, f32)) -> Self {
        OptionX::Some(Stroke { 
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
        pub fill: OptionX<Fill>,
        pub stroke: OptionX<Stroke>,
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
        OptionX::Some(fill) = this.fill => fill,
        OptionX::Some(stroke) = this.stroke => {
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
            bevy_aoui::meta_dsl!($ctx [$crate::ShapeBuilder] {
            $($tt)*
        })
    };
}