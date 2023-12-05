#![allow(non_upper_case_globals)]

pub use crate::{color, size2, markers};
use super::DslInto;
pub use super::util::*;
pub use super::util::Hitbox::*;
pub use super::util::AoUISpacialConsts::*;
pub use super::AoUICommands;
pub use bevy::prelude::BuildChildren;
use bevy::sprite::Anchor;
pub use crate::widgets::shape::Shapes;
pub use std::f32::consts::PI;
pub use std::f32::INFINITY;
pub use bevy::prelude::Color;
pub use bevy_aoui::{Dimension, Opacity, SizeUnit};
pub use bevy_aoui::layout::LayoutControl::{Linebreak, IgnoreLayout};
pub use crate::anim::{Interpolate, Offset, Rotation, Scale};
pub use interpolation::EaseFunction;
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
    OnSubmit,
    LoseFocus,
    CustomCursor,
};
pub use bevy::window::CursorIcon;
pub use crate::widgets::{
    PropagateFocus, DisplayIf, SetCursor
};

pub const DragX: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::X;
pub const DragY: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::Y;
pub const DragBoth: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::BOTH;
pub const DragSnapBack: crate::widgets::drag::DragSnapBack = crate::widgets::drag::DragSnapBack::DEFAULT;

/// This can be use anywhere where you want to use the default anchor.
pub const Inherit: Option<Anchor> = None;

/// Multiply by epsilon, useful in Z.
pub fn eps(value: impl DslInto<f32>) -> f32{
    value.dinto() * f32::EPSILON
}

pub use crate::{frame, sprite, textbox};
pub use crate::{oneshot, handler};
pub use crate::{shape, rectangle, circle};
pub use crate::{compact, paragraph, span, hbox, vbox, hspan, vspan};
pub use crate::{table, flex_table, fixed_grid, sized_grid,};
pub use crate::inputbox;

