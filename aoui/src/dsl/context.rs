use std::{sync::atomic::{AtomicU8, Ordering}, num::NonZeroU8};

use bevy::log::error;

thread_local! {
    static LAYER: AtomicU8 = AtomicU8::new(0);
}

/// Run a function, entities spawned inside uses a different default `RenderLayer`.
pub fn with_layer<T>(layer: u8, f: impl FnOnce() -> T) -> T{
    LAYER.with(|x| {
        let orig = x.swap(layer, Ordering::Relaxed);
        let out = f();
        if x.compare_exchange(layer, orig, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            error!("Failed trying to revert layer from {} to {}", layer, orig)
        }
        out
    })
}

/// Obtain the current default layer, by default None (or 0).
pub fn get_layer() -> Option<NonZeroU8> {
    LAYER.with(|x| NonZeroU8::new(x.load(Ordering::Relaxed)))
}
