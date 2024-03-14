use std::marker::PhantomData;

/// Defines standard signals for `bevy_rectray`, and component that directly interacts with them.
/// 
/// Libraries should avoid using async systems, directly polling signals is advisable here.

use bevy::{ecs::{component::Component, query::{With, Without}, system::Query}, text::Text};

use bevy_defer::{signal_ids, AsObject, Object};
use bevy_defer::signals::{SignalId, SignalReceiver};

use super::{button::RadioButton, inputbox::InputBox, TextFragment};

mod sealed {
    pub enum Sealed{}
}

/// A standard signal with generic value `T`.
pub enum Fac<T> {
    __Sealed(PhantomData<T>, sealed::Sealed)
}

impl<T: AsObject + Default> SignalId for Fac<T> {
    type Data = T;
}

signal_ids!(
    /// A standard signal id with type `String`.
    pub FormatText: String,
    /// A standard signal id with type `&'static str`.
    pub FormatTextStatic: &'static str,
    /// A standard signal id for generic button output.
    pub Invocation: Object,
    /// A standard signal that removes data from a widget.
    pub ClearWidget: Object,
);

/// Uses signal `SetText` fot setting Text.
#[derive(Debug, Clone, Copy, Default, Component)]
#[component(storage="SparseSet")]
pub struct TextFromSignal;

/// Uses signal `ClearWidget` fot clearing Text.
#[derive(Debug, Clone, Copy, Default, Component)]
#[component(storage="SparseSet")]
pub struct ClearTextFromSignal;

pub(crate) fn sig_set_text(
    mut q: Query<(SignalReceiver<FormatText>, Option<&mut TextFragment>, Option<&mut Text>, Option<&mut InputBox>), With<TextFromSignal>>) {
    for (recv, frag, text, input) in q.iter_mut() {
        if let Some(str) = recv.poll_once() {
            if let Some(mut frag) = frag {
                frag.text = str
            } else if let Some(mut input) = input {
                input.set(str)
            } else if let Some(mut text) = text {
                if let Some(section) = text.sections.first_mut() {
                    section.value = str;
                }
            }
        }
    }
}

pub(crate) fn inputbox_clear_widget(
    mut q: Query<(SignalReceiver<ClearWidget>, &mut InputBox)>
) {
    for (recv, mut input) in q.iter_mut() {
        if recv.poll_once().is_some() {
            input.clear()
        }
    }
}

pub(crate) fn text_clear_widget(
    mut q1: Query<(SignalReceiver<ClearWidget>, &mut TextFragment), With<ClearTextFromSignal>>,
    mut q2: Query<(SignalReceiver<ClearWidget>, &mut Text), (With<ClearTextFromSignal>, Without<TextFragment>)>,
) {
    for (recv, mut input) in q1.iter_mut() {
        if recv.poll_once().is_some() {
            input.text.clear();
        }
    }
    for (recv, mut input) in q2.iter_mut() {
        if recv.poll_once().is_some() {
            for sec in &mut input.sections {
                sec.value.clear()
            }
        }
    }
}


pub(crate) fn radio_button_clear_widget(
    mut q: Query<(SignalReceiver<ClearWidget>, &RadioButton)>
) {
    for (recv, radio) in q.iter_mut() {
        if recv.poll_once().is_some() {
            let mut lock = radio.storage.lock();
            *lock = Object::NONE;
        }
    }
}
