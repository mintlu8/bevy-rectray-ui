use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::system::Res};
use once_cell::sync::Lazy;

use super::{Sender, Receiver, signal, SignalReceiver};

pub(crate) static SIG_FPS: Lazy<Sender<f32>> = Lazy::new(|| {
    let (send, _) = signal();
    send.build()
});

pub(crate) fn send_fps(fps: Option<Res<DiagnosticsStore>>) {
    let Some(fps) = fps else {return};
    if let Some(value) = fps
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed()) {
        SIG_FPS.send(value as f32)
    }
}

/// Signal receiver for the `FPS` as a `f32`. requires `FrameTimeDiagnosticsPlugin`
pub fn fps_signal<T: SignalReceiver>(f: impl Fn(f32) -> T::Type + Clone + Send + Sync + 'static) -> Receiver<T> {
    SIG_FPS.new_receiver().map(f)
}