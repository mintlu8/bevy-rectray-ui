use bevy::{ecs::{system::Commands, entity::Entity}, asset::AssetServer, sprite::Sprite, text::{Text, TextSection, TextStyle, BreakLineOn, Text2dBounds, TextLayoutInfo}, render::color::Color};

use crate::{widget_extension, transform2d, dimension, Clipping, bundles::{AouiBundle, BuildTransformBundle}, Hitbox, OpacityWriter, build_frame};

use super::{Widget, DslInto, apply_marker, get_layer, is_using_opacity};

widget_extension!(pub struct FrameBuilder {});
widget_extension!(pub struct SpriteBuilder: Sprite {});
widget_extension!(pub struct TextBuilder: Text {});

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
        } else {
            if let Some(layer) = get_layer() {
                base.insert(layer);
            }
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