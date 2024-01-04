use bevy::asset::AssetServer;
use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::{hierarchy::BuildChildren, transform::components::GlobalTransform};
use bevy::sprite::Mesh2dHandle;
use bevy::window::CursorIcon;
use bevy_aoui::layout::Axis;
use bevy_aoui::signals::signal;
use bevy_aoui::widgets::button::SetCursor;
use bevy_aoui::{material_sprite, Hitbox, vbox};
use bevy_aoui::widgets::drag::{Dragging, DragConstraint};
use bevy_aoui::{widget_extension, build_frame, size2, layout::CompactLayout, BuildMeshTransform};
use bevy_aoui::events::{EventFlags, Handlers, EvMouseDrag};
use bevy_aoui::dsl::{Widget, mesh_rectangle};
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
        pub stroke: f32,
        pub banner_stroke: f32,
        pub radius: f32,
        pub shadow: OptionM<ShadowInfo>,
        pub banner: Option<Entity>,
    }
);

impl Widget for MWindowBuilder {
    fn spawn_with(mut self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        self.z += 0.01;
        self.layout = Some(Box::new(CompactLayout::VBOX));
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
            assets.add(mat),
            Mesh2dHandle(assets.add(mesh_rectangle())),
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        ));
        if let OptionM::Some(shadow) = self.shadow {
            let shadow = shadow.build_rect(commands, assets, self.radius);
            commands.entity(frame).add_child(shadow);
        }
        if let Some(banner) = self.banner {
            let (drag_send, drag_recv) = signal();
            commands.entity(frame).insert(
                drag_recv.invoke::<Dragging>()
            );

            commands.entity(banner).insert((
                Hitbox::FULL,
                EventFlags::Hover|EventFlags::LeftDrag,
                SetCursor { 
                    flags: EventFlags::Hover|EventFlags::LeftDrag,
                    icon: CursorIcon::Hand, 
                },
                Handlers::<EvMouseDrag>::new(drag_send),
            ));
            let banner_entity = vbox!((commands, assets) {
                child: banner,
                child: divider! {
                    inset: 0.1,
                    axis: Axis::Horizontal,
                    color: self.palette.stroke,
                }
            });
            commands.entity(frame).add_child(banner_entity);
        }
        (frame, frame)
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
