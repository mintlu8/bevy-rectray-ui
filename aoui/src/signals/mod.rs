//! This module provides signals and reactivity `bevy_aoui`.
//! 
//! # Getting Started
//! 
//! ```
//! # /*
//! let (sender, recv1, recv2, recv3, ...) = signal();
//! 
//! checkbox! {
//!     change: sender,
//! }
//! text! {
//!     text: "Please click",
//!     extra: recv1.recv::<SigText>().map(|f: bool| if f {
//!         "Checked".to_owned()
//!     } else {
//!         "unchecked".to_owned()
//!     }),
//! }
//! # */
//! ```
//! 
//! # How this works?
//! 
//! Signal is a mpmc cell that can hold one value per frame. 
//! 
//! The `signal()` function can create a list of signal builders that can be turned
//! into senders or receivers.
//! 
//! ```
//! # /*
//! let (sender, receiver) = signal();
//! let sender = Handlers::<EvTextChange>::new(sender);
//! let receiver = receiver::recv::<SigText>::map(|s: String| format! ("Text Contains: {}", s));
//! # */
//! ```
//! 
//! `DslInto` allows builders to be passed into a lot of contexts directly.
//! 
//! ## Marked Signals
//! 
//! Event handlers can be used to send signals. See [`Handlers`](crate::events::Handlers).
//! 
//! Marking the receiver with a signal type like `SigOffset`. 
//! This allows the value of a signal to directly modify a component's value, 
//! thus achieving reactivity.
//! If [`Interpolate`](crate::anim::Interpolate) is present,
//! signals will use `Interpolate` instead.
//! 
//! ## Implementation Details
//! 
//! The value in signal works similar to bevy's [`Events`](bevy::ecs::event::Event). 
//! They are usually written in [`PreUpdate`], and cleaned up in [`Last`].
//! if read on the first frame, they live for `1` frame, if not they live for `2` frames.
//! Keep this in mind if you try to read a signal in multiple places.
//! 
//! Although usually typed, signals in fact holds a dynamic value 
//! [`Box<dyn DataTransfer>`](DataTransfer), and can be type erased. 
//! When working with type erased signals, especially those emitted by `Buttons`, 
//! invalid values will be **ignored**. It is better to keep this in mind.
//! 
//! ## Example: Spinning Arrowhead
//! ```
//! # /*
//! let (send, recv) = signal();
//! check_button! {
//!     checked: true,
//!     change: send,
//!     child: text! {
//!         text: "v",
//!         extra: recv.mark::<SigRotation>().map(|x: bool| if x {PI} else {0.0}),
//!         extra: transition! (Rotation 0.5 CubicInOut default PI)
//!     },
//! }
//! # */
//! ```

mod dto;
mod mpmc;
mod create;
pub mod types;
mod globals;
mod systems;
mod storage;
mod sig;
use std::sync::atomic::AtomicU8;

use atomic::Ordering;
use bevy::{app::{Plugin, Update, PreUpdate, Last}, ecs::{schedule::IntoSystemConfigs, system::{Resource, ResMut}}};
pub use create::signal;
pub use globals::*;
pub use dto::{DataTransfer, Object};
pub use mpmc::*;
pub use storage::KeyStorage;
use sig::Signal;

use crate::schedule::{AouiEventSet, AouiCleanupSet};

//use self::types::{SigSubmit, SigChange, SigDrag, SigScroll};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SignalsPlugin;


/// Flags for making signals live for 2 frames.
#[derive(Debug, Resource)]
pub struct DropFlag(AtomicU8);

impl DropFlag {
    pub fn get(&self) -> u8 {
        self.0.load(Ordering::Relaxed)
    }

    pub fn incr(&mut self) {
        let v = self.0.get_mut();
        *v = 3 - *v;
    }

    pub fn system_incr(mut res: ResMut<DropFlag>) {
        res.incr()
    }
}

impl Default for DropFlag {
    fn default() -> Self {
        Self(AtomicU8::new(1))
    }
}

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use systems::*;
        app
            .init_resource::<KeyStorage>()
            .init_resource::<DropFlag>()
            .add_systems(PreUpdate, globals::send_fps.in_set(AouiEventSet))
            .add_systems(Update, (
                signal_receive_text,
                signal_receive_text,
                signal_receive_offset,
                signal_receive_offset_x,
                signal_receive_offset_y,
                signal_receive_rotation,
                signal_receive_scale,
                signal_receive_scale_x,
                signal_receive_scale_y,
                signal_receive_dimension,
                signal_receive_dimension_x,
                signal_receive_dimension_y,
                signal_receive_opacity,
                signal_receive_disable,
                signal_receive_opacity_disable,
                signal_receive_color_sprite,
                signal_receive_color_atlas,
                signal_receive_color_text,
                signal_receive_color_interpolate,
            ))
            .add_systems(Last, DropFlag::system_incr.before(AouiCleanupSet))
            .add_systems(Last, KeyStorage::system_reset_changed_status)
        ;
    }
}