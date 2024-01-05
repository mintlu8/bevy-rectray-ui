use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::hierarchy::BuildChildren;
use bevy::window::CursorIcon;
use bevy_aoui::anim::{Interpolate, Padding};
use bevy_aoui::layout::Axis;
use bevy_aoui::layout::LayoutControl::IgnoreLayout;
use bevy_aoui::signals::{signal, SignalBuilder, RawReceiver};
use bevy_aoui::{material_sprite, Hitbox, vstack, Size2, Opacity, transition, Dimension, Anchor};
use bevy_aoui::widgets::drag::{Dragging, DragConstraint};
use bevy_aoui::{widget_extension, build_frame, size2, layout::StackLayout};
use bevy_aoui::events::{EventFlags, Handlers, EvMouseDrag, Fetch, Evaluated};
use bevy_aoui::dsl::{Widget, DslInto};
use bevy_aoui::dsl::HandleOrString;
use crate::shapes::RoundedRectangleMaterial;

#[derive(Debug, Default)]
pub struct Divider {
    pub inset: f32,
    pub axis: Axis,
    pub color: Color,
}

impl Widget for Divider {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&AssetServer>) -> (Entity, Entity) {
        let assets = assets.expect("Please pass in the asset server");
        let mat = if self.inset == 0.0 {
            RoundedRectangleMaterial::rect(self.color)
        } else {
            RoundedRectangleMaterial::capsule(self.color)
        };
        match self.axis {
            Axis::Horizontal => {
                let entity = material_sprite!((commands, assets) {
                    dimension: size2!({100.0 - self.inset * 2.0}%, 0.1 em),
                    material: mat,
                });
                (entity, entity)
            },
            Axis::Vertical => {
                let entity = material_sprite!((commands, assets) {
                    dimension: size2!(0.1 em, {100.0 - self.inset * 200.0}%),
                    material: mat,
                });
                (entity, entity)
            }
        }
    }
}


macro_rules! divider {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [Divider] {
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


#[derive(Debug, Default, Clone, Copy)]
pub struct WindowPalette {
    pub background: Color,
    pub banner: Color,
    pub stroke: Color,
}

use super::util::{OptionM, ShadowInfo};
widget_extension!(
    pub struct MWindowBuilder {
        pub cursor: Option<CursorIcon>,
        pub sprite: Option<HandleOrString<Image>>,
        /// This will set `color_pressed` if its not set
        pub palette: WindowPalette,
        pub texture: HandleOrString<Image>,
        pub banner_texture: HandleOrString<Image>,
        pub collapse: Option<SignalBuilder<bool>>, 
        pub stroke: f32,
        pub banner_stroke: f32,
        pub radius: f32,
        pub shadow: OptionM<ShadowInfo>,
        pub banner: Option<Entity>,
        pub window_margin: Option<Vec2>,
    }
);

impl Widget for MWindowBuilder {
    fn spawn_with(mut self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        self.z += 0.01;
        self.layout = Some(Box::new(StackLayout::VSTACK));
        self.event = Some(EventFlags::BlockAll);
        let window_margin = self.window_margin.unwrap_or(Vec2::new(1.0, 0.5));
        self.margin = Size2::em(0.0, window_margin.y).dinto();
        self.padding = Size2::em(window_margin.x, window_margin.y).dinto();
        let frame = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the AssetServer");
        let style = self.palette;
        let frame = frame.id();
        let mat = if let Some(im) = self.texture.try_get(assets) {
            RoundedRectangleMaterial::from_image(im, style.background, self.radius)
        } else {
            RoundedRectangleMaterial::new(style.background, self.radius)
        }.with_stroke((self.stroke, self.palette.stroke));
        commands.entity(frame).insert((
            Dragging::BOTH,
            DragConstraint,
        ));
        let background = material_sprite!((commands, assets) {
            z: -0.05,
            anchor: Anchor::TopCenter,
            dimension: Size2::FULL,
            material: mat,
            extra: IgnoreLayout,
        });
        commands.entity(frame).add_child(background);
        let (dim_max_send, dim_max_recv) = signal();
        let (dim_banner_send, dim_banner_recv) = signal();
        let (padding_send, padding_recv) = signal();
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
                    } else {
                        if let Some(frac) = (||Some((dim.banner.poll()?.y + dim.padding.poll()?.y * 2.0) / dim.total.poll()?.y))() {
                            inter.interpolate_to(Vec2::new(1.0, frac))
                        }
                    }
                })
            ));
            commands.entity(frame).insert((
                Handlers::<Fetch<Evaluated<Dimension>>>::new(dim_max_send),
                Handlers::<Fetch<Evaluated<Padding>>>::new(padding_send)
            ));
        }
        if let OptionM::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, assets, self.radius);
            commands.entity(background).add_child(shadow);
        }
        if let Some(banner) = self.banner {
            let (drag_send, drag_recv) = signal();
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

            let divider = divider!((commands, assets){
                inset: 0.1,
                axis: Axis::Horizontal,
                color: self.palette.stroke,
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
            child: vstack!{
                entity: container,
                margin: self.margin,
                padding: self.padding,
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
macro_rules! mwindow {
    ($ctx: tt {$($tt: tt)*}) => {
        $crate::aoui::meta_dsl!($ctx [$crate::widgets::MWindowBuilder] {
            $($tt)*
        })
    };
}
