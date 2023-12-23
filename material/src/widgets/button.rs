
use bevy::render::color::Color;
use bevy::render::texture::{Image, BevyDefault};
use bevy::render::render_resource::{Extent3d, TextureDimension};
use bevy::{hierarchy::BuildChildren, text::Font, transform::components::GlobalTransform};
use bevy::sprite::{Sprite, Mesh2dHandle};
use bevy::window::CursorIcon;
use bevy::ecs::{component::Component, system::Query};
use bevy_aoui::{widget_extension, build_frame, Hitbox, size2, text, layout::{Container, CompactLayout, FlexDir}, sprite, bundles::BuildTransformBundle, BuildMeshTransform};
use bevy_aoui::anim::{Interpolate, Easing};
use bevy_aoui::events::{EventFlags, CursorFocus, Handlers, EvButtonClick};
use bevy_aoui::widgets::button::{PropagateFocus, Button, SetCursor, Payload};
use bevy_aoui::dsl::{Widget, mesh_rectangle};
use bevy_aoui::dsl::HandleOrString;
use bevy_aoui::dsl::OptionX;
use crate::{shadow, builders::Stroke};
use crate::shapes::{CapsuleMaterial, RoundedRectangleMaterial};

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct ButtonColors {
    idle: Color,
    hover: Color,
    click: Color,
}

pub fn btn_color_change(mut query: Query<(&ButtonColors, Option<&CursorFocus>, &mut Interpolate<Color>)>) {
    query.par_iter_mut().for_each(|(colors, focus, mut color)| {
        match focus {
            Some(focus) if focus.is(EventFlags::Hover)=> color.interpolate_to(colors.hover),
            Some(focus) if focus.is(EventFlags::LeftPressed)=> color.interpolate_to(colors.click),
            _ => color.interpolate_to(colors.idle),
        }
    })
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct ColorOnClick;

widget_extension!(
    pub struct MButtonBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<HandleOrString<Image>>,
        /// This will set `color_pressed` if its not set
        pub background: Option<Color>,
        pub background_hover: Option<Color>,
        pub background_pressed: Option<Color>,
        pub foreground: Option<Color>,
        pub foreground_hover: Option<Color>,
        pub foreground_pressed: Option<Color>,
        pub text: Option<String>,
        pub font: HandleOrString<Font>,
        pub texture: HandleOrString<Image>,
        pub icon: HandleOrString<Image>,
        pub icon_hover: HandleOrString<Image>,
        pub icon_pressed: HandleOrString<Image>,
        pub stroke: Stroke,
        pub signal: Handlers<EvButtonClick>,
        pub payload: OptionX<Payload>,
        pub capsule: bool,
        pub radius: Option<f32>,
        pub shadow: Option<f32>,
        pub shadow_color: Option<Color>,
        pub shadow_z: Option<f32>,
        pub foreground_z: f32,
    }
);

