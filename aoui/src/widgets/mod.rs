
pub mod schedule;
pub mod inputbox;
pub mod drag;
pub mod richtext;
pub mod scroll;
pub mod scrollframe;
pub mod button;
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

