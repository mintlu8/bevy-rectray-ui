use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::{entity::Entity, world::World}};
use bevy_defer::{Arc, AsyncExecutor, AsyncSystemParam, BoxedReadonlyCallback, Signals, oneshot};
use std::future::Future;

/// An `AsyncSystemParam` that gets the `fps` value.
/// 
/// Requires `FrameTimeDiagnosticPlugin`.
pub struct Fps(Arc<AsyncExecutor>);

impl AsyncSystemParam for Fps {
    fn from_async_context(
        _: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
    ) -> Self {
        Self (executor.clone())
    }
}


impl Fps {
    pub fn get(&self) -> impl Future<Output = f32> {
        let (sender, receiver) = oneshot::<f32>();
        let query = BoxedReadonlyCallback::new(
            move |world: &World| {
                world.get_resource::<DiagnosticsStore>().and_then(|x| x
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed().map(|x| x as f32)) 
                ).unwrap_or(0.0)
            },
            sender
        );
        {
            let mut lock = self.0.readonly.lock();
            lock.push(query);
        }
        async {
            receiver.await.unwrap_or(0.0)
        }
    }
}