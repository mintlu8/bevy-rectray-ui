#[doc(hidden)]
#[macro_export]
macro_rules! filter_children {
    ($commands: tt [$($path: tt)*] [$($fields: tt)*]) => {
        $crate::meta_dsl!($commands [$($path)*] {$($fields)*} {} {} {})
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $macro: ident !, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($path)*] [
            $($out)*
            $field: $macro! (
                $commands
            )
        ], $($rest)*)
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*}, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($path)*] [
            $($out)*
            $field: $macro! (
                $commands {
                    $($expr)*
                }
            ),
        ] $($rest)*)
    };

    ($commands: tt [$($path: tt)*] [$($out: tt)*] child: $macro: ident ! {$($expr: tt)*}) => {
        $crate::filter_children!($commands [$($path)*] [
            $($out)*
            child: $macro! (
                $commands {
                    $($expr)*
                }
            )
        ])
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $head: expr, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($path)*] [$($out)* $field: $head,] $($rest)*)
    };

    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $head: expr) => {
        $crate::filter_children!($commands [$($path)*] [$($out)* $field: $head])
    };
}

/// The core macro for our DSL.
#[macro_export]
macro_rules! meta_dsl {

    ($commands: tt [$($path: tt)*] {$($fields: tt)*} ) => {
        $crate::filter_children!($commands [$($path)*] [] $($fields)*)
    };

    ($commands: tt [$($path: tt)*]
        {extra: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($children),*}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children,)* $expr}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($children),*}
        )
    };

    (($commands: expr $(,$e:expr)*) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        {
            use $crate::dsl::DslInto;
            let extras = ($($extras),*);
            let children = [$($children),*];
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
        ::bevy_aoui::Transform2D {
            center: $this.center.unwrap_or(::bevy_aoui::Anchor::Inherit),
            anchor: $this.anchor,
            parent_anchor: $this.parent_anchor.unwrap_or(::bevy_aoui::Anchor::Inherit),
            offset: $this.offset,
            rotation: $this.rotation,
            scale: match $this.scale{
                Some($crate::dsl::prelude::OneOrTwo(vec)) => vec,
                None => ::bevy::math::Vec2::ONE,
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
            Some(size) => ::bevy_aoui::Dimension::owned(size).with_em($this.font_size),
            None => ::bevy_aoui::Dimension::COPIED.with_em($this.font_size),
        }
    }
}

/// Create a widget extension based on the definition of `Frame`
#[macro_export]
macro_rules! widget_extension {
    (
        $vis0: vis struct $name: ident {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        },
        $this: ident,
        $commands: ident
        $(,components: (
            $($comp: expr),* $(,)?
        ))?
        $(,dynamic: (
            $($if: expr => $comp2: expr),* $(,)?
        ))?
        $(,pattern: (
            $($pat: pat = $pat_field: expr => $comp3: expr),* $(,)?
        ))?
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?
    ) => {
        #[derive(Debug, Default)]
        $vis0 struct $name {
            pub anchor: ::bevy_aoui::Anchor,
            pub parent_anchor: Option<::bevy_aoui::Anchor>,
            pub center: Option<::bevy_aoui::Anchor>,
            pub visible: Option<bool>,
            pub offset: ::bevy_aoui::Size2,
            pub rotation: f32,
            pub scale: Option<$crate::dsl::OneOrTwo<::bevy::math::Vec2>>,
            pub z: f32,
            pub dimension: Option<::bevy_aoui::Size2>,
            pub font_size: ::bevy_aoui::SetEM,
            pub hitbox: Option<::bevy_aoui::Hitbox>,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }

        const _: () = {
            use $crate::dsl::DslInto;
            impl $crate::dsl::AoUIWidget for $name {
                fn spawn_with(self, $commands: &mut ::bevy::prelude::Commands) -> ::bevy::prelude::Entity {
                    let $this = self;
                    let mut base = $commands.spawn((
                        bevy_aoui::bundles::AoUIBundle {
                            transform: $crate::transform2d!($this),
                            dimension: $crate::dimension!($this),
                            vis: $this.visible.dinto(),
                            ..Default::default()
                        },
                        $($($comp),*)?
                    ));
                    if let Some(hitbox) = $this.hitbox {
                        base.insert(hitbox);
                    }
                    $($(if $if {
                        base.insert($comp2);
                    })*)?
                    $($(if let $pat = $pat_field {
                        base.insert($comp3);
                    })*)?
                    let base = base.id();
                    $(
                        let children = [$(
                            {
                                let child = $children;
                                $commands.entity(child)$(.insert($comp4))?.id()
                            }
                        ),*];
                        $commands.entity(base).push_children(&children);
                    )?
                    base
                }
            }
        };
    };
}
