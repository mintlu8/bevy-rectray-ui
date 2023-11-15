use bevy::{math::Vec2, sprite::Anchor};
use bevy_aoui::{Size2, SetEM, Alignment, FlexDir};

use super::convert::DslInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hitbox<T: DslInto<OneOrTwo<Vec2>>> {
    Rect(T),
    Ellipse(T),
}

impl<T: DslInto<OneOrTwo<Vec2>>> DslInto<Option<bevy_aoui::Hitbox>> for Hitbox<T> {
    fn dinto(self) -> Option<bevy_aoui::Hitbox> {
        Some(match self {
            Hitbox::Rect(scale) => bevy_aoui::Hitbox {
                shape: bevy_aoui::HitboxShape::Rect,
                scale: scale.dinto().0,
            },
            Hitbox::Ellipse(scale) =>  bevy_aoui::Hitbox {
                shape: bevy_aoui::HitboxShape::Rect,
                scale: scale.dinto().0,
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AoUISpacialConsts {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl DslInto<Anchor> for AoUISpacialConsts {
    fn dinto(self) -> Anchor {
        match self {
            AoUISpacialConsts::TopLeft => Anchor::TopLeft,
            AoUISpacialConsts::TopCenter => Anchor::TopCenter,
            AoUISpacialConsts::TopRight => Anchor::TopRight,
            AoUISpacialConsts::CenterLeft => Anchor::CenterLeft,
            AoUISpacialConsts::Center => Anchor::Center,
            AoUISpacialConsts::CenterRight => Anchor::CenterRight,
            AoUISpacialConsts::BottomLeft => Anchor::BottomLeft,
            AoUISpacialConsts::BottomCenter => Anchor::BottomCenter,
            AoUISpacialConsts::BottomRight => Anchor::BottomRight,
            AoUISpacialConsts::Top => Anchor::TopCenter,
            AoUISpacialConsts::Bottom => Anchor::BottomCenter,
            AoUISpacialConsts::Left => Anchor::CenterLeft,
            AoUISpacialConsts::Right => Anchor::CenterRight,
            c => panic!("{:?} is not an Anchor.", c),
        }
    }
}

impl DslInto<Alignment> for AoUISpacialConsts {
    fn dinto(self) -> Alignment {
        match self {
            AoUISpacialConsts::Center => Alignment::Center,
            AoUISpacialConsts::Top => Alignment::Top,
            AoUISpacialConsts::Bottom => Alignment::Bottom,
            AoUISpacialConsts::Left => Alignment::Left,
            AoUISpacialConsts::Right => Alignment::Right,
            c => panic!("{:?} is not an Alignment.", c),
        }
    }
}

impl DslInto<FlexDir> for AoUISpacialConsts {
    fn dinto(self) -> FlexDir {
        match self {
            AoUISpacialConsts::LeftToRight => FlexDir::LeftToRight,
            AoUISpacialConsts::RightToLeft => FlexDir::RightToLeft,
            AoUISpacialConsts::TopToBottom => FlexDir::TopToBottom,
            AoUISpacialConsts::BottomToTop => FlexDir::BottomToTop,
            c => panic!("{:?} is not an FlexDir.", c),
        }
    }
}

///
/// ```
/// # use bevy_aoui_widgets::color;
/// color!(Red400);
/// ```
#[macro_export]
macro_rules! color {
    ($color: tt) => {
        ::bevy_aoui_widgets::dsl::rgbaf!(
            ::bevy::prelude::Color::RgbaLinear, 
            $color => {red, green, blue, alpha}
        ).as_rgba()
    };
}


/// Convert degrees to radians
pub fn degrees(f: impl DslInto<f32>) -> f32{
    f32::to_radians(f.dinto())
}

/// Convert `Vec2` to radians
pub fn angle(f: impl DslInto<Vec2>) -> f32{
    let v = f.dinto();
    f32::atan2(v.y, v.x)
}

impl DslInto<SetEM> for i32 {
    fn dinto(self) -> SetEM {
        SetEM::Pixels(self as f32)
    }
}

impl DslInto<SetEM> for f32 {
    fn dinto(self) -> SetEM {
        SetEM::Pixels(self)
    }
}

pub fn px(f: impl DslInto<f32>) -> SetEM {
    SetEM::Pixels(f.dinto())
}

pub fn em(f: impl DslInto<f32>) -> SetEM {
    SetEM::Ems(f.dinto())
}

pub fn rem(f: impl DslInto<f32>) -> SetEM {
    SetEM::Rems(f.dinto())
}

pub fn percent(f: impl DslInto<f32>) -> SetEM {
    SetEM::Pixels(f.dinto() / 100.0)
}

/// Vec2 extractor that accepts a singular value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OneOrTwo<T>(pub T);

impl<T> DslInto<OneOrTwo<T>> for T {
    fn dinto(self) -> OneOrTwo<T> {
        OneOrTwo(self)
    }
}

impl<T> DslInto<OneOrTwo<[T; 2]>> for T where T : Clone {
    fn dinto(self) -> OneOrTwo<[T; 2]> {
        OneOrTwo([self.clone(), self])
    }
}

macro_rules! impl_one_or_two {
    ($ty: ty, $x: ident, $y: ident, $expr: expr) => {
        
impl DslInto<OneOrTwo<$ty>> for i32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for f32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for [i32; 2] {
    fn dinto(self) -> OneOrTwo<$ty> {
        let [$x, $y] = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for [f32; 2] {
    fn dinto(self) -> OneOrTwo<$ty> {
        let [$x, $y] = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (i32, i32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (f32, i32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (i32, f32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (f32, f32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}
    };
}

impl_one_or_two!(Vec2, x, y, Vec2::new(x as f32, y as f32));
impl_one_or_two!(Size2, x, y, Size2::pixels(x as f32, y as f32));


#[macro_export]
macro_rules! size {
    ($x: tt) => {
        (::bevy_aoui::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt) => {
        (::bevy_aoui::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt px) => {
        (::bevy_aoui::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt px) => {
        (::bevy_aoui::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt em) => {
        (::bevy_aoui::SizeUnit::Em, $x as f32)
    };
    (-$x: tt em) => {
        (::bevy_aoui::SizeUnit::Em, -($x as f32))
    };
    ($x: tt rem) => {
        (::bevy_aoui::SizeUnit::Rem, $x as f32)
    };
    (-$x: tt rem) => {
        (::bevy_aoui::SizeUnit::Rem, -($x as f32))
    };
    ($x: tt %) => {
        (::bevy_aoui::SizeUnit::Percent, $x as f32 / 100.0)
    };
    (-$x: tt %) => {
        (::bevy_aoui::SizeUnit::Percent, -($x as f32) / 100.0)
    };
    (1 - $x: tt px) => {
        (::bevy_aoui::SizeUnit::MarginPx, $x as f32)
    };
    (1 + $x: tt px) => {
        (::bevy_aoui::SizeUnit::MarginPx, -($x as f32))
    };
    (1 - $x: tt em) => {
        (::bevy_aoui::SizeUnit::MarginEm, $x as f32)
    };
    (1 + $x: tt em) => {
        (::bevy_aoui::SizeUnit::MarginEm, -($x as f32))
    };
    (1 - $x: tt rem) => {
        (::bevy_aoui::SizeUnit::MarginRem, $x as f32)
    };
    (1 + $x: tt rem) => {
        (::bevy_aoui::SizeUnit::MarginRem, -($x as f32))
    };
}


/// Construct a [`Size2`](bevy_aoui::Size2) through CSS like syntax.
/// 
/// # Examples
/// ```
/// # use bevy_aoui_widgets::size2;
/// # let PI = 3.0;
/// // We perform auto float conversion.
/// size2!([40, 50.5]);
/// // Supply a unit like this
/// size2!([1, 1] rem);
/// // Supply multiple unit types like this.
/// size2!([40%, 1 em]);
/// // Aside from the negative sign
/// // expressions need to be in wrapped parenthesis or braces.
/// size2!([-3 px, (PI * 2.0) rem]);
/// size2!([-3 px, {
///     let pi = 3.0;
///     pi * 2.0
/// } rem]);
/// // `1 - 2px` means `100% - 2px`, or 2px smaller than parent dimension.
/// size2!([1 - 2 px, 1 + 4 em]);
/// // or expressed as
/// size2!(1 - [4.5, 6.6] px);
/// ```
/// 
/// # Note
/// 
/// * `1px` is not valid rust syntax, always use `1 px`.
#[macro_export]
macro_rules! size2 {
    ([$($tt:tt)*]) => {
        $crate::size2!(@accumulate [] [$($tt)*])
    };
    (@accumulate [$($tt1:tt)*] []) => {
        compile_error!("Expected 2 expressions, found 1.")
    };
    (@accumulate [$($tt1:tt)*] [, $($tt2:tt)*]) => {
        ::bevy_aoui::Size2::new($crate::size!($($tt1)*), $crate::size!($($tt2)*))
    };
    (@accumulate [$($tt1:tt)*] [$tt:tt $($tt2:tt)*]) => {
        $crate::size2!(@accumulate [$($tt1)* $tt] [$($tt2)*])
    };
    ([$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([$x $unit, $y $unit])
    };
    (1 - [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([1 - $x $unit, 1 - $y $unit])
    };
    (1 + [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([1 + $x $unit, 1 + $y $unit])
    };
}
