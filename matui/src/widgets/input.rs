use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::Font;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::GlobalTransform;
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::widgets::TextFragment;
use bevy_aoui::{Opacity, material_sprite, size2, BuildMeshTransform, color, inputbox, Anchor, text, Size2, rectangle};
use bevy_aoui::widgets::inputbox::{InputOverflow, InputBoxState, InputBoxCursorArea, InputBoxCursorBar, InputBoxText};
use bevy_aoui::{size, widget_extension, build_frame};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvTextChange, EvTextSubmit};
use bevy_aoui::widgets::util::DisplayIf;
use bevy_aoui::dsl::{Widget, mesh_rectangle, AouiCommands, DslInto};
use bevy_aoui::dsl::HandleOrString;
use crate::shapes::{RoundedRectangleMaterial, StrokeColor};

use super::util::{StrokeColors, WidgetPalette};

/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct CursorStateColors {
    pub idle: Color,
    pub hover: Color,
    pub pressed: Color,
    pub disabled: Color,
}

impl Default for CursorStateColors {
    fn default() -> Self {
        Self {
            idle: Color::NONE,
            hover: Color::NONE,
            pressed: Color::NONE,
            disabled: Color::NONE
        }
    }
}

pub fn cursor_color_change(mut query: Query<(&CursorStateColors, &Opacity, Option<&CursorFocus>, &mut Interpolate<Color>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
            return;
        }
        match focus {
            Some(focus) if focus.is(EventFlags::Hover)=> color.interpolate_to(colors.hover),
            Some(focus) if focus.intersects(EventFlags::LeftPressed|EventFlags::LeftDrag)
                => color.interpolate_to(colors.pressed),
            _ => color.interpolate_to(colors.idle),
        }
    })
}


pub fn cursor_stroke_change(mut query: Query<(&StrokeColors<CursorStateColors>, &Opacity, Option<&CursorFocus>, &mut Interpolate<StrokeColor>)>) {
    query.iter_mut().for_each(|(colors, opacity, focus, mut color)| {
        if opacity.is_disabled() {
            color.interpolate_to(colors.disabled);
            return;
        }
        match focus {
            Some(focus) if focus.is(EventFlags::Hover)=> color.interpolate_to(colors.hover),
            Some(focus) if focus.is(EventFlags::LeftPressed)=> color.interpolate_to(colors.pressed),
            _ => color.interpolate_to(colors.idle),
        }
    })
}


#[derive(Debug, Component, Clone, Copy, Default)]
pub struct ColorOnClick;

/// A simple state machine that changes depending on status.
#[derive(Debug, Component, Clone, Copy)]
pub struct InputStateColors {
    pub idle: Color,
    pub focused: Color,
    pub disabled: Color,
}

widget_extension!(
    pub struct MInputBuilder {
        pub placeholder: String,
        pub text: String,
        /// Width of text, in em.
        pub width: f32,
        pub font: HandleOrString<Font>,
        pub radius: f32,
        pub on_change: Handlers<EvTextChange>,
        pub on_submit: Handlers<EvTextSubmit>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
        pub palette: WidgetPalette,
        pub focus_palette: Option<WidgetPalette>,
        pub disabled_palette: Option<WidgetPalette>,

        pub bottom_bar: Option<f32>,
    }
);

impl Widget for MInputBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        bevy_aoui::inject_events!(self.event, EventFlags::Hover|EventFlags::LeftDrag);

        self.dimension = size2!({self.width} em, 2.8 em).dinto();
        let style = self.palette;
        let focus_style = self.focus_palette.unwrap_or(style);
        let disabled_style = self.disabled_palette.unwrap_or(style);

        let rect = commands.add(
            RoundedRectangleMaterial::new(style.background,
                if self.bottom_bar.is_some() {
                    [0.0, 0.0, self.radius, self.radius]
                } else {
                    [self.radius; 4]
                }
            )
        );
        let mesh = commands.add(mesh_rectangle());
        let frame = build_frame!(commands, self).id();
        let input_box = inputbox!(commands {
            color: style.foreground,
            text: &self.text,
            overflow: self.overflow,
            dimension: Size2::FULL,
            font: self.font.clone(),
            width: size!(1 - 1.6 em),
            z: 0.01,
            extra: Mesh2dHandle(mesh),
            extra: rect,
            extra: GlobalTransform::IDENTITY,
            extra: BuildMeshTransform,
            extra: InputStateColors {
                idle: style.background,
                focused: focus_style.background,
                disabled: disabled_style.background,
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                style.background,
                0.15
            ),
            cursor_bar: material_sprite! {
                z: 0.005,
                dimension: size2!(0.15 em, 1.2 em),
                material: RoundedRectangleMaterial::capsule(style.foreground),
                extra: InputBoxCursorBar,
                extra: DisplayIf::<InputBoxState>(InputBoxState::Focus),
            },
            cursor_area: material_sprite! {
                z: -0.005,
                dimension: size2!(0, 1.2 em),
                material: RoundedRectangleMaterial::new(color!(green300), 2.0),
                extra: InputBoxCursorArea,
                extra: DisplayIf::<InputBoxState>(InputBoxState::Focus),
            },
            text_area: rectangle! {
                z: 0.01,
                offset: size2!(0.8 em, 0.0),
                color: style.foreground,
                anchor: Anchor::CENTER_LEFT,
                extra: InputBoxText,
                extra: TextFragment::new(self.text)
                    .with_font(self.font.clone().get(commands))
                    .with_color(style.foreground)
            }
        });
        let has_placeholder = !self.placeholder.is_empty();
        if has_placeholder {
            let placeholder = text!(commands {
                anchor: Anchor::CENTER_LEFT,
                offset: size2!(0.8 em, 0),
                font: self.font.clone(),
                text: self.placeholder,
            });
            commands.entity(input_box).add_child(placeholder);
        }
        if let Some(bottom_bar) = self.bottom_bar {
            let bottom_bar = material_sprite!(commands {
                parent_anchor: Anchor::BOTTOM_CENTER,
                dimension: size2!(100%, bottom_bar em),
                material: RoundedRectangleMaterial::capsule(color!(black)),
            });
            commands.entity(input_box).add_child(bottom_bar);
        }

        commands.entity(frame).add_child(input_box);
        (frame, input_box)
    }
}

#[macro_export]
macro_rules! minput {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MInputBuilder] {
            $($tt)*
        })
    };
}
