
use std::{any::TypeId, marker::PhantomData, sync::Arc, mem, ops::DerefMut};

use bevy::{utils::HashMap, ecs::{query::WorldQuery, component::Component, entity::Entity}};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::util::{Object, AsObject, ComponentCompose};

use super::{AsyncSystemParam, AsyncExecutor, Signals};


pub static DUMMY_STATE: Lazy<States> = Lazy::new(||States::default());

pub trait StateId: 'static {
    type Data: AsObject;
}

#[derive(Debug, Component, Default)]
pub struct States {
    pub inner: HashMap<TypeId, Mutex<Object>>,
}

impl ComponentCompose for States {
    fn compose(&mut self, other: Self) {
        self.inner.extend(other.inner)
    }
}

impl States {
    pub fn tracks<T: StateId>(&self) -> bool {
        self.inner.contains_key(&TypeId::of::<T>())
    }

    pub fn is<T: StateId>(&self, f: impl FnOnce(&T::Data) -> bool ) -> bool {
        self.inner.get(&TypeId::of::<T>())
            .and_then(|x| Some(f(x.lock().get_ref()?)))
            .unwrap_or(false)
    }

    pub fn get<T: StateId>(&self) -> Option<T::Data> {
        self.inner.get(&TypeId::of::<T>()).and_then(|x| x.lock().get())
    }

    pub fn set<T: StateId>(&self, value: T::Data) -> Option<T::Data> {
        if let Some(lock) = self.inner.get(&TypeId::of::<T>()){
            let mut lock = lock.lock();
            mem::replace(lock.deref_mut(), Object::new(value)).get()
        } else {
            None
        }
    }

    pub fn and<T: StateId>(mut self, value: T::Data) -> Self {
        self.inner.insert(TypeId::of::<T>(), Mutex::new(Object::new(value)));
        self
    }

    pub fn insert<T: StateId>(&mut self, value: T::Data) -> Option<T::Data> {
        self.inner
            .insert(TypeId::of::<T>(), Mutex::new(Object::new(value)))
            .and_then(|mut x| Object::take(x.get_mut()))
    }
}
#[derive(Debug)]
pub struct State<T: StateId>(Option<T::Data>);

impl<T: StateId> State<T> {
    pub fn get(&self) -> Option<&T::Data> {
        self.0.as_ref()
    }
}

impl<T: StateId<Data=Object>> State<T> {
    pub fn try_get<A: AsObject>(&self) -> Option<&A> {
        self.0.as_ref().and_then(|x| x.get_ref())
    }
}

impl<T: StateId> AsyncSystemParam for State<T> {
    fn from_async_context(
        _: Entity,
        _: &Arc<AsyncExecutor>,
        _: &Signals,
        states: &States,
    ) -> Self {
        Self(states.get::<T>())
    }
}

#[derive(Debug, WorldQuery)]
pub struct HasState<T:StateId>{ 
    state: &'static States,
    p: PhantomData<T>,
}

impl<T:StateId> HasState<T> {
    pub fn get(&self) -> bool {
        self.state.inner.contains_key(&TypeId::of::<T>())
    }
}

#[derive(Debug, WorldQuery)]
pub struct StateRef<T:StateId>{ 
    state: &'static States,
    p: PhantomData<T>,
}

impl<T:StateId> StateRefItem<'_, T> {
    pub fn get(&self) -> Option<T::Data> {
        self.state.get::<T>()
    }
}

#[derive(Debug, WorldQuery)]
#[world_query(mutable)]
pub struct StateMut<T:StateId>{ 
    state: &'static mut States,
    p: PhantomData<T>,
}

impl<T:StateId> StateMutItem<'_, T> {
    pub fn get(&self) -> Option<T::Data> {
        self.state.get::<T>()
    }

    pub fn set(&mut self, data: T::Data) -> Option<T::Data> {
        self.state.set::<T>(data)
    }
}
