//! This module provides signals and reactivity `bevy_aoui`.
//!
//! # Getting Started
//!
//! ```
//! # /*
//! let (sender, recv1, recv2, recv3, ...) = commands.signal();
//!
//! checkbox! (commands {
//!     change: sender,
//! });
//! text! (commands {
//!     text: "Please click",
//!     extra: recv1.recv::<SigText>().map(|f: bool| if f {
//!         "Checked".to_owned()
//!     } else {
//!         "unchecked".to_owned()
//!     }),
//! });
//! # */
//! ```
//!
//! # How this works?
//!
//! Signal is a mpmc cell that can hold one value per frame.
//!
//! [`AouiCommands`](crate::dsl::AouiCommands) contains a [`SignalPool`], that is used
//! to track, name and cleanup signals.
//!
//! The `signal()` function can create a list of signal builders that can be turned
//! into senders or receivers. Alternatives include `named_signal` and `shared_storage`
//!
//! ```
//! # /*
//! let (sender, receiver) = signal();
//! let sender = Handlers::<EvTextChange>::new(sender);
//! let receiver = receiver::recv0(|s: String, text: &mut Text| { format_widget! (text, "Text Contains: {}", s) });
//! # */
//! ```
//!
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
//! When working with type erased signals, especially those emitted by buttons,
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
mod globals;
mod signal;
mod signal_pool;

use bevy::{app::{Plugin, Update, PreUpdate, Last}, ecs::schedule::IntoSystemConfigs};
pub use globals::*;
pub use dto::{Object, AsObject};
pub use mpmc::*;
pub use receiver::*;
pub mod receiver;
pub(crate) use signal::Signal;

use crate::{schedule::{AouiEventSet, AouiCleanupSet}, dsl::CloneSplit};

pub use signal_pool::SignalPool;

/// Create a shared persistent value as an unmanaged signal.
pub fn storage_signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>() -> S {
    S::clone_split(SignalBuilder::new(Signal::new()))
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<SignalPool>()
            .add_systems(PreUpdate, globals::send_fps.in_set(AouiEventSet))
            .add_systems(Update, (
                signal_receive::<0>,
                signal_receive::<1>,
                signal_receive::<2>,
                signal_receive::<3>,
                signal_receive::<4>,
                signal_receive::<5>,
            ))
            .add_systems(Last, SignalPool::system_signal_cleanup.in_set(AouiCleanupSet))
        ;
    }
}
