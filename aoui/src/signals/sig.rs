use std::sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex};

use super::{Object, DataTransfer};

/// A Signal for sending and receiving data between entities.
/// 
/// This simulates the "double buffered" behavior of bevy's events.
/// If read, Signal lives for 1 frame, if not, lives for 2,
#[derive(Debug, Clone)]
pub(super) struct Signal{ 
    inner: Arc<Mutex<Object>>,
    polled: Arc<AtomicBool>,
}

impl Signal {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Object::NONE)),
            polled: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn try_clean(&self)  {
        if self.polled.swap(true, Ordering::Relaxed) {
            self.inner.lock().unwrap().clean();
        }
    }

    #[allow(dead_code)]
    pub fn write(&self, item: impl DataTransfer) {
        let mut lock = self.inner.lock().unwrap();
        lock.set(item);
        self.polled.store(false, Ordering::Relaxed);
    }

    pub fn write_dyn(&self, item: Object) {
        let mut lock = self.inner.lock().unwrap();
        *lock = item;
        self.polled.store(false, Ordering::Relaxed);
    }

    pub fn read<T: DataTransfer>(&self) -> Option<T> {
        let lock = self.inner.lock().unwrap();
        self.polled.store(true, Ordering::Relaxed);
        lock.get()
    }

    pub fn read_dyn(&self) -> Object {
        let lock = self.inner.lock().unwrap();
        self.polled.store(true, Ordering::Relaxed);
        lock.clone()
    }

    pub fn read_any(&self) -> bool {
        let lock = self.inner.lock().unwrap();
        self.polled.store(true, Ordering::Relaxed);
        lock.is_some()
    }
}
