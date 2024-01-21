use std::mem;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::hierarchy::BuildChildren;
use bevy::window::CursorIcon;
use bevy_aoui::layout::{Axis, BoundsLayout};
use bevy_aoui::layout::LayoutControl::IgnoreLayout;
use bevy_aoui::sync::TypedSignal;
use bevy_aoui::util::{signal, ComposeExtension};
use bevy_aoui::widgets::button::ToggleChange;
use bevy_aoui::widgets::misc::LayoutOpacityLimiter;
use bevy_aoui::{transition, Anchor, Dimension, Hitbox, Opacity, Size2};
use bevy_aoui::widgets::drag::{Dragging, DragConstraint};
use bevy_aoui::{frame, frame_extension, build_frame, size2, layout::StackLayout};
use bevy_aoui::events::EventFlags;
use bevy_aoui::util::{Widget, AouiCommands, convert::{OptionEx, IntoAsset}};
use crate::mframe_extension;
use crate::shaders::RoundedRectangleMaterial;
use crate::style::Palette;

#[derive(Debug, Default)]
pub struct Divider {
    pub width: Option<f32>,
    pub inset: f32,
    pub axis: Axis,
    pub color: Color,
    pub z: f32,
}

impl Widget for Divider {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let mat = if self.inset == 0.0 {
            RoundedRectangleMaterial::rect(self.color)
        } else {
            RoundedRectangleMaterial::capsule(self.color)
        }.into_bundle(commands);
        let width = self.width.unwrap_or(0.1);
        match self.axis {
            Axis::Horizontal => {
                let entity = frame!(commands {
                    dimension: size2!({100.0 - self.inset * 2.0}%, width em),
                    extra: mat,
                    z: self.z,
                });
                (entity, entity)
            },
            Axis::Vertical => {
                let entity = frame!(commands {
                    dimension: size2!(width em, {100.0 - self.inset * 2.0}%),
                    extra: mat,
                    z: self.z,
                });
                (entity, entity)
            }
        }
    }
}

#[macro_export]
macro_rules! mdivider {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::Divider] {
            $($tt)*
        })
    };
}

use super::states::ToggleOpacity;
use super::util::ShadowInfo;

frame_extension!(pub struct MRectangle {
    pub palette: Palette,
    pub stroke: f32,
    pub shadow: Option<ShadowInfo>
});

impl Widget for MRectangle {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let material = RoundedRectangleMaterial::rect(self.palette.background())
            .with_stroke((self.palette.stroke(), self.stroke)).into_bundle(commands);

        let entity = build_frame!(commands, self).insert(material).id();

        if let Some(shadow) = self.shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(entity).add_child(shadow);
        }
        (entity, entity)
    }
}


frame_extension!(pub struct MCapsule {
    pub palette: Palette,
    pub stroke: f32,
    pub shadow: Option<ShadowInfo>
});

impl Widget for MCapsule {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let material = RoundedRectangleMaterial::capsule(self.palette.background())
            .with_stroke((self.palette.stroke(), self.stroke)).into_bundle(commands);

        let entity = build_frame!(commands, self).insert(material).id();

        if let Some(shadow) = self.shadow {
            let shadow = shadow.build_capsule(commands);
            commands.entity(entity).add_child(shadow);
        }
        (entity, entity)
    }
}

mframe_extension!(pub struct MFrameBuilder {});

impl Widget for MFrameBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.z += 0.01;
        if self.layout.is_none() {
            self.layout = Some(BoundsLayout::PADDING.into());
        }
        self.event = EventFlags::BlockAll;
        let material = RoundedRectangleMaterial::new(self.palette.background(), self.radius)
            .with_stroke((self.palette.stroke(), self.stroke)).into_bundle(commands);
        let mut frame = build_frame!(commands, self);
        let id = frame.insert(material).id();
        if let OptionEx::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, self.radius);
            commands.entity(id).add_child(shadow);
        }
        (id, id)
    }
}

#[derive(Debug, Component)]
pub struct DimensionPaddingWatcher{
    dimension: TypedSignal<Size2>,
    padding: TypedSignal<Size2>,
}

#[derive(Debug, Component)]
pub struct DimensionWatcher{
    dimension: TypedSignal<Size2>,
}

#[derive(Debug, Component)]
pub struct WindowDimensionQuery{
    dimension: TypedSignal<Size2>,
    banner: TypedSignal<Size2>,
    padding: TypedSignal<Size2>,
}

#[derive(Debug, Component)]
pub struct OpacityToggle(pub f32, pub f32);

mframe_extension!(
    pub struct MWindowBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<IntoAsset<Image>>,
        /// This will set `color_pressed` if its not set
        pub texture: IntoAsset<Image>,
        pub banner_texture: IntoAsset<Image>,
        pub collapse: Option<TypedSignal<bool>>,
        pub banner: Option<Entity>,
        pub window_margin: Option<Vec2>,
    }
);

