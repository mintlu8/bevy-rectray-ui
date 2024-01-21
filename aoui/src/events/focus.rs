use bevy::ecs::{component::Component, query::Has, system::Query};

use crate::sync::{SignalId, SignalSender};

use super::DescendantHasFocus;


#[derive(Debug, Clone, Copy, Component)]
pub struct FocusStateMachine(bool);

impl FocusStateMachine {
    pub fn new() -> FocusStateMachine {
        Self(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObtainedFocus {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoseFocus {}
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

pub fn run_focus_signals(
    mut query: Query<(&mut FocusStateMachine, 
        SignalSender<ObtainedFocus>, 
        SignalSender<LoseFocus>, 
        SignalSender<FocusChange>, 
        Has<DescendantHasFocus>)>
) {
    for (mut focus, obtain, lose, change, has) in query.iter_mut() {
        if has != focus.0 {
            focus.0 = has;
            change.send(has);
            if has {
                obtain.send(())
            } else {
                lose.send(())
            }
        }
    }
}