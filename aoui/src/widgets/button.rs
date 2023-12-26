use std::{sync::{Mutex, Arc}, ops::Deref, mem};

use bevy::{render::view::Visibility, hierarchy::Children, ecs::{entity::Entity, system::ResMut}};
use bevy::window::{Window, PrimaryWindow, CursorIcon};
use bevy::ecs::{system::{Query, Resource, Res, Commands}, component::Component, query::With};
use crate::{Opacity, dsl::prelude::signal, signals::KeyStorage};
use crate::signals::{Object, DynamicSender, SignalBuilder};
use crate::events::{Handlers, EvButtonClick, EvToggleChange};
use crate::{signals::DataTransfer, dsl::prelude::Interpolate};
use crate::events::{EventFlags, CursorFocus, CursorAction};


/// Set cursor if [`CursorFocus`] is some [`EventFlags`].
///
/// Call `register_cursor_default` on the `App` if your cursor does not revert.
#[derive(Debug, Clone, Copy, Component)]
pub struct SetCursor {
    pub flags: EventFlags,
    pub icon: CursorIcon,
}

/// Visible only when some conditions are met.
/// 
/// Supported conditions are:
/// 
/// * `EventFlags`: For `CursorFocus`
/// * `CheckButtonState`: For `CheckButton` and `RadioButton`'s status
/// 
/// This component uses `Interpolate<Opacity>` if exists, if not, uses `Visibility`.
#[derive(Debug, Clone, Copy, Component)]
pub struct DisplayIf<T>(pub T);

pub fn event_conditional_visibility(mut query: Query<(&DisplayIf<EventFlags>, Option<&CursorFocus>, &mut Visibility, Option<&mut Interpolate<Opacity>>)>){
    query.par_iter_mut().for_each(|(display_if, focus, mut vis, mut opacity)| {
        if focus.is_some() && display_if.0.contains(focus.unwrap().flags()) 
            || focus.is_none() && display_if.0.contains(EventFlags::Idle) {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else if let Some(opacity) = opacity.as_mut() {
            opacity.interpolate_to(0.0);
        } else {
            *vis = Visibility::Hidden;
        }
    })
}

/// Marker for sending the `Submit` signal on click.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub struct Button;

/// This component stores the state of `CheckButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub enum CheckButton{
    #[default]
    Unchecked,
    Checked,
}

/// State of a CheckButton or a RadioButton, 
/// this propagates to children and can be used in `DisplayIf`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum CheckButtonState{
    Unchecked,
    Checked,
}

impl From<bool> for CheckButtonState{
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
            },
            CheckButton::Checked => {
                *self = CheckButton::Unchecked;
                false
            },
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
/// Component for `RadioButton`, contains the shared state.
/// 
/// Discriminant is the `Payload` component.
#[derive(Debug, Clone, Component)]
pub struct RadioButton(Arc<Mutex<Object>>, DynamicSender);

impl RadioButton {
    pub fn new(default: impl DataTransfer) -> Self {
        let (send,) = signal::<(), _>();
        RadioButton(Arc::new(Mutex::new(Object::new(default))), send.clone().dynamic_send())
    }

    pub fn set(&self, payload: &Payload) {
        let mut lock = self.0.lock().unwrap();
        *lock = payload.0.clone();
        self.1.send_dyn(payload.0.clone())
    }

    pub fn get<T: DataTransfer>(&self) -> Option<T> {
        self.0.lock().unwrap().get()
    }

    pub fn recv<T: DataTransfer>(&self) -> SignalBuilder<T> {
        self.1.new_receiver()
    }
}

impl PartialEq<Payload> for RadioButton {
    fn eq(&self, other: &Payload) -> bool {
        let lock = self.0.lock().unwrap();
        lock.deref() == &other.0
    }
}

pub fn button_on_click(
    mut commands: Commands,
    mut key_storage: ResMut<KeyStorage>,
    query: Query<(&CursorAction, &Handlers<EvButtonClick>, Option<&Payload>), With<Button>>
) {
    for (action, submit, payload) in query.iter() {
        if !action.is(EventFlags::LeftClick) { continue }
        if let Some(payload) = payload {
            submit.handle_dyn(&mut commands, &mut key_storage, payload.0.clone());
        } else {
            submit.handle_dyn(&mut commands, &mut key_storage, Object::new(()));
        }
    }
}

pub fn check_button_on_click(
    mut commands: Commands,
    mut key_storage: ResMut<KeyStorage>,
    mut query: Query<(&CursorAction, &mut CheckButton, Option<&Handlers<EvToggleChange>>, Option<&Handlers<EvButtonClick>>, Option<&Payload>)>
) {
    for (action, mut state, change, submit, payload) in query.iter_mut() {
        if !action.is(EventFlags::LeftClick) { continue }
        let state = state.rev();
        if let Some(signal) = change {
            signal.handle(&mut commands, &mut key_storage, state);
        }
        if !state {continue;}
        if let Some(signal) = submit {
            if let Some(payload) = payload {
                signal.handle_dyn(&mut commands, &mut key_storage, payload.0.clone());
            } else {
                signal.handle_dyn(&mut commands, &mut key_storage, Object::new(()));
            }
        }
    }
}

pub fn radio_button_on_click(
    mut commands: Commands,
    mut key_storage: ResMut<KeyStorage>,
    mut query: Query<(&CursorAction, &RadioButton, &Payload, Option<&Handlers<EvButtonClick>>)>
) {
    for (action, state, payload, submit) in query.iter_mut() {
        if !action.is(EventFlags::LeftClick) { continue }
        state.set(payload);
        if let Some(signal) = submit {
            signal.handle_dyn(&mut commands, &mut key_storage, payload.0.clone());
        }
    }
}

