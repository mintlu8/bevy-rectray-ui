use std::{future::Future, sync::Arc};

use bevy::ecs::entity::Entity;

use super::{async_param::AsyncSystemParam, AsyncExecutor, AsyncFailure, Signals, States};

#[macro_export]
macro_rules! async_system {
    (|$($field: ident :$ty: ty),* $(,)?| $body: expr) => {
        $crate::sync::AsyncSystem::new(|$($field :$ty),*| async move {
            let _ = $body;
            Ok(())
        })
    };
}

#[macro_export]
macro_rules! async_systems {
    ($(|$($field: ident: $ty: ty),* $(,)?| $body: expr),* $(,)?) => {
        $crate::sync::AsyncSystems::from_systems([
            $($crate::async_system!(|$($field: $ty),*| $body)),*
        ]);
    };
}

pub trait AsyncSystemFunction<M>: Send + Sync + 'static {
    fn into_future(
        &self,
        entity: Entity,
        executor: &Arc<AsyncExecutor>,
        signals: &Signals,
        states: &States,
    ) -> impl Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static;
}


macro_rules! impl_async_system_fn {
    () => {};
    ($head: ident $(,$tail: ident)*) => {
        impl_async_system_fn!($($tail),*);

        const _: () = {

            impl<F, Fut: Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static, $head $(,$tail)*>
                    AsyncSystemFunction<($head, $($tail,)*)> for F where $head: AsyncSystemParam, $($tail: AsyncSystemParam,)*
                        F: Fn($head $(,$tail)*) -> Fut + Send + Sync + 'static,
                         {
                fn into_future(
                    &self,
                    entity: Entity,
                    executor: &Arc<AsyncExecutor>,
                    signals: &Signals,
                    states: &States,
                ) -> impl Future<Output = Result<(), AsyncFailure>> + Send + Sync + 'static {
                    self(
                        $head::from_async_context(entity, executor, signals, states),
                        $($tail::from_async_context(entity, executor, signals, states)),*
                    )
                }
            }
        };
    };
}

impl_async_system_fn!(
    T0, T1, T2, T3, T4,
    T5, T6, T7, T8, T9,
    T10, T11, T12, T13, T14
);
