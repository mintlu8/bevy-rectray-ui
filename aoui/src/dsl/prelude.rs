#![allow(non_upper_case_globals)]

use crate::Anchor;
use crate::BuildTransform;
pub use crate::{color, colors, gradient, transition, size2, markers};
pub use crate::format_widget;
pub use crate::util::convert::{DslFrom, DslInto};
pub use super::util::*;
pub use super::util::AouiSpacialConsts::*;
pub use crate::util::AouiCommands;
pub use bevy::prelude::BuildChildren;
pub use std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;
pub const EPS: f32 = f32::EPSILON;
pub use bevy::prelude::Color;
pub use crate::{Transform2D, Hitbox, Dimension, Opacity, Detach, SizeUnit, Size2};
pub use crate::layout::LayoutControl::{Linebreak, IgnoreLayout};
pub use crate::anim::{Interpolate, Offset, Rotation, Scale, Index};
pub use interpolation::EaseFunction;

/// Return this inside `AsyncSystem` functions.
#[allow(nonstandard_style)]
pub const AsyncOk: Result<(), crate::sync::AsyncFailure> = Ok(());
pub use crate::events::{
    EventFlags, CustomCursor, TrackCursor,
    GreaterBoundingBox, GreaterBoundingBoxPx, GreaterBoundingBoxPercent,
};
pub use bevy::window::CursorIcon;
pub use crate::widgets::{
    util::{
        PropagateFocus, DisplayIf, SetCursor,
    },
    button::{
        CheckButtonState, radio_button_group,
        CheckButton, RadioButton, ToggleChange, ButtonClick
    },
    constraints::{PositionFac, SharedPosition},
    scroll::{Scrolling, ScrollParent},
    drag::Dragging,
    inputbox::InputOverflow
};
pub use crate::sync:: {
    SigSend, SigRecv, Signals,
    AsyncEntityQuery as Aeq, AsyncEntityCommands, AsyncQuery as Aq,
    AsyncComponent as Ac, AsyncResource as Ar, Fps,
    TypedSignal, RoleSignal, SignalId, SignalMapper,
};

pub const FlipX: [bool; 2] = [true, false];
pub const FlipY: [bool; 2] = [false, true];
pub const FlipBoth: [bool; 2] = [true, true];
pub const DragSnapBack: crate::widgets::drag::DragSnapBack = crate::widgets::drag::DragSnapBack::DEFAULT;
pub const Inherit: Anchor = Anchor::INHERIT;

pub use super::atlas::AtlasRectangles::Grid as AtlasGrid;
pub use super::Aspect::Preserve;

pub use crate::{frame, sprite, text, atlas};
pub use crate::{material_sprite, material_mesh};
//pub use crate::{one_shot, handler};
pub use crate::{padding, paragraph, hstack, vstack, hbox, vbox, linebreak};
pub use crate::{inputbox, button, check_button, radio_button, camera_frame};
pub use crate::rectangle;
pub use crate::signal_ids;

use bevy::ecs::bundle::Bundle;
use bevy::transform::components::GlobalTransform;

pub use crate::util::signal;
pub use crate::widgets::signals::*;

/// A signal with the sender role.
pub fn sender<T: SignalId>(sig: TypedSignal<T::Data>) -> RoleSignal<T> {
    RoleSignal::Sender(sig)
}

/// A signal with the receiver role.
pub fn receiver<T: SignalId>(sig: TypedSignal<T::Data>) -> RoleSignal<T> {
    RoleSignal::Receiver(sig)
}

/// Add a adaptor that polls a signal type's value mapped from a signal of another type.
/// 
/// This only affects sync APIs on receivers, i.e. `poll_once`.
/// Async systems are not affected by this.
pub fn adaptor<From: SignalId, To: SignalId>(f: impl Fn(From::Data) -> To::Data + Clone + Send + Sync + 'static) -> RoleSignal<To> {
    RoleSignal::Adaptor(std::any::TypeId::of::<From>(), SignalMapper::new::<From, To>(f))
}

/// Build transform at an anchor.
pub fn build_transform_at(anc: Anchor) -> impl Bundle {
    (
        BuildTransform(anc),
        GlobalTransform::default()
    )
}
