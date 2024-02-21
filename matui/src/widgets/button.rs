use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, text::Font};
use bevy::window::CursorIcon;
use bevy_defer::{TypedSignal, Object};
use bevy_rectray::util::ComposeExtension;
use bevy_rectray::layout::LayoutRange;
use bevy_rectray::{build_frame, size2, text, layout::{Container, StackLayout}, sprite};
use bevy_rectray::anim::{Interpolate, Easing};
use bevy_rectray::events::EventFlags;
use bevy_rectray::widgets::util::{PropagateFocus, SetCursor};
use bevy_rectray::widgets::button::{Button, ButtonClick, Payload};
use bevy_rectray::util::{Widget, RCommands, convert::{OptionEx, IntoAsset}};
use crate::{build_shape, mframe_extension};
use crate::shaders::{RoundedRectangleMaterial, StrokeColoring};
use crate::style::Palette;
use crate::widgets::states::ButtonColors;
use super::util::StrokeColors;

mframe_extension!(
    pub struct MButtonBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<IntoAsset<Image>>,
        pub palette_hover: Option<Palette>,
        pub palette_pressed: Option<Palette>,
        pub palette_disabled: Option<Palette>,
        pub text: Option<String>,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub icon: IntoAsset<Image>,
        pub icon_hover: IntoAsset<Image>,
        pub icon_pressed: IntoAsset<Image>,
        pub signal: Option<TypedSignal<Object>>,
        pub payload: Option<Payload>,
    }
);

impl Widget for MButtonBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::LeftClick | EventFlags::Hover;
        let mut frame = build_frame!(commands, self);

        let style = self.palette;
        let hover = self.palette_hover.unwrap_or(style);
        let pressed = self.palette_pressed.unwrap_or(hover);
        let disabled = self.palette_disabled.unwrap_or(style);

        frame.insert((
            PropagateFocus,
            Button,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            Container {
                layout: StackLayout::HSTACK.into(),
                margin: size2!(0.5 em, 1 em),
                padding: size2!(1 em, 0.75 em),
                range: LayoutRange::All,
                maximum: usize::MAX
            },
            ButtonColors {
                idle: style.background(),
                hover: hover.background(),
                pressed: pressed.background(),
                disabled: disabled.background(),
            },
            StrokeColors(ButtonColors{
                idle: style.stroke(),
                hover: hover.stroke(),
                pressed: pressed.stroke(),
                disabled: disabled.stroke(),
            }),
            Interpolate::<Color>::new(
                Easing::Linear,
                style.background(),
                0.15
            ),
            Interpolate::<StrokeColoring>::new(
                Easing::Linear,
                style.stroke(),
                0.15
            ),
        ));
        if let Option::Some(payload) = self.payload  {
            frame.insert(payload);
        };
        let frame = frame.id();
        if let Some(signal) = self.signal {
            commands.entity(frame).add_sender::<ButtonClick>(signal);
        }
        if let Some(icon) = commands.try_load(self.icon) {
            let child = sprite!(commands{
                sprite: icon,
                z: 0.01,
                dimension: size2!(1.2 em, 1.2 em),
                extra: ButtonColors {
                    idle: style.foreground(),
                    hover: hover.foreground(),
                    pressed: pressed.foreground(),
                    disabled: disabled.foreground(),
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    style.foreground(),
                    0.15
                ),
            });
            commands.entity(frame).add_child(child);
        } else if self.text.is_some() {
            let left_pad = bevy_rectray::frame!(commands {
                dimension: size2!(0),
            });
            commands.entity(frame).add_child(left_pad);
        }
        if let Some(text) = self.text {
            let child = text!(commands{
                text: text,
                z: 0.01,
                font: commands.load_or_default(self.font),
                extra: ButtonColors {
                    idle: style.foreground(),
                    hover: hover.foreground(),
                    pressed: pressed.foreground(),
                    disabled: disabled.foreground(),
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    style.foreground(),
                    0.15
                ),
            });
            let right_pad = bevy_rectray::frame!(commands {
                dimension: size2!(0),
            });
            commands.entity(frame).push_children(&[child, right_pad]);
        }
        build_shape!(commands, self, frame);
        (frame, frame)
    }
}

#[macro_export]
macro_rules! mbutton {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MButtonBuilder] {
            $($tt)*
        })
    };
}