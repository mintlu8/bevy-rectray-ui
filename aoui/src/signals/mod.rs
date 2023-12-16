mod dto;
mod mpmc;
mod create;
pub mod types;
mod globals;
mod systems;
use bevy::{app::{Plugin, Update, PreUpdate}, ecs::schedule::IntoSystemConfigs};
pub use create::signal;
pub use globals::*;
pub use dto::{DataTransfer, Object};
pub use mpmc::{Sender, Receiver, SignalSender, SignalReceiver, signal_cleanup};

use crate::{WorldExtension, schedule::AoUIEventSet};

use self::types::{SigSubmit, SigChange, SigDrag, SigScroll};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use systems::*;
        app
            .register_signal::<()>()
            .register_signal::<SigSubmit>()
            .register_signal::<SigChange>()
            .register_signal::<SigDrag>()
            .register_signal::<SigScroll>()
            .add_systems(PreUpdate, globals::send_fps.in_set(AoUIEventSet))
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
            .add_systems(Update, signal_receive_color_sprite)
            .add_systems(Update, signal_receive_color_atlas)
            .add_systems(Update, signal_receive_color_text)
            .add_systems(Update, signal_receive_color_interpolate)
        ;
    }
}