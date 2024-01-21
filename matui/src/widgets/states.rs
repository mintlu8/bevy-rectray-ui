use bevy::render::color::Color;
use bevy_aoui::{anim::Fgsm, events::{CursorFocus, EventFlags}, fgsm_interpolation, widgets::button::CheckButtonState, Opacity};

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
            if focus.intersects(EventFlags::LeftPressed) {
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
            return Self::Disabled;
        } else {
            match check {
                CheckButtonState::Unchecked => Self::Unchecked,
                CheckButtonState::Checked => Self::Checked,
            }
        }
    }
}

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