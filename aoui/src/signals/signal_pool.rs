use std::{sync::{RwLock, Arc}, marker::PhantomData, mem};

use bevy::{utils::HashMap, ecs::{component::Component, system::{Resource, ResMut, Local, Query, Commands}, entity::Entity}};
use smallvec::SmallVec;

use crate::dsl::CloneSplit;

use super::{signal::Signal, SignalBuilder, AsObject};

/// A pool of signals, 
/// signals created here will be auto cleaned up at the end of the frame.
/// 
/// Named signals can be obtained elsewhere.
/// 
/// Unbound signals won't be cleaned up.
#[derive(Debug, Resource, Default)]
pub struct SignalPool {
    unnamed: RwLock<Vec<Signal>>,
    named: RwLock<HashMap<String, Signal>>,
}

/// Add signals to the [`SignalPool`], and remove this component once added.
#[derive(Debug, Component, Default, Clone)]
#[component(storage="SparseSet")]
pub struct DeferredSignal(SmallVec<[Signal; 2]>);

impl DeferredSignal {
    pub const fn new() -> Self {
        DeferredSignal(SmallVec::new_const())
    }

    pub fn push<T: AsObject>(&mut self, s: &SignalBuilder<T>) {
        self.0.push(s.signal.clone())
    }
}
 
impl SignalPool {

    /// Get or create a unnamed signal.
    pub fn signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self) -> S{
        let signal = Signal::new();
        let mut unnamed = self.unnamed.write().unwrap();
        unnamed.push(signal.clone());
        S::clone_split(SignalBuilder {
            signal,
            p: PhantomData,
        })
    }

    /// Get or create a named signal.
    pub fn named<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self, name: &str) -> S {
        let named = self.named.read().unwrap();
        S::clone_split(if let Some(signal) = named.get(name) {
            SignalBuilder {
                signal: signal.clone(),
                p: PhantomData,
            }
        } else {
            drop(named);
            let mut named = self.named.write().unwrap();
            if let Some(signal) = named.get(name) {
                SignalBuilder {
                    signal: signal.clone(),
                    p: PhantomData,
                }
            } else {
                let signal = Signal::new();
                named.insert(name.to_owned(), signal.clone());
                SignalBuilder {
                    signal,
                    p: PhantomData,
                }
            }
        })
    }

    pub fn system_signal_cleanup(
        mut flag: Local<u8>,
        mut res: ResMut<SignalPool>
    ){
        let f = *flag;
        res.unnamed.get_mut().unwrap()
            .retain(|sig| if Arc::strong_count(&sig.inner) == 1 {
                false
            } else {
                sig.try_clean(f);
                true
            });
        res.named.get_mut().unwrap()
            .retain(|_, sig| if Arc::strong_count(&sig.inner) == 1 {
                false
            } else {
                sig.try_clean(f);
                true
            });
        *flag = 1 - *flag;
    }

    pub fn system_add_deferred(
        mut commands: Commands,
        mut pool: ResMut<SignalPool>,
        mut query: Query<(Entity, &mut DeferredSignal)>,
    ){
        pool.unnamed.get_mut().unwrap().extend(
            query.iter_mut().flat_map(|(entity, mut item)| {
                commands.entity(entity).remove::<DeferredSignal>();
                mem::take(&mut item.0).into_iter()
            })
        );
    }
}
