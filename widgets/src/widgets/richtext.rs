//! Format
//! 
//! # Markdown
//! 
//! We use a subset of markdown syntax
//! 
//! * Itelics: `*text*`
//! * Bold: `**text**`
//! * Bold Italics: `***text***`
//! * Underline: `__text__` (not implemented)
//! * Strikethroigh: `~~text~~`
//! 
//! Ascii whitespaces are either rendered as one space or linebreaks.
//! Use a unicode space if you want multiple spaces.
//! 
//! Tabs are not supported.
//! 
//! `N` consecutive newlines as rendered as `N-1` newlines.
//! use {br} for a newline.
//! 
//! # Control Codes
//! 
//! The general syntax is {code} or {code:text}. All control codes
//! are case insensitive. {code} is in effect until overwitten, 
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
//! This modifies the anchor of text segments. Changes on the main
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

use std::{collections::HashMap, hash::{Hash, BuildHasher}};

use bevy::{asset::{Handle, Assets}, text::{Font, Text, TextStyle}, ecs::{entity::Entity, system::{Commands, Query, Res}, bundle::Bundle, component::Component, query::Changed}, render::color::Color, math::Vec2};
use bevy_aoui::{Transform2D, Anchor, bundles::{AoUITextBundle, AoUIBundle}, SetEM, Dimension};
use bevy_aoui::LayoutControl;
/// This widget always has the width of a space and line height of a widget.
#[derive(Debug, Clone, Component)]
pub struct FontSpace {
    font: Handle<Font>
}

pub fn synchronize_spaces(mut query: Query<(&FontSpace, &mut Dimension), Changed<Dimension>>, fonts: Res<Assets<Font>> ){
    use ab_glyph::{Font, ScaleFont};
    query.par_iter_mut().for_each(|(font, mut dimension)| {
        if let Some(font) = fonts.get(&font.font) {
            let font = font.font.as_scaled(dimension.em);
            let width = font.h_advance(font.glyph_id(' '));
            let height = font.height();
            dimension.size = Vec2::new(width, height)
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
        HashMap::get(&self, name).cloned().unwrap_or_default()
    }
}

impl<'t, H: BuildHasher> FontFetcher for HashMap<&'t str, Handle<Font>, H> {
    fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
        HashMap::get(&self, name).cloned().unwrap_or_default()
    }
}

impl<'t, H: BuildHasher> FontFetcher for HashMap<(&'t str, FontStyle), Handle<Font>, H> {
    fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
        HashMap::get(&self, &(name, style)).cloned().unwrap_or_else(
            || HashMap::get(&self, &(name, FontStyle::None)).cloned().unwrap_or_default())
    }
}

impl<'t, H: BuildHasher> FontFetcher for HashMap<(String, FontStyle), Handle<Font>, H> {
    fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
        HashMap::get(&self, &(name.to_owned(), style)).cloned().unwrap_or_else(
            || HashMap::get(&self, &(name.to_owned(), FontStyle::None)).cloned().unwrap_or_default())
    }
}

const _: () = {
    use bevy::utils::HashMap;
    impl<'t> FontFetcher for HashMap<String, Handle<Font>> {
        fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
            HashMap::get(&self, name).cloned().unwrap_or_default()
        }
    }
    
    impl<'t> FontFetcher for HashMap<&'t str, Handle<Font>> {
        fn get(&self, name: &str, _: FontStyle) -> Handle<Font> {
            HashMap::get(&self, name).cloned().unwrap_or_default()
        }
    }
    
    impl<'t> FontFetcher for HashMap<(&'t str, FontStyle), Handle<Font>> {
        fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
            HashMap::get(&self, &(name, style)).cloned().unwrap_or_else(
                || HashMap::get(&self, &(name, FontStyle::None)).cloned().unwrap_or_default())
        }
    }
    
    impl<'t> FontFetcher for HashMap<(String, FontStyle), Handle<Font>> {
        fn get(&self, name: &str, style: FontStyle) -> Handle<Font> {
            HashMap::get(&self, &(name.to_owned(), style)).cloned().unwrap_or_else(
                || HashMap::get(&self, &(name.to_owned(), FontStyle::None)).cloned().unwrap_or_default())
        }
    }
};


enum RichTextScope {
    Font,
    Color,
    Size,
    Anchor,
}


fn newlines(s: &str) -> usize {
    s.chars().filter(|x| *x == '\n').count()
}

fn is_ws(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_whitespace())
}

