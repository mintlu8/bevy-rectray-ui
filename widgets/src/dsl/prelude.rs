pub use crate::{color, size2, marker};
use super::DslInto;
pub use super::util::*;
pub use super::util::Hitbox::*;
pub use super::util::AoUISpacialConsts::*;
pub use super::AoUICommands;
pub use bevy::prelude::BuildChildren;
use bevy::sprite::Anchor;
pub use crate::widgets::shape::Shapes;
pub use std::f32::consts::PI;
pub use bevy_aoui::LayoutControl::{Linebreak, IgnoreLayout};
pub use crate::events::{
    EventFlags,
    Down as LeftDown, Click as LeftClick, 
    MidDown, MidClick, 
    RightDown, RightClick,
    DragEnd, Drop, ClickOutside,
    Hover, 
    Pressed as LeftPressed, Drag as LeftDrag,
    MidPressed, MidDrag,
    RightPressed, RightDrag,
    OnSubmit
};
pub use crate::widgets::{
    PropagateFocus, DisplayIf, SetCursor
};

/// This can be use anywhere where you want to use the default anchor.
#[allow(non_upper_case_globals)]
pub const Inherit: Option<Anchor> = None;

/// Multiply by epsilon, useful in Z.
pub fn eps(value: impl DslInto<f32>) -> f32{
    value.dinto() * f32::EPSILON
}

/// Construct an empty sprite.
#[macro_export]
macro_rules! frame {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {$($tt)*})};
}
/// Construct an image based sprite.
#[macro_export]
macro_rules! sprite {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::SpriteBuilder] {$($tt)*})};
}
/// Construct a textbox.
#[macro_export]
macro_rules! textbox {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::TextBoxBuilder] {$($tt)*})};
}
pub use {frame, sprite, textbox};
pub use crate::{oneshot, handler};
pub use crate::{shape, rectangle, circle};
pub use crate::{compact, paragraph, span, hbox, vbox, hspan, vspan};
pub use crate::{fixed_table, flex_table, fixed_grid, sized_grid, sized_table};
pub use crate::inputbox;

