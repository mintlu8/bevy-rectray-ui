use crate::dsl::prelude::Signals;
use crate::events::{CursorAction, EventFlags};
use bevy_defer::{Signal, SignalId, SignalSender, TypedSignal, Object, AsObject};
use crate::util::CloneSplit;
use bevy::ecs::system::{Commands, Query};
use bevy::ecs::{component::Component, query::With};
use bevy::reflect::std_traits::ReflectDefault;
use bevy::{
    ecs::{entity::Entity, query::Has},
    reflect::Reflect,
};
use std::sync::Arc;
use parking_lot::Mutex;

/// Marker for sending the `Submit` signal on click.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component, Reflect)]
pub struct Button;

/// This component stores the state of `CheckButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component, Reflect)]
pub enum CheckButton {
    #[default]
    Unchecked,
    Checked,
}

/// State of a CheckButton or a RadioButton,
/// this propagates to children and can be used in [`DisplayIf`](super::util::DisplayIf)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
pub enum CheckButtonState {
    Unchecked,
    Checked,
}

impl From<bool> for CheckButtonState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Checked,
            false => Self::Unchecked,
        }
    }
}

impl CheckButton {
    pub fn get(&self) -> bool {
        match self {
            CheckButton::Unchecked => false,
            CheckButton::Checked => true,
        }
    }

    pub fn set(&mut self, value: bool) {
        if value{
            *self = CheckButton::Checked;
        } else {
            *self = CheckButton::Unchecked;
        }
    }
}

impl From<bool> for CheckButton {
    fn from(value: bool) -> Self {
        match value {
            true => CheckButton::Checked,
            false => CheckButton::Unchecked,
        }
    }
}

/// Component of `radio_button` containing the shared state.
///
/// Discriminant is the [`Payload`] component.
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Default)]
pub struct RadioButton{
    #[reflect(ignore)]
    pub(crate) storage: Arc<Mutex<Object>>,
    #[reflect(ignore)]
    pub(crate) sender: Signal<Object>,
}

impl Default for RadioButton {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl RadioButton {
    /// Create an empty `RadioButton` context, usually unchecked by default.
    pub fn new_empty() -> Self {
        RadioButton {
            storage: Arc::new(Mutex::new(Object::NONE)),
            sender: Default::default(),
        }
    }

    pub fn new(default: impl AsObject) -> Self {
        let obj = Object::new(default);
        RadioButton {
            storage: Arc::new(Mutex::new(obj.clone())),
            sender: Signal::new(obj),
        }
    }

    pub fn set(&self, payload: &Payload) {
        let mut lock = self.storage.lock();
        *lock = payload.get();
        self.sender.write(payload.get())
    }

    pub fn get<T: AsObject>(&self) -> Option<T> {
        self.storage.lock().get()
    }

    pub fn recv<T: AsObject>(&self) -> TypedSignal<T> {
        TypedSignal::from_inner(self.sender.get_shared())
    }

    pub fn clear(&self) {
        let mut lock = self.storage.lock();
        *lock = Object::NONE;
        self.sender.write(Object::unnameable())
    }
}

impl PartialEq<Payload> for RadioButton {
    fn eq(&self, other: &Payload) -> bool {
        let lock = self.storage.lock();
        lock.equal_to(&other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct ButtonClick;

impl SignalId for ButtonClick {
    type Data = Object;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct ToggleChange;

impl SignalId for ToggleChange {
    type Data = bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct ToggleInvoke;

impl SignalId for ToggleInvoke {
    type Data = bool;
}



/// Component for making `RadioButton` behave like `CheckButton`.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Default, Reflect)]
pub struct RadioButtonCancel;

pub(crate) fn button_on_click(
    query: Query<(&CursorAction, SignalSender<ButtonClick>, Option<&Payload>), With<Button>>,
) {
    for (action, submit, payload) in query.iter() {
        if !action.is(EventFlags::LeftClick) {
            continue;
        }
        if let Some(payload) = payload {
            submit.send(payload.0.clone());
        } else {
            submit.send(Object::new(()));
        }
    }
}

pub(crate) fn check_button_on_click(
    mut query: Query<(Option<&CursorAction>, &mut CheckButton, Option<&mut Signals>, Option<&Payload>)>,
) {
    for (action, mut state, mut signals, payload) in query.iter_mut() {
        let val = if action.map(|x| x.intersects(EventFlags::LeftClick)).unwrap_or(false) {
            !state.get()
        } else if let Some(val) = signals.as_mut().and_then(|s| s.poll_once::<ToggleInvoke>()){
            val
        } else {
            continue;
        };
        if state.get() != val {
            state.set(val);
            let Some(signals) = signals.as_ref() else {continue};
            signals.send::<ToggleChange>(val);
            if val {
                if let Some(payload) = payload {
                    signals.send::<ButtonClick>(payload.0.clone());
                } else {
                    signals.send::<ButtonClick>(Object::new(()));
                }
            }
        }
    }
}

pub(crate) fn radio_button_on_click(
    mut query: Query<(
        &CursorAction, &RadioButton, &Payload, SignalSender<ButtonClick>, Has<RadioButtonCancel>,
    )>,
) {
    for (action, state, payload, submit, cancellable) in query.iter_mut() {
        if !action.is(EventFlags::LeftClick) {
            continue;
        }
        if state == payload {
            if cancellable {
                state.clear();
            }
            continue;
        }
        state.set(payload);
        submit.send(payload.0.clone());
    }
}

pub(crate) fn generate_check_button_state(
    mut commands: Commands,
    query1: Query<(Entity, &CheckButton)>,
    query2: Query<(Entity, &RadioButton, &Payload)>,
) {
    for (entity, btn) in query1.iter() {
        commands
            .entity(entity)
            .insert(CheckButtonState::from(btn.get()));
    }
    for (entity, radio, payload) in query2.iter() {
        commands
            .entity(entity)
            .insert(CheckButtonState::from(radio == payload));
    }
}

/// A dynamic piece of data.
/// When attached to a widget in the button family,
/// the [`ButtonClick`] signals will send the containing data.
///
/// This component is required in `radio_button` as its discriminant.
///
/// # Signal Behavior
///
/// * `button` `EvButtonClick`: sends `Payload` or `()`.
/// * `radio_button` `EvButtonClick`: sends `Payload`, which is required.
/// * `check_button` `EvButtonClick`: If checked, sends `Payload` or `()`.
///
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct Payload(Object);

impl Payload {
    pub const fn empty() -> Self {
        Self(Object::NONE)
    }

    pub fn new(value: impl AsObject) -> Self {
        Self(Object::new(value))
    }

    pub fn get(&self) -> Object {
        self.0.clone()
    }

    /// Mutate the payload.
    pub fn mut_dyn<A: AsObject, B: AsObject>(&mut self, f: impl Fn(&A) -> B) {
        let Some(value) = self.0.get_ref().map(f) else {
            return;
        };
        self.0.set(value)
    }
}

/// Construct an array of shared `RadioButton` contexts.
///
/// # Example
/// ```
/// use bevy_aoui::widgets::button::radio_button_group;
/// let (ferris, gopher, python) = radio_button_group("Ferris");
/// // Construct 4 items as an array.
/// let colors = radio_button_group::<[_; 4]>("Red");
/// ```
pub fn radio_button_group<T: CloneSplit<RadioButton>>(default: impl AsObject) -> T {
    T::clone_split(RadioButton::new(default))
}
