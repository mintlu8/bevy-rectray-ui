//! Async systems and signals for `bevy_aoui`.
//! 
//! # What do you mean? I hate async!
//! 
//! Async system is entirely optional, everything used in async systems
//! also has sync versions. See implementations in `bevy_matui` as that
//! crate does not use async systems in any way.
//! 
//! # Then why does async systems exist?
//! 
//! Async systems provide a way to define logic inside UI code, without
//! explicitly adding systems and using marker components. When you call
//! an async function on an [`AsyncSystemParam`], the query gets sent to
//! the executor to be executed, while the async system waits for it to
//! complete, usually within the same frame. This also works nicely with
//! signals as a way to wait for other widgets to send information.
//! 
//! # Example
//! 
//! This example shows a button changing a text widget.
//! 
//! ```
//! text! (commands {
//!     offset: [300, -100],
//!     color: color!(gold),
//!     text: "Click a button!",
//!     signal: receiver::<FormatText>(sig),
//!     system: |x: SigRecv<FormatText>, text: Ac<Text>| {
//!         let msg = x.recv().await;
//!         text.write(move |text| format_widget!(text, "You clicked button {}!", msg)).await?;
//!     }
//! });
//! ```
//! 
//! Let's break it down:
//! 
//! ```
//! signal: receiver::<FormatText>(sig),
//! ```
//! 
//! This receives a signal `sig` of id `FormatText`.
//! id is unique per entity and `id::Data` informs the type of the signal,
//! in this case `FormatText::Data` is `String`.
//! 
//! ```
//! system: |x: SigRecv<FormatText>, text: Ac<Text>| { ... }
//! ```
//! 
//! We define an [`AsyncSystem`] like a regular system, note this function is async
//! and should be `|..| async move {}`, but the macro saves us from writing that.
//! 
//! [`SigRecv`] receives the signal, `Ac` is an alias for [`AsyncComponent`], which
//! allows us to get or set data on a component within the same entity. 
//! 
//! You can use `WorldQuery` as well with [`AsyncEntityQuery`]
//! and `SystemParam` with [`AsyncQuery`],
//! but those are unfortunately slower
//! due to bevy current not being optimized for this type of usage.
//! 
//! ```
//! let msg = x.recv().await;
//! text.write(move |text| format_widget!(text, "You clicked button {}!", msg)).await?;
//! ```
//! 
//! You can treat this like a loop:
//! 
//! 1. At the start of the frame, run this if not already running.
//! 2. Wait for receiving signal.
//! 3. Write received message to the text.
//! 4. Wait for the write query to complete.
//! 5. End and repeat step 1 on the next frame.
//! 
//! # FAQ
//! 
//! ## Is there a spawn function? Can I use runtime dependent async crates?
//! 
//! No, we only use have a bare bones async runtime with no waking support.
//! 
//! ## Can I use a third party async crate?
//! 
//! Depends, a future is polled a fixed  number of times per frame, which may
//! or may not be ideal.
//! 
//! ## Any tips regarding async usage?
//! 
//! You should use [`futures::join`] whenever you want to wait for multiple
//! independent queries, otherwise your systems might take longer to complete.
//! 
//! 

use std::{sync::Arc, future::Future, pin::Pin, mem, task::Context, fmt::Debug, ops::Deref};
use bevy::tasks::{ComputeTaskPool, ParallelSliceMut};
use bevy::{ecs::component::Component, log::trace, reflect::Reflect, utils::HashMap};
use bevy::ecs::{entity::Entity, system::{Query, Res}, schedule::IntoSystemConfigs};
use bevy::app::{Plugin, PreUpdate, Update, PostUpdate, First};
use bevy::ecs::world::World;
use bevy::ecs::system::Resource;
use parking_lot::Mutex;
mod signals;
mod async_param;
mod async_system;
mod signal_inner;
mod special_query;
use parking_lot::RwLock;
pub use signals::*;
pub use async_system::*;
pub use async_param::*;
pub use signal_inner::*;
pub use special_query::*;

use crate::util::{ComponentCompose, Object};

/// Storage for named signals.
#[derive(Debug, Resource, Default)]
pub struct SignalPool(pub(crate) RwLock<HashMap<String, Arc<SignalData<Object>>>>);

/// Standard errors for the async runtime.
#[derive(Debug, thiserror::Error)]
pub enum AsyncFailure {
    #[error("async channel destroyed")]
    ChannelClosed,
    #[error("entity not found")]
    EntityQueryNotFound,
    #[error("component not found")]
    ComponentNotFound,
    #[error("resource not found")]
    ResourceNotFound,
}

/// Result type of `AsyncSystemFunction`.
pub type AsyncResult<T> = Result<T, AsyncFailure>;

/// A shared storage that cleans up associated futures
/// when their associated entity is destroyed.
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

/// A future representing a running async system.
pub struct SystemFuture{
    future: Pin<Box<dyn Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static>>,
    alive: KeepAlive,
}

/// A parallelizable query on a `World`.
pub struct AsyncReadonlyQuery {
    command: Option<Box<dyn FnOnce(&World) + Send + Sync + 'static>>
}

impl AsyncReadonlyQuery {
    pub fn new<Out: Send + Sync + 'static>(
        query: impl (FnOnce(&World) -> Out) + Send + Sync + 'static,
        channel: futures::channel::oneshot::Sender<Out>
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


pub(crate) struct BoxedQueryCallback {
    command: Box<dyn FnOnce(&mut World) -> Option<BoxedQueryCallback> + Send + Sync + 'static>
}

impl BoxedQueryCallback {
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
                        Some(BoxedQueryCallback::repeat(query, channel))
                    }
                }

            })
        }
    }
}

/// A simple async executor for `bevy_aoui`.
#[derive(Default)]
pub struct AsyncExecutor {
    stream: Mutex<Vec<SystemFuture>>,
    readonly: Mutex<Vec<AsyncReadonlyQuery>>,
    queries: Mutex<Vec<BoxedQueryCallback>>,
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

// A system constructed by an `AsyncSystemFunction`.
pub struct AsyncSystem {
    function: Box<dyn Fn(
        Entity,
        &Arc<AsyncExecutor>,
        &Signals,
    ) -> Pin<Box<dyn Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static>> + Send + Sync> ,
    marker: KeepAlive,
}

impl AsyncSystem {
    pub fn new<F, M>(f: F) -> Self where F: AsyncSystemFunction<M>  {
        AsyncSystem {
            function: Box::new(move |entity, executor, signals| {
                Box::pin(f.as_future(entity, executor, signals))
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

/// A composable component containing an entity's `AsyncSystem`s.
#[derive(Debug, Component, Reflect)]
pub struct AsyncSystems {
    #[reflect(ignore)]
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
                function: Box::new(move |entity, executor, signals| {
                    Box::pin(f.as_future(entity, executor, signals))
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
                function: Box::new(move |entity, executor, signals| {
                    Box::pin(f.as_future(entity, executor, signals))
                }),
                marker: KeepAlive::new()
            }
        );
        self
    }
}

pub fn run_async_systems(
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

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AsyncExecutorPlugin;

impl Plugin for AsyncExecutorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ResAsyncExecutor>()
            .init_resource::<SignalPool>();
        app.add_systems(First, run_async_systems);
        app.add_systems(PreUpdate, (
            run_async_query,
            run_async_executor.after(run_async_query)
        ));
        app.add_systems(Update, (
            run_async_query,
            run_async_executor.after(run_async_query)
        ));
        app.add_systems(PostUpdate, (
            run_async_query,
            run_async_executor.after(run_async_query)
        ));
    }
}
