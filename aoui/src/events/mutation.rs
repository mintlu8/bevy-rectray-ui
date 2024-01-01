
use std::fmt::Debug;

use bevy::ecs::{component::Component, entity::Entity};
use bevy::ecs::system::{EntityCommands, Command, RunSystemOnce, Query};

pub struct MutationCommand<T: Component>{
    entity: Entity,
    command: Box<dyn Fn(&mut T) + Send + Sync>,
}

impl<T> Command for MutationCommand<T> where T: Component{
    fn apply(self, world: &mut bevy::prelude::World) {
        let entity = self.entity;
        let func = self.command;
        world.run_system_once(move |mut q: Query<&mut T>| {
            if let Ok(mut item) = q.get_mut(entity) {
                func(item.as_mut());
            }
        });
    }
}


pub struct PipeCommand<A: Component, T: Component>{
    entity: Entity,
    command: Box<dyn Fn(&A, &mut T) + Send + Sync>,
}

impl<A, T> Command for PipeCommand<A, T> where A: Component, T: Component{
    fn apply(self, world: &mut bevy::prelude::World) {
        let entity = self.entity;
        let func = self.command;
        world.run_system_once(move |mut q: Query<(&A, &mut T)>| {
            if let Ok((a, mut b)) = q.get_mut(entity) {
                func(a, b.as_mut());
            }
        });
    }
}


pub struct Mutation(Box<dyn Fn(&mut EntityCommands) + Send + Sync>);

impl Debug for Mutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mutation").finish()
    }
}

impl Mutation {
    pub fn single<T: Component>(f: impl Fn(&mut T) + Send + Sync + Clone + 'static) -> Self{
        Mutation(Box::new(move |commands: &mut EntityCommands| {
            let entity = commands.id();
            commands.commands().add(MutationCommand {
                entity,
                command: Box::new(f.clone())
            });
        }))
    }

    pub fn pipe<From: Component, To: Component>(f: impl Fn(&From, &mut To) + Send + Sync + Clone + 'static) -> Self{
        Mutation(Box::new(move |commands: &mut EntityCommands| {
            let entity = commands.id();
            commands.commands().add(PipeCommand {
                entity,
                command: Box::new(f.clone())
            });
        }))
    }

    pub fn exec(&self, commands: &mut EntityCommands) {
        (self.0)(commands)
    }
}
