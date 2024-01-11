//! WIP Rich Text implementation for `bevy_aoui`.
//!
//! Format
//!
//! # Markdown
//!
//! We use a subset of markdown syntax
//!
//! * Italics: `*text*`
//! * Bold: `**text**`
//! * Bold Italics: `***text***`
//! * Underline: `__text__` (not implemented)
//! * Strikethrough: `~~text~~` (not implemented)
//!
//! Ascii whitespaces are either rendered as one space or linebreaks.
//! Use a unicode space if you want multiple spaces. Leading and trailing
//! whitespaces are always trimmed. Tabs are not supported.
//!
//! `N` consecutive newlines will be rendered as `N-1` newlines.
//!
//! # Control Codes
//!
//! The general syntax is {code} or {code:text}. All control codes
//! are case insensitive. {code} is in effect until overwritten,
//! {code:text} is in effect until it goes out of scope.
//!
//! * `{br}`
//!
//! Insert a new line.
//!
//! * `{*}` `{_}` `{~}`
//!
//! Escape for '*', '_', '~'
//!
//! `{left}` or `{topleft}`
//!
//! This modifies the anchor of text segments. Changes on the horizontal
//! axis will modify the text's alignment.
//!
//! * `{red}` or `{#FFAABBCC}`
//!
//! This modifies the color of text segments.
//!
//! * `{@OpenSans}`
//!
//! This modifies the font.
//!
//! * `{+14}`
//!
//! This sets the font size to 14
//!
//! * `{*2}`
//!
//! This sets the font size to 2 em
//!
//! * `{0}` - `{9}`
//!
//! Spawn an empty entity for future insertion.
//!
//! * `{zip: {red:a}.}`
//!
//! Zip prevents linebreaks inside by wrapping its contents inside a `compact` layout.
//! This is needed to preserve linebreak behavior across style groups.
//! Changing anchor inside is unspecified behavior.

use std::{collections::HashMap, hash::{Hash, BuildHasher}, num::ParseFloatError};
use bevy::render::view::RenderLayers;
use bevy::{asset::{Handle, Assets}, text::Font, render::color::Color, hierarchy::BuildChildren};
use bevy::ecs::{entity::Entity, system::{Query, Res}, bundle::Bundle, component::Component};
use crate::{Transform2D, Anchor, FontSize, Dimension, Size2, DimensionType, dimension::DimensionMut, dsl::AouiCommands};
use crate::layout::{Container, StackLayout};
use crate::bundles::AouiBundle;
use crate::layout::LayoutControl;
use crate::frame;

/// This widget always has the width of a space and line height of a widget.
#[derive(Debug, Clone, Component)]
pub struct GlyphSpace {
    font: Handle<Font>
}

pub fn synchronize_glyph_spaces(mut query: Query<(&GlyphSpace, DimensionMut)>, fonts: Res<Assets<Font>> ){
    use ab_glyph::{Font, ScaleFont};
    query.iter_mut().for_each(|(font, mut dimension)| {
        if let Some(font) = fonts.get(&font.font) {
            let font = font.font.as_scaled(dimension.dynamic.em);
            let width = font.h_advance(font.glyph_id(' '));
            let height = font.height();
            dimension.source.dimension = DimensionType::Owned(Size2::pixels(width, height));
        }
    })
}


tlbf::tlbf!(
    pub FontStyle: u8 {
        Bold,
        Italic,
    }
);

impl FontStyle {
    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0);
}

pub trait FontFetcher {
    fn get(&self, name: &str, style: FontStyle) -> Handle<Font>;

    fn default(&self) -> Handle<Font> {
        self.get("", FontStyle::None)
    }
}

impl FontFetcher for Handle<Font> {
    fn get(&self, _: &str, _: FontStyle) -> Handle<Font> {
        self.clone()
    }
}

impl<H: BuildHasher> FontFetcher for HashMap<String, Handle<Font>, H> {
    fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
        HashMap::get(self, name).cloned().unwrap_or_default()
    }
}

impl<'t, H: BuildHasher> FontFetcher for HashMap<&'t str, Handle<Font>, H> {
    fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
        HashMap::get(self, name).cloned().unwrap_or_default()
    }
}

impl<'t, H: BuildHasher> FontFetcher for HashMap<(&'t str, FontStyle), Handle<Font>, H> {
    fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
        HashMap::get(self, &(name, style)).cloned().unwrap_or_else(
            || HashMap::get(self, &(name, FontStyle::None)).cloned().unwrap_or_default())
    }
}

