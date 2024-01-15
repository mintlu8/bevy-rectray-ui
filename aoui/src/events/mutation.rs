
use std::fmt::Debug;

use bevy::ecs::world::World;
use bevy::ecs::{component::Component, entity::Entity};
use bevy::ecs::system::{EntityCommands, Command, RunSystemOnce, Query, IntoSystem, In};

use crate::util::{AsObject, Object};

pub trait IntoMutationCommand<T: Clone, M1, M2>: Clone + Send + Sync + 'static {
    fn into_command(self, entity: Entity, data: T) -> impl Command;
}

pub trait IntoMutationCommandWithCtx<Ctx, T: Clone, M1, M2>: Clone + Send + Sync + 'static {
    fn into_command<M>(self, entity: Entity, data: T, f: impl IntoSystem<(), Ctx, M> + Send + Sync + 'static) -> impl Command;
}

macro_rules! multi_mutation {
    ($first: ident) => {
        mutation!($first);
    };
    ($first: ident, $($rest: ident),*) => {
        mutation!($first, $($rest),*);
        multi_mutation!($($rest),*);
    };
}

macro_rules! mutation {
    ($first: ident $(, $rest: ident)*) => {
        mutation!(@mut [$first] [&mut $first] [mut $first] [$first.as_mut()] $($rest),*);
        mutation!(@nomut [$first] [&$first] [$first] [$first] $($rest),*);
    };
    // Discard since no &mut found.
    (@nomut [$($i: ident),*] [$($e1: ty),*] [$($e2: tt)*] [$($e3: expr),*] ) => {};
    (@mut [$($i: ident),*] [$($e1: ty),*] [$($e2: tt)*] [$($e3: expr),*]) => {
        const _: () = {
            #[doc(hidden)]
            pub enum Disambiguate {}
            #[allow(unused_parens, non_snake_case)]
            impl<T: AsObject, Func, $($i: Component),*> IntoMutationCommand<T, ($($i),*), Disambiguate>
                for Func
                    where Func: Fn($($e1),*) + Send + Sync + Clone + 'static {

                fn into_command(self, entity: Entity, _: T) -> impl Command {
                    move |w: &mut World| {
                        w.run_system_once(move |mut q: Query<($($e1),*)>| {
                            if let Ok(($($e2)*)) = q.get_mut(entity) {
                                self($($e3),*);
                            }
                        });
                    }
                }
            }

            #[allow(unused_parens, non_snake_case)]
            impl<Ctx: 'static, T: AsObject, Func, $($i: Component),*> IntoMutationCommandWithCtx<Ctx, T, ($($i),*), Disambiguate>
                for Func
                    where Func: Fn(Ctx, $($e1),*) + Send + Sync + Clone + 'static {

                fn into_command<M>(self, entity: Entity, _: T, ctx: impl IntoSystem<(), Ctx, M> + Send + Sync + 'static) -> impl Command {
                    move |w: &mut World| {
                        let ctx = w.run_system_once(ctx);
                        w.run_system_once_with(ctx, move |ctx: In<Ctx>, mut q: Query<($($e1),*)>| {
                            if let Ok(($($e2)*)) = q.get_mut(entity) {
                                self(ctx.0, $($e3),*);
                            }
                        });
                    }
                }
            }

            #[doc(hidden)]
            pub enum DisambiguateData {}

            #[allow(unused_parens, non_snake_case)]
            impl<T: AsObject, Func, $($i: Component),*> IntoMutationCommand<T, ($($i),*), DisambiguateData>
                for Func
                    where Func: Fn(T, $($e1),*) + Send + Sync + Clone + 'static {

                fn into_command(self, entity: Entity, data: T) -> impl Command {
                    move |w: &mut World| {
                        w.run_system_once(move |mut q: Query<($($e1),*)>| {
                            if let Ok(($($e2)*)) = q.get_mut(entity) {
                                self(data.clone(), $($e3),*);
                            }
                        });
                    }
                }
            }

            #[allow(unused_parens, non_snake_case)]
            impl<Ctx: 'static, T: AsObject, Func, $($i: Component),*> IntoMutationCommandWithCtx<Ctx, T, ($($i),*), DisambiguateData>
                for Func
                    where Func: Fn(Ctx, T, $($e1),*) + Send + Sync + Clone + 'static {

                fn into_command<M>(self, entity: Entity, data: T, ctx: impl IntoSystem<(), Ctx, M> + Send + Sync + 'static) -> impl Command {
                    move |w: &mut World| {
                        let ctx = w.run_system_once(ctx);
                        w.run_system_once_with(ctx, move |ctx: In<Ctx>, mut q: Query<($($e1),*)>| {
                            if let Ok(($($e2)*)) = q.get_mut(entity) {
                                self(ctx.0, data.clone(), $($e3),*);
                            }
                        });
                    }
                }
            }
        };
    };
    (@nomut [$($i: ident),*] [$($e1: ty),*] [$($e2: tt)*] [$($e3: expr),*] $first: ident $(,$rest: ident)*) => {
        mutation!(@mut [$($i,)* $first] [$($e1,)* &mut $first] [$($e2)*, mut $first] [$($e3,)* $first.as_mut()] $($rest),*);
        mutation!(@nomut [$($i,)* $first] [$($e1,)* &$first] [$($e2)*, $first] [$($e3,)* $first] $($rest),*);
    };
    (@mut [$($i: ident),*] [$($e1: ty),*] [$($e2: tt)*] [$($e3: expr),*] $first: ident $(,$rest: ident)*) => {
        mutation!(@mut [$($i,)* $first] [$($e1,)* &mut $first] [$($e2)*, mut $first] [$($e3,)* $first.as_mut()] $($rest),*);
        mutation!(@mut [$($i,)* $first] [$($e1,)* &$first] [$($e2)*, $first] [$($e3,)* $first] $($rest),*);
    };
}

