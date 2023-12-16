use std::{sync::atomic::{AtomicU8, Ordering, AtomicBool}, num::NonZeroU8};

use bevy::log::error;

thread_local! {
    static LAYER: AtomicU8 = AtomicU8::new(0);
    static USE_OPACITY: AtomicBool = AtomicBool::new(false);
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

/// Run a function, entities spawned inside auto inserts `OpacityWriter`.
pub fn use_opacity<T>(f: impl FnOnce() -> T) -> T{
    USE_OPACITY.with(|x| {
        let orig = x.swap(true, Ordering::Relaxed);
        let out = f();
        if x.compare_exchange(true, orig, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            error!("Failed trying to revert USE_OPACITY from {} to {}", true, orig)
        }
        out
    })
}

/// Obtain if we insert `OpacityWriter` or not.
pub fn is_using_opacity() -> bool {
    USE_OPACITY.with(|x| x.load(Ordering::Relaxed))
}
