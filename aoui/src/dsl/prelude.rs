#![allow(non_upper_case_globals)]

use crate::Anchor;
use crate::BuildTransform;
pub use crate::{color, colors, gradient, transition, size2, markers};
pub use crate::format_widget;
pub use super::DslInto;
pub use super::util::*;
pub use super::util::DslHitbox::*;
pub use super::util::AouiSpacialConsts::*;
pub use super::AouiCommands;
pub use bevy::prelude::BuildChildren;
pub use std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;
pub const EPS: f32 = f32::EPSILON;
pub use bevy::prelude::Color;
pub use crate::{Transform2D, Dimension, Opacity, Detach, SizeUnit, Size2};
pub use crate::layout::LayoutControl::{Linebreak, IgnoreLayout};
pub use crate::anim::{Interpolate, Offset, Rotation, Scale, Index};
pub use interpolation::EaseFunction;
pub use crate::events::{
    EventFlags,
    CustomCursor, TrackCursor,
    EvLeftDown, EvLeftClick,
    EvMidDown, EvMidClick,
    EvRightDown, EvRightClick,
    EvDragEnd, EvDrop, EvClickOutside,
    EvHover,
    EvLeftPressed, EvLeftDrag,
    EvMidPressed, EvMidDrag,
    EvRightPressed, EvRightDrag,

    EvButtonClick, EvToggleChange,
    EvObtainFocus, EvLoseFocus,
    EvMouseDrag, EvTextChange, EvTextSubmit,
    EvPositionFactor,
    Handlers, Handler, OneShot, Mutation
};
pub use crate::signals::{storage_signal, fps_channel, SignalSender, SignalReceiver};
pub use bevy::window::CursorIcon;
pub use crate::widgets::SharedPosition;
pub use crate::widgets::button::{
    CheckButtonState, radio_button_group,
    CheckButton, RadioButton
};
pub use crate::widgets::util::{
    PropagateFocus, DisplayIf, SetCursor,
};
pub use crate::widgets::scroll::{Scrolling, IntoScrollingBuilder};
pub use crate::widgets::drag::IntoDraggingBuilder;

pub const FlipX: [bool; 2] = [true, false];
pub const FlipY: [bool; 2] = [false, true];
pub const FlipBoth: [bool; 2] = [true, true];

pub const DragX: crate::widgets::drag::Dragging = crate::widgets::drag::Dragging::X;
pub const DragY: crate::widgets::drag::Dragging = crate::widgets::drag::Dragging::Y;
pub const DragBoth: crate::widgets::drag::Dragging = crate::widgets::drag::Dragging::BOTH;
pub const DragSnapBack: crate::widgets::drag::DragSnapBack = crate::widgets::drag::DragSnapBack::DEFAULT;

pub const ScrollX: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::X;
pub const ScrollY: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::Y;
pub const ScrollNegX: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::NEG_X;
pub const ScrollNegY: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::NEG_Y;
pub const ScrollPosX: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::POS_X;
pub const ScrollPosY: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::POS_Y;

pub const ScrollBoth: crate::widgets::scroll::Scrolling = crate::widgets::scroll::Scrolling::BOTH;
pub const Inherit: Anchor = Anchor::Inherit;

pub use crate::widgets::inputbox::InputOverflow;
pub use super::atlas::AtlasRectangles::Grid as AtlasGrid;
pub use super::Aspect::Preserve;

pub use crate::{frame, sprite, text, atlas};
pub use crate::{material_sprite, material_mesh};
//pub use crate::{one_shot, handler};
pub use crate::{padding, paragraph, hstack, vstack, hbox, vbox, linebreak};
pub use crate::{inputbox, button, check_button, radio_button, camera_frame, scrolling};
pub use crate::rectangle;

use bevy::ecs::bundle::Bundle;
use bevy::transform::components::GlobalTransform;

/// Build transform at an anchor.
pub fn build_transform_at(anc: Anchor) -> impl Bundle {
    (
        BuildTransform(anc),
        GlobalTransform::default()
    )
}
