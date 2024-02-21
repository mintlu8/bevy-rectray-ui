use std::{mem, ops::Deref, pin::Pin, task::Context};
use std::future::Future;
use bevy::{app::App, ecs::{entity::Entity, system::{Commands, Query, Res, Resource}, world::World}, log::trace, tasks::{ComputeTaskPool, ParallelSliceMut}};
use async_oneshot::{oneshot, Sender};
use parking_lot::Mutex;
use triomphe::Arc;

use crate::{AsyncEntityCommands, AsyncSystems, Signals, DUMMY_SIGNALS};

/// Standard errors for the async runtime.
#[derive(Debug, thiserror::Error)]
pub enum AsyncFailure {
    #[error("async channel closed")]
    ChannelClosed,
    #[error("entity not found")]
    EntityQueryNotFound,
    #[error("component not found")]
    ComponentNotFound,
    #[error("resource not found")]
    ResourceNotFound,
}

/// A shared storage that cleans up associated futures
/// when their associated entity is destroyed.
#[derive(Debug, Clone, Default)]
pub struct KeepAlive(Arc<()>);

impl KeepAlive {
    pub fn new() -> Self {
        KeepAlive::default()
    }
    pub fn other_alive(&self) -> bool {
        Arc::count(&self.0) > 1
    }
}


/// A future representing a running async system.
pub struct SystemFuture{
    future: Pin<Box<dyn Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static>>,
    alive: KeepAlive,
}

/// A parallelizable query on a `World`.
pub struct BoxedReadonlyCallback {
    command: Option<Box<dyn FnOnce(&World) + Send + Sync + 'static>>
}

impl BoxedReadonlyCallback {
    pub fn new<Out: Send + Sync + 'static>(
        query: impl (FnOnce(&World) -> Out) + Send + Sync + 'static,
        mut channel: Sender<Out>
    ) -> Self {
        Self {
            command: Some(Box::new(move |w| {
                let result = query(w);
                if channel.send(result).is_err() {
                    trace!("Error: one-shot channel closed.")
                }
            }))
        }
    }
}

/// A boxed function that return its data through a channel.
pub struct BoxedQueryCallback {
    command: Box<dyn FnOnce(&mut World) -> Option<BoxedQueryCallback> + Send + Sync + 'static>
}

impl BoxedQueryCallback {

    pub fn fire_and_forget(
        query: impl (FnOnce(&mut World)) + Send + Sync + 'static,
    ) -> Self {
        Self {
            command: Box::new(move |w| {
                query(w);
                None
            })
        }
    }

    pub fn once<Out: Send + Sync + 'static>(
        query: impl (FnOnce(&mut World) -> Out) + Send + Sync + 'static,
        mut channel: Sender<Out>
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
        mut channel: Sender<Out>
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
                        Some(BoxedQueryCallback::repeat(query, channel))
                    }
                }

            })
        }
    }
}

/// A simple async executor for `bevy_rectray`.
#[derive(Default)]
pub struct AsyncExecutor {
    pub stream: Mutex<Vec<SystemFuture>>,
    pub readonly: Mutex<Vec<BoxedReadonlyCallback>>,
    pub queries: Mutex<Vec<BoxedQueryCallback>>,
}

impl std::fmt::Debug for AsyncExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncExecutor")
            .field("stream", &self.stream.lock().len())
            .field("readonly", &self.readonly.lock().len())
            .field("queries", &self.queries.lock().len())
            .finish()
    }
}

/// Resource containing a reference to an async executor.
#[derive(Default, Resource)]
pub struct ResAsyncExecutor(Arc<AsyncExecutor>);

