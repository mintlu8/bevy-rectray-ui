use bevy::asset::Handle;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::sprite::Sprite;
use bevy::text::{Font, Text};
use bevy::window::CursorIcon;
use bevy::ecs::component::Component;
use bevy_aoui::dsl::OptionEx;
use bevy_aoui::signals::SignalBuilder;
use bevy_aoui::util::convert::IntoEntity;
use bevy_aoui::util::{Object, WidgetBuilder};
use bevy_aoui::widgets::TextFragment;
use bevy_aoui::widgets::button::radio_button_group;
use bevy_aoui::{Opacity, material_sprite, size2, color, inputbox, Anchor, text, Size2, rectangle, transition, frame, format_widget, check_button};
use bevy_aoui::widgets::inputbox::{InputOverflow, InputBoxState, InputBoxCursorArea, InputBoxCursorBar, InputBoxText};
use bevy_aoui::{size, frame_extension, build_frame};
use bevy_aoui::anim::{Interpolate, Easing, Offset, Scale, Rotation};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvTextChange, EvTextSubmit};
use bevy_aoui::util::{Widget, AouiCommands, DslInto, convert::IntoAsset};
use crate::widgets::input::PlaceHolderText;
use crate::widgets::spinner::FocusColors;
use crate::{StrokeColoring, build_shape, mmenu};
use crate::shaders::RoundedRectangleMaterial;
use crate::style::Palette;

use super::{ShadowInfo, MenuItem};


#[derive(Debug, Component, Clone, Copy, Default)]
pub struct ColorOnClick;

/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct InputStateColors {
    pub idle: Color,
    pub focused: Color,
    pub disabled: Color,
}

frame_extension!(
    pub struct MDropdownBuilder {
        pub placeholder: String,
        pub content: Vec<MenuItem>,
        pub selected: Object,
        pub cancellable: bool,
        // Width of text, in em.
        pub width: f32,
        pub dropdown_width: f32,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub stroke: f32,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionEx<ShadowInfo>,
        pub on_change: Handlers<EvTextChange>,
        pub on_submit: Handlers<EvTextSubmit>,
        pub overflow: InputOverflow,

        pub open_icon: IntoAsset<Image>,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
        pub palette: Palette,
        pub focus_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,

        pub callback_signal: Option<SignalBuilder<bool>>,

        pub menu: IntoEntity,
        pub dial: IntoEntity,

    }
);

impl Widget for MDropdownBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftDrag;

        self.dimension = size2!({self.width} em, 2.8 em).dinto();
        
        let palette = self.palette;
        let focus_palette = self.focus_palette.unwrap_or(palette);
        let disabled_palette = self.disabled_palette.unwrap_or(palette);

        let default = if self.selected.is_none() {
            String::new()
        } else {
            self.content.iter().find_map(|x| {
                match x {
                    MenuItem::Divider => None,
                    MenuItem::Text { key, value, .. } | MenuItem::Nest { key, value, .. } => {
                        if key.equal_to(&self.selected) {
                            Some(value.clone())
                        } else {
                            None
                        }
                    },
                }
            }).unwrap_or("???".to_owned())
        };
        
        let (radio, ) = commands.radio_button_group(self.selected);

        let entity = build_frame!(commands, self).id();
        let textbox = text!(commands {
            color: palette.foreground(),
            text: default,
            font: self.font.clone(),
            z: 0.01,
            extra: radio.recv().recv(|input: String, text: &mut Text| {
                format_widget!(text, "{}", input)
            }),
            extra: FocusColors {
                idle: palette.background(),
                focus: focus_palette.background(),
                disabled: disabled_palette.background(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                palette.background(),
                0.15
            ),
        });

        build_shape!(commands, self, textbox);
        let has_placeholder = !self.placeholder.is_empty();
        if has_placeholder {
            let placeholder = text!(commands {
                anchor: Anchor::CENTER_LEFT,
                center: Anchor::CENTER_LEFT,
                offset: size2!(0.8 em, 0 em),
                font: self.font.clone(),
                text: self.placeholder,
                extra: PlaceHolderText {
                    idle_color: palette.foreground(),
                    active_color: focus_palette.foreground()
                },
                extra: transition!(
                    Color 0.15 Linear default {self.palette.foreground()};
                    Offset 0.15 Linear default {Vec2::ZERO};
                    Scale 0.15 Linear default {Vec2::ONE};
                )
            });
            commands.entity(textbox).add_child(placeholder);
        };

        let dial = self.dial.build_expect(commands, "Dial is required.");
        let menu = self.menu.build_expect(commands, "Menu is required.");
        commands.entity(dial).insert(
            FocusColors {
                idle: palette.foreground(),
                focus: focus_palette.foreground(),
                disabled: disabled_palette.foreground(),
            }
        );

        commands.entity(entity)
            .add_child(textbox)
            .add_child(dial)
            .add_child(menu);
        (entity, textbox)
    }
}

pub fn spin_dial(commands: &mut AouiCommands, sprite: impl DslInto<IntoAsset<Image>>, unchecked: f32, checked: f32) -> Entity {
    let (send, recv) = commands.signal();
    check_button!(commands {
        dimension: size2!(1.2 em, 1.2 em),
        on_change: send,
        extra: sprite.dinto().into_bundle(commands, Color::WHITE),
        extra: recv.recv_select(true, 
            Interpolate::<Rotation>::signal_to(checked), 
            Interpolate::<Rotation>::signal_to(unchecked)
        )
    })
}

#[macro_export]
macro_rules! mdropdown {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MDropdownBuilder] {
            $($tt)*
        })
    };
}
