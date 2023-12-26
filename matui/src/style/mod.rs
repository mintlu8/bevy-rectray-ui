use bevy::render::color::Color;

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorGroup {
    pub background: Color,
    pub on_background: Color,
    pub container: Color,
    pub on_container: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorSurface {
    pub background: Color,
    pub on_background: Color,
    pub on_background_variant: Color,
    pub outline: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Palette {
    pub main: ColorGroup,
    pub surface: ColorSurface
}

pub trait WidgetStyle {
    type StyleType;

    fn style(&self, group: Palette) -> Self::StyleType;
}

impl Palette {
    pub fn with_style<S: WidgetStyle>(&self, style: S) -> S::StyleType {
        style.style(*self)
    }

    pub fn into_style<S: WidgetStyle + Default>(&self) -> S::StyleType {
        S::default().style(*self)
    }
}

