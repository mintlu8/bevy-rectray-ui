//! A simple asynchronous runtime for executing deferred queries.
//! 
//! # Why does this crate exist?
//! 
//! `bevy_defer` is the asynchronous runtime of `bevy_aoui` with a simple
//! premise: given two components `Signals` and `AsyncSystems`, you can
//! run any query you want in a deferred manner, and communicate with other widgets
//! through signals. 
//! With code defined entirely at the site of Entity construction on top of that!
//! No marker components, system functions or `World` access needed.
//! 
//! This also provides a wait mechanism for mechanics like animation or
//! dialogue trees, though currently not explored in this crate.
//! 
//! # How do I use an [`AsyncSystem`]?
//! 
//! An entity needs [`AsyncSystems`] and [`Signals`] (optional) to use `AsyncSystems`.
//! 
//! To create an `AsyncSystems`, create an [`AsyncSystem`] first via a macro:
//! 
//! ```
//! // Set scale based on received position
//! let system = async_system!(|recv: SigRecv<PositionChanged>, transform: Ac<Transform>|{
//!     let pos: Vec3 = recv.recv().await;
//!     transform.set(|transform| transform.scale = pos).await?;
//! })
//! ```
//! 
//! Then create a [`AsyncSystems`] from it:
//! 
//! ```
//! let systems = AsyncSystems::from_single(system);
//! // or
//! let systems = AsyncSystems::from_systems([a, b, c, ...]);
//! ```
//! 
//! Add the associated [`Signal`]:
//! ```
//! let signal = Signals::from_receiver::<PositionChanged>(sig);
//! ```
//! 
//! Spawn them as a `Bundle` and that's it! The async executor will 
//! handle it from here.
//! 
//! ## Let's break it down
//! 
//! ```
//! Signals::from_receiver::<PositionChanged>(sig)
//! ```
//! 
//! `PositionChanged` is a [`SignalId`], or a discriminant + an associated type,
//! which informs us the type of the signal is `Vec3`. We can have multiple signals
//! on an `Entity` with type `Vec3`, but not with the same `SignalId`.
//! 
//! ```
//! |recv: SigRecv<PositionChanged>, transform: Ac<Transform>| { .. }
//! ```
//! 
//! [`SigRecv`] receives the signal, `Ac` is an alias for [`AsyncComponent`]
//! defined in the prelude. It allows us to get or set data
//! on a component within the same entity. 
//! 
//! Notice the function signature is a bit weird, since the macro expands to
//! ```
//! move | .. | async move { 
//!     {..}; 
//!     return Ok(()); 
//! }
//! ```
//! 
//! The macro saves us from writing some async closure boilerplate.
//! 
//! ```
//! let pos: Vec3 = recv.recv().await;
//! transform.set(|transform| transform.scale = pos).await?;
//! ```
//! 
//! The body of the [`AsyncSystem`], await means either
//! "wait for" or "deferred" here. 
//! 
//! ## How does this run?
//! 
//! You can treat this like a loop
//! 
//! 1. At the start of the frame, run this function if not already running.
//! 2. Wait until something sends the signal.
//! 3. Write received position to the `Transform` component.
//! 4. Wait for the write query to complete.
//! 5. End and repeat step 1 on the next frame.
//! 
//! # Supported System Params
//! 
//! | Query Type | Corresponding Bevy/Sync Type | Acronym |
//! | ---- | ----- | ---- |
//! | [`AsyncQuery`] | `SystemParam` | `Aq` |
//! | [`AsyncEntityQuery`] | `WorldQuery` | `Aeq` |
//! | [`AsyncEntityCommands`] | `EntityCommands` | -- |
//! | [`AsyncComponent`] | `Component` | `Ac` |
//! | [`AsyncComponentsReadonly`] | Tuple of `Component` | `Acs` |
//! | [`AsyncResource`] | `AsyncResource` | `Ar` |
//! | [`SigSend`] | `Signals` | -- |
//! | [`SigRecv`] | `Signals` | -- |
//! 
//! Note: you can create your own [`AsyncSystemParam`] by implementing it.
//! 
//! # How do signals work?
//! 
//! [`Signal`] is a shared memory location with a version number.
//! To poll a signal requires the version number different from the receiver's
//! previously recorded version. Therefore a signal is read at most once per
//! write for every reader.
//! 
//! # FAQ
//! 
//! ## Is there a spawn function? Can I use a runtime dependent async crate?
//! 
//! No, we only use have a bare bones async runtime with no waking support.
//! 
//! ## Can I use a third party async crate?
//! 
//! Depends, a future is polled a fixed number of times per frame, which may
//! or may not be ideal.
//! 
//! ## Any tips regarding async usage?
//! 
//! You should use [`futures::join`] whenever you want to wait for multiple
//! independent queries, otherwise your systems might take longer to complete.
//! 
//! ## Is this crate blazingly fast?
//! 
//! Depends, this crate excels at waiting for events to occur, for example using signals. 
//! As an async executor that runs queries with extra steps,
//! things that happens every frame should ideally not run here.
//! 
//! 
use bevy::app::{Plugin, PreUpdate, Update, PostUpdate, First};
mod signals;
mod async_param;
mod async_systems;
mod signal_inner;
mod components;
mod object;
mod executor;
pub use executor::*;
pub use signals::*;
pub use async_systems::*;
pub use async_param::*;
pub use signal_inner::*;
pub use components::*;
pub use object::{Object, AsObject};

#[doc(hidden)]
pub use triomphe::Arc;

/// Result type of `AsyncSystemFunction`.
pub type AsyncResult<T> = Result<T, AsyncFailure>;

pub mod prelude {
    pub type Ac<T> = crate::AsyncComponent<T>;
    pub type Ar<T> = crate::AsyncResource<T>;
    pub type Aeq<T> = crate::AsyncEntityQuery<T>;
    pub type Aq<T> = crate::AsyncQuery<T>;
    pub type Acs<T> = crate::AsyncComponentsReadonly<T>;
    pub use crate::{SigSend, SigRecv};
    pub use crate::AsyncEntityCommands;
    pub use crate::async_system;
}

#[derive(Debug, Default, Clone, Copy)]
/// An `bevy_defer` plugin does not run its executor by default.
/// 
/// Add [`run_async_executor!`] to your schedules to run the executor as you like.
pub struct CoreAsyncPlugin;

impl Plugin for CoreAsyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ResAsyncExecutor>()
            .add_systems(First, push_async_systems);
    }
}

/// `SystemConfigs` for running the async executor once.
/// 
/// # Example
/// 
/// ```
/// app.add_systems(Update, run_async_executor!()
///     .after(some_other_system))
/// ```
#[macro_export]
macro_rules! run_async_executor {
    () => {
        bevy::prelude::IntoSystem::pipe(
            $crate::executor::run_async_query,
            $crate::executor::exec_async_executor,
        )
    };
}

/// An `bevy_defer` plugin that runs [`AsyncExecutor`] on [`PreUpdate`], [`Update`] and [`PostUpdate`].
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultAsyncPlugin;

impl Plugin for DefaultAsyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ResAsyncExecutor>()
            .add_systems(First, push_async_systems)
            .add_systems(PreUpdate, run_async_executor!())
            .add_systems(Update, run_async_executor!())
            .add_systems(PostUpdate, run_async_executor!())
        ;
    }
}
