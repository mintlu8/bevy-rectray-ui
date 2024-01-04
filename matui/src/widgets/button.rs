use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, text::Font, transform::components::GlobalTransform};
use bevy::sprite::Mesh2dHandle;
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::Opacity;
use bevy_aoui::{widget_extension, build_frame, Hitbox, size2, text, layout::{Container, StackLayout, LayoutDir}, sprite, BuildMeshTransform};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvButtonClick};
use bevy_aoui::widgets::button::{PropagateFocus, Button, SetCursor, Payload};
use bevy_aoui::dsl::{Widget, mesh_rectangle};
use bevy_aoui::dsl::HandleOrString;
use bevy_aoui::dsl::OptionX;
use crate::style::{Palette, WidgetStyle};
use crate::shapes::{RoundedRectangleMaterial, StrokeColor};

use super::util::{OptionM, ShadowInfo, StrokeColors, WidgetPalette};

#[derive(Debug, Component, Clone, Copy)]
pub struct ButtonColors {
    idle: Color,
    hover: Color,
    pressed: Color,
    disabled: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self { 
            idle: Color::NONE, 
            hover: Color::NONE, 
            pressed: Color::NONE, 
            disabled: Color::NONE 
        }
    }
}

pub fn btn_color_change(mut query: Query<(&ButtonColors, &Opacity, Option<&CursorFocus>, &mut Interpolate<Color>)>) {
    query.par_iter_mut().for_each(|(colors, opacity, focus, mut color)| {
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


pub fn btn_stroke_change(mut query: Query<(&StrokeColors<ButtonColors>, &Opacity, Option<&CursorFocus>, &mut Interpolate<StrokeColor>)>) {
    query.par_iter_mut().for_each(|(colors, opacity, focus, mut color)| {
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    #[default]
    Filled,
    Tonal,
    Outlined,
    Text,
}

impl WidgetStyle for ButtonStyle {
    type StyleType = WidgetPalette;

    fn style(&self, style: Palette) -> Self::StyleType {
        match self {
            ButtonStyle::Filled => WidgetPalette {
                background: style.main.background,
                foreground: style.main.on_background,
                stroke: Color::NONE,
            },
            ButtonStyle::Tonal => WidgetPalette {
                background: style.main.container,
                foreground: style.main.on_container,
                stroke: Color::NONE,
            },
            ButtonStyle::Outlined => WidgetPalette {
                background: style.surface.background,
                foreground: style.main.background,
                stroke: style.surface.on_background,
            },
            ButtonStyle::Text => WidgetPalette {
                background: style.main.background,
                foreground: Color::NONE,
                stroke: Color::NONE,
            },
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct ColorOnClick;

widget_extension!(
    pub struct MButtonBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<HandleOrString<Image>>,
        /// This will set `color_pressed` if its not set
        pub palette: WidgetPalette,
        pub palette_hover: Option<WidgetPalette>,
        pub palette_pressed: Option<WidgetPalette>,
        pub palette_disabled: Option<WidgetPalette>,
        pub text: Option<String>,
        pub font: HandleOrString<Font>,
        pub texture: HandleOrString<Image>,
        pub icon: HandleOrString<Image>,
        pub icon_hover: HandleOrString<Image>,
        pub icon_pressed: HandleOrString<Image>,
        pub stroke: f32,
        pub signal: Handlers<EvButtonClick>,
        pub payload: OptionX<Payload>,
        pub capsule: bool,
        pub radius: f32,
        pub shadow: OptionM<ShadowInfo>,
    }
);

impl Widget for MButtonBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut frame = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the AssetServer");

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
                layout: Box::new(StackLayout { direction: LayoutDir::LeftToRight}),
                margin: size2!(0.5 em, 1 em),
                padding: size2!(1 em, 0.75 em),
                range: None,
            },
            ButtonColors {
                idle: style.background,
                hover: hover.background,
                pressed: pressed.background,
                disabled: disabled.background,
            },
            StrokeColors(ButtonColors{
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
        if let OptionX::Some(payload) = self.payload  {
            frame.insert(payload);
        };
        let frame = frame.id();
        if let Some(icon) = self.icon.try_get(assets) {
            let child = sprite!((commands, assets){
                sprite: icon,
                z: 0.01,
                dimension: size2!(1.2 em, 1.2 em),
                extra: ButtonColors { 
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
            let child = text!((commands, assets){
                text: text,
                z: 0.01,
                font: self.font.get(assets),
                extra: ButtonColors { 
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
                let mat = if let Some(im) = self.texture.try_get(assets) {
                    RoundedRectangleMaterial::capsule_image(im, style.background)
                } else {
                    RoundedRectangleMaterial::capsule(style.background)
                }.with_stroke((self.stroke, self.palette.stroke));
                commands.entity(frame).insert((
                    assets.add(mat),
                    Mesh2dHandle(assets.add(mesh_rectangle())),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let OptionM::Some(shadow) = self.shadow {
                    let shadow = shadow.build_capsule(commands, assets);
                    commands.entity(frame).add_child(shadow);
                }
                (frame, frame)
            },
            (_, radius, ..) => {
                let mat = if let Some(im) = self.texture.try_get(assets) {
                    RoundedRectangleMaterial::from_image(im, style.background, radius)
                } else {
                    RoundedRectangleMaterial::new(style.background, radius)
                }.with_stroke((self.stroke, self.palette.stroke));
                commands.entity(frame).insert((
                    assets.add(mat),
                    Mesh2dHandle(assets.add(mesh_rectangle())),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let OptionM::Some(shadow) = self.shadow {
                    let shadow = shadow.build_rect(commands, assets, radius);
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
