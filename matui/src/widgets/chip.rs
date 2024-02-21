use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, text::Font};
use bevy::window::CursorIcon;
use bevy_defer::TypedSignal;
use bevy_defer::Object;
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
use crate::widgets::states::ToggleFocusColors;
use super::util::StrokeColors;

mframe_extension!(
    pub struct MChipBuilder {
        pub cursor: Option<CursorIcon>,
        pub palette_hover: Option<Palette>,
        pub palette_checked: Option<Palette>,
        pub palette_hover_checked: Option<Palette>,
        pub palette_disabled: Option<Palette>,
        pub text: String,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub checked: bool,
        pub icon: IntoAsset<Image>,
        pub signal: Option<TypedSignal<Object>>,
        pub payload: Option<Payload>,
    }
);

impl Widget for MChipBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.event |= EventFlags::LeftClick | EventFlags::Hover;
        let mut frame = build_frame!(commands, self);

        let pal = self.palette;
        let pal_checked = self.palette_checked.unwrap_or(pal);
        let pal_hover = self.palette_hover.unwrap_or(pal);
        let pal_hover_checked = self.palette_hover_checked.unwrap_or(pal_checked);
        let pal_disabled = self.palette_disabled.unwrap_or(pal);

        frame.insert((
            PropagateFocus,
            Button,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            Container {
                layout: StackLayout::HSTACK.into(),
                margin: size2!(0 em, 0 em),
                padding: size2!(1 em, 0.75 em),
                range: LayoutRange::All,
                maximum: usize::MAX
            },
            ToggleFocusColors {
                unchecked: pal.background(),
                unchecked_focused: pal_hover.background(),
                checked: pal_checked.background(),
                checked_focused: pal_hover_checked.background(),
                disabled: pal_disabled.background(),
            },
            StrokeColors(ToggleFocusColors {
                unchecked: pal.stroke(),
                unchecked_focused: pal_hover.stroke(),
                checked: pal_checked.stroke(),
                checked_focused: pal_hover_checked.stroke(),
                disabled: pal_disabled.stroke(),
            }),
            Interpolate::<Color>::new(
                Easing::Linear,
                pal.background(),
                0.15
            ),
            Interpolate::<StrokeColoring>::new(
                Easing::Linear,
                pal.stroke(),
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
        let icon = commands.load_or_default(self.icon);
        let child = sprite!(commands{
            sprite: icon,
            z: 0.01,
            dimension: size2!(1.2 em, 1.2 em),
            extra: ToggleFocusColors {
                unchecked: pal.foreground(),
                unchecked_focused: pal_hover.foreground(),
                checked: pal_checked.foreground(),
                checked_focused: pal_hover_checked.foreground(),
                disabled: pal_disabled.foreground(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                pal.foreground(),
                0.15
            ),
        });
        commands.entity(frame).add_child(child);
        let left_pad = bevy_rectray::frame!(commands {
            dimension: size2!(0.5 em, 0),
        });
        commands.entity(frame).add_child(left_pad);
        let child = text!(commands{
            text: self.text,
            z: 0.01,
            font: commands.load_or_default(self.font),
            extra: ToggleFocusColors {
                unchecked: pal.foreground(),
                unchecked_focused: pal_hover.foreground(),
                checked: pal_checked.foreground(),
                checked_focused: pal_hover_checked.foreground(),
                disabled: pal_disabled.foreground(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                pal.foreground(),
                0.15
            ),
        });
        commands.entity(frame).add_child(child);
        build_shape!(commands, self, frame);
        (frame, frame)
    }
}

#[macro_export]
macro_rules! mchip {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MChipBuilder] {
            $($tt)*
        })
    };
}
