//! This module provides signals `bevy_aoui`.
//! 
//! # Getting Started
//! 
//! ```
//! # /*
//! let (sender, recv1, recv2, recv3, ...) = signal();
//! 
//! checkbox! {
//!     change: sender.mark::<SigChange>(),
//! }
//! text! {
//!     text: "Please click",
//!     extra: recv1.mark::<SigText>().map(|f: bool| if f {"Checked".to_owned()} else {"unchecked".to_owned()})
//! }
//! # */
//! ```
//! 
//! # How this works?
//! 
//! Signal is a mpmc cell that can hold one value per frame. The value is safe to use in `Update` and `PostUpdate`.
//! Signal holds a dynamic value [`Box<dyn DataTransfer>`](DataTransfer), 
//! essentially a `Box<dyn Any + Clone + PartialEq + Send + Sync>`, both [`Sender`] and [`Receiver`] can map
//! the value using an arbitrary function, and receiver will downcast the value to the desired output.
//! Invalid values will be **ignored** so it's better to keep this in mind.
//! This is extremely flexible but has a few footguns, notably `&'static str` can be sent 
//! but cannot downcast to string.
//! 
//! # Marked Signals
//! 
//! When used as components, we need to mark signals to disambiguate them.
//! Marking senders denote where the sender gets data from, which are implemented by widgets.
//! Marking receivers denote what the signal changes.
//! See implementors [`SignalSender`] and [`SignalReceiver`] for a detailed list.
//! 
//! Receivers use [`Interpolate`](crate::anim::Interpolate) automatically.
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
            //.add_systems(PreUpdate, duplicate_signals.after(AouiWidgetEventSet))
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