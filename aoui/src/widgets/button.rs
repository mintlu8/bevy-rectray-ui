use crate::dsl::{prelude::SignalSender, CloneSplit};
use crate::events::{CursorAction, EventFlags};
use crate::events::{EvButtonClick, EvToggleChange, Handlers};
use crate::signals::{AsObject, KeyStorage, Signal};
use crate::signals::{Object, SignalBuilder};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::ecs::{component::Component, query::With};
use bevy::{
    ecs::{entity::Entity, query::Has},
    reflect::Reflect,
};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

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
/// this propagates to children and can be used in [`DisplayIf`]
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

    pub fn rev(&mut self) -> bool {
        match self {
            CheckButton::Unchecked => {
                *self = CheckButton::Checked;
                true
            }
            CheckButton::Checked => {
                *self = CheckButton::Unchecked;
                false
            }
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
#[derive(Debug, Clone, Component)]
pub struct RadioButton(Arc<Mutex<Object>>, SignalSender<Object>);

impl RadioButton {
    /// Create an empty `RadioButton` context, usually unchecked by default.
    pub fn new_empty() -> Self {
        RadioButton(
            Arc::new(Mutex::new(Object::NONE)),
            SignalBuilder::new(Signal::new()).send(),
        )
    }

    pub fn new(default: impl AsObject) -> Self {
        RadioButton(
            Arc::new(Mutex::new(Object::new(default))),
            SignalBuilder::new(Signal::new()).send(),
        )
    }

    pub fn set(&self, payload: &Payload) {
        let mut lock = self.0.lock().unwrap();
        *lock = payload.0.clone();
        self.1.send_dyn(payload.0.clone())
    }

    pub fn get<T: AsObject>(&self) -> Option<T> {
        self.0.lock().unwrap().get()
    }

    pub fn recv<T: AsObject>(&self) -> SignalBuilder<T> {
        self.1.specialize_receiver()
    }

    pub fn clear(&self) {
        let mut lock = self.0.lock().unwrap();
        *lock = Object::NONE;
        self.1.send(Object::unnameable())
    }
}

impl PartialEq<Payload> for RadioButton {
    fn eq(&self, other: &Payload) -> bool {
        let lock = self.0.lock().unwrap();
        lock.deref().equal_to(&other.0)
    }
}

/// Component for making `RadioButton` behave like `CheckButton`.
#[derive(Debug, Clone, Component, Reflect)]
pub struct RadioButtonCancel;

pub fn button_on_click(
    mut commands: Commands,
    key_storage: Res<KeyStorage>,
    query: Query<
        (
            Entity,
            &CursorAction,
            &Handlers<EvButtonClick>,
            Option<&Payload>,
        ),
        With<Button>,
    >,
) {
    for (entity, action, submit, payload) in query.iter() {
        if !action.is(EventFlags::LeftClick) {
            continue;
        }
        let mut commands = commands.entity(entity);
        if let Some(payload) = payload {
            submit.handle_dyn(&mut commands, &key_storage, payload.0.clone());
        } else {
            submit.handle_dyn(&mut commands, &key_storage, Object::new(()));
        }
    }
}

pub fn check_button_on_click(
    mut commands: Commands,
    key_storage: Res<KeyStorage>,
    mut query: Query<(
        Entity,
        &CursorAction,
        &mut CheckButton,
        Option<&Handlers<EvToggleChange>>,
        Option<&Handlers<EvButtonClick>>,
        Option<&Payload>,
    )>,
) {
    for (entity, action, mut state, change, submit, payload) in query.iter_mut() {
        if !action.is(EventFlags::LeftClick) {
            continue;
        }
        let state = state.rev();
        let mut commands = commands.entity(entity);
        if let Some(signal) = change {
            signal.handle(&mut commands, &key_storage, state);
        }
        if !state {
            continue;
        }
        if let Some(signal) = submit {
            if let Some(payload) = payload {
                signal.handle_dyn(&mut commands, &key_storage, payload.0.clone());
            } else {
                signal.handle_dyn(&mut commands, &key_storage, Object::new(()));
            }
        }
    }
}

pub fn radio_button_on_click(
    mut commands: Commands,
    key_storage: Res<KeyStorage>,
    mut query: Query<(
        Entity,
        &CursorAction,
        &RadioButton,
        &Payload,
        Option<&Handlers<EvButtonClick>>,
        Has<RadioButtonCancel>,
    )>,
) {
    for (entity, action, state, payload, submit, cancellable) in query.iter_mut() {
        let mut commands = commands.entity(entity);
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
        if let Some(signal) = submit {
            signal.handle_dyn(&mut commands, &key_storage, payload.0.clone());
        }
    }
}

pub fn generate_check_button_state(
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
/// the [`EvButtonClick`] signals will send the containing data.
///
/// This component is required in `radio_button` as its discriminant.
///
/// # Signal Behavior
///
/// * `button` `EvButtonClick`: sends `Payload` or `()`.
/// * `radio_button` `EvButtonClick`: sends `Payload`, which is required.
/// * `check_button` `EvButtonClick`: If checked, sends `Payload` or `()`.
///
#[derive(Debug, Clone, Component)]
pub struct Payload(Object);

impl Payload {
    pub const fn empty() -> Self {
        Self(Object::NONE)
    }

    pub fn new(value: impl AsObject) -> Self {
        Self(Object::new(value))
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
