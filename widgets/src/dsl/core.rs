use bevy::text::{Font, Text, TextSection, TextStyle, BreakLineOn, Text2dBounds, TextLayoutInfo};
use bevy::prelude::{Vec2, Image, Handle, Color, Rect};
use bevy_aoui::bundles::BuildGlobalBundle;


use crate::widget_extension;


widget_extension!(pub struct FrameBuilder {}, this, commands);

widget_extension!(
    pub struct SpriteBuilder {
        pub sprite: Handle<Image>,
        pub size: Option<Vec2>,
        pub color: Option<Color>,
        pub rect: Option<Rect>,
        pub flip: [bool; 2],
    },
    this, commands,
    components: (
        bevy::prelude::Sprite {
            custom_size: this.size,
            color: this.color.unwrap_or(Color::WHITE),
            rect: this.rect,
            flip_x: this.flip[0],
            flip_y: this.flip[1],
            ..Default::default()
        },
        this.sprite,
        BuildGlobalBundle::default()
    )
);


widget_extension!(
    pub struct TextBoxBuilder {
        pub text: String,
        pub font: Handle<Font>,
        /// Note if not specified this is `UNBOUNDED`.
        pub bounds: Option<Vec2>,
        pub color: Option<Color>,
        pub wrap: bool,
        pub break_line_on: Option<BreakLineOn>,
    },
    this, commands,
    components: (
        Text {
            sections: vec![TextSection::new(
                this.text,
                TextStyle {
                    font: this.font,
                    color: this.color.unwrap_or(Color::WHITE),
                    ..Default::default()
                }
            )],
            linebreak_behavior: if let Some(b) = this.break_line_on {
                b
            } else if this.wrap {
                BreakLineOn::WordBoundary
            } else {
                BreakLineOn::NoWrap
            },
            ..Default::default()
        },
        match this.bounds {
            Some(size) => Text2dBounds { size },
            None => Text2dBounds::UNBOUNDED,
        },
        TextLayoutInfo::default(),
        Into::<bevy::sprite::Anchor>::into(this.anchor),
        BuildGlobalBundle::default()
    )
);
