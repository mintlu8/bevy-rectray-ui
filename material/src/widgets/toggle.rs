use bevy::{render::{color::Color, texture::Image}, window::CursorIcon, ecs::{component::Component, system::Query}, hierarchy::BuildChildren, math::Vec2};
use bevy_aoui::{widget_extension, build_frame, Hitbox, Dimension, Size2, material_sprite};
use bevy_aoui::anim::{Interpolate, Easing, Offset, EaseFunction};
use bevy_aoui::events::{EventFlags, Handlers, EvButtonClick, EvToggleChange};
use bevy_aoui::widgets::button::{PropagateFocus, CheckButton, Payload, SetCursor, CheckButtonState};
use bevy_aoui::dsl::{Widget, HandleOrString, OptionX};

use crate::{shapes::CapsuleMaterial, builders::Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MToggleStyle {
    #[default]
    Linear,
    /// Like the m3 specification, 
    /// expand, move, shrink.
    Expand,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct ToggleColors {
    active: Color,
    inactive: Color,
}

pub fn toggle_color_change(mut query: Query<(&CheckButtonState, &ToggleColors, &mut Interpolate<Color>)>) {
    query.par_iter_mut().for_each(|(check, colors, mut color)| {
        match check {
            CheckButtonState::Checked => color.interpolate_to(colors.active),
            CheckButtonState::Unchecked => color.interpolate_to(colors.inactive),
        }
    })
}


#[derive(Debug, Clone, Copy, Component)]
pub struct ToggleDial {
    active_offset: Vec2,
    active_dimension: Vec2,
    inactive_offset: Vec2,
    inactive_dimension: Vec2,
}

pub fn toggle_dial_change(mut query: Query<(&CheckButtonState, &ToggleDial, &mut Interpolate<Offset>, &mut Interpolate<Dimension>)>) {
    query.par_iter_mut().for_each(|(check, dial, mut offset, mut dimension)| {
        match check {
            CheckButtonState::Checked => {
                offset.interpolate_to(dial.active_offset);
                dimension.interpolate_to(dial.active_dimension);
            },
            CheckButtonState::Unchecked => {
                offset.interpolate_to(dial.inactive_offset);
                dimension.interpolate_to(dial.inactive_dimension);
            },
        }
    })
}

widget_extension!(
    pub struct MToggleBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked and its value is `true`.
        /// 
        /// Like button, this sends either `()` or `Payload`.
        pub on_checked: Handlers<EvButtonClick>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub on_toggle: Handlers<EvToggleChange>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,

        pub length: Option<f32>,
        pub toggle_style: MToggleStyle,
        /// For M2 style, background is small, foreground is large.
        /// For M3 style, background is large, foreground is small.
        pub background_size: Option<f32>,
        pub background_color: Option<Color>,
        pub background_active: Option<Color>,
        pub background_texture: HandleOrString<Image>,
        pub background_stroke: Stroke,

        pub dial_color: Option<Color>,
        pub dial_size: Option<f32>,
        pub dial_texture: HandleOrString<Image>,
        pub dial_stroke: Stroke,

        pub icon: HandleOrString<Image>,
        
        pub checked_size: Option<f32>,
        pub checked_color: Option<Color>,
        pub checked_pressed: Option<Color>,

        pub background_shadow: Option<f32>,
        pub dial_shadow: Option<f32>,
    }
);

impl Widget for MToggleBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut frame = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the AssetServer");

        let background = self.background_color.unwrap_or(if self.background_texture.is_some() {
            Color::WHITE
        } else {
            Color::NONE
        });

        let dial_color = self.dial_color.unwrap_or(if self.dial_texture.is_some() {
            Color::WHITE
        } else {
            Color::NONE
        });
        
        let horiz_len = self.length.unwrap_or(1.25);
        frame.insert((
            Dimension::owned(Size2::em(2.0 + horiz_len, 2.0)),
            PropagateFocus,
            CheckButton::from(self.checked),
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
        ));
        if self.hitbox.is_none() {
            frame.insert(Hitbox::FULL);
        }
        if !self.on_checked.is_empty()  {
            frame.insert(self.on_checked);
        }
        if !self.on_toggle.is_empty()  {
            frame.insert(self.on_toggle);
        }
        if let OptionX::Some(payload) = self.payload  {
            frame.insert(payload);
        };
        let frame = frame.id();

        let size = self.background_size.map(|x| Size2::em(x + horiz_len, x))
            .unwrap_or(Size2::FULL);
        let background = material_sprite!((commands, assets) {
            dimension: size,
            material: CapsuleMaterial::new(background)
                .with_stroke(self.background_stroke),
            extra: ToggleColors {
                inactive: self.background_color.unwrap_or(Color::GRAY),
                active: self.background_active.unwrap_or(Color::GRAY),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                background, 
                0.25
            ),
        });
        commands.entity(frame).add_child(background);
        let dial_size = self.dial_size.unwrap_or(1.4);
        let checked_size = self.checked_size.unwrap_or(dial_size);
        let dial = material_sprite!((commands, assets) {
            offset: Size2::em(0.0, 0.0),
            dimension: Size2::em(dial_size, dial_size),
            z: 0.01,
            material: CapsuleMaterial::new(dial_color)
                .with_stroke(self.dial_stroke),
            extra: ToggleColors {
                inactive: dial_color,
                active: self.checked_color.unwrap_or(dial_color),
            },
            extra: ToggleDial { 
                inactive_offset: Vec2::new(-horiz_len / 2.0, 0.0), 
                inactive_dimension: Vec2::new(dial_size, dial_size),
                active_offset: Vec2::new(horiz_len / 2.0, 0.0), 
                active_dimension: Vec2::new(checked_size, checked_size),
            },
            extra: Interpolate::<Color>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                dial_color, 
                0.25
            ),
            extra: Interpolate::<Offset>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                if self.checked {
                    Vec2::new(horiz_len / 2.0, 0.0)
                } else {
                    Vec2::new(-horiz_len / 2.0, 0.0)
                },
                0.25
            ),
            extra: Interpolate::<Dimension>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                if self.checked {
                    Vec2::new(checked_size, checked_size)
                } else {
                    Vec2::new(dial_size, dial_size)
                },
                0.25
            ),
        });
        commands.entity(frame).add_child(dial);
        (frame, frame)
    }
}


#[macro_export]
macro_rules! mtoggle {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MToggleBuilder] {
            $($tt)*
        })
    };
}