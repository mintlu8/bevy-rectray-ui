use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use macroex::*;
use macroex_extras::*;

use crate::flex::FlexLayout;
use crate::extractors::*;

pub type SizeLike = Repeat<NumberList<Vec2>, 2>;

#[derive(Default, FromMacro)]
pub struct State {
    pub center: MaybeExpr<AnchorEntry>,
    pub anchor: MaybeExpr<AnchorEntry>,
    pub offset: MaybeExpr<NumberList<Vec2>>,
    pub rotation: MaybeExpr<Angle>,
    pub scale: MaybeExpr<SizeLike>,
    pub z: MaybeExpr<Number<f32>>,
    pub dimension: MaybeExpr<Size2>,
    pub em: MaybeExpr<SetEM>,
    pub fast_core: bool,
    #[macroex(repeat, rename="child")]
    pub children: Vec<TokenStream>,
    #[macroex(repeat, rename="extra")]
    pub extra: Vec<TokenStream>,
    pub into_transform: bool,
    pub hitbox: Option<Hitbox>,
    pub linebreak: Option<Linebreak>,
    pub position: Option<[i64; 2]>,
    pub sprite: Option<TokenStream>,
    pub color: MaybeExpr<Color>,
    pub rect: Option<NumberList<[f32; 4]>>,
    pub flip: Option<[bool; 2]>,
    pub flex: Option<FlexLayout>,
    pub margin: Option<SizeLike>,
    pub direction: Option<Ident>,
    pub wrap_to: Option<Ident>,
    pub major: Option<Ident>,
    pub minor: Option<Ident>,
    pub cell_count: Option<[i32; 2]>,
    pub cell_size: Option<NumberList<Vec2>>,
    pub stretch: bool,
    pub alignment: Option<Ident>,
    pub pad_align: Option<AnchorEntry>,
    pub columns: Option<Vec<f32>>,
    pub sparse: Option<TokenTree>,
    pub size: Option<TokenTree>,
    pub x: Option<IdentString>,
    pub y: Option<IdentString>,
    pub origin: Option<Vec2>,
    pub scene_transform: Option<Mat2>,
    pub text: Option<String>,
    pub font: Option<TokenStream>,
    pub font_size: Option<Number<f32>>,
    pub format: Option<String>,
    pub text_anchor: Option<TokenTree>,
}


impl State {
    pub fn does_render(&self) -> bool {
        self.sprite.is_some() || self.font.is_some()
    }
}