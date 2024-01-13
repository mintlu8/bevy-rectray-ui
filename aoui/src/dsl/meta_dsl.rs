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
            child: $crate::quote_syntax!($ctx $macro { $($expr)* }),
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!($ctx [$($path)*] [$($entity)?] [
            $($out)*
            $field: $macro! ($ctx {
                $($expr)*
            }),
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

/// The core macro for creating DSL for widgets.
///
/// To create a custom widget macro:
/// ```
/// #[macro_export]
/// macro_rules! macro_name {
///     {$commands: tt {$($tt:tt)*}} => {
///         bevy_aoui::meta_dsl!($commands [$crate::absolute::path::to::WidgetBuilder] {
///             $($tt)*
///         })
///     };
/// }
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
        {$($children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($children),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children,)* $expr}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($children),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($out:ident)?}
    ) => {
        {
            #[allow(clippy::needless_update)]
            let entity = $($path)* {
                $($field: $crate::dsl::parse($expr),)*
                ..Default::default()
            };
            let extras = ($($extras),*);
            #[allow(unused_mut)]
            let mut children = ::std::vec::Vec::new();
            $(children.extend($crate::dsl::into_children($children));)*
            let out = $commands.spawn_aoui(
                entity,
                extras,
                children,
            );
            $($out = out;)?
            out
        }
    };
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
            pub parent_anchor: $crate::dsl::ParentAnchor,
            /// Center of the sprite, default is `center`.
            pub center: $crate::Anchor,
            /// Propagated opacity.
            pub opacity: $crate::Opacity,
            /// Visible, default is inherited.
            pub visible: Option<bool>,
            /// Offset of the sprite from parent's anchor.
            pub offset: $crate::Size2,
            /// Rotation of the sprite from `center`.
            pub rotation: f32,
            /// Scale of the sprite from `center`.
            pub scale: $crate::dsl::Scale,
            /// Z depth of the sprite.
            pub z: f32,
            /// If true, clips its children, currently only affects events.
            pub clipping: Option<bool>,
            /// Owned dimension of the sprite.
            ///
            /// If not set, size is fetched dynamically from various sources.
            ///
            /// The `size` field from `SpriteBuilder` sets the size of the underlying sprite instead.
            pub dimension: $crate::DimensionType,
            /// Aspect ratio of sprite, default unused.
            pub aspect: $crate::dsl::Aspect,
            /// Propagated font size.
            pub font_size: $crate::FontSize,
            /// Sets up which event this receives.
            ///
            /// Due to this being a confusing footgun,
            /// setting event here automatically sets hitbox to `Hitbox::rect(1)` if not set manually.
            pub event: $crate::events::EventFlags,
            /// The click detection area of the sprite.
            pub hitbox: Option<$crate::Hitbox>,
            /// The render layer of the sprite.
            pub layer: Option<$crate::bevy::render::view::RenderLayers>,
            /// Layout of the widget's children.
            ///
            /// If this is `Some`, the default `dimension` is `Dynamic` instead of `Copied`.
            pub layout: Option<$crate::layout::LayoutObject>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub margin: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub padding: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Displayed range of children, default is all, has no effect if widget has no layout.
            pub children_range: $crate::layout::LayoutRange,
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
