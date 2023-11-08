use macroex::*;
use macroex_extras::*;
use proc_macro2::*;
use quote::{quote, format_ident, ToTokens};

pub struct Length(pub Ident, pub f32);

ident_validator!(PxLit "px");
ident_validator!(EmLit "em");
ident_validator!(RemLit "rem");

impl FromMacro for Length {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Number(v) = Number::from_one(tt)?;
        Ok(Self(format_ident!("Pixels"), v))
    }

    fn from_many(tokens: TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        let Number(v) = iter.extract()?;
        Ok(match iter.extract()? {
            Either4::A(PxLit) => Self(format_ident!("Pixels"), v),
            Either4::B(EmLit) => Self(format_ident!("Em"), v),
            Either4::C(RemLit) => Self(format_ident!("Rem"), v),
            Either4::D(PunctOf::<'%'>) => Self(format_ident!("Percent"), v/100.0),
        })
    }
}

pub struct Size2([Length;2]);

impl FromMacro for Size2 {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Repeat::<_, 2>(v) = Repeat::from_one(tt)?;
        Ok(Self(v))
    }
}

impl ToTokens for Size2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.0[0].0;
        let b = &self.0[0].1;
        let c = &self.0[1].0;
        let d = &self.0[1].1;
        quote!(::bevy_aoui::Size2::new(
            (::bevy_aoui::SizeUnit::#a, #b), 
            (::bevy_aoui::SizeUnit::#c, #d),
        )).to_tokens(tokens)
    }
}

pub struct SetEM(Ident, [f32; 2]);

impl FromMacro for SetEM {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Repeat::<_, 2>(NumberList(list)) = Repeat::from_one(tt)?;
        Ok(Self(format_ident!("Pixels"), list))
    }

    fn from_many(tokens: TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        let Repeat::<_, 2>(NumberList(list)) = iter.extract()?;
        match iter.extract()? {
            OrEndOfStream(None) => Ok(Self(format_ident!("Pixels"), list)),
            OrEndOfStream(Some(Either3::A(PxLit))) => Ok(Self(format_ident!("Pixels"), list)),
            OrEndOfStream(Some(Either3::B(EmLit))) => Ok(Self(format_ident!("Ems"), list)),
            OrEndOfStream(Some(Either3::C(RemLit))) => Ok(Self(format_ident!("Rems"), list)),
        }
    }
}

impl ToTokens for SetEM {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.0;
        let [a, b] = self.1;
        quote!(::bevy_aoui::SetEM::#ident(
            ::bevy::prelude::Vec2(#a, #b)
        )).to_tokens(tokens)
    }
}

#[derive(Debug, FromMacro)]
pub struct Rect {
    pub dimension: Option<NumberList<[f32; 2]>>,
    pub center: Option<NumberList<[f32; 2]>>,
    pub min: Option<NumberList<[f32; 2]>>,
    pub max: Option<NumberList<[f32; 2]>>,
}

macroex::call_syntax!(
    "::bevy::prelude::Vec2::new($)", 
    #[derive(Debug, Clone, Copy, Default)]
    pub Vec2(pub [f32; 2])
);

macroex::call_syntax!(
    "::bevy::prelude::UVec2::new($)", 
    #[derive(Debug, Clone, Copy, Default)]
    pub UVec2(pub [u32; 2])
);

#[derive(Debug, FromMacro)]
#[macroex(rename_all = "lowercase")]
pub enum HitboxShape {
    Rectangle,
    Ellipse,
}

impl ToTokens for HitboxShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HitboxShape::Rectangle => {
                quote!(::bevy_aoui::HitboxShape::Rectangle)
                    .to_tokens(tokens)
            },
            HitboxShape::Ellipse => {
                quote!(::bevy_aoui::HitboxShape::Ellipse)
                    .to_tokens(tokens)
            },
        }
    }
}


#[derive(Debug)]
pub struct AnchorEntry(Either<Ident, [f32; 2]>);

impl FromMacro for AnchorEntry {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        Ok(Self(Either::from_one(tt)?))
    }
}


impl ToTokens for AnchorEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Either::A(ident) => 
                quote!(::bevy::sprite::Anchor::#ident).to_tokens(tokens),
            Either::B([x,y]) => 
                quote!(::bevy::sprite::Anchor::Custom(::bevy::prelude::Vec2::new(#x, #y))).to_tokens(tokens),
        }
    }
}

#[derive(Debug, FromMacro)]
pub struct Affine2{
    pub translation: Vec2,
    pub rotation: Number<f32>,
    #[macroex(default = "Vec2([1.0, 1.0])")]
    pub scale: Vec2,
}


impl ToTokens for Affine2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Affine2 {translation, rotation, scale} = self;
        quote!(::bevy::prelude::Affine2::from_scale_angle_translation(
            #scale, #rotation, #translation
        )).to_tokens(tokens)
    }
}

#[derive(Debug)]
pub struct Color(pub Rgba<[f32; 4]>);


impl FromMacro for Color {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        Ok(Self(Rgba::from_one(tt)?))
    }
}

impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Color(Rgba([r,g,b,a])) = self;
        quote!(::bevy::prelude::Color::RgbaLinear {
            red: #r, green: #g, blue: #b, alpha: #a
        }.as_rgba()).to_tokens(tokens)
    }
}
