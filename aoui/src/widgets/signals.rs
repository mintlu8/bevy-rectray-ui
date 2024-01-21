/// Defines standard signals for `bevy_aoui`, and component that directly interacts with them.
/// 
/// Libraries should avoid using async systems, directly polling signals is advisable here.

use bevy::{ecs::{component::Component, query::With, system::Query}, text::Text};

use crate::{signal_ids, sync::SignalReceiver, util::Object};

use super::{inputbox::InputBox, TextFragment};

signal_ids!(
    /// A standard signal id with type `String`.
    FormatText: String,
    /// A standard signal id with type `&'static str`.
    FormatTextStatic: &'static str,
    /// A standard signal id for generic button output.
    Invocation: Object
);

/// Uses signal `SetText` fot setting Text.
#[derive(Debug, Clone, Copy, Default, Component)]
pub struct TextFromSignal;

pub(crate) fn sig_set_text(
    mut q: Query<(SignalReceiver<FormatText>, Option<&mut TextFragment>, Option<&mut Text>, Option<&mut InputBox>), With<TextFromSignal>>) {
    for (mut recv, frag, text, input) in q.iter_mut() {
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