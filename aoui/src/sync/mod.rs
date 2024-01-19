use std::{sync::Arc, future::Future, pin::Pin, mem, task::Context, fmt::Debug, ops::Deref, any::{Any, TypeId}};
use bevy::{log::trace, ecs::component::Component, ecs::{entity::Entity, system::{Query, IntoSystem, Res}, schedule::IntoSystemConfigs}, app::{Plugin, PreUpdate, Update, PostUpdate, First}, utils::HashMap};
use bevy::ecs::world::World;
use bevy::ecs::system::{Resource, In};
mod signals;
mod async_param;
mod async_system;
mod state;
mod signal_inner;
use parking_lot::RwLock;
pub use signals::*;
pub use async_system::*;
pub use async_param::*;
pub use state::*;
pub use parking_lot::Mutex;
pub use signal_inner::*;

use crate::util::{ComponentCompose, Object};

#[derive(Debug, Resource, Default)]
pub struct SignalPool(pub(crate) RwLock<HashMap<String, Arc<SignalData<Object>>>>);

#[derive(Debug, thiserror::Error)]
pub enum AsyncFailure {
    #[error("async channel destroyed")]
    ChannelDestroyed,
    #[error("entity not found")]
    EntityQueryNotFound,
    #[error("component not found")]
    ComponentNotFound,
    #[error("resource not found")]
    ResourceNotFound,
}

pub type AsyncResult<T> = Result<T, AsyncFailure>;


#[derive(Debug, Clone, Default)]
pub struct KeepAlive(Arc<()>);

impl KeepAlive {
    pub fn new() -> Self {
        KeepAlive::default()
    }
    pub fn other_alive(&self) -> bool {
        Arc::strong_count(&self.0) > 1
    }
}

pub struct SystemFuture{
    future: Pin<Box<dyn Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static>>,
    alive: KeepAlive,
}

pub struct AsyncQuery {
    command: Box<dyn FnOnce(&mut World) -> Option<AsyncQuery> + Send + Sync + 'static>
}

impl AsyncQuery {
    pub fn new<Out: Send + Sync + 'static>(
        query: impl (FnOnce(&mut World) -> Out) + Send + Sync + 'static,
        channel: futures::channel::oneshot::Sender<Out>
    ) -> Self {
        Self {
            command: Box::new(move |w| {
                let result = query(w);
                if channel.send(result).is_err() {
                    trace!("Error: one-shot channel closed.")
                }
                None
            })
        }
    }

    pub fn repeat<Out: Send + Sync + 'static>(
        query: impl (Fn(&mut World) -> Option<Out>) + Send + Sync + 'static,
        channel: futures::channel::oneshot::Sender<Out>
    ) -> Self {
        Self {
            command: Box::new(move |w| {
                match query(w) {
                    Some(x) => {
                        if channel.send(x).is_err() {
                            trace!("Error: one-shot channel closed.")
                        }
                        None
                    }
                    None => {
                        Some(AsyncQuery::repeat(query, channel))
                    }
                }

            })
        }
    }
}

#[derive(Default)]
pub struct AsyncExecutor {
    stream: Mutex<Vec<SystemFuture>>,
    queries: Mutex<Vec<AsyncQuery>>,
    states: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>
}

#[derive(Default, Resource)]
pub struct ResAsyncExecutor(Arc<AsyncExecutor>);

