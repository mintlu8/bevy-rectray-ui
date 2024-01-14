use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::render::color::Color;
use bevy::text::Font;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::GlobalTransform;
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::widgets::TextFragment;
use bevy_aoui::{Opacity, material_sprite, size2, BuildMeshTransform, color, inputbox, Anchor, text, Size2, rectangle, transition};
use bevy_aoui::widgets::inputbox::{InputOverflow, InputBoxState, InputBoxCursorArea, InputBoxCursorBar, InputBoxText};
use bevy_aoui::{size, frame_extension, build_frame};
use bevy_aoui::anim::{Interpolate, Easing, Offset, Scale};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvTextChange, EvTextSubmit};
use bevy_aoui::util::{Widget, mesh_rectangle, AouiCommands, DslInto, convert::IntoAsset};
use crate::shaders::{RoundedRectangleMaterial, StrokeColor};
use crate::style::Palette;

use super::util::StrokeColors;

#[derive(Debug, Clone, Copy, Component)]
pub struct PlaceHolderText {
    pub idle_color: Color,
    pub active_color: Color,
}

pub fn text_placeholder(
    mut query: Query<(
        &PlaceHolderText,
        Option<&InputBoxState>,
        &mut Interpolate<Color>,
        &mut Interpolate<Offset>,
        &mut Interpolate<Scale>,
)>) {
    for (placeholder, state, mut color, mut offset, mut scale) in query.iter_mut() {
        match state {
            Some(_) => {
                color.interpolate_to(placeholder.active_color);
                offset.interpolate_to(Vec2::new(0.8, 0.7));
                scale.interpolate_to(Vec2::new(0.8, 0.8));
            },
            None => {
                color.interpolate_to(placeholder.idle_color);
                offset.interpolate_to(Vec2::new(0.8, 0.0));
                scale.interpolate_to(Vec2::new(1.0, 1.0));
            }
        }
    }
}


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

frame_extension!(
    pub struct MInputBuilder {
        pub placeholder: String,
        pub text: String,
        /// Width of text, in em.
        pub width: f32,
        pub font: IntoAsset<Font>,
        pub radius: f32,
        pub on_change: Handlers<EvTextChange>,
        pub on_submit: Handlers<EvTextSubmit>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
        pub palette: Palette,
        pub focus_palette: Option<Palette>,
        pub disabled_palette: Option<Palette>,

        pub bottom_bar: Option<f32>,
    }
);

impl Widget for MInputBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.event |= EventFlags::Hover|EventFlags::LeftDrag;

        self.dimension = size2!({self.width} em, 2.8 em).dinto();
        let style = self.palette;
        let focus_style = self.focus_palette.unwrap_or(style);
        let disabled_style = self.disabled_palette.unwrap_or(style);

        let rect = commands.add_asset(
            RoundedRectangleMaterial::new(style.background(),
                if self.bottom_bar.is_some() {
                    [0.0, 0.0, self.radius, self.radius]
                } else {
                    [self.radius; 4]
                }
            )
        );
        let mesh = commands.add_asset(mesh_rectangle());
        let frame = build_frame!(commands, self).id();
        let input_box = inputbox!(commands {
            color: style.foreground(),
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
                idle: style.background(),
                focused: focus_style.background(),
                disabled: disabled_style.background(),
            },
            extra: Interpolate::<Color>::new(
                Easing::Linear,
                style.background(),
                0.15
            ),
            cursor_bar: material_sprite! {
                z: 0.005,
                dimension: size2!(0.15 em, 1.2 em),
                material: RoundedRectangleMaterial::capsule(style.foreground()),
                extra: InputBoxCursorBar,
            },
            cursor_area: material_sprite! {
                z: -0.005,
                dimension: size2!(0, 1.2 em),
                material: RoundedRectangleMaterial::new(color!(green300), 2.0),
                extra: InputBoxCursorArea,
            },
            text_area: rectangle! {
                z: 0.01,
                offset: size2!(0.8 em, {if self.placeholder.is_empty() {
                    0.0
                } else {
                    -0.4
                }} em),
                color: style.foreground(),
                anchor: Anchor::CENTER_LEFT,
                extra: InputBoxText,
                extra: TextFragment::new(self.text)
                    .with_font(commands.load_or_default(self.font.clone()))
                    .with_color(style.foreground())
            }
        });
        let has_placeholder = !self.placeholder.is_empty();
        if has_placeholder {
            let placeholder = text!(commands {
                anchor: Anchor::CENTER_LEFT,
                offset: size2!(0.8 em, 0 em),
                center: Anchor::CENTER_LEFT,
                font: self.font.clone(),
                text: self.placeholder,
                extra: PlaceHolderText {
                    idle_color: style.foreground(),
                    active_color: focus_style.foreground()
                },
                extra: transition!(
                    Color 0.15 Linear default {self.palette.foreground()};
                    Offset 0.15 Linear default {Vec2::ZERO};
                    Scale 0.15 Linear default {Vec2::ONE};
                )
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
