use bevy::{sprite::Anchor, prelude::{Vec2, Image, Handle, Color, Rect, Commands, Entity}, text::{Font, Text, TextSection, TextStyle, BreakLineOn, Text2dBounds}, math::bool};
use bevy_aoui::{Size2, SetEM, bundles::{AoUIBundle, AoUISpriteBundle, AoUITextBundle}, Hitbox};


use crate::{dsl::DslInto, transform2d, dimension};

use super::{prelude::OneOrTwo, AoUIWidget};

#[cfg(never)]
/// Template for a minimal DSL item.
#[derive(Debug, Default)]
pub struct Minimal {
    pub center: Option<Anchor>,
    pub anchor: Anchor,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,
}


/// An empty sprite.
#[derive(Debug, Default)]
pub struct FrameBuilder {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,
}

impl AoUIWidget for FrameBuilder {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                ..Default::default()
            },
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }
        base.id()
    }
}
   


/// An image base sprite.
#[derive(Debug, Default)]
pub struct SpriteBuilder {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,

    pub sprite: Handle<Image>,
    pub size: Option<Vec2>,
    pub color: Option<Color>,
    pub rect: Option<Rect>,
    pub flip: [bool; 2],
}

impl AoUIWidget for SpriteBuilder {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let [flip_x, flip_y] = self.flip;
        let mut base = commands.spawn((
            AoUISpriteBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                sprite: bevy::prelude::Sprite {
                    custom_size: self.size,
                    rect: self.rect,
                    color: self.color.unwrap_or(Color::WHITE),
                    flip_x,
                    flip_y,
                    ..Default::default()
                }, 
                texture: self.sprite,
                vis: self.visible.dinto(),
                ..Default::default()
            },
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }
        base.id()
    }
}

/// A text box.
#[derive(Debug, Default)]
pub struct TextBoxBuilder {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,

    pub text: String,
    pub font: Handle<Font>,
    /// Note if not specified this is `UNBOUNDED`.
    pub bounds: Option<Vec2>,
    pub color: Option<Color>,
    pub wrap: bool,
    pub break_line_on: Option<BreakLineOn>,
}


impl AoUIWidget for TextBoxBuilder {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUITextBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                text: Text {
                    sections: vec![TextSection::new(
                        self.text, 
                        TextStyle {
                            font: self.font,
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
                text_bounds: match self.bounds {
                    Some(size) => Text2dBounds { size },
                    None => Text2dBounds::UNBOUNDED,
                },
                ..Default::default()
            },
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }       
        base.id()
    }
}
