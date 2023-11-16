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
pub use layouts::{SpanContainer, SpanContainerNames, GridContainer, GridContainerNames};
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
    pub use crate::{shape, rectangle, circle};
    pub use crate::{compact, paragraph, span, hbox, vbox, hspan, vspan};
    pub use crate::{fixed_table, flex_table, fixed_grid, sized_grid, sized_table};
    
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
    ($commands: tt [$($path: tt)*] [$($fields: tt)*]) => {
        $crate::meta_dsl!($commands [$($path)*] {$($fields)*} {} {} {})
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] child: $macro: ident !, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($path)*] [
            $($out)* 
            child: $macro! (
                $commands
            )
        ], $($rest)*)
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] child: $macro: ident ! {$($expr: tt)*}, $($rest: tt)*) => {
        $crate::filter_children!($commands [$($path)*] [
            $($out)* 
            child: $macro! (
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
            let children = [$($children),*];
            $commands.spawn_aoui((
                $($path)* {
                    $($field: ($expr).dinto(),)*
                    ..Default::default()
                },
                ($($extras),*),
                children,
            ))
        }
    };
}