use bevy::{window::CursorIcon, hierarchy::BuildChildren, math::Vec2};
use bevy::render::{color::Color, texture::Image};
use bevy::ecs::entity::Entity;
use bevy_defer::{TypedSignal, Object};
use bevy_aoui::util::ComposeExtension;
use bevy_aoui::{build_frame, fgsm_interpolation, frame_extension, size2, sprite, Dimension, Hitbox, Size2};
use bevy_aoui::util::{AouiCommands, Widget, convert::{OptionEx, IntoAsset}};
use bevy_aoui::anim::{Interpolate, Easing, Offset, EaseFunction};
use bevy_aoui::events::EventFlags;
use bevy_aoui::widgets::button::{ButtonClick, CheckButton, Payload, ToggleChange};
use bevy_aoui::widgets::util::{PropagateFocus, SetCursor};

use crate::widgets::states::ToggleColors;
use crate::{shaders::{RoundedRectangleMaterial, StrokeColoring}, style::Palette};

use super::states::CoreToggleState;
use super::util::{ShadowInfo, StrokeColors};


fgsm_interpolation!(
    pub struct ToggleDialOffset: CoreToggleState as Vec2 => Offset {
        unchecked: Unchecked,
        checked: Checked,
    }
);


fgsm_interpolation!(
    pub struct ToggleDialDimension: CoreToggleState as Vec2 => Dimension {
        unchecked: Unchecked,
        checked: Checked,
    }
);

frame_extension!(
    pub struct MToggleBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// If set, `submit` sends its contents.
        pub payload: Option<Payload>,
        /// Sends a signal whenever the button is clicked and its value is `true`.
        ///
        /// Like button, this sends either `()` or `Payload`.
        pub on_checked: Option<TypedSignal<Object>>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub on_toggle: Option<TypedSignal<bool>>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,

        /// The length the dial travels in em, default is 1.25 em.
        pub length: Option<f32>,

        pub palette: Palette,
        pub checked_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,

        /// Size of the background in em, default is `Full` (evaluates to 2.0 em).
        pub background_size: Option<f32>,
        pub background_texture: IntoAsset<Image>,
        pub background_stroke: f32,

        /// Size of the dial, default is 1.4 em.
        pub dial_size: Option<f32>,
        pub dial_texture: IntoAsset<Image>,
        pub dial_stroke: f32,

        /// Icon of the dial, if `icon_checked` exists, fade out when checked.
        pub icon: IntoAsset<Image>,
        /// Icon of the dial, fade in when checked.
        pub icon_checked: IntoAsset<Image>,

        /// Changes the size of dial when checked, in em.
        pub checked_size: Option<f32>,

        /// Shadow for background.
        pub shadow: OptionEx<ShadowInfo>,
        /// Shadow for the dial.
        pub dial_shadow: OptionEx<ShadowInfo>,
    }
);

impl Widget for MToggleBuilder {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let mut frame = build_frame!(commands, self);

        let unchecked_palette = self.palette;
        let checked_palette = self.checked_palette.unwrap_or(unchecked_palette);
        let disabled_palette = self.disabled_palette.unwrap_or(unchecked_palette);
        let active_palette = if self.checked {
            checked_palette
        } else {
            unchecked_palette
        };

        let horiz_len = self.length.unwrap_or(1.25);
        frame.insert((
            Dimension::owned(Size2::em(2.0 + horiz_len, 2.0)),
            PropagateFocus,
            CheckButton::from(self.checked),
            self.event | EventFlags::LeftClick | EventFlags::Hover,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
        ));
        if self.hitbox.is_none() {
            frame.insert(Hitbox::FULL);
        }
        if let Some(on_checked) = self.on_checked {
            frame.add_sender::<ButtonClick>(on_checked);
        }
        if let Some(on_toggle) = self.on_toggle {
            frame.add_sender::<ToggleChange>(on_toggle);
        }
        if let Some(payload) = self.payload  {
            frame.insert(payload);
        };
        let frame = frame.id();