multi_mutation!(A, B, C, D, E, F);

/// A function thet mutates associated components based on inputs.
pub struct Mutation<T: AsObject>(Box<dyn Fn(&mut EntityCommands, T) + Send + Sync>);

impl<T: AsObject> Debug for Mutation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mutation").finish()
    }
}

impl<T: AsObject> Mutation<T> {

    /// Construct a mutation for associated components.
    ///
    /// # Example
    /// ```
    /// let _: Mutation<()> = Mutation::new(|dim: &mut Dimension, transform: &mut Transform2D| {
    ///     dim.edit_raw(|v| v.x += 1.0);
    ///     transform.rotation += 1.0;
    /// });
    /// ```
    ///
    /// # Note
    ///
    /// If input is type erased, aka `Payload`, use `Mutation::dynamic` instead.
    pub fn new<M, N>(f: impl IntoMutationCommand<T, M, N>) -> Self{
        Mutation(Box::new(move |commands: &mut EntityCommands, data: T| {
            let entity = commands.id();
            commands.commands().add(f.clone().into_command(entity, data));
        }))
    }

    pub fn with_context<Ctx, M, N, U> (
        ctx: impl IntoSystem<(), Ctx, U> + Send + Sync + Clone + 'static,
        f: impl IntoMutationCommandWithCtx<Ctx, T, M, N>
    ) -> Self {
        Mutation(Box::new(move |commands: &mut EntityCommands, data: T| {
            let entity = commands.id();
            commands.commands().add(f.clone().into_command(entity, data, ctx.clone()));
        }))
    }

    pub fn exec(&self, commands: &mut EntityCommands, data: T) {
        (self.0)(commands, data)
    }

}

impl Mutation<Object> {
    pub fn dynamic<T: AsObject, M, N>(f: impl IntoMutationCommand<T, M, N>) -> Self{
        Mutation(Box::new(move |commands: &mut EntityCommands, data: Object| {
            let entity = commands.id();
            if let Some(data) = data.get() {
                commands.commands().add(f.clone().into_command(entity, data));
            }
        }))
    }
}
