use std::cell::Cell;

use bevy::{ecs::{system::EntityCommands, component::Component}, render::view::RenderLayers};

use super::DslInto;

thread_local! {
    static LAYER: Cell<RenderLayers> = Cell::new(RenderLayers::layer(0));
    static USE_OPACITY: Cell<bool> = Cell::new(false);
    static MARKER: Cell<fn(&mut EntityCommands)> = Cell::new(|_|());
}

/// Run a closure, entities spawned inside uses a different default `RenderLayer`.
pub fn with_layer<T>(layer: impl DslInto<RenderLayers>, f: impl FnOnce() -> T) -> T{
    LAYER.with(|x| {
        let orig = x.replace(layer.dinto());
        let out = f();
        let _ = x.replace(orig);
        out
    })
}

/// Run a closure, entities spawned inside uses a marker component,
/// spawned using its `Default` implementation.
/// 
/// Nested calls will **overwrite** which component gets added.
pub fn with_marker<M: Component + Default, T>(f: impl FnOnce() -> T) -> T {
    MARKER.with(|x| {
        let func = |entity: &mut EntityCommands|{entity.insert(M::default());};
        let orig = x.replace(func);
        let out = f();
        let _ = x.replace(orig);
        out
    })
}

/// Apply the active marker in context, created by using `with_marker`.
pub fn apply_marker(entity: &mut EntityCommands) {
    MARKER.with(|x| (x.get())(entity) )
}

/// Obtain the current default layer, by default None (or 0).
pub fn get_layer() -> Option<RenderLayers> {
    LAYER.with(|x| {
        let layer = x.get();
        if layer == RenderLayers::layer(0) {
            None
        } else {
            Some(layer)
        }
    })
}

/// Run a closure, entities spawned inside auto inserts `OpacityWriter`.
pub fn use_opacity<T>(f: impl FnOnce() -> T) -> T{
    USE_OPACITY.with(|x| {
        let orig = x.replace(true);
        let out = f();
        let _ = x.replace(orig);
        out
    })
}

/// Obtain whether auto insert `OpacityWriter` or not.
pub fn is_using_opacity() -> bool {
    USE_OPACITY.with(|x| x.get())
}