impl Widget for MWindowBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        self.z += 0.01;
        let layout = mem::replace(&mut self.layout, Some(StackLayout::VSTACK.into()));
        self.event = EventFlags::BlockAll;
        let window_margin = self.window_margin.unwrap_or(Vec2::new(1.0, 0.5));
        let margin = mem::replace(&mut self.margin.0, Size2::em(0.0, window_margin.y));
        let padding = mem::replace(&mut self.padding.0, Size2::em(window_margin.x, window_margin.y));
        //self.dimension = Some(size2!(0, 0));
        let frame = build_frame!(commands, self);
        let style = self.palette;
        let frame = frame.id();
        let mat = if let Some(im) = commands.try_load(self.texture) {
            RoundedRectangleMaterial::from_image(im, style.background(), self.radius)
        } else {
            RoundedRectangleMaterial::new(style.background(), self.radius)
        }.with_stroke((self.stroke, self.palette.stroke())).into_bundle(commands);
        commands.entity(frame).insert((
            Dragging::BOTH,
            DragConstraint,
        ));
        let background = frame!(commands {
            z: -0.05,
            anchor: Anchor::TOP_CENTER,
            dimension: Size2::FULL,
            extra: mat,
            extra: IgnoreLayout,
        });
        commands.entity(frame).add_child(background);
        let (dim_max_send, dim_max_recv) = signal();
        let (dim_banner_send, dim_banner_recv) = signal();
        let (padding_send, padding_recv) = signal();
        if let Some(collapse) = &self.collapse {
            commands.entity(background)
                .add_receiver::<ToggleChange>(collapse.clone())
                .insert((
                transition!(Dimension 0.2 Linear default Vec2::ONE),
                WindowDimensionQuery {
                    banner: dim_banner_recv,
                    padding: padding_recv,
                    dimension: dim_max_recv,
                },

                // collapse.clone().recv(|open: bool, dim: &WindowDimensionQuery, inter: &mut Interpolate<Dimension>| {
                //     if open {
                //         inter.interpolate_to(Vec2::ONE);
                //     } else if let Some(frac) = (||Some((dim.banner.poll()?.y + dim.padding.poll()?.y * 2.0) / dim.total.poll()?.y))() {
                //         inter.interpolate_to(Vec2::new(1.0, frac))
                //     }
                // })
            ));
            commands.entity(frame).insert(DimensionPaddingWatcher{
                dimension: dim_max_send,
                padding: padding_send,
            });
        }
        if let OptionEx::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, self.radius);
            commands.entity(background).add_child(shadow);
        }
        if let Some(banner) = self.banner {
            let (drag_send, drag_recv) = signal();
            commands.entity(frame).add_receiver::<Dragging>(drag_recv);
            commands.entity(banner)
                .add_sender::<Dragging>(drag_send)
                .insert(
                    (
                        Hitbox::FULL,
                        EventFlags::LeftDrag,
                        DimensionWatcher{
                            dimension: dim_banner_send
                        },
                    )
            );
            commands.entity(frame).add_child(banner);

            let divider = mdivider!(commands{
                inset: 10,
                axis: Axis::Horizontal,
                color: self.palette.stroke_lite,
            });
            commands.entity(frame).add_child(divider);

            if let Some(collapse) = &self.collapse {
                commands.entity(divider)
                    .add_receiver::<ToggleChange>(collapse.clone())
                    .insert((
                    transition!(Opacity 0.2 CubicOut default 1.0),
                    ToggleOpacity::new(0.0, 1.0)
                ));
            }
        }
        let container;
        let rest = bevy_aoui::padding!(commands {
            child: frame!{
                entity: container,
                margin: margin,
                padding: padding,
                layout: layout.unwrap_or(StackLayout::VSTACK.into()),
                extra: LayoutOpacityLimiter,
            }
        });

        if let Some(collapse) = self.collapse {
            commands.entity(rest)
                .add_receiver::<ToggleChange>(collapse.clone())
                .insert((
                transition!(Opacity 0.2 CubicInOut default 1.0),
                ToggleOpacity::new(0.0, 1.0)
            ));
        }
        commands.entity(frame).add_child(rest);
        (frame, container)
    }
}

#[macro_export]
macro_rules! mrectangle {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MRectangle] {
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! mcapsule {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MCapsule] {
            $($tt)*
        })
    };
}



#[macro_export]
macro_rules! mframe {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MFrameBuilder] {
            $($tt)*
        })
    };
}


#[macro_export]
macro_rules! mwindow {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MWindowBuilder] {
            $($tt)*
        })
    };
}
