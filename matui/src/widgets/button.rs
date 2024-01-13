use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, text::Font, transform::components::GlobalTransform};
use bevy::sprite::Mesh2dHandle;
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::Opacity;
use bevy_aoui::layout::LayoutRange;
use bevy_aoui::{widget_extension, build_frame, Hitbox, size2, text, layout::{Container, StackLayout}, sprite, BuildMeshTransform};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvButtonClick};
use bevy_aoui::widgets::util::{PropagateFocus, SetCursor};
use bevy_aoui::widgets::button::{Button, Payload};
use bevy_aoui::dsl::{Widget, mesh_rectangle, AouiCommands, OptionEx, IntoAsset};
use crate::shapes::{RoundedRectangleMaterial, StrokeColor};

use super::util::{ShadowInfo, StrokeColors, WidgetPalette};

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

widget_extension!(
    pub struct MButtonBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<IntoAsset<Image>>,
        /// This will set `color_pressed` if its not set
        pub palette: WidgetPalette,
        pub palette_hover: Option<WidgetPalette>,
        pub palette_pressed: Option<WidgetPalette>,
        pub palette_disabled: Option<WidgetPalette>,
        pub text: Option<String>,
        pub font: IntoAsset<Font>,
        pub texture: IntoAsset<Image>,
        pub icon: IntoAsset<Image>,
        pub icon_hover: IntoAsset<Image>,
        pub icon_pressed: IntoAsset<Image>,
        pub stroke: f32,
        pub signal: Handlers<EvButtonClick>,
        pub payload: Option<Payload>,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionEx<ShadowInfo>,
    }
);

impl Widget for MButtonBuilder {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let mut frame = build_frame!(commands, self);

        let style = self.palette;
        let hover = self.palette_hover.unwrap_or(style);
        let pressed = self.palette_pressed.unwrap_or(hover);
        let disabled = self.palette_disabled.unwrap_or(style);

        frame.insert((
            PropagateFocus,
            Button,
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            Container {
                layout: Box::new(StackLayout::HSTACK),
                margin: size2!(0.5 em, 1 em),
                padding: size2!(1 em, 0.75 em),
                range: LayoutRange::All,
                maximum: usize::MAX
            },
            CursorStateColors {
                idle: style.background,
                hover: hover.background,
                pressed: pressed.background,
                disabled: disabled.background,
            },
            StrokeColors(CursorStateColors{
                idle: style.stroke,
                hover: hover.stroke,
                pressed: pressed.stroke,
                disabled: disabled.stroke,
            }),
            Interpolate::<Color>::new(
                Easing::Linear,
                style.background,
                0.15
            ),
            Interpolate::<StrokeColor>::new(
                Easing::Linear,
                style.stroke,
                0.15
            ),
        ));
        if self.hitbox.is_none() {
            frame.insert(Hitbox::FULL);
        }
        if !self.signal.is_empty() {
            frame.insert(self.signal);
        }
        if let Option::Some(payload) = self.payload  {
            frame.insert(payload);
        };
        let frame = frame.id();
        if let Some(icon) = commands.try_load(self.icon) {
            let child = sprite!(commands{
                sprite: icon,
                z: 0.01,
                dimension: size2!(1.2 em, 1.2 em),
                extra: CursorStateColors {
                    idle: style.foreground,
                    hover: hover.foreground,
                    pressed: pressed.foreground,
                    disabled: disabled.foreground,
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    style.foreground,
                    0.15
                ),
            });
            commands.entity(frame).add_child(child);
        } else if self.text.is_some() {
            let left_pad = bevy_aoui::frame!(commands {
                dimension: size2!(0),
            });
            commands.entity(frame).add_child(left_pad);
        }
        if let Some(text) = self.text {
            let child = text!(commands{
                text: text,
                z: 0.01,
                font: commands.load_or_default(self.font),
                extra: CursorStateColors {
                    idle: style.foreground,
                    hover: hover.foreground,
                    pressed: pressed.foreground,
                    disabled: disabled.foreground,
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    style.foreground,
                    0.15
                ),
            });
            let right_pad = bevy_aoui::frame!(commands {
                dimension: size2!(0),
            });
            commands.entity(frame).push_children(&[child, right_pad]);
        }

        match (self.capsule, self.radius) {
            (true, ..) => {
                let mat = commands.add_asset(if let Some(im) = commands.try_load(self.texture) {
                    RoundedRectangleMaterial::capsule_image(im, style.background)
                } else {
                    RoundedRectangleMaterial::capsule(style.background)
                }.with_stroke((self.stroke, self.palette.stroke)));
                let rect = commands.add_asset(mesh_rectangle());
                commands.entity(frame).insert((
                    mat,
                    Mesh2dHandle(rect),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let OptionEx::Some(shadow) = self.shadow {
                    let shadow = shadow.build_capsule(commands);
                    commands.entity(frame).add_child(shadow);
                }
                (frame, frame)
            },
            (_, radius, ..) => {
                let mat = commands.add_asset(if let Some(im) = commands.try_load(self.texture) {
                    RoundedRectangleMaterial::from_image(im, style.background, radius)
                } else {
                    RoundedRectangleMaterial::new(style.background, radius)
                }.with_stroke((self.stroke, self.palette.stroke)));
                let rect = commands.add_asset(mesh_rectangle());
                commands.entity(frame).insert((
                    mat,
                    Mesh2dHandle(rect),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let OptionEx::Some(shadow) = self.shadow {
                    let shadow = shadow.build_rect(commands, radius);
                    commands.entity(frame).add_child(shadow);
                }
                (frame, frame)
            }
        }
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
