mod convert;
mod util;
use std::fmt::Debug;

use bevy::prelude::{Commands, Entity, BuildChildren, Bundle};
#[doc(hidden)]
pub use colorthis::rgbaf;

mod core;
mod layouts;
mod shapes;
pub use shapes::Shape;
pub use convert::DslInto;
pub use core::{Frame, Sprite, TextBox};

pub mod prelude {
    pub use crate::color;
    pub use crate::size2;
    pub use super::util::*;
    pub use super::util::AoUISpacialConsts::*;
    pub use super::AouiCommands;
    pub use bevy::prelude::BuildChildren;
    use bevy::sprite::Anchor;
    pub use crate::widgets::shape::Shapes;
    pub use super::layouts::linebreak;
    pub use std::f32::consts::PI;

    #[allow(non_upper_case_globals)]
    pub const Inherit: Option<Option<Anchor>> = Some(None);

    #[macro_export]
    macro_rules! frame {
        {$commands: tt {$($tt:tt)*}} => 
            {$crate::meta_dsl!($commands [$crate::dsl::Frame] {$($tt)*})};
    }
    pub use frame;
    #[macro_export]
    macro_rules! sprite {
        {$commands: tt {$($tt:tt)*}} => 
            {$crate::meta_dsl!($commands [$crate::dsl::Sprite] {$($tt)*})};
    }
    pub use sprite;
    #[macro_export]
    macro_rules! textbox {
        {$commands: tt {$($tt:tt)*}} => 
            {$crate::meta_dsl!($commands [$crate::dsl::TextBox] {$($tt)*})};
    }
    pub use textbox;
    #[macro_export]
    macro_rules! shape {
        {($commands: expr, $server: expr $(, $ctx: expr)*) {$($tt:tt)*}} => 
            {$crate::meta_dsl!(($commands, $server $(, $ctx)*) [$crate::dsl::Shape] {
                default_material: $server.add(::bevy::prelude::ColorMaterial::default()),
                $($tt)*
            })};
    }
    pub use shape;
    
}

pub trait FnChildren {
    type Out: AsRef<[Entity]> + Default;
    fn exec(self, commands: &mut Commands) -> Self::Out;
}

impl<F, Out> FnChildren for F where F: FnOnce(&mut Commands) -> Out, Out: AsRef<[Entity]> + Default {
    type Out = Out;

    fn exec(self, commands: &mut Commands) -> Self::Out {
        self(commands)
    }
}

#[derive(Debug, Default)]
pub enum EntitiesBuilder<F: FnChildren>{
    Some(F),
    #[default]
    None,
}

impl<F: FnChildren> EntitiesBuilder<F> {
    pub fn build_entities(self, commands: &mut Commands) -> F::Out{
        match self {
            EntitiesBuilder::Some(f) => f.exec(commands),
            EntitiesBuilder::None => Default::default(),
        }
    }
}

#[doc(hidden)]
pub trait AouiCommands {
    fn spawn_aoui(&mut self, a: (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity;
}

impl<'w, 's> AouiCommands for Commands<'w, 's> {
    fn spawn_aoui(&mut self, (widget, extras, children): (impl AoUIWidget, impl Bundle, impl AsRef<[Entity]>)) -> Entity {
        let id = widget.spawn_with(self);
        self.entity(id)
            .insert(extras)
            .push_children(children.as_ref());
        id
    }
}

pub trait AoUIWidget: Sized {
    fn spawn_with(self, commands: &mut Commands) -> Entity;
}

#[macro_export]
macro_rules! filter_children {
    ($commands: tt [$($tt: tt)*] [$($fields: tt)*]) => {
        $crate::meta_dsl!($commands [$($tt)*] {$($fields)*} {} {} {})
    };
    ($commands: tt [$($tt: tt)*] [$($out: tt)*] child: $macro: ident !, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($tt)*] [
            $($out)* 
            child: $macro! (
                $commands
            )
        ], $($rest)*)
    };
    ($commands: tt [$($tt: tt)*] [$($out: tt)*] child: $macro: ident ! {$($expr: tt)*} $($rest: tt)*) => {
        $crate::filter_children!($commands [$($tt)*] [
            $($out)* 
            child: $macro! (
                $commands {
                    $($expr)*
                }
            )
        ] $($rest)*)
    };
    ($commands: tt [$($tt: tt)*] [$($out: tt)*] $head: tt $($rest: tt)*) => {
        $crate::filter_children!($commands [$($tt)*] [$($out)* $head] $($rest)*)
    };
}

#[macro_export]
macro_rules! meta_dsl {
    // ($commands: tt [$($tt: tt)*] {$($field: ident: $expr: expr),* $(,)?} ) => {
    //     $crate::meta_dsl!($commands [$($tt)*] {$($field: $expr),*} {} {} {})
    // };
    ($commands: tt [$($tt: tt)*] {$($fields: tt)*} ) => {
        $crate::filter_children!($commands [$($tt)*] [] $($fields)*)
    };

    ($commands: tt [$($tt: tt)*] 
        {extra: $expr: expr $(,$f: ident: $e: expr)* $(,)?} 
        {$($f2: ident: $e2: expr),*} 
        {$($extras: expr),*} 
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($tt)*] 
            {$($f: $e),*} 
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($children),*}
        )
    };

    ($commands: tt [$($tt: tt)*] 
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?} 
        {$($f2: ident: $e2: expr),*} 
        {$($extras: expr),*} 
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($tt)*] 
            {$($f: $e),*} 
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children,)* $expr}
        )
    };

    ($commands: tt [$($tt: tt)*] 
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?} 
        {$($f2: ident: $e2: expr),*} 
        {$($extras: expr),*} 
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($tt)*] 
            {$($f: $e),*} 
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($children),*}
        )
    };


    (($commands: expr $(,$e:expr)*) [$($tt: tt)*] {$(,)?} 
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*} 
        {$($children: expr),*}
    ) => {
        {  
            use $crate::dsl::DslInto;
            let children = [$($children),*];
            $commands.spawn_aoui((
                $($tt)* {
                    $($field: ($expr).dinto(),)*
                    ..Default::default()
                },
                ($($extras),*),
                children,
            ))
        }
    };
}