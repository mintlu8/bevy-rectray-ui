use bevy::render::color::Color;
use bevy_rectray::util::{DslFrom, DslInto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color8(pub [u8; 4]);

impl From<Color> for Color8 {
    fn from(value: Color) -> Self {
        Self(value.as_rgba_u8())
    }
}

impl From<Color8> for Color {
    fn from(Color8([r, g, b, a]): Color8) -> Self {
        Color::rgba_u8(r, g, b, a)
    }
}

impl DslFrom<Color> for Color8 {
    fn dfrom(value: Color) -> Self {
        Self(value.as_rgba_u8())
    }
}

impl DslInto<Color> for Color8 {
    fn dinto(self) -> Color {
        let Color8([r, g, b, a]) = self;
        Color::rgba_u8(r, g, b, a)
    }
}

#[macro_export]
macro_rules! color8 {
    ($tt: tt) => {
        $bevy_rectray::color!($tt).as_rgba_u8()
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Palette {
    pub background: Color8,
    pub foreground: Color8,
    pub stroke: Color8,
    /// Used for dividers
    pub stroke_lite: Color8,
    /// Used for selection
    pub background_lite: Color8,
    /// Used for supporting text
    pub foreground_lite: Color8,

    pub on_foreground: Color8,

    pub foreground_stroke: Color8,
}

impl Palette {
    pub fn background(&self) -> Color {
        self.background.into()
    }

    pub fn foreground(&self) -> Color {
        self.foreground.into()
    }

    pub fn stroke(&self) -> Color {
        self.stroke.into()
    }

    pub fn background_lite(&self) -> Color {
        self.background_lite.into()
    }

    pub fn foreground_lite(&self) -> Color {
        self.foreground_lite.into()
    }

    pub fn stroke_lite(&self) -> Color {
        self.stroke_lite.into()
    }

    pub fn on_foreground(&self) -> Color {
        self.on_foreground.into()
    }

    pub fn foreground_stroke(&self) -> Color {
        self.foreground_stroke.into()
    }
}
/// Create a palette struct, every field must be a color.
///
/// ```
/// palette! {
///     foreground: red,
///     background: green,
/// }
/// ```
/// Translates to:
/// ```
/// Palette {
///     foreground: color!(red),
///     background: color!(green),
///     ..Default::default()
/// }
/// ```
#[macro_export]
macro_rules! palette {
    {$($field: ident: $color: tt),* $(,)?} => {
        $crate::style::Palette {
            $($field: $crate::aoui::color!($color).into(),)*
            ..Default::default()
        }
    };
    // hack for bevy_rectray's propagation
    ($_: tt {$($field: ident: $color: tt),* $(,)?}) => {
        $crate::style::Palette {
            $($field: $crate::aoui::color!($color).into(),)*
            ..Default::default()
        }
    };
}