impl Widget for MButtonBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut frame = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the AssetServer");

        let background = self.background.unwrap_or(if self.texture.is_some() {
            Color::WHITE
        } else {
            Color::NONE
        });
        let foreground = self.foreground.unwrap_or({
            let [r, g, b, a] = background.as_linear_rgba_f32();
            if r + g + b < 2.5 && a > 0.2 {
                Color::WHITE
            } else {
                Color::BLACK
            }
        });
        let no_background = self.background.is_none() 
            && self.background_hover.is_none() 
            && self.background_pressed.is_none()
            && self.texture.is_none();
        frame.insert((
            PropagateFocus,
            Button,
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            Container {
                layout: Box::new(CompactLayout { direction: FlexDir::LeftToRight}),
                margin: size2!(0.5 em, 1 em),
                padding: if no_background {size2!(0)} else {size2!(1 em, 0.75 em)},
            },
            ButtonColors {
                idle: background,
                hover: self.background_hover.unwrap_or(background),
                click: self.background_pressed.or(self.background_hover).unwrap_or(background),
            },
            Interpolate::<Color>::new(
                Easing::Linear,
                background, 
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
                color: foreground,
                dimension: size2!(1.2 em, 1.2 em),
                z: self.foreground_z,
                extra: ButtonColors { 
                    idle: foreground, 
                    hover: self.foreground_hover.unwrap_or(foreground), 
                    click: self.foreground_pressed.or(self.foreground_hover).unwrap_or(foreground),
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    foreground, 
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
                color: foreground,
                font: self.font.get(assets),
                z: self.foreground_z,
                extra: ButtonColors { 
                    idle: foreground, 
                    hover: self.foreground_hover.unwrap_or(foreground), 
                    click: self.foreground_pressed.or(self.foreground_hover).unwrap_or(foreground),
                },
                extra: Interpolate::<Color>::new(
                    Easing::Linear,
                    foreground, 
                    0.15
                ),
            });
            let right_pad = bevy_aoui::frame!(commands {
                dimension: size2!(0),
            });
            commands.entity(frame).push_children(&[child, right_pad]);
        }

        match (self.capsule, self.radius, no_background) {
            (.., true) => (frame, frame),
            (true, ..) => {
                let mat = if let Some(im) = self.texture.try_get(assets) {
                    CapsuleMaterial::from_image(im, background)
                } else {
                    CapsuleMaterial::new(background)
                }.with_stroke(self.stroke);
                commands.entity(frame).insert((
                    assets.add(mat),
                    Mesh2dHandle(assets.add(mesh_rectangle())),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let Some(shadow_size) = self.shadow {
                    let shadow_color = self.shadow_color.unwrap_or(Color::BLACK);
                    let shadow_z = self.shadow_z.unwrap_or(-0.01);

                    let shadow = shadow!(commands, assets, shadow_color, shadow_size, shadow_z);
                    commands.entity(frame).add_child(shadow);
                }
                (frame, frame)
            },
            (_, Some(radius), ..) => {
                let mat = if let Some(im) = self.texture.try_get(assets) {
                    RoundedRectangleMaterial::from_image(im, background, radius)
                } else {
                    RoundedRectangleMaterial::new(background, radius)
                }.with_stroke(self.stroke);
                commands.entity(frame).insert((
                    assets.add(mat),
                    Mesh2dHandle(assets.add(mesh_rectangle())),
                    GlobalTransform::IDENTITY,
                    BuildMeshTransform,
                ));
                if let Some(shadow_size) = self.shadow {
                    let shadow_color = self.shadow_color.unwrap_or(Color::BLACK);
                    let shadow_z = self.shadow_z.unwrap_or(-0.01);
                    let shadow = shadow!(commands, assets, shadow_color, radius, shadow_size, shadow_z);
                    commands.entity(frame).add_child(shadow);
                }
                (frame, frame)
            }
            _ => {
                let texture = Image::new(Extent3d {
                    width: 1,
                    height: 1,
                    ..Default::default()
                }, TextureDimension::D2, vec![255, 255, 255, 255], BevyDefault::bevy_default());
                if let Some(shadow_size) = self.shadow {
                    let shadow_color = self.shadow_color.unwrap_or(Color::BLACK);
                    let shadow_z = self.shadow_z.unwrap_or(f32::EPSILON * 8.0);
                    let shadow = shadow!(commands, assets, shadow_color, 0.0, shadow_size, shadow_z);
                    commands.entity(frame).insert((
                        Sprite::default(),
                        assets.add(texture),
                        BuildTransformBundle::default(),
                    )).add_child(shadow);
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

#[doc(hidden)]
#[macro_export]
macro_rules! shadow {
    ($commands: expr, $assets: expr, $color: expr, $radius: expr, $size: expr, $z: expr) => {
        $crate::aoui::material_sprite!(($commands, $assets) {
            dimension: $crate::aoui::size2![1 + {$size * 2.0} px, 1 + {$size * 2.0} px],
            z: $z,
            material: $crate::RoundedShadowMaterial::new($color, $radius, $size),
            extra: $crate::aoui::layout::LayoutControl::IgnoreLayout,
        })
    };
    ($commands: expr, $assets: expr, $color: expr, $size: expr, $z: expr) => {
        $crate::aoui::material_sprite!(($commands, $assets) {
            dimension: $crate::aoui::size2![1 + {$size * 2.0} px, 1 + {$size * 2.0} px],
            z: $z,
            material: $crate::CapsuleShadowMaterial::new($color, $size),
            extra: $crate::aoui::layout::LayoutControl::IgnoreLayout,
        })
    };
}