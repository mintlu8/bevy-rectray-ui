use std::collections::HashSet;

use macroex::*;
use proc_macro2::{TokenStream, token_stream::IntoIter, Ident};
use proc_macro_error::{proc_macro_error, OptionExt};
use quote::{quote, format_ident};
use crate::{state::*, flex::FlexLayout};
use crate::extractors::*;

mod state;
mod flex;
mod extractors;

/// DSL for AoUI
/// 
/// See [`AoUI Book`] for more information.
#[proc_macro]
#[proc_macro_error]
pub fn sprite(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: TokenStream = tokens.into();
    let mut iter = tokens.into_iter();
    let CommaExtractor::<TokenStream>(commands) = match iter.extract(){
        Ok(tokens) => tokens,
        Err(e) => abort_this!(e),
    };
    parse_one(commands, iter).into()
}

fn extract_fields(iter: &mut impl Iterator<Item=proc_macro2::TokenTree>) -> Vec<Ident> {
    let CurlyBraced(Iter(iter)) =  match CurlyBraced::extract(iter) {
        Ok(s) => s,
        Err(e) => abort_this!(e),
    };
    let mut last_is_comma = true;
    let set: HashSet<_> = iter.filter_map(|tt| {
        match tt {
            proc_macro2::TokenTree::Ident(ident) if last_is_comma => {
                last_is_comma = false;
                Some(ident)
            },
            proc_macro2::TokenTree::Punct(p) if p.as_char() == ',' => {
                last_is_comma = true;
                None
            },
            _ => {
                last_is_comma = false;
                None
            },
        }
    }).collect();
    set.into_iter().collect()
}

