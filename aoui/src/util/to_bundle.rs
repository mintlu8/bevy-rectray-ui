use bevy::{ecs::bundle::Bundle, sprite::Sprite, transform::components::GlobalTransform};
use bevy::render::{color::Color, texture::Image};

use crate::{dsl::IntoAsset, BuildTransform, Coloring};

use super::AouiCommands;


impl IntoAsset<Image> {
    pub fn into_bundle(self, commands: &mut AouiCommands, color: Color) -> impl Bundle {
        let handle = commands.load_or_default(self);
        (
            Sprite::default(),
            handle,
            BuildTransform::default(),
            GlobalTransform::default(),
            Coloring::new(color)
        )
    }
}