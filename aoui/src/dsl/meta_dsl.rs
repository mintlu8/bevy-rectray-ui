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
