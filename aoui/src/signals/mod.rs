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
use bevy::{app::{Plugin, Update, PreUpdate}, ecs::{schedule::IntoSystemConfigs, component::Component, system::Query}};
pub use create::signal;
pub use globals::*;
pub use dto::{DataTransfer, Object};
pub use mpmc::*;
pub use storage::KeyStorage;
use sig::Signal;

use crate::{WorldExtension, schedule::{AouiEventSet, AouiWidgetEventSet}};

//use self::types::{SigSubmit, SigChange, SigDrag, SigScroll};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SignalsPlugin;

// /// Duplicates a signal input to many targets.
// #[derive(Debug, Component)]
// pub struct SignalDuplicator {
//     pub input: Receiver,
//     pub senders: Vec<Sender>
// }

// fn duplicate_signals(mut query: Query<&SignalDuplicator>) {
//     query.par_iter_mut().for_each(|x| {
//         let value = x.input.poll_dyn();
//         if value.is_some() {
//             for sender in x.senders.iter() {
//                 sender.send_object(value.clone())
//             }
//         }
//     })
// } 

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use systems::*;
        app
            // .register_signal::<()>()
            // .register_signal::<SigSubmit>()
            // .register_signal::<SigChange>()
            // .register_signal::<SigDrag>()
            // .register_signal::<SigScroll>()
            .init_resource::<KeyStorage>()
            .add_systems(PreUpdate, globals::send_fps.in_set(AouiEventSet))
            //.add_systems(PreUpdate, duplicate_signals.after(AouiWidgetEventSet))
            .add_systems(Update, signal_receive_text)
            .add_systems(Update, signal_receive_offset)
            .add_systems(Update, signal_receive_offset_x)
            .add_systems(Update, signal_receive_offset_y)
            .add_systems(Update, signal_receive_rotation)
            .add_systems(Update, signal_receive_scale)
            .add_systems(Update, signal_receive_scale_x)
            .add_systems(Update, signal_receive_scale_y)
            .add_systems(Update, signal_receive_dimension)
            .add_systems(Update, signal_receive_dimension_x)
            .add_systems(Update, signal_receive_dimension_y)
            .add_systems(Update, signal_receive_opacity)
            .add_systems(Update, signal_receive_disable)
            .add_systems(Update, signal_receive_opacity_disable)
            .add_systems(Update, signal_receive_color_sprite)
            .add_systems(Update, signal_receive_color_atlas)
            .add_systems(Update, signal_receive_color_text)
            .add_systems(Update, signal_receive_color_interpolate)
        ;
    }
}