fn parse_one(commands: TokenStream, mut iter: IntoIter) -> TokenStream {
    let extracted_fields = extract_fields(&mut iter.clone());
    let state = match State::extract(&mut iter) {
        Ok(s) => s,
        Err(e) => abort_this!(e),
    };

    let mut bundle = Vec::new();
    let mut children: Vec<TokenStream> = Vec::new();
    bundle.push(match state.fast_core {
        true => quote!(::bevy_aoui::Core::Fast),
        false => quote!(::bevy_aoui::Core::Full),
    });
    {
        let mut props = Vec::new();
        if let Some(center) = state.center.get() {
            props.push(quote!(center: Some(#center)));
        }

        if let Some(anchor) = state.anchor.get() {
            props.push(quote!(anchor: #anchor));
        }
        bundle.push(quote!{::bevy_aoui::Anchors {#(#props,)* ..Default::default()}});
    }

    {
        let mut dimemsion = Vec::new();
        if let Some(dim) = state.dimension.get() {
            dimemsion.push(quote!(
                dim: ::bevy_aoui::DimensionSize::Owned(#dim);
            ));
        }
        if let Some(em) = state.em.get() {
            dimemsion.push(quote!(set_em: #em));
        }
        bundle.push(quote!{::bevy_aoui::Dimension {#(#dimemsion,)* ..Default::default()}});
    }

    {
        let mut transform = Vec::new();
        if let Some(vec) = state.offset.get() {
            transform.push(quote!(offset: #vec));
        }
        if let Some(rot) = state.rotation.get() {
            transform.push(quote!(rotation: #rot));
        }
        if let Some(scale) = state.scale.get() {
            transform.push(quote!(scale: #scale));
        }
        if let Some(z) = state.z.get() {
            transform.push(quote!(z: #z));
        }
        bundle.push(quote!{::bevy_aoui::Transform2D {#(#transform,)* ..Default::default()}});
    }
    bundle.push(quote!{::bevy_aoui::RotatedRect::default()});
    bundle.push(quote!{::bevy::prelude::VisibilityBundle::default()});

    if state.sprite.is_some() || state.font.is_some() {
        bundle.push(quote!{::bevy_aoui::ScreenSpaceTransform::default()});
        bundle.push(quote!{::bevy::prelude::GlobalTransform::default()});
    }

    if state.into_transform{
        bundle.push(quote!{::bevy_aoui::IntoTransform});
        bundle.push(quote!{::bevy::prelude::Transform::default()});
    }

    if let Some(hitbox) = state.hitbox{
        let shape = hitbox.shape;
        match &hitbox.size.get(){
            Some(scale) => {
                bundle.push(quote!{::bevy_aoui::Hitbox {
                    shape: #shape,
                    scale: #scale
                }});
            }
            None => {
                bundle.push(quote!{::bevy_aoui::Hitbox {
                    shape: #shape,
                    scale: ::bevy::prelude::Vec2::ONE;
                }});
            }
        }
    }

    if let Some(sprite_expr) = state.sprite{
        let mut sprite = Vec::new();

        bundle.push(quote!({
            let _texture: ::bevy::asset::Handle<::bevy::render::texture::Image> = #sprite_expr;
            _texture
        }));

        if let Some(color) = state.color.get() {
            sprite.push(quote!(color: #color));
        }

        if let Some([x, y]) = state.flip {
            sprite.push(quote!(flip_x: #x, flip_y: #y));
        }

        if let Some(NumberList([a, b, c, d])) = &state.rect {
            let x = a + c; 
            let y = b + d;
            sprite.push(quote!(rect: Some(::bevy::prelude::Rect {
                min: ::bevy::prelude::Vec2::new(#a, #b);
                max: ::bevy::prelude::Vec2::new(#x, #y);
            })));
        }

        if let Some(anchor) = state.anchor.get() {
            sprite.push(quote!(anchor: #anchor));
        }

        bundle.push(quote!{::bevy::prelude::Sprite {#(#sprite,)* ..Default::default()}});
    } else if state.text.is_some() {
        let mut section = Vec::new();
        let mut style = Vec::new();

        if let Some(text) = state.text {
            section.push(quote!(value: #text));
        }

        if let Some(font) = state.font {
            style.push(quote!(font: #font));
        }

        if let Some(Number(size)) = state.font_size {
            style.push(quote!(font_size: #size));
        }

        if let Some(color) = state.color.get() {
            style.push(quote!(color: #color));
        }

        if let Some(anchor) = state.anchor.get() {
            bundle.push(quote!(#anchor));
        } else {
            bundle.push(quote!(::bevy::sprite::Anchor::default()))
        }

        if let Some(anchor) = state.anchor.get() {
            bundle.push(quote!(#anchor));
        }
        bundle.push(quote!(::bevy::text::Text2dBounds::default()));
        bundle.push(quote!(::bevy::text::TextLayoutInfo::default()));
        bundle.push(quote!{::bevy::text::Text {
            sections: vec![
                ::bevy::text::TextSection {
                    style: ::bevy::text::TextStyle {
                        #(#style,)*
                        ..Default::default()
                    },
                    #(#section,)*
                    ..Default::default()
                }
            ], 
            ..Default::default()
        }});
    }

    if let Some(linebreak) = state.linebreak {
        match linebreak {
            Linebreak::Linebreak => bundle.push(quote!(::bevy_aoui::FlexControl::Linebreak)),
            Linebreak::BreakOnSelf => bundle.push(quote!(::bevy_aoui::FlexControl::LinebreakMarker)),
        }
    }

    use FlexLayout::*;
    
    if let Some(flex) = state.flex {
        let mut flexbox = Vec::new();
        if let Some(Repeat(NumberList(vec))) = state.margin {
            flexbox.push(quote!(margin: #vec))
        }
        match flex {
            Span => {
                let direction = &state.alignment.expect_or_abort("Expected direction.");
                flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                    direction: #direction
                }));
            },
            HBox => {
                flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                    direction: ::bevy_aoui::FlexDir::LeftToRight,
                }));
            },
            VBox => {
                flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                    direction: ::bevy_aoui::FlexDir::TopToBottom,
                }));
            }
            Paragraph => {
                let alignment = state.alignment.unwrap_or(format_ident!("Center"));
                flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Wrapping{
                    direction: ::bevy_aoui::FlexDir::LeftToRight,
                    alignment: ::bevy::sprite::Anchor::#alignment,
                    direction: ::bevy_aoui::WrapTo::Down,
                }));
            },
            WrapBox => {
                let direction = state.direction.expect_or_abort("Expected direction.");
                let alignment = state.alignment.unwrap_or(format_ident!("Center"));
                let wrap_to = state.wrap_to.unwrap_or(
                    if matches!(alignment.to_string().as_str(), "TopToBottom"|"BottomToTop") {
                        format_ident!("Right")
                    } else {
                        format_ident!("Down")
                    }
                );
                
                flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Wrapping{
                    direction: ::bevy_aoui::FlexDir::#direction,
                    alignment: ::bevy::sprite::Anchor::#alignment,
                    direction: ::bevy_aoui::WrapTo::#wrap_to,
                }));
            },
            Grid => todo!(),
            Table => todo!(),
            FixedGrid => todo!(),
            SizedGrid => todo!(),
            FixedTable => todo!(),
            FlexTable => todo!(),
        }
        bundle.push(quote!(::bevy_aoui::FlexContainer {#(#flexbox,)* ..Default::default()}));
    }

    for child in state.children{
        children.push(parse_one(commands.clone(), child.into_iter()));
    }

    for item in state.extra{
        bundle.push(item)
    }


    let all = State::idents();
    quote!({
        #[derive(Default)]
        struct _Sprite { #(#all: ()),* }
        {
            #[allow(unused_variables)]
            if let _Sprite{#(#extracted_fields,)* ..} = _Sprite::default() {}
        }

        let parent = #commands.spawn((#(#bundle),*)).id();
        let children = [#(#children),*];
        commands.entity(parent).push_children(&children);
        parent
    })
}
