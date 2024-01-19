use std::marker::PhantomData;
use std::sync::Arc;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::bundle::Bundle;
use bevy::ecs::change_detection::DetectChanges;
use bevy::ecs::component::Component;
use bevy::ecs::{query::WorldQuery, entity::Entity, world::World};
use bevy::ecs::system::{Query, RunSystemOnce, SystemParam, StaticSystemParam, Command, Resource};
use bevy::hierarchy::{DespawnRecursive, DespawnChildrenRecursive, AddChild};
use futures::Future;
use futures::channel::oneshot::channel;

use super::{AsyncExecutor, AsyncFailure, AsyncQuery, AsyncResult, Signals, States};

pub trait AsyncSystemParam: Sized {
    fn from_async_context(
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        signals: &Signals,
        states: &States,
    ) -> Self;
}

pub struct AsyncEntityQuery<Q: WorldQuery + 'static>{
    entity: Entity,
    executor: Arc<AsyncExecutor>,
    p: PhantomData<Box<Q>>
}

unsafe impl<Q: WorldQuery> Send for AsyncEntityQuery<Q> {}
unsafe impl<Q: WorldQuery> Sync for AsyncEntityQuery<Q> {}

impl<Q: WorldQuery> AsyncSystemParam for AsyncEntityQuery<Q> {
    fn from_async_context(
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        AsyncEntityQuery {
            entity,
            executor: executor.clone(),
            p: PhantomData
        }
    }
}

impl<Q: WorldQuery + 'static> AsyncEntityQuery<Q> {
    pub fn new(executor: &Arc<AsyncExecutor>, entity: Entity) -> Self{
        AsyncEntityQuery {
            entity,
            executor: executor.clone(),
            p: PhantomData
        }
    }

    pub async fn get<T: Send + Sync + 'static>(&self,
        f: impl Fn(<Q::ReadOnly as WorldQuery>::Item<'_>) -> T + Send + Sync + 'static
    ) -> AsyncResult<T> {
        let (sender, receiver) = channel::<Option<T>>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.run_system_once(move |q: Query<Q>| {
                    match q.get(entity) {
                        Ok(item) => Some(f(item)),
                        Err(_) => None,
                    }
                })
            },
            sender,
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        match receiver.await {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(AsyncFailure::EntityQueryNotFound),
            Err(_) => Err(AsyncFailure::ChannelDestroyed),
        }
    }

    pub fn set<T: Send + Sync + 'static>(&self,
        f: impl Fn(Q::Item<'_>) -> T + Send + Sync + 'static
    ) -> impl Future<Output = Option<T>> + Send + Sync + 'static {
        let (sender, receiver) = channel::<Option<T>>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.run_system_once(move |mut q: Query<Q>| {
                    match q.get_mut(entity) {
                        Ok(item) => Some(f(item)),
                        Err(_) => None,
                    }
                })
            },
            sender,
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.ok().flatten()
        }
    }
}

pub struct AsyncWorldQuery<P: SystemParam + 'static>{
    executor: Arc<AsyncExecutor>,
    p: PhantomData<P>
}

unsafe impl<P: SystemParam> Send for AsyncWorldQuery<P> {}
unsafe impl<P: SystemParam> Sync for AsyncWorldQuery<P> {}

impl<P: SystemParam> AsyncSystemParam for AsyncWorldQuery<P> {
    fn from_async_context(
        _: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        AsyncWorldQuery {
            executor: executor.clone(),
            p: PhantomData
        }
    }
}

impl<Q: SystemParam + 'static> AsyncWorldQuery<Q> {
    pub fn new(executor: &Arc<AsyncExecutor>) -> Self{
        AsyncWorldQuery {
            executor: executor.clone(),
            p: PhantomData
        }
    }

    pub fn run<T: Send + Sync + 'static>(&self,
        f: impl (Fn(StaticSystemParam<Q>) -> T) + Send + Sync + 'static
    ) -> impl Future<Output = AsyncResult<T>> + Send + Sync + 'static{
        let (sender, receiver) = channel::<T>();
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.run_system_once(move |q: StaticSystemParam<Q>| {
                    f(q)
                })
            },
            sender,
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.map_err(|_|AsyncFailure::ChannelDestroyed)
        }
    }
}

pub struct AsyncEntityCommands{
    entity: Entity,
    executor: Arc<AsyncExecutor>,
}

impl AsyncSystemParam for AsyncEntityCommands {
    fn from_async_context(
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        AsyncEntityCommands {
            entity,
            executor: executor.clone(),
        }
    }
}

impl AsyncEntityCommands {
    pub fn new(executor: &Arc<AsyncExecutor>, entity: Entity) -> Self{
        AsyncEntityCommands {
            entity,
            executor: executor.clone(),
        }
    }