fn hex1(s: u8) -> f32 {
    (
        match s {
            b'0'..=b'9' => s - b'0',
            b'a'..=b'z' => s - b'a' + 10 as u8,
            _ => panic!("Invalid hex number {s}")
        } * 0x11
    ) as f32 / 255.0
}

fn hex2(a: u8, b: u8) -> f32 {
    (
        match a {
            b'0'..=b'9' => a - b'0',
            b'a'..=b'z' => a - b'a' + 10 as u8,
            _ => panic!("Invalid hex number {a}")
        } * 16 + 
        match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'z' => b - b'a' + 10 as u8,
            _ => panic!("Invalid hex number {b}")
        }
    ) as f32 / 255.0
}

pub struct RichTextBuilder<'t, 'w, 's, F: FontFetcher, B: Bundle + Clone = ()>{
    /// This will be bundled into every text children
    bundle: B,
    /// This determines the inserted `LinebreakBundle`'s height.
    line_gap:(Handle<Font>, SetEM),
    commands: &'t mut Commands<'w, 's>,
    font: F,
    style: FontStyle,
    color_stack: Vec<Color>,
    size_stack: Vec<SetEM>,
    font_stack: Vec<String>,
    anchor_stack: Vec<Anchor>,
    buffer: Vec<Entity>,
    pop_stack: Vec<RichTextScope>,
    ignore_space: bool,
}

