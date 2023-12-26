use bevy::{render::{color::Color, texture::Image}, window::CursorIcon, ecs::{component::Component, system::Query}, hierarchy::BuildChildren, math::Vec2};
use bevy_aoui::{widget_extension, build_frame, Hitbox, Dimension, Size2, material_sprite, sprite, size2};
use bevy_aoui::anim::{Interpolate, Easing, Offset, EaseFunction};
use bevy_aoui::events::{EventFlags, Handlers, EvButtonClick, EvToggleChange};
use bevy_aoui::widgets::button::{PropagateFocus, CheckButton, Payload, SetCursor, CheckButtonState};
use bevy_aoui::dsl::{Widget, HandleOrString, OptionX};

use crate::shapes::{RoundedRectangleMaterial, StrokeColor};

use super::util::{OptionM, ShadowInfo, StrokeColors};

#[derive(Debug, Component, Clone, Copy)]
pub struct TogglePalette {
    pub background: Color,
    pub dial: Color,
    pub background_stroke: Color,
    pub dial_stroke: Color,
    pub icon: Color,
}

impl Default for TogglePalette {
    fn default() -> Self {
        Self { 
            background: Color::NONE, 
            dial: Color::NONE, 
            background_stroke: Color::NONE, 
            dial_stroke: Color::NONE, 
            icon: Color::NONE 
        }
    }
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

pub fn toggle_stroke_change(mut query: Query<(&CheckButtonState, &StrokeColors<ToggleColors>, &mut Interpolate<StrokeColor>)>) {
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

        /// The length the dial travels in em, default is 1.25 em.
        pub length: Option<f32>,
        
        pub palette: TogglePalette,
        pub checked_palette: Option<TogglePalette>,
        
        /// Size of the background in em, default is `Full` (evaluates to 2.0 em).
        pub background_size: Option<f32>,
        pub background_texture: HandleOrString<Image>,
        pub background_stroke: f32,

        /// Size of the dial, default is 1.4 em.
        pub dial_size: Option<f32>,
        pub dial_texture: HandleOrString<Image>,
        pub dial_stroke: f32,

        /// Icon of the dial, if `icon_checked` exists, fade out when checked.
        pub icon: HandleOrString<Image>,
        /// Icon of the dial, fade in when checked.
        pub icon_checked: HandleOrString<Image>,
        
        /// Changes the size of dial when checked, in em.
        pub checked_size: Option<f32>,

        /// Shadow for background.
        pub shadow: OptionM<ShadowInfo>,
        /// Shadow for the dial.
        pub dial_shadow: OptionM<ShadowInfo>,
    }
);

impl Widget for MToggleBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut frame = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the AssetServer");

        let unchecked_palette = self.palette;
        let checked_palette = self.checked_palette.unwrap_or(unchecked_palette);
        let TogglePalette { background, dial, background_stroke, dial_stroke, icon: icon_color } = if self.checked {
            checked_palette
        } else {
            unchecked_palette
        };
        
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
            z: 0.01,
            material: RoundedRectangleMaterial::capsule(background)
                .with_stroke((background_stroke, self.background_stroke)),
            extra: ToggleColors {
                inactive: unchecked_palette.background,
                active: checked_palette.background,
            },
            extra: StrokeColors(ToggleColors {
                inactive: unchecked_palette.background_stroke,
                active: checked_palette.background_stroke,
            }),
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                background, 
                0.25
            ),
            extra: Interpolate::<StrokeColor>::new(
                Easing::Linear,
                background_stroke, 
                0.25
            ),
        });
        if let OptionM::Some(shadow) = self.shadow {
            let shadow = shadow.build_capsule(commands, assets);
            commands.entity(background).add_child(shadow);
        }
        commands.entity(frame).add_child(background);
        let dial_size = self.dial_size.unwrap_or(1.4);
        let checked_size = self.checked_size.unwrap_or(dial_size);
        let dial = material_sprite!((commands, assets) {
            offset: Size2::em(0.0, 0.0),
            dimension: Size2::em(dial_size, dial_size),
            z: 0.02,
            material: RoundedRectangleMaterial::capsule(dial)
                .with_stroke((dial_stroke, self.dial_stroke)),
            extra: ToggleColors {
                inactive: unchecked_palette.dial,
                active: checked_palette.dial,
            },
            extra: StrokeColors(ToggleColors {
                inactive: unchecked_palette.dial_stroke,
                active: checked_palette.dial_stroke,
            }),
            extra: ToggleDial { 
                inactive_offset: Vec2::new(-horiz_len / 2.0, 0.0), 
                inactive_dimension: Vec2::new(dial_size, dial_size),
                active_offset: Vec2::new(horiz_len / 2.0, 0.0), 
                active_dimension: Vec2::new(checked_size, checked_size),
            },
            extra: Interpolate::<Color>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                dial, 
                0.25
            ),
            extra: Interpolate::<StrokeColor>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                dial_stroke, 
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
        if let OptionM::Some(shadow) = self.dial_shadow {
            let shadow = shadow.build_capsule(commands, assets);
            commands.entity(dial).add_child(shadow);
        }
        if self.icon.is_some() && self.icon_checked.is_none() {
            let icon = sprite!((commands, assets) {
                sprite: self.icon,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    icon_color, 
                    0.25
                ),
                extra: ToggleColors { 
                    inactive: unchecked_palette.icon, 
                    active: checked_palette.icon,
                }
            });
            commands.entity(dial).add_child(icon);
        } else if self.icon.is_some() {
            let icon = sprite!((commands, assets) {
                sprite: self.icon,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    if self.checked { Color::NONE } else { unchecked_palette.icon }, 
                    0.25
                ),
                extra: ToggleColors { 
                    inactive: unchecked_palette.icon, 
                    active: Color::NONE,
                }
            });
            commands.entity(dial).add_child(icon);
        } 
        if self.icon_checked.is_some() {
            let icon = sprite!((commands, assets) {
                sprite: self.icon_checked,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    if !self.checked { Color::NONE } else { checked_palette.icon }, 
                    0.25
                ),
                extra: ToggleColors { 
                    inactive: Color::NONE,
                    active: checked_palette.icon,
                }
            });
            commands.entity(dial).add_child(icon);
        }
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