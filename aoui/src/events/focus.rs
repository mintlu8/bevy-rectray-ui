use bevy::{ecs::{component::Component, query::Has, system::{Query, Res}}, input::{mouse::MouseButton, Input}};

use bevy_defer::{SignalId, SignalSender};

use super::{CursorClickOutside, DescendantHasFocus};

/// Tracks when this entity obtain and lose focus, operates signals 
/// `ObtainedFocus`, `LoseFocus` and `FocusChange`.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub enum FocusStateMachine{
    NoFocus,
    Focus
}

/// Tracks when this entity obtains and lose focus, operates signals 
/// `ObtainedFocus`, `LoseFocus` and `FocusChange`.
/// 
/// Obtain `StrongFocus` when clicked, which can only be cancelled by clicking outside.
/// Requires `EventFlags` `ClickOutside`.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub enum StrongFocusStateMachine{
    NoFocus,
    Weak,
    Strong
}

/// Signal for obtaining `DescendantHasFocus`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObtainedFocus {}
/// Signal for losing `DescendantHasFocus`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoseFocus {}
/// Signal for `DescendantHasFocus` being added or removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusChange {}

impl SignalId for ObtainedFocus {
    type Data = ();
}

impl SignalId for LoseFocus {
    type Data = ();
}

impl SignalId for FocusChange {
    type Data = bool;
}

pub(crate) fn run_focus_signals(
    mut query: Query<(&mut FocusStateMachine, 
        SignalSender<ObtainedFocus>, 
        SignalSender<LoseFocus>, 
        SignalSender<FocusChange>, 
        Has<DescendantHasFocus>)>
) {
    for (mut focus, obtain, lose, change, has) in query.iter_mut() {
        let new = if has {FocusStateMachine::Focus} else {FocusStateMachine::NoFocus};
        if focus.as_ref() != &new {
            *focus = new;
            change.send(has);
            if has {
                obtain.send(())
            } else {
                lose.send(())
            }
        }
    }
}

pub(crate) fn run_strong_focus_signals(
    state: Res<Input<MouseButton>>,
    mut query: Query<(&mut StrongFocusStateMachine, 
        SignalSender<ObtainedFocus>, 
        SignalSender<LoseFocus>, 
        SignalSender<FocusChange>, 
        Option<&CursorClickOutside>,
        Has<DescendantHasFocus>)>
) {
    for (mut focus, obtain, lose, change, outside, has) in query.iter_mut() {
        let new =  if outside.is_some() {
            StrongFocusStateMachine::NoFocus
        } else if has && state.any_pressed([MouseButton::Left, MouseButton::Middle, MouseButton::Right]) 
                || focus.as_ref() == &StrongFocusStateMachine::Strong {
            StrongFocusStateMachine::Strong
        } else if has {
            StrongFocusStateMachine::Weak
        } else {
            StrongFocusStateMachine::NoFocus
        };
        if focus.as_ref() != &new {
            *focus = new;
            change.send(has);
            if new != StrongFocusStateMachine::NoFocus {
                obtain.send(())
            } else {
                lose.send(())
            }
        }
    }
}