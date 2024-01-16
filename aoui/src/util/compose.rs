use bevy::{ecs::bundle::Bundle, sprite::Sprite, render::{texture::Image, color::Color}, transform::components::GlobalTransform};

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