impl<H: BuildHasher> FontFetcher for HashMap<(String, FontStyle), Handle<Font>, H> {
    fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
        HashMap::get(self, &(name.to_owned(), style)).cloned().unwrap_or_else(
            || HashMap::get(self, &(name.to_owned(), FontStyle::None)).cloned().unwrap_or_default())
    }
}

const _: () = {
    use bevy::utils::HashMap;
    impl FontFetcher for HashMap<String, Handle<Font>> {
        fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
            HashMap::get(self, name).cloned().unwrap_or_default()
        }
    }

    impl<'t> FontFetcher for HashMap<&'t str, Handle<Font>> {
        fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
            HashMap::get(self, name).cloned().unwrap_or_default()
        }
    }

    impl<'t> FontFetcher for HashMap<(&'t str, FontStyle), Handle<Font>> {
        fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
            HashMap::get(self, &(name, style)).cloned().unwrap_or_else(
                || HashMap::get(self, &(name, FontStyle::None)).cloned().unwrap_or_default())
        }
    }

    impl FontFetcher for HashMap<(String, FontStyle), Handle<Font>> {
        fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
            HashMap::get(self, &(name.to_owned(), style)).cloned().unwrap_or_else(
                || HashMap::get(self, &(name.to_owned(), FontStyle::None)).cloned().unwrap_or_default())
        }
    }
};


enum RichTextScope {
    Font,
    Color,
    Size,
    Anchor,
    Zip,
}


fn newlines(s: &str) -> usize {
    s.chars().filter(|x| *x == '\n').count()
}

fn is_ws(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_whitespace())
}

fn hex1(s: u8) -> Result<f32, RichTextError> {
    Ok((
        match s {
            b'0'..=b'9' => s - b'0',
            b'a'..=b'z' => s - b'a' + 10_u8,
            _ => return Err(RichTextError::InvalidHexDigit(s))
        } * 0x11
    ) as f32 / 255.0)
}

fn hex2(a: u8, b: u8) -> Result<f32, RichTextError> {
    Ok((
        match a {
            b'0'..=b'9' => a - b'0',
            b'a'..=b'z' => a - b'a' + 10_u8,
            _ => return Err(RichTextError::InvalidHexDigit(a))
        } * 16 +
        match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'z' => b - b'a' + 10_u8,
            _ => return Err(RichTextError::InvalidHexDigit(b))
        }
    ) as f32 / 255.0)
}

pub struct RichTextBuilder<'t, 'w, 's, F: FontFetcher, B: Bundle + Clone = ()>{
    /// This will be bundled into every text children
    bundle: B,
    /// This determines the inserted `LinebreakBundle`'s height.
    line_gap:(Handle<Font>, FontSize),
    commands: &'t mut AouiCommands<'w, 's>,
    font: F,
    style: FontStyle,
    color_stack: Vec<Color>,
    size_stack: Vec<FontSize>,
    font_stack: Vec<String>,
    anchor_stack: Vec<Anchor>,
    zip: Option<Vec<Entity>>,
    buffer: Vec<Entity>,
    pop_stack: Vec<RichTextScope>,
    layer: u8,
}

impl<'a, 'w, 's, F: FontFetcher> RichTextBuilder<'a, 'w, 's, F> {
    pub fn new(commands: &'a mut AouiCommands<'w, 's>, font: F) -> Self {
        Self {
            bundle: (),
            line_gap: (font.default(), FontSize::None),
            commands,
            font,
            style: FontStyle::None,
            color_stack: Vec::new(),
            size_stack: Vec::new(),
            font_stack: Vec::new(),
            anchor_stack: Vec::new(),
            zip: None,
            buffer: Vec::new(),
            pop_stack: Vec::new(),
            layer: 0,
        }
    }
}

struct FindSplit<'t, 'a> {
    s: &'t str,
    pat: &'a [char],
    one: bool,
}

impl<'t> Iterator for FindSplit<'t, '_> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }
        if self.one {
            let (result, s) = self.s.split_at(1);
            self.s = s;
            self.one = false;
            Some(result)
        } else if let Some(pos) = self.s.find(self.pat){
            if pos == 0 {
                let (result, s) = self.s.split_at(1);
                self.s = s;
                self.one = false;
                Some(result)
            } else {
                let (result, s) = self.s.split_at(pos);
                self.s = s;
                self.one = true;
                Some(result)
            }
        } else {
            let result = self.s;
            self.s = "";
            Some(result)
        }
    }
}

