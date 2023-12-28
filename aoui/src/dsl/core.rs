use bevy::{ecs::{system::Commands, entity::Entity}, asset::AssetServer, sprite::Sprite, text::{Text, TextSection, TextStyle, BreakLineOn, Text2dBounds, TextLayoutInfo, Font}, render::{color::Color, texture::{Image, BevyDefault}, render_resource::{Extent3d, TextureDimension}}, math::{Vec2, Rect}};

use crate::{widget_extension, transform2d, dimension, Clipping, bundles::{AouiBundle, BuildTransformBundle}, Hitbox, OpacityWriter, build_frame};

use super::{Widget, DslInto, apply_marker, get_layer, is_using_opacity, HandleOrString};

widget_extension!(pub struct FrameBuilder {});
widget_extension!(
    pub struct SpriteBuilder {
        /// Handle of the image asset.
        pub sprite: HandleOrString<Image>,
        /// Size of the image.
        pub size: Option<Vec2>,
        /// Color of the image.
        pub color: Option<Color>,
        /// Atlas rectangle of the image.
        pub rect: Option<Rect>,
        /// Flips the image.
        pub flip: [bool; 2],
    }
);

widget_extension!(
    pub struct RectangleBuilder {
        /// Size of the image.
        pub size: Option<Vec2>,
        /// Color of the image.
        pub color: Option<Color>,
    }
);


widget_extension!(
    pub struct TextBuilder {
        /// The text string.
        pub text: String,
        /// Handle of the font asset.
        pub font: HandleOrString<Font>,
        /// Bounds of the text, should not be set most of the time.
        ///
        /// If not specified this is `UNBOUNDED`.
        pub bounds: Option<Vec2>,
        /// Color of the text.
        pub color: Option<Color>,
        /// Sets if the text wraps.
        pub wrap: bool,
        /// Break line on, maybe use wrap instead.
        pub break_line_on: Option<BreakLineOn>,
    }
);

impl Widget for FrameBuilder {
    fn spawn_with(self, commands: &mut Commands, _: Option<&AssetServer>) -> (Entity, Entity) {

        let mut base = commands.spawn(
            AouiBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                opacity: self.opacity,
                vis: self.visible.dinto(),
                clipping: Clipping::new(self.clipping),
                ..Default::default()
            }
        );
        apply_marker(&mut base);
        if let Some(event) = self.event {
            base.insert(event);
        }
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        } else if self.event.is_some() {
            base.insert(Hitbox::FULL);
        }
        if let Some(layer) = self.layer {
            base.insert(layer);
        } else if let Some(layer) = get_layer() {
            base.insert(layer);
        }
        if is_using_opacity() {
            base.insert(OpacityWriter);
        }
        let base = base.id();
        (base, base)
    }
}

impl Widget for SpriteBuilder {
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> (Entity, Entity) {
        let mut frame = build_frame!(commands, self);
        frame.insert((
            Sprite {
                custom_size: self.size,
                color: self.color.unwrap_or(bevy::prelude::Color::WHITE),
                rect: self.rect,
                flip_x: self.flip[0],
                flip_y: self.flip[1],
                ..Default::default()
            },
            self.sprite.get(assets),
            BuildTransformBundle::default(),
        ));
        (frame.id(), frame.id())
    }
}


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

impl Widget for TextBuilder {
    fn spawn_with(self, commands: &mut Commands, assets: Option<&AssetServer>) -> (Entity, Entity) {
        let mut frame = build_frame!(commands, self);
        frame.insert((
            Text {
                sections: vec![TextSection::new(
                    self.text,
                    TextStyle {
                        font: self.font.get(assets),
                        color: self.color.unwrap_or(Color::WHITE),
                        ..Default::default()
                    }
                )],
                linebreak_behavior: if let Some(b) = self.break_line_on {
                    b
                } else if self.wrap {
                    BreakLineOn::WordBoundary
                } else {
                    BreakLineOn::NoWrap
                },
                ..Default::default()
            },
            match self.bounds {
                Some(size) => Text2dBounds { size },
                None => Text2dBounds::UNBOUNDED,
            },
            TextLayoutInfo::default(),
            Into::<bevy::sprite::Anchor>::into(self.anchor),
            BuildTransformBundle::default(),
        ));
        (frame.id(), frame.id())
    }
}