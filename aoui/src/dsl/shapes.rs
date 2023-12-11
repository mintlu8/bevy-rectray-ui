use bevy::{ecs::system::Commands, asset::AssetServer, render::{texture::{Image, BevyDefault}, render_resource::{Extent3d, TextureDimension}}};

use crate::{widget_extension, dsl::{Widget, builders::SpriteBuilder}, map_builder};

use super::HandleOrString;


widget_extension!(
    pub struct RectangleBuilder: Sprite {
        pub radius: f32
    }
);


impl Widget for RectangleBuilder {
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> bevy::prelude::Entity {
        let texture = Image::new(Extent3d {
            width: 1,
            height: 1,
            ..Default::default()
        }, TextureDimension::D2, vec![255, 255, 255, 255], BevyDefault::bevy_default());
        let texture = assets.expect("Please pass in the AssetServer").add(texture);
        map_builder!(self => SpriteBuilder move (
            anchor,
            parent_anchor,
            center,
            opacity,
            visible,
            offset,
            rotation,
            scale,
            z,
            dimension,
            font_size,
            event,
            hitbox,
            layer,
            size,
            color,
            rect,
            flip,
        ) {
            sprite: HandleOrString::Handle(texture),
        }).spawn_with(commands, assets)
    }
}


#[macro_export]
macro_rules! rectangle {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RectangleBuilder] {$($tt)*})};
}
