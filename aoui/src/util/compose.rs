use bevy::{ecs::{bundle::Bundle, component::Component, system::{Command, EntityCommands}, entity::Entity}, sprite::Sprite, render::{texture::Image, color::Color}, transform::components::GlobalTransform};

use crate::{dsl::{prelude::Signals, IntoAsset}, sync::{SignalId, TypedSignal}, BuildTransform, Coloring};

use super::AouiCommands;

impl IntoAsset<Image> {
    pub fn into_bundle(self, commands: &mut AouiCommands, color: Color) -> impl Bundle {
        let handle = commands.load_or_default(self);
        (
            Sprite::default(),
            handle,
            BuildTransform::default(),
            GlobalTransform::default(),
            Coloring::new(color)
        )
    }
}

pub trait ComponentCompose: Component {
    fn compose(&mut self, other: Self);
}

pub struct ComposeInsert<T: ComponentCompose>(pub Entity, pub T);

impl<T: ComponentCompose> Command for ComposeInsert<T> {
    fn apply(self, world: &mut bevy::prelude::World) {
        match world.get_entity_mut(self.0) {
            Some(mut entity) => match entity.get_mut::<T>() {
                Some(mut component) => component.compose(self.1),
                None => { entity.insert(self.1); },
            },
            None => (),
        }
    }
}

pub struct AddSignalSend<T: SignalId>(pub Entity, pub TypedSignal<T::Data>);

impl<T: SignalId> Command for AddSignalSend<T> {
    fn apply(self, world: &mut bevy::prelude::World) {
        match world.get_entity_mut(self.0) {
            Some(mut entity) => match entity.get_mut::<Signals>() {
                Some(mut component) => component.add_sender::<T>(self.1),
                None => { entity.insert(Signals::from_sender::<T>(self.1)); },
            },
            None => (),
        }
    }
}

pub struct AddSignalRecv<T: SignalId>(pub Entity, pub TypedSignal<T::Data>);

impl<T: SignalId> Command for AddSignalRecv<T> {
    fn apply(self, world: &mut bevy::prelude::World) {
        match world.get_entity_mut(self.0) {
            Some(mut entity) => match entity.get_mut::<Signals>() {
                Some(mut component) => component.add_receiver::<T>(self.1),
                None => { entity.insert(Signals::from_receiver::<T>(self.1)); },
            },
            None => (),
        }
    }
}

pub trait ComposeExtension {
    fn compose(&mut self, component: impl ComponentCompose) -> &mut Self;
    fn compose2<T: ComponentCompose>(&mut self, a: Option<T>, b: Option<T>) -> &mut Self;
    fn add_sender<T: SignalId>(&mut self, component: TypedSignal<T::Data>) -> &mut Self;
    fn add_receiver<T: SignalId>(&mut self, component: TypedSignal<T::Data>) -> &mut Self;
}

impl ComposeExtension for EntityCommands<'_, '_, '_> {
    fn compose(&mut self, component: impl ComponentCompose) -> &mut Self{
        let entity = self.id();
        self.commands().add(ComposeInsert(entity, component));
        self
    }

    fn compose2<T: ComponentCompose>(&mut self, a: Option<T>, b: Option<T>)  -> &mut Self{
        match (a, b) {
            (None, None) => (),
            (Some(a), None) => { self.compose(a); },
            (None, Some(b)) => { self.compose(b); },
            (Some(mut a), Some(b)) => {
                a.compose(b);
                self.compose(a);
            },
        }
        self
    }

    fn add_sender<T: SignalId>(&mut self, component: TypedSignal<T::Data>)  -> &mut Self{
        let entity = self.id();
        self.commands().add(AddSignalSend::<T>(entity, component));
        self
    }

    fn add_receiver<T: SignalId>(&mut self, component: TypedSignal<T::Data>)  -> &mut Self{
        let entity = self.id();
        self.commands().add(AddSignalRecv::<T>(entity, component));
        self
    }
}
