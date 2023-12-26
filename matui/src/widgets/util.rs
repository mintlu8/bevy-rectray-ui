use std::ops::{Deref, DerefMut};

use bevy::{render::color::Color, ecs::{component::Component, system::Commands, entity::Entity}, asset::AssetServer};
use bevy_aoui::{dsl::DslFrom, material_sprite, size2, layout::LayoutControl};

use crate::shapes::RoundedShadowMaterial;

/// Create a palette struct, every field must be a color.
/// 
/// ```
/// # /*
/// palette!(FramePalette {
///     foreground: red,
///     background: green,
/// })
/// # */
/// ```
/// Translates to:
/// ```
/// # /*
/// FramePalette {
///     foreground: color!(red),
///     background: color!(green),
///     ..Default::default()
/// }
/// # */
/// ```
#[macro_export]
macro_rules! palette {
    ($ty: ident {$($field: ident: $color: tt),* $(,)?}) => {
        $ty {
            $($field: $crate::aoui::color!($color),)*
            ..Default::default()
        }
    };
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OptionM<T> {
    #[default]
    None,
    Some(T),
}

impl<T> OptionM<T> {
    pub fn expect(self, s: &str) -> T {
        match self {
            OptionM::Some(v) => v,
            OptionM::None => panic!("{}", s),
        }
    }

    pub fn unwrap_or(self, or: T) -> T {
        match self {
            OptionM::Some(v) => v,
            OptionM::None => or,
        }
    }

    pub fn unwrap_or_else(self, or: impl FnOnce() -> T) -> T {
        match self {
            OptionM::Some(v) => v,
            OptionM::None => or(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowInfo {
    /// Size of the shadow.
    pub size: f32,
    /// Color of the shadow, default is `black`.
    pub color: Color,
    /// Darkens the shadow.
    /// 
    /// Value should be in `0..=1` but should realistically 
    /// be around `0.0..=0.4` as `1` is completely dark.
    /// 
    /// This effectively computes a more compact shadow 
    /// of a slightly larger sprite to produce a darker shadow. 
    pub darken: f32,
}

impl ShadowInfo {
    
    pub fn build_capsule(&self, commands: &mut Commands, assets: &AssetServer) -> Entity {
        material_sprite!((commands, assets) {
            dimension: size2![1 + {self.size * 2.0} px, 1 + {self.size * 2.0} px],
            z: -0.005,
            material: RoundedShadowMaterial::capsule(self.color, self.size - self.size * self.darken),
            extra: LayoutControl::IgnoreLayout,
        })
    }

    pub fn build_rect(&self, commands: &mut Commands, assets: &AssetServer, corner: f32) -> Entity {
        material_sprite!((commands, assets) {
            dimension: size2![1 + {self.size * 2.0} px, 1 + {self.size * 2.0} px],
            z: -0.005,
            material: RoundedShadowMaterial::new(self.color, corner, self.size - self.size * self.darken),
            extra: LayoutControl::IgnoreLayout,
        })
    }
}
impl Default for ShadowInfo {
    fn default() -> Self {
        Self { 
            size: 0.0, 
            color: Color::BLACK, 
            darken: 0.0, 
        }
    }
}


impl DslFrom<ShadowInfo> for OptionM<ShadowInfo> {
    fn dfrom(value: ShadowInfo) -> Self {
        OptionM::Some(value)
    }
}


impl DslFrom<i32> for OptionM<ShadowInfo> {
    fn dfrom(value: i32) -> Self {
        OptionM::Some(ShadowInfo { 
            size: value as f32, 
            ..Default::default()
        })
    }
}

impl DslFrom<f32> for OptionM<ShadowInfo> {
    fn dfrom(value: f32) -> Self {
        OptionM::Some(ShadowInfo { 
            size: value, 
            ..Default::default()
        })
    }
}

impl DslFrom<(Color, i32)> for OptionM<ShadowInfo> {
    fn dfrom((color, size): (Color, i32)) -> Self {
        OptionM::Some(ShadowInfo { 
            size: size as f32, 
            color,
            ..Default::default()
        })
    }
}

impl DslFrom<(Color, f32)> for OptionM<ShadowInfo> {
    fn dfrom((color, size): (Color, f32)) -> Self {
        OptionM::Some(ShadowInfo { 
            size, 
            color,
            ..Default::default()
        })
    }
}

impl DslFrom<(i32, Color)> for OptionM<ShadowInfo> {
    fn dfrom((size, color): (i32, Color)) -> Self {
        OptionM::Some(ShadowInfo { 
            size: size as f32, 
            color,
            ..Default::default()
        })
    }
}

impl DslFrom<(f32, Color)> for OptionM<ShadowInfo> {
    fn dfrom((size, color): (f32, Color)) -> Self {
        OptionM::Some(ShadowInfo { 
            size, 
            color,
            ..Default::default()
        })
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct StrokeColors<T>(pub T);

impl<T> Deref for StrokeColors<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StrokeColors<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct WidgetPalette {
    pub background: Color,
    pub foreground: Color,
    pub stroke: Color,
}