        let size = self.background_size.map(|x| Size2::em(x + horiz_len, x))
            .unwrap_or(Size2::FULL);
        let background = bevy_aoui::frame!(commands {
            dimension: size,
            z: 0.01,
            extra: RoundedRectangleMaterial::capsule(active_palette.background())
                .with_stroke((active_palette.stroke(), self.background_stroke))
                .into_bundle(commands),
            extra: ToggleColors {
                unchecked: unchecked_palette.background(),
                checked: checked_palette.background(),
                disabled: disabled_palette.background(),
            },
            extra: StrokeColors(ToggleColors {
                unchecked: unchecked_palette.stroke(),
                checked: checked_palette.stroke(),
                disabled: disabled_palette.stroke(),
            }),
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                active_palette.background(),
                0.25
            ),
            extra: Interpolate::<StrokeColoring>::new(
                Easing::Linear,
                active_palette.stroke(),
                0.25
            ),
        });
        if let OptionEx::Some(shadow) = self.shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(background).add_child(shadow);
        }
        commands.entity(frame).add_child(background);
        let dial_size = self.dial_size.unwrap_or(1.4);
        let checked_size = self.checked_size.unwrap_or(dial_size);
        let dial = bevy_aoui::frame!(commands {
            offset: Size2::em(0.0, 0.0),
            dimension: Size2::em(dial_size, dial_size),
            z: 0.02,
            extra: RoundedRectangleMaterial::capsule(active_palette.foreground())
                .with_stroke((active_palette.foreground_stroke(), self.dial_stroke))
                .into_bundle(commands),
            extra: ToggleColors {
                unchecked: unchecked_palette.foreground(),
                checked: checked_palette.foreground(),
                disabled: disabled_palette.foreground(),
            },
            extra: StrokeColors(ToggleColors {
                unchecked: unchecked_palette.foreground_stroke(),
                checked: checked_palette.foreground_stroke(),
                disabled: disabled_palette.foreground_stroke(),
            }),
            extra: ToggleDialOffset { 
                unchecked: Vec2::new(-horiz_len / 2.0, 0.0), 
                checked: Vec2::new(horiz_len / 2.0, 0.0) 
            },
            extra: ToggleDialDimension { 
                unchecked: Vec2::new(dial_size, dial_size),
                checked: Vec2::new(checked_size, checked_size),
            },
            extra: Interpolate::<Color>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                active_palette.foreground(),
                0.25
            ),
            extra: Interpolate::<StrokeColoring>::new(
                Easing::Ease(EaseFunction::CubicInOut),
                active_palette.foreground_stroke(),
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
        if let OptionEx::Some(shadow) = self.dial_shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(dial).add_child(shadow);
        }
        if self.icon.is_some() && self.icon_checked.is_none() {
            let icon = sprite!(commands {
                sprite: self.icon,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    active_palette.on_foreground(),
                    0.25
                ),
                extra: ToggleColors {
                    unchecked: unchecked_palette.on_foreground(),
                    checked: checked_palette.on_foreground(),
                    disabled: disabled_palette.on_foreground(),
                }
            });
            commands.entity(dial).add_child(icon);
        } else if self.icon.is_some() {
            let icon = sprite!(commands {
                sprite: self.icon,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    if self.checked { Color::NONE } else { unchecked_palette.on_foreground() },
                    0.25
                ),
                extra: if self.icon_checked.is_none() {
                    ToggleColors {
                        unchecked: unchecked_palette.on_foreground(),
                        checked: checked_palette.on_foreground(),
                        disabled: disabled_palette.on_foreground(),
                    }
                } else {
                    ToggleColors {
                        unchecked: unchecked_palette.on_foreground(),
                        checked: Color::NONE,
                        disabled: Color::NONE,
                    }
                }
            });
            commands.entity(dial).add_child(icon);
        }
        if self.icon_checked.is_some() {
            let icon = sprite!(commands {
                sprite: self.icon_checked,
                dimension: size2!(66.6%, 66.6%),
                extra: Interpolate::<Color>::new(
                    Easing::Ease(EaseFunction::CubicInOut),
                    if !self.checked { Color::NONE } else { checked_palette.on_foreground() },
                    0.25
                ),
                extra: ToggleColors {
                    unchecked: Color::NONE,
                    checked: checked_palette.on_foreground(),
                    disabled: Color::NONE,
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
