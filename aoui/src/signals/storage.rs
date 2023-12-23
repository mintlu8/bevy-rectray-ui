use std::mem;

use bevy::{utils::HashMap, ecs::system::Resource};

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

/// A resource that contains 
#[derive(Debug, Clone, Resource, Default)]
pub struct KeyStorage(HashMap<String, ChangeDetectObject>);

impl KeyStorage {
    pub fn new() -> Self {
        KeyStorage(HashMap::new())
    }

    /// Obtain a value with a key
    pub fn get<T: DataTransfer>(&self, name: impl AsRef<str>) -> Option<T> {
        self.0.get(name.as_ref()).and_then(|x| x.value.get())
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    pub fn set(&mut self, name: impl Into<String> + AsRef<str>, data: impl DataTransfer) -> Option<Object> {
        let obj = Object::new(data);
        match self.0.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                self.0.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Sets a value, returns the original value if exists.
    /// 
    /// Sets 'changed' if value is **different**, aka `!=`.
    pub fn set_dyn(&mut self, name: impl Into<String> + AsRef<str>, obj: Object) -> Option<Object> {
        match self.0.get_mut(name.as_ref()) {
            Some(original) => {
                if original.value == obj {
                    Some(obj)
                } else {
                    Some(mem::replace(original, ChangeDetectObject::new(obj)).value)
                }
            },
            None => {
                self.0.insert(name.into(), ChangeDetectObject::new(obj));
                None
            },
        }
    }

    /// Remove a value and return it.
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<Object> {
        self.0.remove(name.as_ref()).map(|x| x.value)
    }

    /// Check if a key exists.
    pub fn exists(&self, name: impl AsRef<str>) -> bool {
        self.0.contains_key(name.as_ref())
    }

    /// Check if a key exists and is changed.
    pub fn is_changed(&self, name: impl AsRef<str>) -> bool {
        self.0.get(name.as_ref())
            .map(|x| x.changed)
            .unwrap_or(false)
    }

    /// Obtain a value with a key only if it is changed.
    pub fn get_changed<T: DataTransfer>(&self, name: impl AsRef<str>) -> Option<T> {
        self.0.get(name.as_ref())
            .filter(|x| x.changed)
            .and_then(|x| x.value.get())
    }

    /// Set all `changed` values to false.
    pub fn reset_changed_status(&mut self) {
        self.0.iter_mut().for_each(|(_, v)| v.changed = false )
    }
}
