use std::{sync::{Mutex, Arc}, ops::Deref};

use bevy::{render::view::Visibility, window::{Window, PrimaryWindow, CursorIcon}, hierarchy::Children, ecs::entity::Entity};
use bevy::ecs::{system::{Query, Resource, Res, Commands}, component::Component, query::With};
use crate::{Opacity, signals::Object, dsl::prelude::{SigSubmit, SigChange, signal, Receiver}};

use crate::{events::{EventFlags, CursorFocus, CursorAction}, signals::DataTransfer, dsl::prelude::Interpolate, signals::Sender};


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
                opacity.interpolate_to_or_reverse(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(0.0);
            } else {
                *vis = Visibility::Hidden;
            }
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
#[derive(Debug, Component)]
pub struct RadioButton(Arc<Mutex<Object>>, Sender);

impl Clone for RadioButton {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.fork())
    }
}

impl RadioButton {
    pub fn new(default: impl DataTransfer) -> (Self, Receiver) {
        let (send, recv) = signal();
        (
            RadioButton(Arc::new(Mutex::new(Object::new(default))), send),
            recv
        )
    }

    pub fn set(&self, payload: &Payload) {
        let mut lock = self.0.lock().unwrap();
        *lock = payload.0.clone();
        self.1.send_object(payload.0.clone())
    }

    pub fn get<T: DataTransfer>(&self) -> Option<T> {
        self.0.lock().unwrap().get()
    }

    pub fn fork_signal<T: DataTransfer>(&self) -> Receiver {
        self.1.get_receiver()
    }
}

impl PartialEq<Payload> for RadioButton {
    fn eq(&self, other: &Payload) -> bool {
        let lock = self.0.lock().unwrap();
        lock.deref() == &other.0
    }
}

pub fn button_on_click(
    query: Query<(&CursorAction, &Sender<SigSubmit>, Option<&Payload>), With<Button>>
) {
    for (action, submit, payload) in query.iter() {
        if !action.is(EventFlags::Click) { continue }
        if let Some(payload) = payload {
            submit.send_object(payload.0.clone());
        } else {
            submit.send_empty()
        }
    }
}

pub fn check_button_on_click(
    mut query: Query<(&CursorAction, &mut CheckButton, Option<&Sender<SigChange>>, Option<&Sender<SigSubmit>>, Option<&Payload>)>
) {
    for (action, mut state, change, submit, payload) in query.iter_mut() {
        if !action.is(EventFlags::Click) { continue }
        let state = state.rev();
        if let Some(signal) = change {
            signal.send(state)
        }
        if !state {continue;}
        if let Some(signal) = submit {
            if let Some(payload) = payload {
                signal.send_object(payload.0.clone());
            } else {
                signal.send_empty()
            }
        }
    }
}

pub fn radio_button_on_click(
    mut query: Query<(&CursorAction, &RadioButton, &Payload, Option<&Sender<SigSubmit>>)>
) {
    for (action, state, payload, submit) in query.iter_mut() {
        if !action.is(EventFlags::Click) { continue }
        state.set(payload);
        if let Some(signal) = submit {
            signal.send_object(payload.0.clone());
        }
    }
}

pub fn check_conditional_visibility(
    mut query: Query<(&DisplayIf<CheckButtonState>, &CheckButtonState, &mut Visibility, Option<&mut Interpolate<Opacity>>)>
) {
    query.par_iter_mut().for_each(|(display_if, state, mut vis, mut opacity)| {
        if &display_if.0 == state {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(0.0);
            } else {
                *vis = Visibility::Hidden;
            }
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


/// Marker component for passing `CursorFocus`, 
/// `CursorAction` and `CheckButtonState` to their children.
/// 
/// Does **not** propagate through hierarchy if chained.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct PropagateFocus;

/// Propagate [`CursorFocus`] and [`CursorAction`] down to children.
pub fn propagate_focus(mut commands: Commands, 
    query1: Query<(&CursorFocus, &Children), With<PropagateFocus>>, 
    query2: Query<(&CursorAction, &Children), With<PropagateFocus>>,
    query3: Query<(&CheckButton, &Children), With<PropagateFocus>>,
    query4: Query<(&RadioButton, &Payload, &Children), With<PropagateFocus>>,
) {
    for (focus, children) in query1.iter() {
        for child in children {
            commands.entity(*child).insert(*focus);
        }
    }
    for (focus, children) in query2.iter() {
        for child in children {
            commands.entity(*child).insert(*focus);
        }
    }
    for (focus, children) in query3.iter() {
        for child in children {
            let check: CheckButtonState = focus.get().into();
            commands.entity(*child).insert(check);
        }
    }
    for (radio, payload, children) in query4.iter() {
        for child in children {
            let check: CheckButtonState = (radio == payload).into();
            commands.entity(*child).insert(check);
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

mod sealed {
    use std::usize;

    use super::RadioButton;

    use crate::signals::{DataTransfer, Receiver};

    pub trait ConstructRadioButtonSignal<const N: usize>: Sized {
        fn construct(default: impl DataTransfer) -> Self;
    }

    impl<const N: usize> ConstructRadioButtonSignal<N> for [RadioButton; N] {
        fn construct(default: impl DataTransfer) -> Self {
            let (result, _) = RadioButton::new(default);
            array_init::array_init(|_|result.clone())
        }
    }

    impl<const N: usize> ConstructRadioButtonSignal<N> for ([RadioButton; N], Receiver) {
        fn construct(default: impl DataTransfer) -> Self {
            let (result, recv) = RadioButton::new(default);
            (array_init::array_init(|_|result.clone()), recv)
        }
    }
    
}

use sealed::ConstructRadioButtonSignal;    

/// Construct a tuple of shared `RadioButton` contexts. 
/// 
/// # Example
/// ```
/// let (he_him, she_her, they_them) = radio_button_group();
/// ```
pub fn radio_button_group<T: ConstructRadioButtonSignal<N>, const N: usize>(default: impl DataTransfer) -> T {
    T::construct(default)
}

