use bevy::{math::Vec2, render::color::Color};
use bevy_aoui::util::DslFrom;

#[derive(Debug, Default)]
pub enum FillShape {
    #[default]
    None,
    Rectangle,
    Capsule,
    RoundedRectangle(Vec2),
}

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub color: Color,
    pub size: f32,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::NONE, 
            size: 0.0, 
        }
    }
}

impl DslFrom<(Color, f32)> for Stroke {
    fn dfrom((color, size): (Color, f32)) -> Self {
        Stroke { color, size }
    }
}

impl DslFrom<(f32, Color)> for Stroke {
    fn dfrom((size, color): (f32, Color)) -> Self {
        Stroke { color, size }
    }
}

impl DslFrom<(Color, i32)> for Stroke {
    fn dfrom((color, size): (Color, i32)) -> Self {
        Stroke { color, size: size as f32 }
    }
}

impl DslFrom<(i32, Color)> for Stroke {
    fn dfrom((size, color): (i32, Color)) -> Self {
        Stroke { color, size: size as f32 }
    }
}