pub fn check_conditional_visibility(
    mut query: Query<(&DisplayIf<CheckButtonState>, &CheckButtonState, &mut Visibility, Option<&mut Interpolate<Opacity>>)>
) {
    query.par_iter_mut().for_each(|(display_if, state, mut vis, mut opacity)| {
        if &display_if.0 == state {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else if let Some(opacity) = opacity.as_mut() {
            opacity.interpolate_to(0.0);
        } else {
            *vis = Visibility::Hidden;
        }
    })
}

/// If set, we set the cursor to a default value every frame.
/// 
/// Not a part of the standard plugin, but
/// can be used if you are using `SetCursor`.
#[derive(Debug, Resource, Clone, Copy)]
pub struct CursorDefault(pub CursorIcon);

impl Default for CursorDefault {
    fn default() -> Self {
        Self(CursorIcon::Arrow)
    }
}

pub fn set_cursor(
    default_cursor: Option<Res<CursorDefault>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    query: Query<(&SetCursor, &CursorFocus)>,
){
    if let Some(icon) = default_cursor{
        window.single_mut().cursor.icon = icon.0;
    }
    for (cursor, focus) in query.iter() {
        if cursor.flags.contains(focus.flags()) {
            window.single_mut().cursor.icon = cursor.icon;
            break;
        }
    }
}


pub fn generate_check_button_state(
    mut commands: Commands,
    query1: Query<(Entity, &CheckButton)>,
    query2: Query<(Entity, &RadioButton, &Payload)>
) {
    for (entity, btn) in query1.iter() {
        commands.entity(entity).insert(CheckButtonState::from(btn.get()));
    }
    for (entity, radio, payload) in query2.iter() {
        commands.entity(entity).insert(CheckButtonState::from(radio == payload));
    }
}

/// Marker component for passing `CursorFocus`, 
/// `CursorAction` and `CheckButtonState` to their descendants.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct PropagateFocus;

/// Propagate [`CursorFocus`] and [`CursorAction`] down descendants.
pub fn propagate_focus<T: Component + Clone>(
    mut commands: Commands, 
    query: Query<(&T, &Children), With<PropagateFocus>>, 
    descendent: Query<&Children>
) {
    let mut queue = Vec::new();
    for (focus, children) in query.iter() {
        for child in children {
            commands.entity(*child).insert(focus.clone());
            queue.push((child, focus));
        }
    }
    while !queue.is_empty() {
        for (entity, focus) in mem::take(&mut queue) {
            let Ok(children) = descendent.get(*entity) else {return};
            for child in children {
                commands.entity(*child).insert(focus.clone());
                queue.push((child, focus));
            }
        }
    }
}

/// Remove [`CheckButtonState`].
pub fn remove_check_button_state(mut commands: Commands, 
    query: Query<Entity, With<CheckButtonState>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<CheckButtonState>();
    }
}


/// When attached to a widget in the button family,
/// the submit signal will send the containing data.
/// 
/// # Submit signal behavior:
/// 
/// * Button OnClick: sends `Payload` or `()`.
/// * RadioButton OnClick: sends `Payload` or `()`.
/// * CheckButton OnClick: If `true`, sends `Payload` or `()`.
/// 
/// Also serves as the `RadioButton`'s discriminant.
#[derive(Debug, Clone, PartialEq, Component)] 
pub struct Payload(Object);

impl Payload {
    pub const fn empty() -> Self {
        Self(Object::NONE)
    }

    pub fn new(value: impl DataTransfer + Clone) -> Self {
        Self(Object::new(value))
    }
}


pub trait ConstructRadioButton: Sized {
    fn construct(default: impl DataTransfer) -> Self;
    fn recv<T: DataTransfer>(&self) -> SignalBuilder<T>;
}

impl<const N: usize> ConstructRadioButton for [RadioButton; N] {
    fn construct(default: impl DataTransfer) -> Self {
        let result = RadioButton::new(default);
        core::array::from_fn(|_|result.clone())
    }
    fn recv<T: DataTransfer>(&self) -> SignalBuilder<T> {
        self[0].recv::<T>()
    }
}

macro_rules! radio_button_create {
    ($first: ident) => {
        impl ConstructRadioButton for ($first,) {
            fn construct(default: impl DataTransfer) -> Self {
                (RadioButton::new(default), )
            }
            fn recv<T: DataTransfer>(&self) -> SignalBuilder<T> {
                self.0.recv::<T>()
            }
        }
    };
    ($first: ident, $($receivers: ident),*) => {
        impl ConstructRadioButton for ($($receivers),* , $first) {
            fn construct(default: impl DataTransfer) -> Self {                    
                let result = RadioButton::new(default);
                (
                    $({
                        let btn: $receivers = result.clone();
                        btn
                    },)*
                    result
                )
            }
            fn recv<T: DataTransfer>(&self) -> SignalBuilder<T> {
                self.0.recv::<T>()
            }
        }

        radio_button_create!($($receivers),*);
    };
}

radio_button_create!(RadioButton, 
    RadioButton, RadioButton, RadioButton, RadioButton,
    RadioButton, RadioButton, RadioButton, RadioButton,
    RadioButton, RadioButton, RadioButton, RadioButton
);  

/// Construct an array of shared `RadioButton` contexts. 
/// 
/// # Example
/// ```
/// use bevy_aoui::widgets::button::radio_button_group;
/// let ([he_him, she_her, they_them], gender_sender) = radio_button_group("He/Him");
/// // Construct 4 items as an array.
/// let colors = radio_button_group::<_, 4>("Red");
/// ```
pub fn radio_button_group<T: ConstructRadioButton>(default: impl DataTransfer) -> T {
    T::construct(default)
}