impl<'a, 'w, 's, F: FontFetcher> RichTextBuilder<'a, 'w, 's, F> {
    pub fn new(commands: &'a mut Commands<'w, 's>, font: F) -> Self {
        Self { 
            bundle: (), 
            line_gap: (font.default(), SetEM::None),
            commands, 
            font, 
            style: FontStyle::None, 
            color_stack: Vec::new(),
            size_stack: Vec::new(),
            font_stack: Vec::new(),
            anchor_stack: Vec::new(),
            buffer: Vec::new(), 
            pop_stack: Vec::new(),
            ignore_space: false,
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
        if self.s.len() == 0 {
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
        let RichTextBuilder { bundle:_, line_gap, commands, font, style, color_stack, size_stack, font_stack, anchor_stack, buffer, pop_stack, ignore_space } = self;
        let bundle = bun;
        RichTextBuilder { bundle, line_gap, commands, font, style, color_stack, size_stack, font_stack, anchor_stack, buffer, pop_stack, ignore_space }
    }

    #[must_use]
    pub fn configure_size(mut self, font: Handle<Font>, size: impl Into<SetEM>) -> Self{
        self.line_gap = (font, size.into());
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self{
        self.color_stack.push(color);
        self
    }

    #[must_use]
    pub fn with_size(mut self, size: impl Into<SetEM>) -> Self{
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

    /// Edit the space handling strategy, if set, spaces are rendered.
    /// 
    /// If not set, you can use the margin of the `paragraph` layout
    /// to simulate spaces.
    /// 
    /// # Limitations:
    /// 
    /// ## Margin mode
    /// 
    /// * Not portable across languages. 
    /// * Must group punctuations with style groups.
    /// 
    /// ## Space mode
    /// 
    /// * Spaces will disrupt flow.
    #[must_use]
    pub fn with_ignore_space(mut self, ignore_space: bool) -> Self{
        self.ignore_space = ignore_space;
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

    fn push_size(&mut self, v: SetEM, scoped: bool) {
        if !scoped {
            self.size_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Size);
        self.size_stack.push(v);
    }

    fn size(&self) -> SetEM {
        self.size_stack.last().copied().unwrap_or(SetEM::None)
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

    fn push_anchor(&mut self, v: Anchor, scoped: bool) {
        if !scoped {
            self.anchor_stack.pop();
        }
        self.pop_stack.push(RichTextScope::Anchor);
        self.anchor_stack.push(v);
    }

    fn anchor(&self) -> Anchor {
        self.anchor_stack.last().copied().unwrap_or(Anchor::CenterLeft)
    }

    pub fn push_bundle(&mut self, bun: impl Bundle) {
        let anchor = self.anchor();
        let entity = self.commands.spawn(bun).insert(
            Transform2D::UNIT.with_anchor(anchor)
        ).id();
        self.buffer.push(entity);
    }

    pub fn push_str(&mut self, s: &str) {
        use xi_unicode::LineBreakIterator;

        macro_rules! spawn {
            ($s: expr) => {
                {
                    let anchor = self.anchor();
                    self.buffer.push(self.commands.spawn(
                        AoUITextBundle {
                            dimension: bevy_aoui::Dimension {
                                set_em: self.size(),
                                ..Default::default()
                            },
                            text: Text::from_section($s, TextStyle{
                                font: self.font.get(self.font(), self.style),
                                font_size: 0.0,
                                color: self.color(),
                            }).with_no_wrap(),
                            ..Default::default()
                        }
                    )
                    .insert(self.bundle.clone())
                    .insert(Transform2D::UNIT.with_anchor(anchor))
                    .id());
                }
            };
        }

        macro_rules! line_gap {
            () => {
                self.buffer.push(self.commands.spawn((
                    AoUIBundle{
                        dimension: Dimension {
                            set_em: self.line_gap.1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    LayoutControl::LinebreakMarker,
                    FontSpace {
                        font: self.line_gap.0.clone()
                    }
                )).id())
            };
        }

        let mut last_space = 0;

        macro_rules! space {
            () => {
                if !self.ignore_space && self.buffer.len() != last_space { 
                    last_space = self.buffer.len() + 1;
                    self.buffer.push(self.commands.spawn((
                        AoUIBundle{
                            transform: Transform2D {
                                anchor: self.anchor(),
                                ..Default::default()
                            },
                            dimension: Dimension {
                                set_em: self.size(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        FontSpace {
                            font: self.font.get(self.font(), self.style),
                        },
                        LayoutControl::WhiteSpace,
                    )).id())
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
                    let mut cc = iter.next().expect("Brackets Mismatch1");
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
                        cc = iter.next().expect("Brackets Mismatch2");
                        if cc == "}" {
                            spawn!(prefix);
                            return;
                        }
                    }
                    let scoped = if cc.ends_with(":") {
                        let len = cc.len();
                        cc = &cc[..len - 1];
                        true
                    } else {
                        match iter.next() {
                            Some(":") => true,
                            Some("}") => false,
                            Some(cc) => panic!("Expected ':' or '}}', found {}", cc),
                            None => panic!("Expected ':' or '}}'."),
                        }
                    };
                    match cc.to_lowercase().as_str() {
                        "br" => line_gap!(),
                        "left" => self.push_anchor(Anchor::CenterLeft, scoped),
                        "right" => self.push_anchor(Anchor::CenterRight, scoped),
                        "top" => self.push_anchor(Anchor::TopCenter, scoped),
                        "bottom" => self.push_anchor(Anchor::BottomCenter, scoped),
                        "center" => self.push_anchor(Anchor::Center, scoped),
                        "centerleft" => self.push_anchor(Anchor::CenterLeft, scoped),
                        "centerright" => self.push_anchor(Anchor::CenterRight, scoped),
                        "topcenter" => self.push_anchor(Anchor::TopCenter, scoped),
                        "bottomcenter" => self.push_anchor(Anchor::BottomCenter, scoped),
                        "topleft" => self.push_anchor(Anchor::TopLeft, scoped),
                        "topright" => self.push_anchor(Anchor::TopRight, scoped),
                        "bottomleft" => self.push_anchor(Anchor::BottomLeft, scoped),
                        "bottomright" => self.push_anchor(Anchor::BottomRight, scoped),
                        cc => match prefix {
                            Some('@') => self.push_font(cc.to_owned(), scoped),
                            Some('+') => {
                                let size = cc.parse().expect(&format!("{} is not a valid font size.", cc));
                                self.push_size(SetEM::Pixels(size), scoped);
                            },
                            Some('*') => {
                                let size = cc.parse().expect(&format!("{} is not a valid font size.", cc));
                                self.push_size(SetEM::Ems(size), scoped);
                            },
                            Some('#') => {
                                let b = cc.as_bytes();
                                let color = match b {
                                    [a,b,c] => Color::rgba_linear(hex1(*a), hex1(*b), hex1(*c), 1.0),
                                    [a,b,c,d] => Color::rgba_linear(hex1(*a), hex1(*b), hex1(*c), hex1(*d)),
                                    [a,b,c,d,e,f] => Color::rgba_linear(hex2(*a, *b), hex2(*c, *d), hex2(*e, *f), 1.0),
                                    [a,b,c,d,e,f,g,h] => Color::rgba_linear(hex2(*a, *b), hex2(*c, *d), hex2(*e, *f), hex2(*g, *h)),
                                    _ => panic!("Invalid hex color {cc}, has to be of length 3, 4, 6 or 8.")
                                };
                                self.push_color(color, scoped);
                            },
                            Some(pfx) => panic!("Invalid prefix {pfx}"),
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
                                    panic!("Invalid control code {cc}");
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
                            None => panic!("brackets mismatch"),
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
    }

}

