use std::mem;

use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::hierarchy::BuildChildren;
use bevy::sprite::Mesh2dHandle;
use bevy::transform::components::GlobalTransform;
use bevy::window::CursorIcon;
use bevy_aoui::anim::{Interpolate, Padding};
use bevy_aoui::layout::{Axis, BoundsLayout};
use bevy_aoui::layout::LayoutControl::IgnoreLayout;
use bevy_aoui::signals::{SignalBuilder, RawReceiver};
use bevy_aoui::widgets::misc::LayoutOpacityLimiter;
use bevy_aoui::{material_sprite, Hitbox, Size2, Opacity, transition, Dimension, Anchor, BuildMeshTransform};
use bevy_aoui::widgets::drag::{Dragging, DragConstraint};
use bevy_aoui::{frame, frame_extension, build_frame, size2, layout::StackLayout};
use bevy_aoui::events::{EventFlags, Handlers, EvMouseDrag, Fetch, Evaluated};
use bevy_aoui::util::{Widget, AouiCommands, mesh_rectangle, convert::{OptionEx, IntoAsset}};
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
        };
        let width = self.width.unwrap_or(0.1);
        match self.axis {
            Axis::Horizontal => {
                let entity = material_sprite!(commands {
                    dimension: size2!({100.0 - self.inset * 2.0}%, width em),
                    material: mat,
                    z: self.z,
                });
                (entity, entity)
            },
            Axis::Vertical => {
                let entity = material_sprite!(commands {
                    dimension: size2!(width em, {100.0 - self.inset * 2.0}%),
                    material: mat,
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

#[derive(Debug, Component)]
pub struct WindowDimensionQuery {
    pub banner: RawReceiver<Vec2>,
    pub padding: RawReceiver<Vec2>,
    pub total: RawReceiver<Vec2>,
}

use super::util::ShadowInfo;


frame_extension!(pub struct MCapsule {
    pub palette: Palette,
    pub stroke: f32,
    pub shadow: Option<ShadowInfo>
});

impl Widget for MCapsule {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let mesh = commands.add_asset(mesh_rectangle());
        let material = commands.add_asset(
            RoundedRectangleMaterial::capsule(self.palette.background())
                .with_stroke((self.palette.stroke(), self.stroke)));

        let entity = build_frame!(commands, self)
            .insert((
                material,
                Mesh2dHandle(mesh),
                GlobalTransform::IDENTITY,
                BuildMeshTransform,
            )).id();

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
        let mesh = commands.add_asset(mesh_rectangle());
        let material = commands.add_asset(
            RoundedRectangleMaterial::new(self.palette.background(), self.radius)
                .with_stroke((self.palette.stroke(), self.stroke)));
        let mut frame = build_frame!(commands, self);
        let id = frame.insert((
            Mesh2dHandle(mesh),
            material,
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        )).id();
        if let OptionEx::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, self.radius);
            commands.entity(id).add_child(shadow);
        }
        (id, id)
    }
}


mframe_extension!(
    pub struct MWindowBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<IntoAsset<Image>>,
        /// This will set `color_pressed` if its not set
        pub texture: IntoAsset<Image>,
        pub banner_texture: IntoAsset<Image>,
        pub collapse: Option<SignalBuilder<bool>>,
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
        }.with_stroke((self.stroke, self.palette.stroke()));
        commands.entity(frame).insert((
            Dragging::BOTH,
            DragConstraint,
        ));
        let background = material_sprite!(commands {
            z: -0.05,
            anchor: Anchor::TOP_CENTER,
            dimension: Size2::FULL,
            material: mat,
            extra: IgnoreLayout,
        });
        commands.entity(frame).add_child(background);
        let (dim_max_send, dim_max_recv) = commands.signal();
        let (dim_banner_send, dim_banner_recv) = commands.signal();
        let (padding_send, padding_recv) = commands.signal();
        if let Some(collapse) = &self.collapse {
            commands.entity(background).insert((
                transition!(Dimension 0.2 Linear default Vec2::ONE),
                WindowDimensionQuery {
                    banner: dim_banner_recv.recv_raw(),
                    padding: padding_recv.recv_raw(),
                    total: dim_max_recv.recv_raw(),
                },
                collapse.clone().recv(|open: bool, dim: &WindowDimensionQuery, inter: &mut Interpolate<Dimension>| {
                    if open {
                        inter.interpolate_to(Vec2::ONE);
                    } else if let Some(frac) = (||Some((dim.banner.poll()?.y + dim.padding.poll()?.y * 2.0) / dim.total.poll()?.y))() {
                            inter.interpolate_to(Vec2::new(1.0, frac))
                    }
                })
            ));
            commands.entity(frame).insert((
                Handlers::<Fetch<Evaluated<Dimension>>>::new(dim_max_send),
                Handlers::<Fetch<Evaluated<Padding>>>::new(padding_send)
            ));
        }
        if let OptionEx::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, self.radius);
            commands.entity(background).add_child(shadow);
        }
        if let Some(banner) = self.banner {
            let (drag_send, drag_recv) = commands.signal();
            commands.entity(frame).insert(
                drag_recv.invoke::<Dragging>()
            );
            commands.entity(banner).insert((
                Hitbox::FULL,
                EventFlags::LeftDrag,
                Handlers::<EvMouseDrag>::new(drag_send),
                Handlers::<Fetch<Evaluated<Dimension>>>::new(dim_banner_send),
            ));
            commands.entity(frame).add_child(banner);

            let divider = mdivider!(commands{
                inset: 10,
                axis: Axis::Horizontal,
                color: self.palette.stroke_lite,
            });
            commands.entity(frame).add_child(divider);

            if let Some(collapse) = &self.collapse {
                commands.entity(divider).insert((
                    transition!(Opacity 0.2 CubicOut default 1.0),
                    collapse.clone().recv_select(true,
                        Interpolate::<Opacity>::signal_to(1.0),
                        Interpolate::<Opacity>::signal_to(0.0)
                    ),
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
            commands.entity(rest).insert((
                transition!(Opacity 0.2 CubicInOut default 1.0),
                collapse.recv_select(true,
                    Interpolate::<Opacity>::signal_to(1.0),
                    Interpolate::<Opacity>::signal_to(0.0)
                )
            ));
        }
        commands.entity(frame).add_child(rest);
        (frame, container)
    }
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
