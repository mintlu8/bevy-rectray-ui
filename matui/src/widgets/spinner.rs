use bevy::ecs::entity::Entity;
use bevy::ecs::query::Has;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, text::Font};
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::util::Object;
use bevy_aoui::widgets::spinbox::{SpinnerText, Decrement, Increment};
use bevy_aoui::{Opacity, button, Size2};
use bevy_aoui::layout::{LayoutRange, Axis};
use bevy_aoui::{frame_extension, build_frame, Hitbox, size2, frame, text, layout::{Container, StackLayout}, sprite};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, Handlers, DescendantHasFocus, EvSpinChange, EvTextChange};
use bevy_aoui::widgets::util::PropagateFocus;
use bevy_aoui::util::{Widget, AouiCommands, convert::{OptionEx, IntoAsset}};
use crate::shaders::{RoundedRectangleMaterial, StrokeColoring};
use crate::style::Palette;

use super::util::{ShadowInfo, StrokeColors};

/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct FocusColors {
    pub idle: Color,
    pub focus: Color,
    pub disabled: Color,
}

impl Default for FocusColors {
    fn default() -> Self {
        Self {
            idle: Color::NONE,
            focus: Color::NONE,
            disabled: Color::NONE
        }
    }
}

pub fn cursor_color_change(mut query: Query<(&FocusColors, &Opacity, Has<DescendantHasFocus>, &mut Interpolate<Color>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
        } else if focus {
            color.interpolate_to(colors.focus);
        } else {
            color.interpolate_to(colors.idle);
        }
    })
}


pub fn cursor_stroke_change(mut query: Query<(&StrokeColors<FocusColors>, &Opacity, Has<DescendantHasFocus>, &mut Interpolate<StrokeColoring>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
        } else if focus {
            color.interpolate_to(colors.focus);
        } else {
            color.interpolate_to(colors.idle);
        }
    })
}

frame_extension!(
    pub struct MSpinnerBuilder {
        pub axis: Axis,
        pub cursor: Option<CursorIcon>,
        pub decrement_icon: IntoAsset<Image>,
        pub increment_icon: IntoAsset<Image>,
        /// This will set `color_pressed` if its not set
        pub palette: Palette,
        pub palette_focus: Option<Palette>,
        pub palette_disabled: Option<Palette>,
        
        pub texture: IntoAsset<Image>,
        pub stroke: f32,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionEx<ShadowInfo>,

        pub width: Option<f32>,

        pub content: SpinnerText,
        pub selected: Object,
        
        pub font: IntoAsset<Font>,
        pub signal: Handlers<EvSpinChange>,
        pub text_signal: Handlers<EvTextChange>,
    }
);

impl Widget for MSpinnerBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.event |= EventFlags::LeftClick | EventFlags::Hover;
        let mut frame = build_frame!(commands, self);

        let palette = self.palette;
        let focus_palette = self.palette_focus.unwrap_or(palette);
        let disabled_palette = self.palette_disabled.unwrap_or(palette);

        frame.insert((
            PropagateFocus,
            Container {
                layout: match self.axis {
                    Axis::Horizontal => StackLayout::HSTACK.into(),
                    Axis::Vertical => StackLayout::VSTACK.into(),
                },
                margin: size2!(0.5 em, 0.5 em),
                padding:  match self.axis {
                    Axis::Horizontal => size2!(1 em, 0.75 em),
                    Axis::Vertical => size2!(0.75 em, 0.75 em),
                },
                range: LayoutRange::All,
                maximum: usize::MAX
            },
            FocusColors {
                idle: palette.background(),
                focus: focus_palette.background(),
                disabled: disabled_palette.background(),
            },
            StrokeColors(FocusColors {
                idle: palette.background(),
                focus: focus_palette.background(),
                disabled: disabled_palette.background(),
            }),
            Interpolate::<Color>::new(
                Easing::Linear,
                palette.background(),
                0.15
            ),
            Interpolate::<StrokeColoring>::new(
                Easing::Linear,
                palette.stroke(),
                0.15
            ),
        ));
        if self.hitbox.is_none() {
            frame.insert(Hitbox::FULL);
        }
        let frame = frame.id();

        let (decr_send, decr_recv) = commands.signal();
        let (incr_send, incr_recv) = commands.signal();

        let left = button!(commands{
            dimension: size2!(1.2 em, 1.2 em),
            on_click: decr_send,
            extra: FocusColors {
                idle: palette.foreground(),
                focus: focus_palette.foreground(),
                disabled: disabled_palette.foreground(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                palette.foreground(),
                0.15
            ),
            child: sprite! {
                dimension: Size2::FULL,  
                sprite: self.decrement_icon,
            },
            z: 0.01,
        });

        let right = button!(commands{
            dimension: size2!(1.2 em, 1.2 em),
            on_click: incr_send,
            extra: FocusColors {
                idle: palette.foreground(),
                focus: focus_palette.foreground(),
                disabled: disabled_palette.foreground(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                palette.foreground(),
                0.15
            ),
            child: sprite! {
                dimension: Size2::FULL,  
                sprite: self.increment_icon,
            },
            z: 0.01,
        });

        self.content.select(self.selected);
        
        let text = frame!(commands {
            dimension: size2!({self.width.unwrap_or(4.0)} em, 1 em),
            child: text! {
                text: self.content.get(),
                z: 0.01,
                font: commands.load_or_default(self.font),
                extra: self.content,
                extra: FocusColors {
                    idle: palette.foreground(),
                    focus: focus_palette.foreground(),
                    disabled: disabled_palette.foreground(),
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    palette.foreground(),
                    0.15
                ),
                extra: decr_recv.invoke::<Decrement>(),
                extra: incr_recv.invoke::<Increment>(),
                extra: self.signal,
                extra: self.text_signal,
            }
        });
        commands.entity(frame).add_child(left);
        commands.entity(frame).add_child(text);
        commands.entity(frame).add_child(right);

        crate::build_shape!(commands, self, frame);
        (frame, frame)
    }
}

#[macro_export]
macro_rules! build_shape {
    ($commands: expr, $this: expr, $at: expr) => {
        match ($this.capsule, $this.radius) {
            (true, ..) => {
                let mat = if let Some(im) = $commands.try_load($this.texture) {
                    RoundedRectangleMaterial::capsule_image(im, $this.palette.background())
                } else {
                    RoundedRectangleMaterial::capsule($this.palette.background())
                }.with_stroke(($this.stroke, $this.palette.stroke())).into_bundle($commands);
                $commands.entity($at).insert(mat);
                if let OptionEx::Some(shadow) = $this.shadow {
                    let shadow = shadow.build_capsule($commands);
                    $commands.entity($at).add_child(shadow);
                }
            },
            (_, radius) => {
                let mat = if let Some(im) = $commands.try_load($this.texture) {
                    RoundedRectangleMaterial::from_image(im, $this.palette.background(), radius)
                } else {
                    RoundedRectangleMaterial::new($this.palette.background(), radius)
                }.with_stroke(($this.stroke, $this.palette.stroke())).into_bundle($commands);
                $commands.entity($at).insert(mat);
                if let OptionEx::Some(shadow) = $this.shadow {
                    let shadow = shadow.build_rect($commands, radius);
                    $commands.entity($at).add_child(shadow);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! mspinner {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MSpinnerBuilder] {
            $($tt)*
        })
    };
}
