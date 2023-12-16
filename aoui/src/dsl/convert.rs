use bevy::{prelude::{Vec2, UVec2, IVec2, Rect}, sprite::Anchor, render::view::{VisibilityBundle, Visibility, RenderLayers}};
use crate::{Size2, Opacity, SizeUnit, FontSize};

use super::OneOrTwo;

/// The `From` trait for `bevy_aoui`'s DSL.
pub trait DslFrom<T> {
    fn dfrom(value: T) -> Self;
}

/// The `Into` trait for `bevy_aoui`'s DSL.
pub trait DslInto<T> {
    fn dinto(self) -> T;
}

impl<T, U> DslInto<U> for T where U: DslFrom<T> {
    fn dinto(self) -> U {
        U::dfrom(self)
    }
}

impl<T> DslFrom<T> for T {
    fn dfrom(value: T) -> Self {
        value
    }
}

impl<T> DslFrom<T> for Option<T> {
    fn dfrom(value: T) -> Self {
        Some(value)
    }
}

impl<'t, T> DslFrom<&'t T> for T where T: Clone{
    fn dfrom(value: &'t T) -> Self {
        value.clone()
    }
}

impl<'t, T> DslFrom<&'t mut T> for T where T: Clone{
    fn dfrom(value: &'t mut T) -> Self {
        value.clone()
    }
}

impl DslFrom<i32> for f32 {
    fn dfrom(value: i32) -> Self {
        value as f32
    }
}

impl DslFrom<usize> for f32 {
    fn dfrom(value: usize) -> Self {
        value as f32
    }
}

impl DslFrom<f32> for Opacity {
    fn dfrom(value: f32) -> Self {
        Opacity::new(value)
    }
}

impl DslFrom<bool> for Opacity {
    fn dfrom(value: bool) -> Self {
        Opacity::new(if value {1.0} else {0.0})
    }
}

impl DslFrom<char> for String {
    fn dfrom(value: char) -> Self {
        value.to_string()
    }
}

impl DslFrom<&str> for String {
    fn dfrom(value: &str) -> Self {
        value.to_string()
    }
}

impl<T, const N: usize> DslFrom<[T; N]> for Vec<T> {
    fn dfrom(value: [T; N]) -> Self {
        value.into()
    }
}

impl<T> DslFrom<&[T]> for Vec<T> where T: Clone {
    fn dfrom(value: &[T]) -> Self {
        value.to_vec()
    }
}

impl<const N: usize> DslFrom<[i32; N]> for Vec<f32> {
    fn dfrom(value: [i32; N]) -> Self {
        value.into_iter().map(|x| x as f32).collect()
    }
}

impl DslFrom<&[i32]> for Vec<f32> {
    fn dfrom(value: &[i32]) -> Self {
        value.iter().map(|x| *x as f32).collect()
    }
}

impl<const N: usize> DslFrom<[i32; N]> for Vec<(SizeUnit, f32)> {
    fn dfrom(value: [i32; N]) -> Self {
        value.into_iter().map(|x| (SizeUnit::Pixels, x as f32)).collect()
    }
}

impl<const N: usize> DslFrom<[f32; N]> for Vec<(SizeUnit, f32)> {
    fn dfrom(value: [f32; N]) -> Self {
        value.into_iter().map(|x| (SizeUnit::Pixels, x)).collect()
    }
}

impl DslFrom<&[i32]> for Vec<(SizeUnit, f32)> {
    fn dfrom(value: &[i32]) -> Self {
        value.iter().map(|x| (SizeUnit::Pixels, *x as f32)).collect()
    }
}

impl DslFrom<&[f32]> for Vec<(SizeUnit, f32)> {
    fn dfrom(value: &[f32]) -> Self {
        value.iter().map(|x| (SizeUnit::Pixels, *x)).collect()
    }
}

impl<const N: usize> DslFrom<[i32; N]> for [f32; N] {
    fn dfrom(value: [i32; N]) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = value[i] as f32
        }
        result
    }
}

macro_rules! ivec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslFrom<[i32; 2]> for $name {
            fn dfrom([$x, $y]: [i32; 2]) -> Self {
                $expr
            }
        }

        impl DslFrom<(i32, i32)> for $name {
            fn dfrom(($x, $y): (i32, i32)) -> Self {
                $expr
            }
        }

    }
}

macro_rules! uvec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslFrom<[u32; 2]> for $name {
            fn dfrom([$x, $y]: [u32; 2]) -> Self {
                $expr
            }
        }

        impl DslFrom<(u32, u32)> for $name {
            fn dfrom(($x, $y): (u32, u32)) -> Self {
                $expr
            }
        }

    }
}