    pub fn insert(&self, bundle: impl Bundle) -> impl Future<Output = ()> {
        let (sender, receiver) = channel::<()>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.entity_mut(entity).insert(bundle);
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            let _ = receiver.await;
        }
    }


    pub fn spawn(&self, bundle: impl Bundle) -> impl Future<Output = Option<Entity>> {
        let (sender, receiver) = channel::<Entity>();
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.spawn(bundle).id()
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.ok()
        }
    }

    pub fn add_child(&self, child: Entity) -> impl Future<Output = bool> {
        let (sender, receiver) = channel::<()>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                AddChild {
                    parent: entity,
                    child,
                }.apply(world);
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.is_ok()
        }
    }

    // Calls despawn_recursive
    pub fn despawn(&self) -> impl Future<Output = bool> {
        let (sender, receiver) = channel::<()>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                DespawnRecursive {
                    entity
                }.apply(world);
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.is_ok()
        }
    }

    // Calls despawn_recursive
    pub fn despawn_descendants(&self) -> impl Future<Output = bool> {
        let (sender, receiver) = channel::<()>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                DespawnChildrenRecursive {
                    entity
                }.apply(world)
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.is_ok()
        }
    }
}


pub struct AsyncComponent<C: Component>{
    entity: Entity,
    executor: Arc<AsyncExecutor>,
    p: PhantomData<C>
}

impl<C: Component> AsyncSystemParam for AsyncComponent<C> {
    fn from_async_context(
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        Self {
            entity,
            executor: executor.clone(),
            p: PhantomData
        }
    }
}

impl<C: Component> AsyncComponent<C> {
    pub fn get<Out: Send + Sync + 'static>(&self, f: impl FnOnce(&C) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = channel::<Option<Out>>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.entity_mut(entity)
                    .get::<C>()
                    .map(f)
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            match receiver.await {
                Ok(Some(out)) => Ok(out),
                Ok(None) => Err(AsyncFailure::ComponentNotFound),
                Err(_) => Err(AsyncFailure::ChannelDestroyed),
            }
        }
    }

    pub fn watch<Out: Send + Sync + 'static>(&self, f: impl Fn(&C) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = channel::<Out>();
        let entity = self.entity;
        let query = AsyncQuery::repeat(
            move |world: &mut World| {
                world.entity_mut(entity)
                    .get_ref::<C>()
                    .and_then(|r| if r.is_changed() {
                        Some(f(r.as_ref()))
                    } else {
                        None
                    })
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.map_err(|_| AsyncFailure::ChannelDestroyed)
        }
    }

    pub fn set<Out: Send + Sync + 'static>(&self, f: impl FnOnce(&mut C) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = channel::<Option<Out>>();
        let entity = self.entity;
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.entity_mut(entity)
                    .get_mut::<C>()
                    .map(|mut x| f(x.as_mut()))
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            match receiver.await {
                Ok(Some(out)) => Ok(out),
                Ok(None) => Err(AsyncFailure::ComponentNotFound),
                Err(_) => Err(AsyncFailure::ChannelDestroyed),
            }
        }
    }
}


pub struct AsyncResource<R: Resource>{
    executor: Arc<AsyncExecutor>,
    p: PhantomData<R>
}

impl<R: Resource> AsyncSystemParam for AsyncResource<R> {
    fn from_async_context(
        _: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        Self {
            executor: executor.clone(),
            p: PhantomData
        }
    }
}

impl<R: Resource> AsyncResource<R> {
    pub fn get<Out: Send + Sync + 'static>(&self, f: impl FnOnce(&R) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = channel::<Option<Out>>();
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.get_resource::<R>().map(f)
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            match receiver.await {
                Ok(Some(out)) => Ok(out),
                Ok(None) => Err(AsyncFailure::ResourceNotFound),
                Err(_) => Err(AsyncFailure::ChannelDestroyed),
            }
        }
    }

    pub fn set<Out: Send + Sync + 'static>(&self, f: impl FnOnce(&mut R) -> Out + Send + Sync + 'static)
            -> impl Future<Output = AsyncResult<Out>> {
        let (sender, receiver) = channel::<Option<Out>>();
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.get_resource_mut::<R>()
                    .map(|mut x| f(x.as_mut()))
            },
            sender
        );
        {
            let mut lock = self.executor.queries.lock();
            lock.push(query);
        }
        async {
            match receiver.await {
                Ok(Some(out)) => Ok(out),
                Ok(None) => Err(AsyncFailure::ComponentNotFound),
                Err(_) => Err(AsyncFailure::ChannelDestroyed),
            }
        }
    }
}

pub struct FPS(Arc<AsyncExecutor>);

impl AsyncSystemParam for FPS {
    fn from_async_context(
        _: Entity,
        executor: &Arc<AsyncExecutor>,
        _: &Signals,
        _: &States,
    ) -> Self {
        Self (executor.clone())
    }
}


impl FPS {
    pub fn get(&self) -> impl Future<Output = f32> {
        let (sender, receiver) = channel::<f32>();
        let query = AsyncQuery::new(
            move |world: &mut World| {
                world.get_resource::<DiagnosticsStore>().map(|x| x
                    .get(FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed().map(|x| x as f32)) 
                ).flatten().unwrap_or(0.0)
            },
            sender
        );
        {
            let mut lock = self.0.queries.lock();
            lock.push(query);
        }
        async {
            receiver.await.unwrap_or(0.0)
        }
    }
}