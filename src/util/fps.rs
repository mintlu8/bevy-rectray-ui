use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::{entity::Entity, world::World}};
use bevy_defer::{Arc, AsyncEntityParam, AsyncQueryQueue, BoxedReadonlyCallback, channel};
use bevy_defer::signals::Signals;
use std::{borrow::Cow, future::Future};

/// An `AsyncSystemParam` that gets the `fps` value.
/// 
/// Requires `FrameTimeDiagnosticPlugin`.
pub struct Fps<'t>(Cow<'t, Arc<AsyncQueryQueue>>);

impl<'t> AsyncEntityParam<'t> for Fps<'t> {
    type Signal = ();
    
    fn fetch_signal(_: &Signals) -> Option<Self::Signal> {
        Some(())
    }

    fn from_async_context(
        _: Entity,
        executor: &'t Arc<AsyncQueryQueue>,
        _: (),
    ) -> Self {
        Self (Cow::Borrowed(executor))
    }
    
}


impl Fps<'_> {
    pub fn get(&self) -> impl Future<Output = f32> {
        let (sender, receiver) = channel();
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