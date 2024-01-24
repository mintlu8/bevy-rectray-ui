use bevy::math::{Vec2, UVec2, IVec2, Rect};

use crate::{DimensionType, Size2, Anchor, FontSize, SizeUnit};

use super::DslFrom;


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
fvec2!(Size2, x, y, Size2::pixels(x, y));
fvec2!(Option<Size2>, x, y, Some(Size2::pixels(x, y)));
fvec2!(Anchor, x, y, Anchor::custom(x, y));
fvec2!(Option<Anchor>, x, y, Some(Anchor::custom(x, y)));
fvec2!(DimensionType, x, y, DimensionType::Owned(Size2::pixels(x, y)));
uvec2!(UVec2, x, y, UVec2 { x, y });
ivec2!(IVec2, x, y, IVec2 { x, y });

impl DslFrom<Size2> for DimensionType {
    fn dfrom(value: Size2) -> Self {
        DimensionType::Owned(value)
    }
}

impl DslFrom<Vec2> for DimensionType {
    fn dfrom(value: Vec2) -> Self {
        DimensionType::Owned(value.into())
    }
}

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
