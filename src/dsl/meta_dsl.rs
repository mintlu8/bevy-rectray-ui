/// this maps `macro! {}` into `macro! (ctx {})`
///
/// and `child: #macro!{}` into `children: quote_syntax! (ctx macro! {})`
#[doc(hidden)]
#[macro_export]
macro_rules! inline_context {
    (@ $ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*]) => {
        $crate::meta_dsl2!($ctx [$($path)*] {$($field: $value),*} {} {} {} {} {} {$($entity)?})
    };

    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*]) => {
        $crate::meta_dsl2!($ctx [$($path)*] {$($field: $value),*} {} {} {} {} {} {$($entity)?})
    };

    (@ $ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*] $field2: ident $($rest: tt)*) => {
        {
            $crate::format_intrinsics!($field2, $field2);
            $crate::inline_context!($ctx [$($path)*] [$($entity)?] [$($field: $value),*] $field2 $($rest)*)
        }
    };

    ($ctx: tt [$($path: tt)*] [] [$($field: ident: $value: expr),*] entity: $entity: ident $(,$($rest: tt)*)?) => {
        $crate::inline_context!(@ $ctx [$($path)*] [$entity] [$($field: $value),*] $($($rest)*)?)
    };

    ($ctx: tt [$($path: tt)*] [$e: ident] [$($field: ident: $value: expr),*] entity: $entity: ident $(,$($rest: tt)*)?) => {
        compile_error!("Duplicate field: entity.")
    };
    
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*] child: #$macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!(@ $ctx [$($path)*] [$($entity)?] [
            $($field: $value,)*
            child: $crate::quote_syntax!($ctx $macro { $($expr)* })
        ] $($($rest)*)?)
    };
    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*] $field2: ident: $macro: ident ! {$($expr: tt)*} $(,$($rest: tt)*)?) => {
        $crate::inline_context!(@ $ctx [$($path)*] [$($entity)?] [
            $($field: $value,)*
            $field2: $macro! ($ctx {
                $($expr)*
            })
        ] $($($rest)*)?)
    };

    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*] system: |$($arg:ident: $ty: ty),* $(,)?| $expr: expr $(,$($rest: tt)*)?) => {
        $crate::inline_context!(@ $ctx [$($path)*] [$($entity)?] [
            $($field: $value,)*
            system: $crate::async_system!(|$($arg: $ty),*| $expr)
        ] $($($rest)*)?)
    };

    ($ctx: tt [$($path: tt)*] [$($entity:ident)?] [$($field: ident: $value: expr),*] $field2: ident: $head: expr $(,$($rest: tt)*)?) => {
        $crate::inline_context!(@ $ctx [$($path)*] [$($entity)?] [
            $($field: $value,)*
            $field2: $head
        ] $($($rest)*)?)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! format_intrinsics {
    ($name: ident, entity) => {
        { 
            let _ = $crate::dsl::intrinsics::IntrinsicEntity { 
                $name: $crate::dsl::intrinsics::MutEntity
            };
        }
    };
    ($name: ident, extra) => {
        { 
            let _ = $crate::dsl::intrinsics::IntrinsicExtra { 
                $name: $crate::dsl::intrinsics::ImplBundle
            };
        }
    };
    ($name: ident, child) => {
        { 
            let _ = $crate::dsl::intrinsics::IntrinsicChild { 
                $name: $crate::dsl::intrinsics::EntityOrIterator
            };
        }
    };
    ($name: ident, signal) => {
        { 
            let _ = $crate::dsl::intrinsics::IntrinsicSignal { 
                $name: $crate::dsl::intrinsics::RoleSignal
            };
        }
    };
    ($name: ident, system) => {
        { 
            let _ = $crate::dsl::intrinsics::IntrinsicSystem { 
                $name: $crate::dsl::intrinsics::AsyncSystem
            };
        }
    };
    ($span: ident, $unmatched: ident) => {};
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
///         bevy_rectray::meta_dsl!($commands [$crate::absolute::path::to::WidgetBuilder] {
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
}

#[doc(hidden)]
#[macro_export]
macro_rules! meta_dsl2 {
    ($commands: tt [$($path: tt)*]
        {extra: $expr: expr $(,$f: ident: $e: expr)*}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($signal: expr),*}
        {$($system: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl2!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($children),*}
            {$($signal),*}
            {$($system),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)*}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($signal: expr),*}
        {$($system: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl2!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children,)* $expr}
            {$($signal),*}
            {$($system),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {signal: $expr: expr $(,$f: ident: $e: expr)*}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($signal: expr),*}
        {$($system: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl2!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children),*}
            {$($signal,)* $expr}
            {$($system),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {system: $expr: expr $(,$f: ident: $e: expr)*}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($signal: expr),*}
        {$($system: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl2!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children),*}
            {$($signal),*}
            {$($system,)* $expr}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)*}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($signal: expr),*}
        {$($system: expr),*}
        {$($out:ident)?}
    ) => {
        $crate::meta_dsl2!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($children),*}
            {$($signal),*}
            {$($system),*}
            {$($out)?}
        )
    };

    ($commands: tt [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
        {$($first_sig: expr $(,$signal: expr)*)?}
        {$($first_sys: expr $(,$system: expr)*)?}
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
            $(
                let signal = $first_sig $(.and($signal))*;
                $crate::util::ComposeExtension::compose(
                    &mut $commands.entity(out),
                    signal.into_signals()
                );
            )?
            $(
                let system = $crate::defer::AsyncSystems::from_iter([$first_sys, $($system),*]);
                $crate::util::ComposeExtension::compose(
                    &mut $commands.entity(out),
                    system
                );
            )?
            $($out = out;)?
            out
        }
    };
}
