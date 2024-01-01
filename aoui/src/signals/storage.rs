use std::{mem, sync::RwLock};

use bevy::{utils::HashMap, ecs::system::{Resource, ResMut}};

use super::{DataTransfer, Object};

#[derive(Debug, Clone)]
struct ChangeDetectObject {
    value: Object,
    changed: bool,
}

impl ChangeDetectObject {
    pub fn new(value: Object) -> Self{
        Self {
            value,
            changed: true,
        }
    }
}

/// A resource that contains keys with dynamic objects,
/// while offering change detection per frame.
#[derive(Debug, Resource, Default)]
pub struct KeyStorage(RwLock<HashMap<String, ChangeDetectObject>>);

impl KeyStorage {
    pub fn new() -> Self {
        KeyStorage(RwLock::new(HashMap::new()))
    }

    /// Obtain a value with a key
    pub fn get<T: DataTransfer>(&self, name: impl AsRef<str>) -> Option<T> {
        let lock = self.0.read().unwrap();
        lock.get(name.as_ref()).and_then(|x| x.value.get())
    }

    /// Obtain a value with a key.
    /// 
    /// This should be used if the resource is uncontested.
    pub fn get_owned<T: DataTransfer>(&mut self, name: impl AsRef<str>) -> Option<T> {
        let lock = self.0.get_mut().unwrap();
        lock.get(name.as_ref()).and_then(|x| x.value.get())
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    pub fn set(&self, name: impl Into<String> + AsRef<str>, data: impl DataTransfer) -> Option<Object> {
        let mut lock = self.0.write().unwrap();
        let obj = Object::new(data);
        match lock.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                lock.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    /// 
    /// This should be used if the resource is uncontested.
    pub fn set_dyn(&self, name: impl Into<String> + AsRef<str>, obj: Object) -> Option<Object> {
        let mut lock = self.0.write().unwrap();
        match lock.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                lock.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    /// 
    /// This should be used if the resource is uncontested.
    pub fn set_owned(&mut self, name: impl Into<String> + AsRef<str>, data: impl DataTransfer) -> Option<Object> {
        let lock = self.0.get_mut().unwrap();
        let obj = Object::new(data);
        match lock.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                lock.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    pub fn set_dyn_owned(&mut self, name: impl Into<String> + AsRef<str>, obj: Object) -> Option<Object> {
        let lock = self.0.get_mut().unwrap();
        match lock.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                lock.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Remove a value and return it.
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Object> {
        let mut lock = self.0.write().unwrap();
        lock.remove(name.as_ref()).map(|x| x.value)
    }

    /// Check if a key exists.
    pub fn exists(&self, name: impl AsRef<str>) -> bool {
        let lock = self.0.read().unwrap();
        lock.contains_key(name.as_ref())
    }

    /// Check if a key exists and is changed.
    pub fn is_changed(&self, name: impl AsRef<str>) -> bool {
        let lock = self.0.read().unwrap();
        lock.get(name.as_ref())
            .map(|x| x.changed)
            .unwrap_or(false)
    }

    /// Obtain a value with a key only if it is changed.
    pub fn get_changed<T: DataTransfer>(&self, name: impl AsRef<str>) -> Option<T> {
        let lock = self.0.read().unwrap();
        lock.get(name.as_ref())
            .filter(|x| x.changed)
            .and_then(|x| x.value.get())
    }

    /// Sets all `changed` values to false.
    pub fn reset_changed_status(&mut self) {
        self.0.get_mut().unwrap().iter_mut().for_each(|(_, v)| v.changed = false )
    }


    /// A system that sets all `changed` values to false.
    pub fn system_reset_changed_status(mut res: ResMut<Self>) {
        res.reset_changed_status()
    }
}
