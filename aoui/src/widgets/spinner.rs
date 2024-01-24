use std::fmt::Debug;

use bevy::{ecs::{component::Component, system::Query, world::Mut, query::{Without, Changed}}, text::Text};
use itertools::Itertools;

use crate::{dsl::prelude::Signals, sync::SignalId, util::{Object, convert::{DslConvert, SealToken}, AsObject}};

use super::{TextFragment, inputbox::TextChange};

#[derive(Debug)]
pub enum Increment {}

impl SignalId for Increment {
    type Data = Object;
}
#[derive(Debug)]
pub enum Decrement {}

impl SignalId for Decrement {
    type Data = Object;
}

#[derive(Debug)]
pub enum SpinChange {}

impl SignalId for SpinChange {
    type Data = Object;
}

/// A text based spinner implementation.
#[derive(Clone, Component)]
pub struct SpinnerText {
    pub current: usize,
    pub map: fn(Object) -> String,
    pub contents: Vec<Object>,
    pub looping: bool,
}
impl SpinnerText {
    pub fn new<T: SpinDisplay, F>(iter: F) -> Self where F: IntoIterator<Item = T>  {
        let contents = iter.into_iter().map(|x| Object::new(x)).collect();
        SpinnerText { 
            current: 0, 
            map: T::mapper, 
            contents,
            looping: false
        }
    }

    pub fn select(&mut self, obj: Object) -> bool {
        if obj.is_none() { return false; }
        let Some((index, _)) = self.contents.iter()
            .find_position(|x| x.equal_to(&obj)) else {return false;};
        self.current = index;
        true
    }

    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    pub fn increment(item: &mut Mut<Self>) {
        if item.current < item.contents.len() - 1 {
            item.current += 1;
        }
    }

    pub fn decrement(item: &mut Mut<Self>) {
        if item.current != 0 {
            item.current = item.current.min(item.contents.len() - 1) - 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn get_object(&self) -> Object {
        if self.is_empty() { return Object::NONE; }
        self.contents[self.current.min(self.len() - 1)].clone()
    }

    pub fn get(&self) -> String {
        if self.is_empty() { return String::new(); }
        (self.map)(self.contents[self.current.min(self.len() - 1)].clone())
    }
}

impl Debug for SpinnerText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpinText")
            .field("current", &self.current)
            .field("contents", &self.contents)
            .field("looping", &self.looping)
            .finish()
    }
}

impl Default for SpinnerText {
    fn default() -> Self {
        Self { 
            current: 0,
            map: |_|String::new(),
            contents: Vec::new(),
            looping: false
        }
    }
}

pub trait SpinDisplay: AsObject {
    fn mapper(obj: Object) -> String;
}

impl SpinDisplay for i32 {
    fn mapper(obj: Object) -> String {
        obj.get_ref::<Self>()
            .map(|x| x.to_string())
            .unwrap_or("???".to_owned())
    }
}

impl SpinDisplay for f32 {
    fn mapper(obj: Object) -> String {
        obj.get_ref::<Self>()
            .map(|x| format!("{:.2}", x))
            .unwrap_or("???".to_owned())
    }
}

impl SpinDisplay for String {
    fn mapper(obj: Object) -> String {
        obj.get::<Self>()
            .unwrap_or("???".to_owned())
    }
}

impl SpinDisplay for &'static str {
    fn mapper(obj: Object) -> String {
        obj.get_ref::<Self>()
            .map(|x| x.to_string())
            .unwrap_or("???".to_owned())
    }
}

impl<T: SpinDisplay, F> DslConvert<SpinnerText, '±'> for F where F: IntoIterator<Item = T> {
    fn parse(self) -> SpinnerText {
        let contents = self.into_iter().map(|x| Object::new(x)).collect();
        SpinnerText { 
            current: 0, 
            map: T::mapper, 
            contents,
            looping: false
        }
    }
    fn sealed(_: SealToken) {}
}

pub fn spin_text_change(
    mut query: Query<(&mut SpinnerText, &mut Signals)>,
) {
    for (mut spin, signals) in query.iter_mut() {
        let mut changed = false;
        if signals.poll_once::<Increment>().is_some() {
            SpinnerText::increment(&mut spin);
            changed = true;
        }

        if signals.poll_once::<Decrement>().is_some() {
            SpinnerText::decrement(&mut spin);
            changed = true;
        }

        if changed {
            signals.send::<TextChange>(spin.get());
            signals.send::<SpinChange>(spin.get_object());
        }
    }
}

pub fn sync_spin_text_with_text(
    mut text: Query<(&SpinnerText, &mut Text), (Changed<SpinnerText>, Without<TextFragment>)>,
    mut frag: Query<(&SpinnerText, &mut TextFragment), Changed<SpinnerText>>,
) {
    for (spin, mut text) in text.iter_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = spin.get()
        }
    }
    for (spin, mut text) in frag.iter_mut() {
        text.text = spin.get()
    }
}