use bevy::{prelude::{Vec2, Resource}, reflect::Reflect};

/// The Root EM of the window.
/// 
/// By default this is `[16, 16]` if not found.
#[derive(Debug, Resource)]
pub struct AorsREM(pub Vec2);
impl Default for AorsREM {
    fn default() -> Self {
        Self(Vec2::new(16.0, 16.0))
    }
}

/// Set the em relative to parent.
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum SetEM {
    #[default]
    None,
    Pixels(Vec2),
    Scale(Vec2),
    ScaleRem(Vec2),
}

/// The unit of a Size `px`, `em`, `rem`, `percent`
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum SizeUnit{
    #[default]
    Pixels,
    Em,
    Rem,
    Percent,
}

/// A context sensitive Vec2
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Size2 {
    x: SizeUnit,
    y: SizeUnit,
    raw: Vec2,
}


impl SizeUnit {
    #[inline]
    pub fn as_pixels(self, value: f32, parent: f32, em: f32, rem: f32) -> f32 {
        match self {
            SizeUnit::Pixels => value,
            SizeUnit::Em => value * em,
            SizeUnit::Rem => value * rem,
            SizeUnit::Percent => value * parent,
        }
    }
}

impl Size2 {
    pub const ZERO: Self = Self {
        x: SizeUnit::Pixels,
        y: SizeUnit::Pixels,
        raw: Vec2::ZERO,
    };

    pub fn new(x: (SizeUnit, f32), y: (SizeUnit, f32)) -> Self{
        Self {
            x: x.0,
            y: y.0,
            raw: Vec2::new(x.1, y.1)
        }
    }

    pub fn pixels(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Pixels,
            y: SizeUnit::Pixels,
            raw: Vec2::new(x, y),
        }
    }

    pub fn em(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Em,
            y: SizeUnit::Em,
            raw: Vec2::new(x, y),
        }
    }

    pub fn rem(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Rem,
            y: SizeUnit::Rem,
            raw: Vec2::new(x, y),
        }
    }

    pub fn percent(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Percent,
            y: SizeUnit::Percent,
            raw: Vec2::new(x, y),
        }
    }

    #[inline]
    pub fn as_pixels(&self, parent: Vec2, em: Vec2, rem: Vec2) -> Vec2 {
        Vec2::new(
            self.x.as_pixels(self.raw.x, parent.x, em.x, rem.x),
            self.y.as_pixels(self.raw.y, parent.y, em.y, rem.y),
        )
    }

    /// Units of x and y.
    pub fn units(&self) -> (SizeUnit, SizeUnit) {
        (self.x, self.y)
    }

    /// A loose function that obtains a vec2 from this struct.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn raw(&self) -> Vec2 {
        self.raw
    }

    /// A loose function that updates this struct's value.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn set_raw(&mut self, value: Vec2) {
        self.raw = value
    }

    /// A loose function that updates this struct's value.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        f(&mut self.raw)
    }
}

impl From<Vec2> for Size2 {
    fn from(value: Vec2) -> Self {
        Self { 
            x: SizeUnit::Pixels, 
            y: SizeUnit::Pixels,
            raw: value
        }
    }
}