macro_rules! fvec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslFrom<[f32; 2]> for $name {
            fn dfrom([$x, $y]: [f32; 2]) -> Self {
                $expr
            }
        }

        impl DslFrom<[i32; 2]> for $name {
            fn dfrom([$x, $y]: [i32; 2]) -> Self {
                let ($x, $y) = ($x as f32, $y as f32);
                $expr
            }
        }

        impl DslFrom<(f32, f32)> for $name {
            fn dfrom(($x, $y): (f32, f32)) -> Self {
                $expr
            }
        }

        impl DslFrom<(f32, i32)> for $name {
            fn dfrom(($x, $y): (f32, i32)) -> Self {
                let ($x, $y) = ($x, $y as f32);
                $expr
            }
        }

        impl DslFrom<(i32, f32)> for $name {
            fn dfrom(($x, $y): (i32, f32)) -> Self {
                let ($x, $y) = ($x as f32, $y);
                $expr
            }
        }

        impl DslFrom<(i32, i32)> for $name {
            fn dfrom(($x, $y): (i32, i32)) -> Self {
                let ($x, $y) = ($x as f32, $y as f32);
                $expr
            }
        }
    };
}

fvec2!(Vec2, x, y, Vec2 {x, y});
fvec2!(Option<Vec2>, x, y, Some(Vec2 {x, y}));
fvec2!(Option<OneOrTwo<Vec2>>, x, y, Some(OneOrTwo(Vec2 {x, y})));
fvec2!(Size2, x, y, Size2::pixels(x, y));
fvec2!(Option<Size2>, x, y, Some(Size2::pixels(x, y)));
fvec2!(Anchor, x, y, Anchor::Custom(Vec2 { x, y }));
fvec2!(Option<Anchor>, x, y, Some(Anchor::Custom(Vec2 { x, y })));
uvec2!(UVec2, x, y, UVec2 { x, y });
ivec2!(IVec2, x, y, IVec2 { x, y });

impl DslFrom<[f32; 4]> for Rect {
    fn dfrom([a, b, c, d]: [f32; 4]) -> Rect {
        Rect { 
            min: Vec2::new(a, b), 
            max: Vec2::new(a + c, b + d), 
        }
    }
}

impl DslFrom<[Vec2; 2]> for Rect {
    fn dfrom([min, dim]: [Vec2; 2]) -> Rect {
        Rect { 
            min, 
            max: min + dim, 
        }
    }
}

impl DslFrom<(Vec2, Vec2)> for Rect {
    fn dfrom((min, dim): (Vec2, Vec2)) -> Rect {
        Rect { 
            min, 
            max: min + dim, 
        }
    }
}

impl DslFrom<i32> for FontSize {
    fn dfrom(value: i32) -> Self {
        FontSize::Pixels(value as f32)
    }
}

impl DslFrom<f32> for FontSize {
    fn dfrom(value: f32) -> Self {
        FontSize::Pixels(value)
    }
}

impl DslFrom<(SizeUnit, f32)> for FontSize {
    fn dfrom((unit, value): (SizeUnit, f32)) -> Self {
        match unit {
            SizeUnit::Pixels => FontSize::Pixels(value),
            SizeUnit::Em => FontSize::Ems(value),
            SizeUnit::Rem => FontSize::Rems(value),
            _ => panic!("Cannot set font size to parent dimension. Choose a different unit."),
        }
    }
}

impl DslFrom<Option<bool>> for VisibilityBundle {
    fn dfrom(value: Option<bool>) -> Self {
        VisibilityBundle {
            visibility: match value {
                Some(true) => Visibility::Visible,
                Some(false) => Visibility::Hidden,
                None => Visibility::Inherited,
            },
            ..Default::default()
        }
    }
}

impl DslFrom<u8> for RenderLayers {
    fn dfrom(value: u8) -> Self {
        RenderLayers::layer(value)
    }
}

impl DslFrom<u8> for Option<RenderLayers> {
    fn dfrom(value: u8) -> Self {
        Some(RenderLayers::layer(value))
    }
}

impl<const N: usize> DslFrom<[u8; N]> for RenderLayers {
    fn dfrom(value: [u8; N]) -> Self {
        RenderLayers::from_layers(&value)
    }
}

impl<const N: usize> DslFrom<[u8; N]> for Option<RenderLayers> {
    fn dfrom(value: [u8; N]) -> Self {
        Some(RenderLayers::from_layers(&value))
    }
}