impl<'a, 'w, 's, F: FontFetcher, B: Bundle + Clone> RichTextBuilder<'a, 'w, 's, F, B> {
    #[must_use]
    pub fn build(self) -> Vec<Entity> {
        self.buffer
    }

    #[must_use]
    pub fn with_bundle<B2: Bundle + Clone>(self, bun: B2) -> RichTextBuilder<'a, 'w, 's, F, B2>{
        let RichTextBuilder { bundle:_, line_gap, commands, font, style, layer, color_stack, size_stack, font_stack, anchor_stack, zip, buffer, pop_stack } = self;
        let bundle = bun;
        RichTextBuilder { bundle, line_gap, commands, font, style, layer, color_stack, size_stack, font_stack, anchor_stack, zip, buffer, pop_stack }
    }

    #[must_use]
    pub fn configure_size(mut self, font: Handle<Font>, size: impl Into<FontSize>) -> Self{
        self.line_gap = (font, size.into());
        self
    }

    #[must_use]
    pub fn with_layer(mut self, layer: u8) -> Self{
        self.layer = layer;
        self
    }


    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self{
        self.color_stack.push(color);
        self
    }

    #[must_use]
    pub fn with_size(mut self, size: impl Into<FontSize>) -> Self{
        self.size_stack.push(size.into());
        self
    }

    #[must_use]
    pub fn with_font(mut self, font: impl Into<String>) -> Self{
        self.font_stack.push(font.into());
        self
    }

    #[must_use]
    pub fn with_anchor(mut self, anchor: Anchor) -> Self{
        self.anchor_stack.push(anchor);
        self
    }

    fn push_font(&mut self, v: String, scoped: bool) {
        if !scoped {
            self.font_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Font);
        self.font_stack.push(v);
    }

    fn font(&self) -> &str {
        self.font_stack.last().map(|x| x.as_str()).unwrap_or("")
    }

    fn push_size(&mut self, v: FontSize, scoped: bool) {
        if !scoped {
            self.size_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Size);
        self.size_stack.push(v);
    }

    fn size(&self) -> FontSize {
        self.size_stack.last().copied().unwrap_or(FontSize::None)
    }

    fn push_color(&mut self, v: Color, scoped: bool) {
        if !scoped {
            self.color_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Color);
        self.color_stack.push(v);
    }

    fn color(&self) -> Color {
        self.color_stack.last().copied().unwrap_or(Color::WHITE)
    }

    fn push_zip(&mut self) -> Result<(), RichTextError> {
        if self.zip.is_some() {
            return Err(RichTextError::ZipInZip);
        }
        self.zip = Some(Vec::new());
        self.pop_stack.push(RichTextScope::Zip);
        Ok(())
    }


    fn push_anchor(&mut self, v: Anchor, scoped: bool) {
        if !scoped {
            self.anchor_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Anchor);
        self.anchor_stack.push(v);
    }

    fn anchor(&self) -> Anchor {
        self.anchor_stack.last().copied().unwrap_or(Anchor::CENTER_LEFT)
    }

    pub fn push_bundle(&mut self, bun: impl Bundle) {
        let anchor = self.anchor();
        let entity = self.commands.spawn_bundle(bun).insert(
            Transform2D::UNIT.with_anchor(anchor)
        ).id();
        self.buffer.push(entity);
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), RichTextError>{
        use xi_unicode::LineBreakIterator;

        macro_rules! spawn {
            ($s: expr) => {
                {
                    let entity = $crate::text! ((self.commands) {
                        text: $s,
                        anchor: self.anchor(),
                        font_size: self.size(),
                        font: self.font.get(self.font(), self.style),
                        color: self.color(),
                        extra: self.bundle.clone(),
                    });
                    // unfortunately the macro doesn't work for this
                    if self.layer != 0 {
                        self.commands.entity(entity).insert(RenderLayers::layer(self.layer));
                    }
                    if let Some(zip) = &mut self.zip {
                        zip.push(entity);
                    } else {
                        self.buffer.push(entity)
                    };
                }
            };
        }

        macro_rules! line_gap {
            () => {
                self.buffer.push(self.commands.spawn_bundle((
                    AouiBundle{
                        dimension: Dimension {
                            font_size: self.line_gap.1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    LayoutControl::LinebreakMarker,
                    GlyphSpace {
                        font: self.line_gap.0.clone()
                    }
                )).id())
            };
        }

        let mut last_space = 0;

        macro_rules! space {
            () => {
                if self.buffer.len() != last_space {
                    last_space = self.buffer.len() + 1;
                    let entity = frame!((self.commands) {
                        anchor: self.anchor(),
                        font_size: self.size(),
                        extra: GlyphSpace {
                            font: self.font.get(self.font(), self.style),
                        },
                        extra: LayoutControl::WhiteSpace,
                    });
                    if let Some(zip) = &mut self.zip {
                        zip.push(entity);
                    } else {
                        self.buffer.push(entity)
                    };
                }
            };
        }
        let s = s.trim();

        let mut last = 0;
        let mut iter = LineBreakIterator::new(s).map(|(next, _)| {
            let string = &s[last..next];
            last = next;
            string
        }).flat_map(|s| FindSplit {
            s,
            pat: &['{', '}', ':', '@', '*', '+', '#', ' ', '\n', '\t'],
            one: false,
        }).peekable();

        while let Some(item) = iter.next() {
            match item {
                "{" => {
                    let mut cc = iter.next().ok_or(RichTextError::BracketsNotClosed)?;
                    let prefix = match cc {
                        "{" => { spawn!("{"); continue; }
                        "*" => Some('*'),
                        "+" => Some('+'),
                        "_" => Some('_'),
                        "~" => Some('~'),
                        "@" => Some('@'),
                        "#" => Some('#'),
                        _ => None,
                    };
                    if let Some(prefix) = prefix {
                        cc = iter.next().ok_or(RichTextError::BracketsNotClosed)?;
                        if cc == "}" {
                            spawn!(prefix);
                            break;
                        }
                    }
                    let scoped = if cc.ends_with(':') {
                        let len = cc.len();
                        cc = &cc[..len - 1];
                        true
                    } else {
                        match iter.next() {
                            Some(":") => true,
                            Some("}") => false,
                            Some(cc) => return Err(RichTextError::NotColonOrEndParam(cc.to_owned())),
                            None => return Err(RichTextError::NotColonOrEndParam("end of string.".to_owned())),
                        }
                    };
                    match cc.to_lowercase().as_str() {
                        "br" => line_gap!(),
                        "zip" => self.push_zip()?,
                        "left" => self.push_anchor(Anchor::CENTER_LEFT, scoped),
                        "right" => self.push_anchor(Anchor::CENTER_RIGHT, scoped),
                        "top" => self.push_anchor(Anchor::TOP_CENTER, scoped),
                        "bottom" => self.push_anchor(Anchor::BOTTOM_CENTER, scoped),
                        "center" => self.push_anchor(Anchor::CENTER, scoped),
                        "centerleft" => self.push_anchor(Anchor::CENTER_LEFT, scoped),
                        "centerright" => self.push_anchor(Anchor::CENTER_RIGHT, scoped),
                        "topcenter" => self.push_anchor(Anchor::TOP_CENTER, scoped),
                        "bottomcenter" => self.push_anchor(Anchor::BOTTOM_CENTER, scoped),
                        "topleft" => self.push_anchor(Anchor::TOP_LEFT, scoped),
                        "topright" => self.push_anchor(Anchor::TOP_RIGHT, scoped),
                        "bottomleft" => self.push_anchor(Anchor::BOTTOM_LEFT, scoped),
                        "bottomright" => self.push_anchor(Anchor::BOTTOM_RIGHT, scoped),
                        cc => match prefix {
                            Some('@') => self.push_font(cc.to_owned(), scoped),
                            Some('+') => {
                                let size = cc.parse()?;
                                self.push_size(FontSize::Pixels(size), scoped);
                            },
                            Some('*') => {
                                let size = cc.parse()?;
                                self.push_size(FontSize::Ems(size), scoped);
                            },
                            Some('#') => {
                                let b = cc.as_bytes();
                                let color = match b {
                                    [a,b,c] => Color::rgba_linear(hex1(*a)?, hex1(*b)?, hex1(*c)?, 1.0),
                                    [a,b,c,d] => Color::rgba_linear(hex1(*a)?, hex1(*b)?, hex1(*c)?, hex1(*d)?),
                                    [a,b,c,d,e,f] => Color::rgba_linear(hex2(*a, *b)?, hex2(*c, *d)?, hex2(*e, *f)?, 1.0),
                                    [a,b,c,d,e,f,g,h] => Color::rgba_linear(hex2(*a, *b)?, hex2(*c, *d)?, hex2(*e, *f)?, hex2(*g, *h)?),
                                    _ => return Err(RichTextError::InvalidHexColor(cc.to_owned()))
                                };
                                self.push_color(color, scoped);
                            },
                            Some(pfx) => return Err(RichTextError::UnsupportedPrefix(pfx)),
                            None => {
                                if let Some([r, g, b, a]) = parse_color::parse_flat_lower(cc) {
                                    let color = Color::rgba_linear(
                                        r as f32 / 255.0,
                                        g as f32 / 255.0,
                                        b as f32 / 255.0,
                                        a as f32 / 255.0,
                                    );
                                    self.push_color(color, scoped);
                                } else {
                                    return Err(RichTextError::InvalidControlCode(cc.to_owned()))
                                }
                            }
                        }
                    }
                }
                "*" => {
                    let mut flag = FontStyle::Italic;
                    if iter.peek() == Some(&"*") {
                        flag = FontStyle::Bold;
                        iter.next();
                        if iter.peek() == Some(&"*") {
                            flag = FontStyle::Bold | FontStyle::Italic;
                            iter.next();
                        }
                    }
                    self.style ^= flag;
                }
                "}" => {
                    match iter.peek() {
                        Some(&"}") => spawn!("}"),
                        _ => match self.pop_stack.pop() {
                            Some(RichTextScope::Anchor) => { self.anchor_stack.pop(); },
                            Some(RichTextScope::Color) => { self.color_stack.pop(); },
                            Some(RichTextScope::Font) => { self.font_stack.pop(); },
                            Some(RichTextScope::Size) => { self.size_stack.pop(); },
                            Some(RichTextScope::Zip) => {
                                let anchor = self.anchor();
                                self.buffer.push(self.commands.spawn_bundle((
                                    AouiBundle {
                                        dimension: Dimension {
                                            font_size: self.size(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    Container {
                                        layout: Box::new(StackLayout::HSTACK),
                                        margin: Size2::ZERO,
                                        padding: Size2::ZERO,
                                        range: Default::default(),
                                        maximum: usize::MAX,
                                    }
                                ))
                                .insert(Transform2D::UNIT.with_anchor(anchor))
                                .push_children(&self.zip.take().ok_or(RichTextError::HierarchyMismatch)?)
                                .id());
                            },
                            None => return Err(RichTextError::BracketsMismatch),
                        }
                    }
                },
                s if is_ws(s) => {
                    let mut lines = newlines(s);
                    loop {
                        match iter.peek() {
                            Some(ws) if is_ws(ws) => {
                                lines += newlines(ws);
                                iter.next();
                            }
                            Some(_) => {
                                match lines {
                                    0|1 => space!(),
                                    x => for _ in 0..x-1 {
                                        line_gap!();
                                    }
                                }
                                break;
                            }
                            None => break,
                        }
                    }
                },
                s => spawn!(s),
            }
        }
        Ok(())
    }

}

#[derive(Debug, thiserror::Error)]
pub enum RichTextError {
    #[error("Open brackets not closed. Maybe escape with '{{'?")]
    BracketsNotClosed,
    #[error("Closing Bracket '}}' found without an opening bracket '{{'. Maybe escape with '}}}}'?")]
    BracketsMismatch,
    #[error("Hierarchy mismatch, this is likely a bug.")]
    HierarchyMismatch,
    #[error("Prefix {} is reserved, but not implemented yet.", 0)]
    UnsupportedPrefix(char),
    #[error("Invalid control code {}", 0)]
    InvalidControlCode(String),
    #[error("Invalid font size: {}", 0)]
    InvalidFontSizeCode(#[from] ParseFloatError),
    #[error("Invalid hex color {}, has to be of length 3, 4, 6 or 8.", 0)]
    InvalidHexColor(String),
    #[error("Invalid hex digit 0x{}, expected '0-9|a-f'.", 0)]
    InvalidHexDigit(u8),
    #[error("Cannot zip in a zip block")]
    ZipInZip,
    #[error("Expected ':' or '}}', found {}.", 0)]
    NotColonOrEndParam(String)
}
