use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use macroex::*;
use macroex_extras::*;

use crate::flex::FlexLayout;
use crate::extractors::*;
use crate::flex::SparseLayout;

pub type SizeLike = Repeat<NumberList<Vec2>, 2>;

#[derive(Default, FromMacro)]
pub struct State {
    // Anchors
    pub center: MaybeExpr<AnchorEntry>,
    pub anchor: MaybeExpr<AnchorEntry>,

    // Transform2D
    pub offset: MaybeExpr<NumberList<Vec2>>,
    pub rotation: MaybeExpr<Angle>,
    pub scale: MaybeExpr<SizeLike>,
    pub z: MaybeExpr<Number<f32>>,

    // dimension
    pub dimension: MaybeExpr<Size2>,
    pub em: MaybeExpr<SetEM>,


    // shared
    pub color: MaybeExpr<Color>,
    pub size: MaybeExpr<Vec2>,
    
    /// children
    #[macroex(repeat, rename="child")]
    pub children: Vec<TokenStream>,
    // #[macroex(repeat, rename="children")]
    // pub child_iter: Vec<TokenStream>,

    /// extra components
    #[macroex(repeat, rename="extra")]
    pub extras: Vec<TokenStream>,

    // misc
    pub build_transform: bool,
    pub linebreak: bool,

    // hitbox
    pub hitbox: MaybeExpr<HitboxShape>,
    pub hitbox_size: MaybeExpr<Vec2>,
    pub mouse_event: MaybeExpr<u32>,


    // sprite
    pub sprite: Option<TokenStream>,
    pub rect: MaybeExpr<NumberList<[f32; 4]>>,
    pub flip: Option<[bool; 2]>,
    
    // flex_container
    pub flex: MaybeExpr<FlexLayout>,
    pub margin: MaybeExpr<SizeLike>,
    pub direction: Option<Ident>,
    pub stack: Option<Ident>,
    pub row_dir: Option<Ident>,
    pub column_dir: Option<Ident>,
    pub row_align: Option<Ident>,
    pub column_align: Option<Ident>,
    pub cell_count: MaybeExpr<UVec2>,
    pub cell_size: MaybeExpr<NumberList<Vec2>>,
    pub stretch: bool,
    pub alignment: Option<Ident>,
    pub columns: Option<Either<usize, Vec<f32>>>,

    // sparse_container
    // we reuse cell_size for sprase size
    pub scene: MaybeExpr<SparseLayout>,
    pub x_axis: Option<Ident>,
    pub y_axis: Option<Ident>,
    pub origin: MaybeExpr<Vec2>,
    pub scene_transform: MaybeExpr<Affine2>,
    pub unit_rect: MaybeExpr<NumberList<[f32; 4]>>,
    pub position: MaybeExpr<Vec2>,

    // text
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