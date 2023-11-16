use bevy::{prelude::{Vec2, UVec2, IVec2, Rect}, sprite::Anchor, render::view::{VisibilityBundle, Visibility}};
use bevy_aoui::Size2;


pub trait DslInto<T> {
    fn dinto(self) -> T;
}

impl<T> DslInto<T> for T {
    fn dinto(self) -> T {
        self
    }
}

impl<T> DslInto<Option<T>> for T {
    fn dinto(self) -> Option<T> {
        Some(self)
    }
}

impl<'t, T> DslInto<T> for &'t T where T: Clone{
    fn dinto(self) -> T {
        self.clone()
    }
}

impl<'t, T> DslInto<T> for &'t mut T where T: Clone{
    fn dinto(self) -> T {
        self.clone()
    }
}

impl DslInto<f32> for i32 {
    fn dinto(self) -> f32 {
        self as f32
    }
}

impl DslInto<f32> for usize {
    fn dinto(self) -> f32 {
        self as f32
    }
}

impl DslInto<String> for &str {
    fn dinto(self) -> String {
        self.to_owned()
    }
}

impl<T, const N: usize> DslInto<Vec<T>> for [T; N] {
    fn dinto(self) -> Vec<T> {
        self.into_iter().collect()
    }
}

impl<T: Clone> DslInto<Vec<T>> for &[T] {
    fn dinto(self) -> Vec<T> {
        self.into_iter().cloned().collect()
    }
}

impl<const N: usize> DslInto<Vec<f32>> for [i32; N] {
    fn dinto(self) -> Vec<f32> {
        self.into_iter().map(|x| x as f32).collect()
    }
}

impl DslInto<Vec<f32>> for &[i32] {
    fn dinto(self) -> Vec<f32> {
        self.into_iter().map(|x| *x as f32).collect()
    }
}

impl<const N: usize> DslInto<[f32; N]> for [i32; N] {
    fn dinto(self) -> [f32; N] {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = self[i] as f32
        }
        result
    }
}

macro_rules! ivec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslInto<$name> for [i32; 2] {
            fn dinto(self) -> $name {
                let [$x, $y] = self;
                $expr
            }
        }

        impl DslInto<$name> for (i32, i32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
                $expr
            }
        }

    }
}

macro_rules! uvec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslInto<$name> for [u32; 2] {
            fn dinto(self) -> $name {
                let [$x, $y] = self;
                $expr
            }
        }

        impl DslInto<$name> for (u32, u32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
                $expr
            }
        }

    }
}

macro_rules! fvec2 {
    ($name: ty, $x: ident, $y: ident, $expr: expr) => {
        impl DslInto<$name> for [f32; 2] {
            fn dinto(self) -> $name {
                let [$x, $y] = self;
                $expr
            }
        }

        impl DslInto<$name> for [i32; 2] {
            fn dinto(self) -> $name {
                let [$x, $y] = self;
                let ($x, $y) = ($x as f32, $y as f32);
                $expr
            }
        }

        impl DslInto<$name> for (f32, f32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
                $expr
            }
        }

        impl DslInto<$name> for (f32, i32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
                let ($x, $y) = ($x, $y as f32);
                $expr
            }
        }

        impl DslInto<$name> for (i32, f32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
                let ($x, $y) = ($x as f32, $y);
                $expr
            }
        }

        impl DslInto<$name> for (i32, i32) {
            fn dinto(self) -> $name {
                let ($x, $y) = self;
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
fvec2!(Anchor, x, y, Anchor::Custom(Vec2 { x, y }));
fvec2!(Option<Anchor>, x, y, Some(Anchor::Custom(Vec2 { x, y })));
uvec2!(UVec2, x, y, UVec2 { x, y });
ivec2!(IVec2, x, y, IVec2 { x, y });

impl DslInto<Rect> for [f32; 4] {
    fn dinto(self) -> Rect {
        let [a, b, c, d] = self;
        Rect { 
            min: Vec2::new(a, b), 
            max: Vec2::new(a + c, b + d), 
        }
    }
}

impl DslInto<Rect> for [Vec2; 2] {
    fn dinto(self) -> Rect {
        let [min, dim] = self;
        Rect { 
            min, 
            max: min + dim, 
        }
    }
}

impl DslInto<Rect> for (Vec2, Vec2) {
    fn dinto(self) -> Rect {
        let (min, dim) = self;
        Rect { 
            min, 
            max: min + dim, 
        }
    }
}

impl DslInto<VisibilityBundle> for Option<bool> {
    fn dinto(self) -> VisibilityBundle {
        VisibilityBundle {
            visibility: match self {
                Some(true) => Visibility::Visible,
                Some(false) => Visibility::Hidden,
                None => Visibility::Inherited,
            },
            ..Default::default()
        }
    }
}
