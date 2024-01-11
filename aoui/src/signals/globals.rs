use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::system::Res};
use once_cell::sync::Lazy;

use crate::events::mutation::IntoMutationCommand;

use super::{SignalSender, storage_signal, receiver::SignalReceiver};

pub(crate) static SIG_FPS: Lazy<SignalSender<f32>> = Lazy::new(|| {
    let (send, _) = storage_signal();
    send.send()
});

pub(crate) fn send_fps(fps: Option<Res<DiagnosticsStore>>) {
    let Some(fps) = fps else {return};
    if let Some(value) = fps
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed()) {
        SIG_FPS.send(value as f32)
    }
}

/// Signal receiver for the `FPS` as a `f32`. Requires `FrameTimeDiagnosticsPlugin`.
pub fn fps_signal<A, B>(f: impl IntoMutationCommand<f32, A, B>) -> SignalReceiver<0> {
    SIG_FPS.new_receiver().recv(f)
}
