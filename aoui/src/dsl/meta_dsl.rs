/// this maps `macro! {}` into `macro! (ctx {})`
/// 
/// and `child: #macro!{}` into `children: quote_syntax! (ctx macro! {})`
#[doc(hidden)]
#[macro_export]
macro_rules! inline_context {
    ($ctx: tt [$($path: tt)*] [$($fields: tt)*]) => {
        $crate::meta_dsl!($ctx [$($path)*] {$($fields)*} {} {} {})
    };
    ($ctx: tt [$($path: tt)*] [$($out: tt)*] child: #$macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [
            $($out)*
            children: $crate::quote_syntax!($ctx $macro { $($expr)* }),
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [
            $($out)*
            $field: $macro! (
                $ctx {
                    $($expr)*
                }
            ),
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $head: expr $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$($out)* $field: $head,] $($($rest)*)?)
    };
}

/// Original syntax is `child: #macro! {color: #colors}`
///
/// Turns into `vec.extend(this)`
#[doc(hidden)]
#[macro_export]
macro_rules! quote_syntax {
    ($ctx: tt $macro:ident { $($tt: tt)* }) => {
        $crate::quote_syntax!($ctx $macro () () [{}] [{$($tt)*}])
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) [{$($tt: tt)*}] [{}]) => {
        $crate::dsl::izip!($($cap),*).map(|($($vars),*)|
            $macro! ($ctx {
                $($tt)*
            })
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [($($a: tt)*) $($b: tt)*] 
            [(:#$var: ident ! {$($m:tt)*} $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [($($a)* :#$var! {$($m)*}) $($b)*] 
            [($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [[$($a: tt)*] $($b: tt)*] 
            [[:#$var: ident ! {$($m:tt)*} $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[$($a)* :#$var! {$($m)*}] $($b)*] 
            [[$($x)*] $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [{$($a: tt)*} $($b: tt)*] 
            [{:#$var: ident ! {$($m:tt)*} $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{$($a)* :#$var! {$($m)*}} $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };

    // `var` is the hygienic version of the original,
    // can't use the original name because duplicate
    // See `izip` in itertools.
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [($($a: tt)*) $($b: tt)*] 
            [(#$var: ident $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [($($a)* var) $($b)*] 
            [($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [[$($a: tt)*] $($b: tt)*] 
            [[#$var: ident $($x: tt)* ] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [[$($a)* var] $($b)*] 
            [[$($x)*] $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [{$($a: tt)*} $($b: tt)*] 
            [{#$var: ident $($x: tt)* } $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [{$($a)* var} $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [($($a: tt)*) $($b: tt)*] 
            [(#$var: ident $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [($($a)* var) $($b)*] 
            [($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [[$($a: tt)*] $($b: tt)*] 
            [[#$var: ident $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [[$($a)* var] $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [{$($a: tt)*} $($b: tt)*] 
            [{#$var: ident $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap,)* $var) ($($vars,)* var)
            [{$($a)* var} $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [(($($f: tt)*) $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [() $($a)*] 
            [($($f)*) ($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [([$($f: tt)*] $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[] $($a)*] 
            [[$($f)*] ($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [({$($f: tt)*} $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{} $($a)*] 
            [{$($f)*} ($($x)*) $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [[($($f: tt)*) $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [() $($a)*] 
            [($($f)*) [$($x)*] $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [[[$(f: tt)*] $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[] $($a)*] 
            [[$($f)*] [$($x)*] $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [[{$($f: tt)*} $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{} $($a)*] 
            [{$($f)*} [$($x)*] $($y)*] 
        )
    };
    
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [{($($f: tt)*) $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [() $($a)*] 
            [($($f)*) {$($x)*} $($y)*]
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [{[$($f: tt)*] $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[] $($a)*] 
            [[$($f)*] {$($x)*} $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$($a: tt)*] 
            [{{$($f: tt)*} $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{} $($a)*] 
            [{$($f)*} {$($x)*} $($y)*] 
        )
    };
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [($($a: tt)*) $($b: tt)*] 
            [($head: tt $($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [($($a)* $head) $($b)*] 
            [($($x)*) $($y)*] 
        )
    };
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [[$($a: tt)*] $($b: tt)*] 
            [[$head: tt $($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[$($a)* $head] $($b)*] 
            [[$($x)*] $($y)*] 
        )
    };
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [{$($a: tt)*} $($b: tt)*] 
            [{$head: tt $($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{$($a)* $head} $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };

    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$head: tt ($($a: tt)*) $($b: tt)*] 
            [$empty: tt ($($x: tt)*) $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [($($a)* $head) $($b)*] 
            [($($x)*) $($y)*] 
        )
    };
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$head: tt [$($a: tt)*] $($b: tt)*] 
            [$empty: tt [$($x: tt)*] $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [[$($a)* $head] $($b)*] 
            [[$($x)*] $($y)*] 
        )
    };
    ($ctx: tt $macro:ident ($($cap: ident),*) ($($vars: ident),*) 
            [$head: tt {$($a: tt)*} $($b: tt)*] 
            [$empty: tt {$($x: tt)*} $($y: tt)*]) => {
        $crate::quote_syntax!($ctx $macro ($($cap),*) ($($vars),*)
            [{$($a)* $head} $($b)*] 
            [{$($x)*} $($y)*] 
        )
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! parse_children {
    ($ctx: tt $ident: ident {} {$($stmt: stmt;)*}) => {
        {
            let mut $ident = ::std::vec::Vec::new();
            $($stmt;)*
            $ident
        }
    };
    ($ctx: tt $ident: ident {child: $expr: expr $(,$($rest:tt)*)?} {$($stmt: stmt;)*}) => {
        $crate::parse_children!($ctx $ident {$($($rest)*)?} {$($stmt;)* $ident.push($expr);} )
    };
    ($ctx: tt $ident: ident {children: $expr: expr $(,$($rest:tt)*)?} {$($stmt: stmt;)*}) => {
        $crate::parse_children!($ctx $ident {$($($rest)*)?} {$($stmt;)* $ident.extend($expr.into_iter().map(|x| x.clone()));} )
    };
}

/// The core macro for our DSL.
#[macro_export]
macro_rules! meta_dsl {

    ($commands: tt [$($path: tt)*] {$($fields: tt)*} ) => {
        $crate::inline_context!($commands [$($path)*] [] $($fields)*)
    };

    ($commands: tt [$($path: tt)*]
        {extra: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($child_type: $children),*}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($child_type: $children,)* child: $expr}
        )
    };

    ($commands: tt [$($path: tt)*]
        {children: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($child_type: $children,)* children: $expr}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($child_type: $children),*}
        )
    };

    (($commands: expr$(,)?) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        {
            use $crate::dsl::{DslInto, AoUICommands};
            let extras = ($($extras),*);
            let children = $crate::parse_children!(($commands) _children {$($child_type: $children),*} {});
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            $commands.spawn_aoui((
                entity,
                extras,
                children,
            ))
        }
    };

    (($commands: expr, $assets: expr) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        {
            use $crate::dsl::{DslInto, AoUICommands};
            let extras = ($($extras),*);
            let children = $crate::parse_children!(($commands, $assets) _children {$($child_type: $children),*} {});
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            $commands.spawn_aoui_with_assets(
                &$assets, (
                    entity,
                    extras,
                    children,
                )
            )
        }
    };

    ($commands: ident [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
    ) => {
        {
            use $crate::dsl::{DslInto, AoUICommands};
            let extras = ($($extras),*);
            let children = $crate::parse_children!($commands _children {$($child_type: $children),*} {});
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            $commands.spawn_aoui((
                entity,
                extras,
                children,
            ))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! transform2d {
    ($this: expr) => {
        $crate::Transform2D {
            center: $this.center.unwrap_or($crate::Anchor::Inherit),
            anchor: $this.anchor,
            parent_anchor: $this.parent_anchor.unwrap_or($crate::Anchor::Inherit),
            offset: $this.offset,
            rotation: $this.rotation,
            scale: match $this.scale{
                Some($crate::dsl::prelude::OneOrTwo(vec)) => vec,
                None => $crate::bevy::math::Vec2::ONE,
            },
            z: $this.z
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! dimension {
    ($this: expr) => {
        match $this.dimension {
            Some(size) => $crate::Dimension::owned(size).with_em($this.font_size),
            None => $crate::Dimension::COPIED.with_em($this.font_size),
        }
    }
}

/// Create a widget extension based on the definition of `Frame`
#[macro_export]
macro_rules! widget_extension {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* }
        // Due to macro_rules, this shadows self.
        $(,$this: ident,
        $commands: ident,
        $assets: ident,
        components: ($($input: tt)*)
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { $($fields)* }
            // Due to macro_rules, this shadows self.
            $(,$this,
            $commands,
            $assets,
            input: ($($input)*),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? ))?
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident: Sprite { $($fields: tt)* }
        $(,$this: ident,
        $commands: ident,
        $assets: ident,
        components: (  $($input: tt)* )
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { 
                /// Handle of the image asset.
                pub sprite: $crate::dsl::HandleOrString<bevy::prelude::Image>,
                /// Size of the image.
                pub size: Option<$crate::bevy::prelude::Vec2>,
                /// Color of the image.
                pub color: Option<$crate::bevy::prelude::Color>,
                /// Atlas rectangle of the image.
                pub rect: Option<$crate::bevy::prelude::Rect>,
                /// Flips the image.
                pub flip: [bool; 2],
                $($fields)* 
            }
            $(,$this,
            $commands,
            $assets,
            input: (
                $crate::bevy::prelude::Sprite {
                    custom_size: $this.size,
                    color: $this.color.unwrap_or(bevy::prelude::Color::WHITE),
                    rect: $this.rect,
                    flip_x: $this.flip[0],
                    flip_y: $this.flip[1],
                    ..Default::default()
                },
                $this.sprite.get($assets),
                $crate::bundles::BuildTransformBundle::default(),
                $($input)*
            ),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? ))?
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident: Text { $($fields: tt)* }
        $(,$this: ident,
        $commands: ident,
        $assets: ident,
        components: ($($input: tt)*)
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { 
                /// The text string.
                pub text: String,
                /// Handle of the font asset.
                pub font: $crate::dsl::HandleOrString<bevy::prelude::Font>,
                /// Bounds of the text, should not be set most of the time.
                ///
                /// If not specified this is `UNBOUNDED`.
                pub bounds: Option<bevy::prelude::Vec2>,
                /// Color of the text.
                pub color: Option<bevy::prelude::Color>,
                /// Sets if the text wraps.
                pub wrap: bool,
                /// Break line on, maybe use wrap instead.
                pub break_line_on: Option<bevy::text::BreakLineOn>,
                $($fields)* 
            }
            $(,$this,
            $commands,
            $assets,
            input: (
                $crate::bevy::text::Text {
                    sections: vec![$crate::bevy::text::TextSection::new(
                        $this.text,
                        $crate::bevy::text::TextStyle {
                            font: $this.font.get($assets),
                            color: $this.color.unwrap_or($crate::bevy::prelude::Color::WHITE),
                            ..Default::default()
                        }
                    )],
                    linebreak_behavior: if let Some(b) = $this.break_line_on {
                        b
                    } else if $this.wrap {
                        $crate::bevy::text::BreakLineOn::WordBoundary
                    } else {
                        $crate::bevy::text::BreakLineOn::NoWrap
                    },
                    ..Default::default()
                },
                match $this.bounds {
                    Some(size) => $crate::bevy::text::Text2dBounds { size },
                    None => $crate::bevy::text::Text2dBounds::UNBOUNDED,
                },
                $crate::bevy::text::TextLayoutInfo::default(),
                Into::<bevy::sprite::Anchor>::into($this.anchor),
                $crate::bundles::BuildTransformBundle::default()
                $($input)*
            ),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? ))?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! widget_extension2 {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        $(#[$($parent_attr)*])*
        $vis0 struct $name {
            /// Anchor of the sprite.
            pub anchor: $crate::Anchor,
            /// Matched parent anchor of the sprite, default is `anchor`.
            /// Usually should not be set in idiomatic use.
            pub parent_anchor: Option<$crate::Anchor>,
            /// Center of the sprite, default is `anchor`.
            pub center: Option<$crate::Anchor>,
            /// Propagated opacity.
            pub opacity: $crate::Opacity,
            /// Visible, default is inherited.
            pub visible: Option<bool>,
            /// Offset of the sprite from parent's anchor.
            pub offset: $crate::Size2,
            /// Rotation of the sprite from `center`.
            pub rotation: f32,
            /// Scale of the sprite.
            pub scale: Option<$crate::dsl::OneOrTwo<$crate::bevy::math::Vec2>>,
            /// Z depth of the sprite.
            pub z: f32,
            /// Owned dimension of the sprite.
            /// 
            /// If not set, size is fetched dynamically from various sources.
            /// 
            /// The `size` field from `SpriteBuilder` sets the size of the underlying sprite instead.
            pub dimension: Option<$crate::Size2>,
            /// Propagated font size.
            pub font_size: $crate::FontSize,
            /// Sets up which event this receives.
            /// 
            /// Due to this being a confusing footgun, 
            /// setting event here automatically sets hitbox to `Rect(1)` if not set manually.
            pub event: Option<$crate::events::EventFlags>,
            /// The click detection area of the sprite.
            pub hitbox: Option<$crate::Hitbox>,
            /// The render layer of the sprite.
            pub layer: Option<$crate::bevy::render::view::RenderLayers>,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: (),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*
            $vis0 struct $name {
                $(
                    $(#[$($attr)*])*
                    $vis $field: $ty
                ),*
            }
        );

        const _: () = {
            use $crate::dsl::DslInto;
            use $crate::bevy::prelude::BuildChildren;
            impl $crate::dsl::Widget for $name {
                #[allow(unused)]
                fn spawn_with(self, $commands: &mut $crate::bevy::prelude::Commands, $assets: Option<&$crate::bevy::asset::AssetServer>) -> $crate::bevy::prelude::Entity {
                    let $this = self;
                    let mut base = $commands.spawn((
                        $crate::bundles::AoUIBundle {
                            transform: $crate::transform2d!($this),
                            dimension: $crate::dimension!($this),
                            opacity: $this.opacity,
                            vis: $this.visible.dinto(),
                            ..Default::default()
                        },
                        $($comp),*
                    ));
                    if let Some(event) = $this.event {
                        base.insert(event);
                    }
                    if let Some(hitbox) = $this.hitbox {
                        base.insert(hitbox);
                    } else if $this.event.is_some() {
                        base.insert($crate::Hitbox::FULL);
                    }
                    if let Some(layer) = $this.layer {
                        base.insert(layer);
                    } else {
                        if let Some(layer) = $crate::dsl::get_layer() {
                            base.insert($crate::bevy::render::view::RenderLayers::layer(layer.get()));
                        }
                    }
                    if $crate::dsl::is_using_opacity() {
                        base.insert($crate::OpacityWriter);
                    }
                    $(if $if {
                        base.insert($comp2);
                    })*
                    $(if let $pat = $pat_field {
                        base.insert($comp3);
                    })*
                    let base = base.id();
                    let children = [$(
                        {
                            let child = $children;
                            $commands.entity(child)$(.insert($comp4))?.id()
                        }
                    ),*];
                    $commands.entity(base).push_children(&children);
                    base
                }
            }
        };
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($bundle: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: ( $($children: expr $(=> $comp4: expr)? ),* )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: ($($rest)*),
            components: ( $($comp,)* $bundle),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: ( $($children $(=> $comp4)? ),* )
        );
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($bundle: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: ( $($children: expr $(=> $comp4: expr)? ),* )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: (),
            components: ( $($comp,)* $bundle),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: ( $($children $(=> $comp4)? ),* )
        );
    };
    (
        $(#[$($parent_attr:tt)*])*        
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($pat0: pat = $pat_field0: expr => $expr0: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: ($($rest)*),
            components: ($($comp),*),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3,)* $pat0 = $pat_field0 => $expr0),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*        
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($pat0: pat = $pat_field0: expr => $expr0: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: (),
            components: ($($comp),*),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3,)* $pat0 = $pat_field0 => $expr0),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($if0: expr => $expr0: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*            
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: ($($rest)*),
            components: ($($comp),*),
            dynamic: ($($if => $comp2,)* $if0 => $expr0),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        );
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        $assets: ident,
        input: ($if0: expr => $expr0: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*            
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            $assets,
            input: (),
            components: ($($comp),*),
            dynamic: ($($if => $comp2,)* $if0 => $expr0),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        );
    };
}

#[macro_export]
macro_rules! map_builder {
    ($this: expr => $target: ident move (
        $($moved:ident),* $(,)?
    )$({
        $($added: ident: $expr: expr),* $(,)?
    })?) => {
        $target {
            $($moved: $this.$moved,)*
            $($($added: $expr,)*)?
        }
    };
}