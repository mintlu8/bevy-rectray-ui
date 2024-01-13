use bevy::{math::Vec2, sprite::{Mesh2dHandle, ColorMaterial}, prelude::Color, ecs::entity::Entity};
use bevy_aoui::{widget_extension, dsl::{prelude::OneOrTwo, DslFrom, Widget, AouiCommands}, bundles::BuildTransformBundle, build_frame};
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
    }
}

impl Widget for ShapeBuilder {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let color_material = commands.add_asset(ColorMaterial::default());
        let mut frame = build_frame!(commands, self);
        frame.insert((
            BuildTransformBundle::default(),
            self.shape.build_path(self.anchor, self.size.unwrap_or(Vec2::ONE)),
            self.shape,
            ShapeDimension {
                size: self.size.unwrap_or(Vec2::ONE),
                anchor: self.anchor,
            },
            Mesh2dHandle::default(),
            color_material,
        ));
        if let OptionX::Some(fill) = self.fill {
            frame.insert(fill);
        }
        if let OptionX::Some(stroke) = self.stroke {
            frame.insert(stroke);
        }
        (frame.id(), frame.id())
    }
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
