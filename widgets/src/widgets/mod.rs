
pub mod shape;
pub mod schedule;
pub mod inputbox;
use bevy::{render::color::Color, ecs::component::Component};

#[derive(Debug, Clone, Copy, Component)]
pub struct TextColor(pub Color);

impl TextColor {
    pub fn get(&self) -> Color {
        self.0
    }
    pub fn set(&mut self, color: Color) {
        self.0 = color
    }
}

pub use shape::{Shapes, ShapeDimension};