impl Deref for ResAsyncExecutor {
    type Target = AsyncExecutor;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[doc(hidden)]
pub fn run_async_query(
    w: &mut World,
) {
    let executor = w.resource::<ResAsyncExecutor>().0.clone();
    let mut lock = executor.readonly.lock();
    let mut inner: Vec<_> = mem::take(lock.as_mut());
    drop(lock);

    if !inner.is_empty() {
        let pool = ComputeTaskPool::get();
        inner.par_splat_map_mut(pool, None, |chunks| for item in chunks {
            if let Some(f) = item.command.take() { f(w) }
        });
    }
    
    
    let mut lock = executor.queries.lock();
    let inner: Vec<_> = mem::take(lock.as_mut());
    *lock = inner.into_iter().filter_map(|query| (query.command)(w)).collect();
}



#[doc(hidden)]
pub fn exec_async_executor(
    executor: Res<ResAsyncExecutor>,
) {
    let waker = noop_waker::noop_waker();
    let mut ctx = Context::from_waker(&waker);
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

#[doc(hidden)]
pub fn push_async_systems(
    executor: Res<ResAsyncExecutor>,
    query: Query<(Entity, Option<&Signals>, &AsyncSystems)>
) {
    let mut stream = executor.stream.lock();
    let dummy = DUMMY_SIGNALS.deref();
    for (entity, signals, systems) in query.iter() {
        let signals = signals.unwrap_or(dummy);
        for system in systems.systems.iter(){
            if !system.marker.other_alive() {
                let fut = SystemFuture{
                    future: (system.function)(entity, &executor.0, signals),
                    alive: system.marker.clone()
                };
                stream.push(fut)
            }
        }
    }
}

/// Extension to various things for spawning futures onto the executor.
#[allow(async_fn_in_trait)]
pub trait AsyncSpawn {
    /// Spawn a function the continuously checks the world until `Some` is returned.
    async fn spawn_watch<T, F>(&mut self, 
        f: impl Fn(&mut World) -> Option<T> + Send + Sync + 'static
    ) -> T where T: Send + Sync + 'static, F: Future<Output = T> + Send + Sync + 'static;
}

impl AsyncSpawn for App{
    async fn spawn_watch<T, F>(&mut self, 
        f: impl Fn(&mut World) -> Option<T>  + Send + Sync + 'static
    ) -> T where T: Send + Sync + 'static, F: Future<Output = T> + Send + Sync + 'static {
        let res = self.world.get_resource::<ResAsyncExecutor>().expect("Expected ResAsyncExecutor.");
        let mut queries = res.0.queries.lock();
        let (send, recv) = oneshot::<T>();
        queries.push(BoxedQueryCallback::repeat(f, send));
        recv.await.unwrap()
    }
}


impl AsyncSpawn for World{
    async fn spawn_watch<T, F>(&mut self, 
        f: impl Fn(&mut World) -> Option<T>  + Send + Sync + 'static
    ) -> T where T: Send + Sync + 'static, F: Future<Output = T> + Send + Sync + 'static {
        let res = self.get_resource::<ResAsyncExecutor>().expect("Expected ResAsyncExecutor.");
        let mut queries = res.0.queries.lock();
        let (send, recv) = oneshot::<T>();
        queries.push(BoxedQueryCallback::repeat(f, send));
        recv.await.unwrap()
    }
}

impl AsyncSpawn for Commands<'_, '_> {
    async fn spawn_watch<T, F>(&mut self, 
        f: impl Fn(&mut World) -> Option<T> + Send + Sync + 'static
    ) -> T where T: Send + Sync + 'static, F: Future<Output = T> + Send + Sync + 'static {
        let (send, recv) = oneshot::<T>();
        self.add(move |w: &mut World| {
            let res = w.get_resource::<ResAsyncExecutor>().expect("Expected ResAsyncExecutor.");
            let mut queries = res.0.queries.lock();
            queries.push(BoxedQueryCallback::repeat(f, send));
        });
        recv.await.unwrap()
    }
}

impl AsyncSpawn for AsyncEntityCommands {
    async fn spawn_watch<T, F>(&mut self, 
        f: impl Fn(&mut World) -> Option<T> + Send + Sync + 'static
    ) -> T where T: Send + Sync + 'static, F: Future<Output = T> + Send + Sync + 'static {
        let (send, recv) = oneshot::<T>();
        self.add(move |w: &mut World| {
            let res = w.get_resource::<ResAsyncExecutor>().expect("Expected ResAsyncExecutor.");
            let mut queries = res.0.queries.lock();
            queries.push(BoxedQueryCallback::repeat(f, send));
        });
        recv.await.unwrap()
    }
}