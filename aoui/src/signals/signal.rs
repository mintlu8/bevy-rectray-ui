use std::sync::{Arc, atomic::{AtomicBool, Ordering, AtomicU8}, Mutex};

use super::{Object, AsObject};

/// A Signal for sending and receiving data between entities.
/// 
/// This simulates the "double buffered" behavior of bevy's events.
/// If read, Signal lives for 1 frame, if not, lives for 2,
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct Signal{ 
    pub(super) inner: Arc<SignalInner>,
}

#[derive(Debug)]
pub struct SignalInner {
    polled: AtomicBool,
    drop_flag: AtomicU8,
    object: Mutex<Object>,
}

impl SignalInner {
    fn new() -> Self {
        Self {
            polled: AtomicBool::new(false),
            drop_flag: AtomicU8::new(255),
            object: Mutex::new(Object::NONE),
        }
    }
}

impl Signal {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(SignalInner::new())
        }
    }

    pub(crate) fn try_clean(&self, flag: u8)  {
        if self.inner.polled.load(Ordering::Relaxed) 
                || ![255, flag].contains(&self.inner.drop_flag.swap(flag, Ordering::Relaxed)) {
            self.inner.object.lock().unwrap().clean();
            self.inner.polled.store(false, Ordering::Relaxed);
            self.inner.drop_flag.store(0, Ordering::Relaxed);
        }
    }

    pub(crate) fn write(&self, item: impl AsObject) {
        let mut lock = self.inner.object.lock().unwrap();
        lock.set(item);
        self.inner.drop_flag.store(255, Ordering::Relaxed);
        self.inner.polled.store(false, Ordering::Relaxed);
    }

    pub(crate) fn read<T: AsObject>(&self) -> Option<T> {
        let lock = self.inner.object.lock().unwrap();
        self.inner.polled.store(true, Ordering::Relaxed);
        lock.get()
    }

    pub(crate) fn read_any(&self) -> bool {
        let lock = self.inner.object.lock().unwrap();
        self.inner.polled.store(true, Ordering::Relaxed);
        lock.is_some()
    }
}
