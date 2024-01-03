/// this maps `macro! {}` into `macro! (ctx {})`
/// 
/// and `child: #macro!{}` into `children: quote_syntax! (ctx macro! {})`
#[doc(hidden)]
#[macro_export]
macro_rules! inline_context {
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($fields: tt)*]) => {
        $crate::meta_dsl!($ctx [$($path)*] {$($fields)*} {} {} {} {$($entity)?})
    };
    ($ctx: tt [$($path: tt)*] [] [$($out: tt)*] entity: $entity: ident $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$entity] [$($out)*] $($($rest)*)?)
    };

    ($ctx: tt [$($path: tt)*] [$e: ident] [$($out: tt)*] entity: $entity: ident $(,$($rest: tt)*)?) => {
        compile_error!("Duplicate field: entity.")
    };
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($out: tt)*] child: #$macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$($entity)?] [
            $($out)*
            children: $crate::quote_syntax!($ctx $macro { $($expr)* }),
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$($entity)?] [
            $($out)*
            $field: $macro! (
                $ctx {
                    $($expr)*
                }
            ),
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($out: tt)*] $field: ident: $head: expr $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$($entity)?] [$($out)* $field: $head,] $($($rest)*)?)
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

/// The core macro for creating DSL for widgets.
/// 
/// To create a custom widget macro:
/// ```
/// # /*
/// #[macro_export]
/// macro_rules! macro_name {
///     {$commands: tt {$($tt:tt)*}} => {
///         bevy_aoui::meta_dsl!($commands [$crate::absolute::path::to::WidgetBuilder] {
///             $($tt)*
///         })
///     };
/// }
/// # */
/// ```
#[macro_export]
macro_rules! meta_dsl {

    ($commands: tt [$($path: tt)*] {$($fields: tt)*} ) => {
        $crate::inline_context!($commands [$($path)*] [] [] $($fields)*)
    };

    ($commands: tt [$($path: tt)*]
        {extra: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($child_type: $children),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($child_type: $children,)* child: $expr}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {children: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($child_type: $children,)* children: $expr}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($child_type: $children),*}
            {$($out)?}
        )
    };

    (($commands: expr$(,)?) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        {
            use $crate::dsl::{DslInto, AouiCommands};
            #[allow(clippy::needless_update)]
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            let extras = ($($extras),*);
            let children = $crate::parse_children!(($commands) _children {$($child_type: $children),*} {});
            let out = $commands.spawn_aoui((
                entity,
                extras,
                children,
            ));
            $($out = out;)?
            out
        }
    };

    (($commands: expr, $assets: expr) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        {
            use $crate::dsl::{DslInto, AouiCommands};
            #[allow(clippy::needless_update)]
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            let extras = ($($extras),*);
            let children = $crate::parse_children!(($commands,$assets) _children {$($child_type: $children),*} {});
            let out = $commands.spawn_aoui_with_assets(
                &$assets, (
                    entity,
                    extras,
                    children,
                )
            );
            $($out = out;)?
            out
        }
    };

    ($commands: ident [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($child_type: ident: $children: expr),*}
        {$($out:ident)?}
    ) => {
        {
            use $crate::dsl::{DslInto, AouiCommands};
            #[allow(clippy::needless_update)]
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            let extras = ($($extras),*);
            let children = $crate::parse_children!($commands _children {$($child_type: $children),*} {});
            let out = $commands.spawn_aoui((
                entity,
                extras,
                children,
            ));
            $($out = out;)?
            out
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
        {
            let dimension = match $this.dimension {
                Some(size) => $crate::Dimension::owned(size),
                None => $crate::Dimension::COPIED.with_em($this.font_size),
            }.with_em($this.font_size);
            match $this.aspect {
                $crate::dsl::Aspect::None => dimension,
                $crate::dsl::Aspect::Preserve => dimension.with_preserve_aspect(true),
                $crate::dsl::Aspect::Owned(_) => dimension.with_preserve_aspect(true),
            }
        }
    }
}

/// Create a widget builder based on the definition of a primitive widget `Frame`.
/// 
/// Use `build_frame!` to utilize this definition.
#[macro_export]
macro_rules! widget_extension {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident $([$($generics: tt)*])? {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        $(#[$($parent_attr)*])*
        $vis0 struct $name $(<$($generics)*>)? {
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
            /// If true, clips its children, requires no rotation to function properly
            pub clipping: Option<bool>,
            /// Owned dimension of the sprite.
            /// 
            /// If not set, size is fetched dynamically from various sources.
            /// 
            /// The `size` field from `SpriteBuilder` sets the size of the underlying sprite instead.
            pub dimension: Option<$crate::Size2>,
            /// Aspect ration of sprite, default is o
            pub aspect: $crate::dsl::Aspect,
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
            /// Layout of the widget's children.
            pub layout: Option<Box<dyn $crate::layout::Layout>>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub margin: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub padding: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Displayed range of children, default is all.
            pub children_range: Option<::std::ops::Range<usize>>,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }
    };
}

/// Use a `FrameBuilder` to build a frame, returns an `EntityCommands`.
#[macro_export]
macro_rules! build_frame {
    ($commands: expr, $this: expr) => {
        {
            let entity = $crate::dsl::Widget::spawn($crate::dsl::builders::FrameBuilder {
                anchor: $this.anchor,
                parent_anchor: $this.parent_anchor,
                center: $this.center,
                opacity: $this.opacity,
                visible: $this.visible,
                offset: $this.offset,
                rotation: $this.rotation,
                scale: $this.scale,
                z: $this.z,
                dimension: $this.dimension,
                font_size: $this.font_size,
                event: $this.event,
                hitbox: $this.hitbox,
                layer: $this.layer,
                aspect: $this.aspect,
                clipping: $this.clipping,
                layout: $this.layout,
                margin: $this.margin,
                padding: $this.padding,
                children_range: $this.children_range,
            }, $commands);
            $commands.entity(entity.0)
        }
    }
}