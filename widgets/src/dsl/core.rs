use bevy::{sprite::Anchor, prelude::{Vec2, Image, Handle, Color, Rect, Commands, Entity}, text::{Font, Text, TextSection, TextStyle, BreakLineOn, Text2dBounds}, math::bool};
use bevy_aoui::{Size2, SetEM, bundles::{AoUIBundle, AoUISpriteBundle, AoUITextBundle}, Hitbox};

// name based meta prgramming
macro_rules! transform2d {
    ($this: expr) => {
        ::bevy_aoui::Transform2D {
            center: $this.center,
            anchor: $this.anchor,
            parent_anchor: $this.parent_anchor,
            offset: $this.offset,
            rotation: $this.rotation,
            scale: match $this.scale{
                Some($crate::dsl::prelude::OneOrTwo(vec)) => vec,
                None => ::bevy::math::Vec2::ONE,
            },
            z: $this.z
        }
    };
}

macro_rules! dimension {
    ($this: expr) => {
        match $this.dimension {
            Some(size) => ::bevy_aoui::Dimension::owned(size).with_em($this.em),
            None => ::bevy_aoui::Dimension::COPIED.with_em($this.em),
        }
    }
}

macro_rules! common_plugins {
    ($this: expr, $commands: expr) => {
        if $this.linebreak {
            $commands.insert(::bevy_aoui::LayoutControl::Linebreak);
        }
        if let Some(hitbox) = $this.hitbox {
            $commands.insert(hitbox);
        }
    }
}

pub(crate) use transform2d;
pub(crate) use dimension;
pub(crate) use common_plugins;

use crate::dsl::DslInto;

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
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,
}

macro_rules! number_of_somes {
    ($($expr: expr),*) => {
        {
            let mut count = 0usize;
            $(if $expr.is_some() { count += 1 })*
            count
        }
    };
}


/// A Sized AoUI Component with no rendering.
#[derive(Debug, Default)]
pub struct Frame {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,
    /// If specified, produce `BuildGlobal`
    /// 
    /// Put `Inherit` here to use the default anchor.
    pub global: Option<Option<Anchor>>,
    /// If specified, produce `BuildTransform``
    pub transform: Option<Anchor>,
    /// If specified, produce `BuildSpacial`
    pub spacial: Option<Anchor>,
}

impl AoUIWidget for Frame {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                ..Default::default()
            },
        ));
        common_plugins!(self, base);
        match number_of_somes!(self.global, self.transform, self.spacial) {
            0 => (),
            1 => {
                if let Some(anc) = self.global {
                    base.insert(::bevy_aoui::BuildGlobal(anc));
                } else if let Some(anc) = self.transform {
                    base.insert(::bevy_aoui::BuildTransform(anc));
                } else if let Some(anc) = self.spacial {
                    base.insert(::bevy_aoui::BuildTransform(anc));
                }
            },
            _ => panic!("`global`, `transform`, `spacial` are mutually exclusive."),
        }
        base.id()
    }
}
   


/// A Sized AoUI Component with no rendering.
#[derive(Debug, Default)]
pub struct Sprite {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,

    pub sprite: Handle<Image>,
    pub size: Option<Vec2>,
    pub color: Option<Color>,
    pub rect: Option<Rect>,
    pub flip: [bool; 2],
}

impl AoUIWidget for Sprite {
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
                    flip_x: flip_x,
                    flip_y: flip_y,
                    ..Default::default()
                }, 
                texture: self.sprite,
                vis: self.visible.dinto(),
                ..Default::default()
            },
        ));
        common_plugins!(self, base);
        base.id()
    }
}

/// A Sized AoUI Component with no rendering.
#[derive(Debug, Default)]
pub struct TextBox {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,

    pub text: String,
    pub font: Handle<Font>,
    /// Note if not specified this is `UNBOUNDED`.
    pub bounds: Option<Vec2>,
    pub color: Option<Color>,
    pub wrap: bool,
    pub break_line_on: Option<BreakLineOn>,
}


impl AoUIWidget for TextBox {
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
        common_plugins!(self, base);
        base.id()
    }
}
