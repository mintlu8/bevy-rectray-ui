use std::{sync::{RwLock, Arc}, marker::PhantomData};
use bevy::{utils::HashMap, ecs::system::{Resource, ResMut, Local}};
use crate::util::CloneSplit;
use super::{signal::Signal, SignalBuilder, AsObject};

/// A pool of signals.
///
/// There are four types of signal.
/// `named`, `unnamed`, `tracked` and `storage`.
///
/// * `tracked`: Signal value is cleaned up every frame.
///     if polled, lives for `1` frame, if not, live for `2`.
/// * `storage`: Signal value is not cleaned up and can be read and change detected anytime.
/// * `named`: Named signals can be obtained from the signal pool anywhere by name.
/// * `unnamed`: Unnamed signals cannot be recreated by the `SignalPool`.
#[derive(Debug, Resource, Default)]
pub struct SignalPool {
    unnamed: RwLock<Vec<Signal>>,
    named: RwLock<HashMap<String, Signal>>,
    storage: RwLock<HashMap<String, Signal>>,
}

impl SignalPool {

    /// Create a `unnamed`, `tracked` signal.
    pub fn signal<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self) -> S{
        let signal = Signal::new();
        let mut unnamed = self.unnamed.write().unwrap();
        unnamed.push(signal.clone());
        S::clone_split(SignalBuilder {
            signal,
            p: PhantomData,
        })
    }

    /// Get or create a `named`, `tracked` signal.
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

    /// Get or create a `named`, `storage` signal.
    pub fn shared_storage<T: AsObject, S: CloneSplit<SignalBuilder<T>>>(&self, name: &str) -> S {
        let storage = self.storage.read().unwrap();
        S::clone_split(if let Some(signal) = storage.get(name) {
            SignalBuilder {
                signal: signal.clone(),
                p: PhantomData,
            }
        } else {
            drop(storage);
            let mut storage = self.storage.write().unwrap();
            if let Some(signal) = storage.get(name) {
                SignalBuilder {
                    signal: signal.clone(),
                    p: PhantomData,
                }
            } else {
                let signal = Signal::new();
                storage.insert(name.to_owned(), signal.clone());
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
            .retain(|sig| if Arc::strong_count(&sig.inner) > 1 {
                sig.try_clean(f); true
            } else {
                false
            });
        res.named.get_mut().unwrap()
            .retain(|_, sig| if Arc::strong_count(&sig.inner) > 1 {
                sig.try_clean(f); true
            } else {
                false
            });
        res.storage.get_mut().unwrap()
            .retain(|_, sig| Arc::strong_count(&sig.inner) > 1);
        *flag = 1 - *flag;
    }
}
