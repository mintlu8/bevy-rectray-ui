
use bevy::render::view::RenderLayers;
use crate::SizeUnit;
use crate::util::DslFrom;

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