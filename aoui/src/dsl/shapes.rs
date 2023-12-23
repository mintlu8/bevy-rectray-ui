use bevy::{ecs::{system::Commands, entity::Entity}, asset::AssetServer, render::{texture::{Image, BevyDefault}, render_resource::{Extent3d, TextureDimension}}, sprite::Sprite};

use crate::{widget_extension, dsl::Widget, build_frame, bundles::BuildTransformBundle};


widget_extension!(
    pub struct RectangleBuilder: Sprite {
        pub radius: f32
    }
);


impl Widget for RectangleBuilder {
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> (Entity, Entity) {
        let texture = Image::new(Extent3d {
            width: 1,
            height: 1,
            ..Default::default()
        }, TextureDimension::D2, vec![255, 255, 255, 255], BevyDefault::bevy_default());
        let texture = assets.expect("Please pass in the AssetServer").add(texture);
        let frame = build_frame!(commands, self)
            .insert((
            Sprite {
                custom_size: self.size,
                color: self.color.unwrap_or(bevy::prelude::Color::WHITE),
                rect: self.rect,
                flip_x: self.flip[0],
                flip_y: self.flip[1],
                ..Default::default()
            },
            texture,
            BuildTransformBundle::default(),
        )).id();
        (frame, frame)
    }
}


#[macro_export]
macro_rules! rectangle {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RectangleBuilder] {$($tt)*})};
}
