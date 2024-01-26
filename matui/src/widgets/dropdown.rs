use bevy::ecs::entity::Entity;
use bevy::ecs::system::Query;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::reflect::Reflect;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::text::Font;
use bevy::window::CursorIcon;
use bevy::ecs::component::Component;
use bevy_aoui::dsl::prelude::{adaptor, receiver, signal_ids};
use bevy_aoui::dsl::OptionEx;
use bevy_defer::{SignalReceiver, TypedSignal, Object};
use bevy_aoui::util::ComposeExtension;
use bevy_aoui::widgets::button::{ToggleChange, ToggleInvoke};
use bevy_aoui::widgets::signals::{FormatText, Invocation, TextFromSignal};
use bevy_aoui::{size2, text, transition, Anchor, Transform2D};
use bevy_aoui::widgets::inputbox::InputOverflow;
use bevy_aoui::{frame_extension, build_frame};
use bevy_aoui::anim::{Attr, Easing, Interpolate, Offset, Rotation, Scale};
use bevy_aoui::events::{EventFlags, LoseFocus, StrongFocusStateMachine};
use bevy_aoui::util::{Widget, AouiCommands, DslInto, convert::IntoAsset};
use crate::widgets::input::{DisplayIfHasText, PlaceHolderText};
use crate::build_shape;
use crate::shaders::RoundedRectangleMaterial;
use crate::style::Palette;
use crate::widgets::menu::{MenuCloseOnCallback, MenuItemMarker};
use crate::widgets::states::{FocusColors, SignalToggleOpacity};

use super::menu::{MenuCallback, MenuState};
use super::{ShadowInfo, MenuItem};

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
        // Width of text, in em.
        pub width: f32,
        pub dropdown_width: f32,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub stroke: f32,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionEx<ShadowInfo>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
        pub palette: Palette,
        pub focus_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,
        pub callback: TypedSignal<MenuState>,
        pub menu: Option<Entity>,
        pub cancel: Option<Entity>,
        pub dial: Option<Entity>,
        pub cancel_signal: TypedSignal<()>,
        pub toggle_signal: TypedSignal<bool>,
    }
);

impl Widget for MDropdownBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftDrag;

        self.dimension = size2!({self.width} em, 2.8 em).dinto();

        let palette = self.palette;
        let focus_palette = self.focus_palette.unwrap_or(palette);
        let disabled_palette = self.disabled_palette.unwrap_or(palette);

        let entity = build_frame!(commands, self).id();

        let text_area = text!(commands {
            z: 0.01,
            offset: size2!(0.8 em, {if self.placeholder.is_empty() {
                0.0
            } else {
                -0.4
            }} em),
            color: palette.foreground(),
            anchor: Anchor::CENTER_LEFT,
            extra: TextFromSignal,
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
            signal: receiver::<MenuCallback>(self.callback.clone()),
            signal: adaptor::<MenuCallback, FormatText>(|callback| callback.name),
        });

        build_shape!(commands, self, entity);

        if let Some(cancel) = self.cancel {
            commands.entity(cancel).insert((
                DisplayIfHasText { points_to: text_area },
                MenuItemMarker,
                MenuState {
                    value: Object::NONE,
                    name: String::new(),
                }
            ))
            .add_sender::<MenuCallback>(self.callback.clone());
            commands.entity(entity).add_child(cancel);
        }

        if let Some(button) = self.dial {
            commands.entity(button).insert(MenuCloseOnCallback)
                .add_sender::<ToggleChange>(self.toggle_signal.clone())
                .add_receiver::<MenuCallback>(self.callback.clone())
                .add_receiver::<Invocation>(self.cancel_signal.clone().type_erase())
                .add_adaptor::<Invocation, ToggleInvoke>(|_| false);
            commands.entity(entity).add_child(button);
        }

        commands.entity(entity)
            .add_sender::<LoseFocus>(self.cancel_signal)
            .compose(EventFlags::ClickOutside)
            .insert(StrongFocusStateMachine::NoFocus);
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
                    active_color: focus_palette.foreground(),
                    points_to: text_area,
                },
                extra: transition!(
                    Color 0.15 Linear default {self.palette.foreground()};
                    Offset 0.15 Linear default {Vec2::ZERO};
                    Scale 0.15 Linear default {Vec2::ONE};
                )
            });
            commands.entity(entity).add_child(placeholder);
        };

        commands.entity(entity).add_child(text_area);

        if let Some(menu) = self.menu {
            commands.entity(menu)
                .add_receiver::<ToggleChange>(self.toggle_signal)
                .insert(SignalToggleOpacity::new(0.0, 1.0));
            commands.entity(entity).add_child(menu);
        }
        (entity, entity)
    }
}

signal_ids!(
    pub SpinSignal: bool
);

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct SpinDial{
    pub from: f32,
    pub to: f32,
}

pub fn spin_dial_system(mut q: Query<(SignalReceiver<SpinSignal>, &SpinDial, Attr<Transform2D, Rotation>)>){
    for (sig, dial, mut rot) in q.iter_mut(){
        if let Some(val) = sig.poll_once() {
            if val {
                rot.set(dial.to);
            } else {
                rot.set(dial.from);
            }
        }
    }
}

#[macro_export]
macro_rules! mdropdown {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MDropdownBuilder] {
            $($tt)*
        })
    };
}