impl Deref for ResAsyncExecutor {
    type Target = AsyncExecutor;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub fn run_async_query(
    executor: Res<ResAsyncExecutor>,
) -> Vec<AsyncQuery> {
    let mut lock = executor.queries.lock();
    mem::take(&mut lock)
}

pub fn apply_commands(input: In<Vec<AsyncQuery>>, w: &mut World) -> Vec<AsyncQuery> {
    input.0.into_iter().filter_map(|query| (query.command)(w)).collect()
}

pub fn collect_async_query(
    input: In<Vec<AsyncQuery>>,
    executor: Res<ResAsyncExecutor>,
)  {
    let mut lock = executor.queries.lock();
    *lock = input.0;
}




pub fn run_async_executor(
    executor: Res<ResAsyncExecutor>,
) {
    let mut ctx = Context::from_waker(futures::task::noop_waker_ref());
    let mut lock = executor.stream.lock();
    lock.retain_mut(|fut| {
        if !fut.alive.other_alive() {return false;}
        match fut.future.as_mut().poll(&mut ctx) {
            std::task::Poll::Ready(Ok(_)) => false,
            std::task::Poll::Ready(Err(fail)) => {
                trace!("Future dropped: {fail}.");
                false
            },
            std::task::Poll::Pending => true,
        }
    })
}

pub struct AsyncSystem {
    function: Box<dyn Fn(
        Entity,
        &Arc<AsyncExecutor>,
        &Signals,
        &States,
    ) -> Pin<Box<dyn Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static>> + Send + Sync> ,
    marker: KeepAlive,
}

impl AsyncSystem {
    pub fn new<F, M>(f: F) -> Self where F: AsyncSystemFunction<M>  {
        AsyncSystem {
            function: Box::new(move |entity, executor, signals, states| {
                Box::pin(f.into_future(entity, executor, signals, states))
            }),
            marker: KeepAlive::new()
        }
    }
}

impl Debug for AsyncSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncSystem").finish()
    }
}

#[derive(Debug, Component)]
pub struct AsyncSystems {
    systems: Vec<AsyncSystem>,
}

impl ComponentCompose for AsyncSystems {
    fn compose(&mut self, mut other: Self) {
        self.systems.append(&mut other.systems)
    }
}

impl AsyncSystems {
    pub fn new<F, M>(f: F) -> Self where F: AsyncSystemFunction<M>  {
        AsyncSystems {
            systems: vec![AsyncSystem {
                function: Box::new(move |entity, executor, signals, states| {
                    Box::pin(f.into_future(entity, executor, signals, states))
                }),
                marker: KeepAlive::new()
            }]
        }
    }

    pub fn from_systems(iter: impl IntoIterator<Item = AsyncSystem>) -> Self  {
        AsyncSystems {
            systems: iter.into_iter().collect()
        }
    }

    pub fn and<F, M>(mut self, f: F) -> Self where F: AsyncSystemFunction<M>  {
        self.systems.push(
            AsyncSystem {
                function: Box::new(move |entity, executor, signals, states| {
                    Box::pin(f.into_future(entity, executor, signals, states))
                }),
                marker: KeepAlive::new()
            }
        );
        self
    }
}

pub fn run_async_systems(
    executor: Res<ResAsyncExecutor>,
    query: Query<(Entity, Option<&Signals>, Option<&States>, &AsyncSystems)>
) {
    let mut stream = executor.stream.lock();
    for (entity, signals, states, systems) in query.iter() {
        let signals = signals.unwrap_or(&DUMMY_SIGNALS);
        let states = states.unwrap_or(&DUMMY_STATE);
        for system in systems.systems.iter(){
            if !system.marker.other_alive() {
                let fut = SystemFuture{
                    future: (system.function)(entity, &executor.0, &signals, &states),
                    alive: system.marker.clone()
                };
                stream.push(fut)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AsyncExecutorPlugin;

impl Plugin for AsyncExecutorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ResAsyncExecutor>()
            .init_resource::<SignalPool>();
        app.add_systems(First, run_async_systems);
        app.add_systems(PreUpdate, (
            run_async_query.pipe(
                apply_commands
            ).pipe(
                collect_async_query
            ),
            run_async_executor.after(apply_commands)
        ));
        app.add_systems(Update, (
            run_async_query.pipe(
                apply_commands
            ).pipe(
                collect_async_query
            ),
            run_async_executor.after(apply_commands)
        ));
        app.add_systems(PostUpdate, (
            run_async_query.pipe(
                apply_commands
            ).pipe(
                collect_async_query
            ),
            run_async_executor.after(apply_commands)
        ));
    }
}
