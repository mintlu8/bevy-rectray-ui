use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::Font;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::GlobalTransform;
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::{Opacity, material_sprite, size2, BuildMeshTransform, color, inputbox};
use bevy_aoui::widgets::inputbox::{InputOverflow, InputBoxFocus, InputBoxCursorArea, InputBoxCursorBar};
use bevy_aoui::{widget_extension, build_frame};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvTextChange, EvTextSubmit};
use bevy_aoui::widgets::util::{PropagateFocus, DisplayIf};
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

    }
);

impl Widget for MInputBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        bevy_aoui::inject_event!(self.event, EventFlags::Hover|EventFlags::LeftDrag);

        self.dimension = size2!({self.width} em, 2.8 em).dinto();
        let style = self.palette;
        let focus_style = self.focus_palette.unwrap_or(style);
        let disabled_style = self.disabled_palette.unwrap_or(style);

        let rect = commands.add(
            RoundedRectangleMaterial::new(style.background, [0.0, 0.0, self.radius, self.radius])
        );
        let mesh = commands.add(mesh_rectangle());
        let mut frame = build_frame!(commands, self);
        frame.insert((
            PropagateFocus,
            rect,
            Mesh2dHandle(mesh),
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
            InputStateColors {
                idle: style.background,
                focused: focus_style.background,
                disabled: disabled_style.background,
            },
            Interpolate::<Color>::new(
                Easing::Linear,
                style.background, 
                0.15
            ),
        ));
        let frame = frame.id();
        let input_box = inputbox!(commands {
            z: 0.01,
            dimension: size2!(1 - 1.6 em, 1 em),
            color: style.foreground,
            font: self.font,
            text: self.text,
            overflow: self.overflow,
            cursor_bar: material_sprite! {
                z: 0.015,
                dimension: size2!(0.15 em, 1.2 em),
                material: RoundedRectangleMaterial::capsule(style.foreground),
                extra: InputBoxCursorBar,
                extra: DisplayIf::<InputBoxFocus>::default(),
            },
            cursor_area: material_sprite! {
                z: 0.005,
                dimension: size2!(0, 1.2 em),
                material: RoundedRectangleMaterial::new(color!(green300), 2.0),
                extra: InputBoxCursorArea,
                extra: DisplayIf::<InputBoxFocus>::default(),
            },
        });
        commands.entity(frame).add_child(input_box);
        (frame, frame)
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
