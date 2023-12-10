#![allow(non_upper_case_globals)]

pub use crate::{color, colorv4, gradient, size2, markers};
pub use super::DslInto;
pub use super::util::*;
pub use super::util::DslHitbox::*;
pub use super::util::AoUISpacialConsts::*;
pub use super::AoUICommands;
pub use bevy::prelude::BuildChildren;
use bevy::sprite::Anchor;
pub use std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;
pub const EPS: f32 = f32::EPSILON;
pub use bevy::prelude::Color;
pub use crate::{Dimension, Opacity, SizeUnit, Size2};
pub use crate::layout::LayoutControl::{Linebreak, IgnoreLayout};
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
    LoseFocus,
    CustomCursor,
};
pub use crate::util::{signal, Submit, Change, Sender, Receiver};
pub use bevy::window::CursorIcon;
pub use crate::widgets::{
    PropagateFocus, DisplayIf, SetCursor, drag::DragSignal
};

pub const FlipX: [bool; 2] = [true, false];
pub const FlipY: [bool; 2] = [false, true];
pub const FlipBoth: [bool; 2] = [true, true];

pub const DragX: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::X;
pub const DragY: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::Y;
pub const DragBoth: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::BOTH;
pub const DragSnapBack: crate::widgets::drag::DragSnapBack = crate::widgets::drag::DragSnapBack::DEFAULT;

/// This can be use anywhere where you want to use the default anchor.
pub const Inherit: Option<Anchor> = None;

pub use crate::{frame, sprite, textbox};
pub use crate::{oneshot, handler};
pub use crate::{padding, compact, paragraph, span, hbox, vbox, hspan, vspan};
pub use crate::{table, flex_table, fixed_grid, sized_grid,};
pub use crate::{inputbox, button, clipping_frame};
pub use crate::rectangle;

pub use crate::dsl::context::with_layer;