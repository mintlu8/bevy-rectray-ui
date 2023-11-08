use std::collections::HashSet;

use macroex::*;
use macroex_extras::{ValueOrExpr, MaybeExpr};
use proc_macro2::{TokenStream, token_stream::IntoIter, Ident};
use proc_macro_error::{proc_macro_error, OptionExt, abort};
use quote::{quote, format_ident};
use crate::{state::*, flex::{FlexLayout, SparseLayout}};

mod state;
mod flex;
mod extractors;

/// Domain specific language for AoUI.
/// 
/// See `AoUI Book` for more information.
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
    let (span, state) = match <Spanned<State>>::extract(&mut iter) {
        Ok(Spanned(a,b)) => (a, b),
        Err(e) => abort_this!(e),
    };

    let mut bundle = Vec::new();
    let mut children: Vec<TokenStream> = Vec::new();
    bundle.push(quote!(::bevy_aoui::AoUI));

    if state.does_render() {
        bundle.push(quote!{::bevy_aoui::ScreenSpaceTransform::default()});
        bundle.push(quote!{::bevy::prelude::GlobalTransform::default()});
    }
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
                dim: ::bevy_aoui::DimensionSize::Owned(#dim)
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

    if state.build_transform{
        bundle.push(quote!{::bevy_aoui::BuildTransform});
        bundle.push(quote!{::bevy::prelude::Transform::default()});
    }

    if let Some(shape) = state.hitbox.get(){
        let mut hitbox = Vec::new();
        hitbox.push(quote!(shape: #shape));
        if let Some(scale) = state.hitbox_size.get() {
            hitbox.push(quote!(scale: #scale));
        }
        if let Some(flag) = state.hitbox_flag.get() {
            hitbox.push(quote!(flag: #flag));
        }
        bundle.push(quote!{::bevy_aoui::Hitbox {#(#hitbox,)* ..Default::default()}});
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

        match state.rect.get() {
            Some(ValueOrExpr::Value(NumberList([a, b, c, d]))) => {
                let x = a + c; 
                let y = b + d;
                sprite.push(quote!(rect: Some(::bevy::prelude::Rect {
                    min: ::bevy::prelude::Vec2::new(#a, #b);
                    max: ::bevy::prelude::Vec2::new(#x, #y);
                })));
            },
            Some(ValueOrExpr::Expr(e)) => {
                sprite.push(quote!(rect: #e));
            }
            None => (),
        }

        if let Some(anchor) = state.anchor.get() {
            sprite.push(quote!(anchor: #anchor));
        }

        if let Some(custom_size) = state.size.get() {
            sprite.push(quote!(custom_size: Some(#custom_size)));
        }

        bundle.push(quote!{::bevy::prelude::Sprite {#(#sprite,)* ..Default::default()}});
    } 
    
    if state.text.is_some() {
        let mut section = Vec::new();
        let mut style = Vec::new();

        if let Some(text) = state.text {
            section.push(quote!(value: #text.to_string()));
        }

        if let Some(font) = state.font {
            style.push(quote!(font: #font));
        }else {
            abort!(span, "Expect font.")
        }

        if let Some(Number(size)) = state.font_size {
            style.push(quote!(font_size: #size));
        } else {
            abort!(span, "Expect font size.")
        }

        if let Some(color) = state.color.get() {
            style.push(quote!(color: #color));
        }

        if let Some(anchor) = state.anchor.get() {
            bundle.push(quote!(#anchor));
        } else {
            bundle.push(quote!(::bevy::sprite::Anchor::default()))
        }

        if let Some(size) = state.size.get() {
            bundle.push(quote!(::bevy::text::Text2dBounds{size: #size}));
        } else {
            bundle.push(quote!(::bevy::text::Text2dBounds::default()));
        }

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

    if state.linebreak {
        bundle.push(quote!(::bevy_aoui::FlexControl::Linebreak))
    }

    match state.flex {
        MaybeExpr::None => (),
        MaybeExpr::Value(flex) => {
            use FlexLayout::*;
            let mut flexbox = Vec::new();
            if let Some(margin) = state.margin.get() {
                flexbox.push(quote!(margin: #margin))
            }
            let stretch = state.stretch;
            match flex {
                Span => {
                    let direction = &state.alignment.expect_or_abort("Expected direction.");
                    flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                        direction: #direction,
                        stretch: #stretch,
                    }));
                },
                HBox => {
                    flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                        direction: ::bevy_aoui::FlexDir::LeftToRight,
                        stretch: #stretch,
                    }));
                },
                VBox => {
                    flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Span{
                        direction: ::bevy_aoui::FlexDir::TopToBottom,
                        stretch: #stretch,
                    }));
                }
                Paragraph => {
                    let direction = state.direction.unwrap_or(format_ident!("LeftToRight"));
                    let stack = state.stack.unwrap_or(format_ident!("TopToBottom"));
                    let alignment = state.alignment.unwrap_or(format_ident!("Top"));
                    flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Paragraph{
                        direction: ::bevy_aoui::FlexDir::#direction,
                        alignment: ::bevy_aoui::Alignment::#alignment,
                        stack: ::bevy_aoui::FlexDir::#stack,
                        stretch: #stretch,
                    }));
                },
                Grid => {
                    let row_dir = state.row_dir.unwrap_or(format_ident!("LeftToRight"));
                    let column_dir = state.column_dir.unwrap_or(format_ident!("TopToBottom"));
    
                    let row_align = state.row_align.unwrap_or(format_ident!("Left"));
                    let column_align = state.column_align.unwrap_or(format_ident!("Top"));
                    if let Some(cells) = state.cell_count.get() {
                        flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Paragraph{
                            cell: ::bevy_aoui::Cells::Counted(#cells),
                            row_dir: ::bevy_aoui::FlexDir::#row_dir,
                            column_dir: ::bevy_aoui::FlexDir::#column_dir,
                            row_align: ::bevy_aoui::Alignment::#row_align,
                            column_align: ::bevy_aoui::Alignment::#column_align,
                            stretch: #stretch,
                        }));
                    } else if let Some(size) = state.cell_size.get() {
                        flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Paragraph{
                            cell: ::bevy_aoui::Cells::Sized(#size),
                            rorow_dirw: ::bevy_aoui::FlexDir::#row_dir,
                            column_dir: ::bevy_aoui::FlexDir::#column_dir,
                            row_align: ::bevy_aoui::Alignment::#row_align,
                            column_align: ::bevy_aoui::Alignment::#column_align,
                            stretch: #stretch,
                        }));
                    } else {
                        abort!(span, "Expected cell_count or cell_size in FlexLayout::Grid")
                    }
                },
                Table => {
                    let row_dir = state.row_dir.unwrap_or(format_ident!("LeftToRight"));
                    let column_dir = state.column_dir.unwrap_or(format_ident!("TopToBottom"));
    
                    let row_align = state.row_align.unwrap_or(format_ident!("Left"));
                    let column_align = state.column_align.unwrap_or(format_ident!("Top"));
                    match state.columns {
                        Some(Either::A(count)) => {
                            flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Paragraph{
                                cell: ::bevy_aoui::Columns::Dynamic(#count),
                                row_dir: ::bevy_aoui::FlexDir::#row_dir,
                                column_dir: ::bevy_aoui::FlexDir::#column_dir,
                                row_align: ::bevy_aoui::Alignment::#row_align,
                                column_align: ::bevy_aoui::Alignment::#column_align,
                                stretch: #stretch,
                            }));
                        },
                        Some(Either::B(separators)) => {
                            flexbox.push(quote!(layout: ::bevy_aoui::FlexLayout::Paragraph{
                                cell: ::bevy_aoui::Columns::Fixed(vec![#(#separators),*]),
                                row_dir: ::bevy_aoui::FlexDir::#row_dir,
                                column_dir: ::bevy_aoui::FlexDir::#column_dir,
                                row_align: ::bevy_aoui::Alignment::#row_align,
                                column_align: ::bevy_aoui::Alignment::#column_align,
                                stretch: #stretch,
                            }));
                        },
                        None => abort!(span, "Expected columns or cell_size in FlexLayout::Table"),
                    }
                },
            }
            bundle.push(quote!(::bevy_aoui::FlexContainer {#(#flexbox,)* ..Default::default()}));
        },
        macroex_extras::MaybeExpr::Expr(e) => {
            let mut flexbox = Vec::new();
            if let Some(margin) = state.margin.get() {
                flexbox.push(quote!(margin: #margin))
                
            }
            bundle.push(quote!(::bevy_aoui::FlexContainer {
                layout: #e,
                #(#flexbox,)* 
                ..Default::default()
            }));
        },
    }
    
    match state.scene {
        MaybeExpr::None => (),
        MaybeExpr::Value(layout) => {
            let mut container = Vec::new();

            let size = match state.cell_size.get(){
                Some(s) => s,
                None => abort!(span, "Expect cell_size in scene.")
            };

            match layout {
                SparseLayout::Rectangles => {
                    let x = state.x_axis.unwrap_or(format_ident!("LeftToRight"));
                    let y = state.y_axis.unwrap_or(format_ident!("BottomToTop"));
                    container.push(quote!(::bevy_aoui::SparseLayout::Rectangles{
                        size: #size,
                        x: ::bevy_aoui::FlexDir::#x,
                        y: ::bevy_aoui::FlexDir::#y,
                    }))
                },
                SparseLayout::Isometric => {
                    let x = state.x_axis.unwrap_or(format_ident!("TopLeft"));
                    let y = state.y_axis.unwrap_or(format_ident!("TopRight"));
                    container.push(quote!(::bevy_aoui::SparseLayout::Isometric{
                        size: #size,
                        x: ::bevy_aoui::IsometricDir::#x,
                        y: ::bevy_aoui::IsometricDir::#y,
                    }))
                },
                SparseLayout::HexGrid => {
                    let x = state.x_axis.unwrap_or(format_ident!("TopRight"));
                    let y = state.y_axis.unwrap_or(format_ident!("Top"));
                    container.push(quote!(::bevy_aoui::SparseLayout::HexGrid{
                        size: #size,
                        x: ::bevy_aoui::HexDir::#x,
                        y: ::bevy_aoui::HexDir::#y,
                    }))
                },
            }

            if let Some(origin) = state.origin.get() {
                container.push(quote!(origin: #origin))
            }

            if let Some(transform) = state.scene_transform.get() {
                container.push(quote!(transform: #transform))
            }

            bundle.push(quote!(::bevy_aoui::SparseContainer {
                #(#container),*
                ..Default::default()
            }));

        },
        MaybeExpr::Expr(e) => {
            let mut container = Vec::new();
            container.push(quote!(layout: #e));
            if let Some(origin) = state.origin.get() {
                container.push(quote!(origin: #origin))
            }
            if let Some(transform) = state.scene_transform.get() {
                container.push(quote!(transform: #transform))
            }
            bundle.push(quote!(::bevy_aoui::SparseContainer {
                #(#container),*
                ..Default::default()
            }));

        },
    }

    for child in state.children{
        children.push(parse_one(commands.clone(), child.into_iter()));
    }

    for item in state.extras{
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
