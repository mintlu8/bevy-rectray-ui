use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::system::Res};
use once_cell::sync::Lazy;

use super::{Sender, Receiver, signal};



pub(crate) static SIG_FPS: Lazy<(Sender, Receiver)> = Lazy::new(signal);

pub(crate) fn send_fps(fps: Option<Res<DiagnosticsStore>>) {
    let Some(fps) = fps else {return};
    if let Some(value) = fps
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed()) {
        SIG_FPS.0.send(value as f32)
    }
}

/// Signal receiver for the `FPS` as a `f32`. requires `FrameTimeDiagnosticsPlugin`
pub fn sig_fps() -> Receiver {
    SIG_FPS.1.fork()
}