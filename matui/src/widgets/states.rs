use bevy::{ecs::{component::Component, query::Has, system::Query}, render::color::Color};
use bevy_aoui::{fgsm_interpolation,  Opacity};
use bevy_aoui::anim::{Attr, Fgsm, Rotation};
use bevy_aoui::widgets::button::{CheckButtonState, ToggleChange};
use bevy_aoui::events::{CursorFocus, DescendantHasFocus, EventFlags};
use bevy_defer::SignalReceiver;
use crate::StrokeColoring;

use super::StrokeColors;


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Idle,
    Hover,
    Pressed,
    Disabled,
}

impl Fgsm for ButtonState {
    type State = (&'static Opacity, Option<&'static CursorFocus>);

    fn from_state(state: &<Self::State as bevy::ecs::query::WorldQuery>::Item<'_>) -> Self {
        let (opacity, focus) = state;
        if opacity.disabled {
            return Self::Disabled
        }
        if let Some(focus) = focus {
            if focus.intersects(EventFlags::LeftPressed | EventFlags::LeftDrag) {
                return Self::Pressed
            } else if focus.intersects(EventFlags::Hover) {
                return Self::Hover
            }
        }
        Self::Idle
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleState {
    #[default]
    Unchecked,
    Checked,
    Disabled,
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleFocusState {
    #[default]
    Unchecked,
    Checked,
    UncheckedFocused,
    CheckedFocused,
    Disabled,
}

impl Fgsm for ToggleFocusState {
    type State = (&'static CheckButtonState, Has<DescendantHasFocus>, &'static Opacity);

    fn from_state(state: &<Self::State as bevy::ecs::query::WorldQuery>::Item<'_>) -> Self {
        let (check, has, opacity) = state;
        if opacity.computed_disabled {
            return Self::Disabled
        } 
        match (check, has) {
            (CheckButtonState::Unchecked, true) => Self::UncheckedFocused,
            (CheckButtonState::Unchecked, false) => Self::Unchecked,
            (CheckButtonState::Checked, true) => Self::CheckedFocused,
            (CheckButtonState::Checked, false) => Self::Checked,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CoreToggleState {
    #[default]
    Unchecked,
    Checked,
}
impl Fgsm for CoreToggleState {
    type State = &'static CheckButtonState;

    fn from_state(state: &<Self::State as bevy::ecs::query::WorldQuery>::Item<'_>) -> Self {
        match state {
            CheckButtonState::Unchecked => Self::Unchecked,
            CheckButtonState::Checked => Self::Checked,
        }
    }
}

impl Fgsm for ToggleState {
    type State = (&'static Opacity, &'static CheckButtonState);

    fn from_state(state: &<Self::State as bevy::ecs::query::WorldQuery>::Item<'_>) -> Self {
        let (opacity, check) = state;
        if opacity.disabled {
            Self::Disabled
        } else {
            match check {
                CheckButtonState::Unchecked => Self::Unchecked,
                CheckButtonState::Checked => Self::Checked,
            }
        }
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FocusState {
    #[default]
    Idle,
    Focus,
    Disabled,
}

impl Fgsm for FocusState {
    type State = (&'static Opacity, Has<DescendantHasFocus>);

    fn from_state(state: &<Self::State as bevy::ecs::query::WorldQuery>::Item<'_>) -> Self {
        let (opacity, focus) = state;
        if opacity.disabled {
            Self::Disabled
        } else if *focus {
            Self::Focus
        } else {
            Self::Idle
        }
    }
}

fgsm_interpolation!(
    pub struct FocusColors: FocusState as Color => Color {
        idle: Idle,
        focus: Focus,
        disabled: Disabled,
    }
);


fgsm_interpolation!(
    impl StrokeColors<FocusColors>: FocusState as Color => StrokeColoring {
        idle: Idle,
        focus: Focus,
        disabled: Disabled,
    }
);


fgsm_interpolation!(
    pub struct ButtonColors: ButtonState as Color => Color {
        idle: Idle,
        hover: Hover,
        pressed: Pressed,
        disabled: Disabled,
    }
);


fgsm_interpolation!(
    impl StrokeColors<ButtonColors>: ButtonState as Color => StrokeColoring {
        idle: Idle,
        hover: Hover,
        pressed: Pressed,
        disabled: Disabled,
    }
);

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            idle: Color::NONE,
            hover: Color::NONE,
            pressed: Color::NONE,
            disabled: Color::NONE
        }
    }
}

fgsm_interpolation!(
    pub struct ToggleColors: ToggleState as Color => Color {
        unchecked: Unchecked,
        checked: Checked,
        disabled: Disabled,
    }
);


fgsm_interpolation!(
    impl StrokeColors<ToggleColors>: ToggleState as Color => StrokeColoring {
        unchecked: Unchecked,
        checked: Checked,
        disabled: Disabled,
    }
);

impl Default for ToggleColors {
    fn default() -> Self {
        Self {
            unchecked: Color::NONE,
            checked: Color::NONE,
            disabled: Color::NONE,
        }
    }
}

fgsm_interpolation!(
    #[derive(Debug, Default)]
    pub struct ToggleRotation: CoreToggleState as f32 => Rotation {
        unchecked: Unchecked,
        checked: Checked,
    }
);
impl ToggleRotation {
    pub fn new(unchecked: f32, checked: f32) -> Self {
        Self { unchecked, checked }
    }
}


fgsm_interpolation!(
    #[derive(Debug, Default)]
    pub struct ToggleOpacity: CoreToggleState as f32 => Opacity {
        unchecked: Unchecked,
        checked: Checked,
    }
);
impl ToggleOpacity {
    pub fn new(unchecked: f32, checked: f32) -> Self {
        Self { unchecked, checked }
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct SignalToggleOpacity {
    unchecked: f32,
    checked: f32,
}

impl SignalToggleOpacity {
    pub fn new(unchecked: f32, checked: f32) -> Self {
        Self { unchecked, checked }
    }
}

pub fn toggle_opacity_signal (
    mut query: Query<(&SignalToggleOpacity, Attr<Opacity, Opacity>, SignalReceiver<ToggleChange>)>
) {
    for (opacity, mut attr, recv) in query.iter_mut() {
        if let Some(val) = recv.poll_once() {
            if val {
                attr.set(opacity.checked)
            } else {
                attr.set(opacity.unchecked)
            }
        }
    }
}

fgsm_interpolation!(
    pub struct ToggleFocusColors: ToggleFocusState as Color => Color {
        unchecked: Unchecked,
        unchecked_focused: UncheckedFocused,
        checked: Checked,        
        checked_focused: CheckedFocused,
        disabled: Disabled,
    }
);