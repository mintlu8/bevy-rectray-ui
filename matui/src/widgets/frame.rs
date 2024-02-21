use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, Query};
use bevy::math::Vec2;
use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::render::texture::Image;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::window::CursorIcon;
use bevy_rectray::anim::Attr;
use bevy_rectray::layout::{Axis, BoundsLayout};
use bevy_rectray::layout::LayoutControl::IgnoreLayout;
use bevy_defer::{Signal, TypedSignal, Object};
use bevy_rectray::util::{signal, ComposeExtension};
use bevy_rectray::widgets::button::ToggleChange;
use bevy_rectray::{transition, vstack, Anchor, Dimension, DimensionData, Hitbox, Opacity, Size2};
use bevy_rectray::widgets::drag::Dragging;
use bevy_rectray::{frame, frame_extension, build_frame, size2, layout::StackLayout};
use bevy_rectray::events::EventFlags;
use bevy_rectray::util::{Widget, RCommands, convert::{OptionEx, IntoAsset}};
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
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
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

use super::states::SignalToggleOpacity;
use super::util::ShadowInfo;

frame_extension!(pub struct MRectangle {
    pub palette: Palette,
    pub stroke: f32,
    pub shadow: Option<ShadowInfo>
});

impl Widget for MRectangle {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
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
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
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
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
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
    }
);

#[derive(Debug, Component)]
pub struct WindowCollapse {
    banner: Entity,
    window: Entity,
    signal: Signal<Object>,
}


pub fn window_collapse_transfer(
    mut commands: Commands,
    mut query: Query<(Entity, &DimensionData, Attr<Dimension, Dimension>, &WindowCollapse)>,
    dimension: Query<&DimensionData>,
) {
    for (entity, read, mut write, transfer) in query.iter_mut() {
        let Some(sig) = transfer.signal.try_read_as::<bool>() else {continue};
        if sig {
            let Ok(dim) = dimension.get(transfer.window) else {continue};
            let mut frac = read.size / dim.size;
            if frac.is_nan() {
                frac = Vec2::ZERO;
            }
            write.force_set(frac);
            write.set(Vec2::ONE);
            match commands.get_entity(transfer.window) {
                Some(mut e) => {e.add_child(entity);},
                None => {commands.entity(entity).despawn_recursive();},
            }

        } else {
            let Ok(dim) = dimension.get(transfer.banner) else {continue};
            let mut frac = read.size / dim.size;
            if frac.is_nan() {
                frac = Vec2::ZERO;
            }
            write.force_set(frac);
            write.set(Vec2::ONE);
            match commands.get_entity(transfer.banner) {
                Some(mut e) => {e.add_child(entity);},
                None => {commands.entity(entity).despawn_recursive();},
            }

        }
    }
}

impl Widget for MWindowBuilder {
    fn spawn(mut self, commands: &mut RCommands) -> (Entity, Entity) {
        self.z += 0.01;
        self.event = EventFlags::BlockAll;
        //self.dimension = Some(size2!(0, 0));
        let style = self.palette;
        let frame = vstack!(commands{
            offset: self.offset,
            z: self.z,
            extra: Dragging::BOTH
        });
        let mat = if let Some(im) = commands.try_load(self.texture) {
            RoundedRectangleMaterial::from_image(im, style.background(), self.radius)
        } else {
            RoundedRectangleMaterial::new(style.background(), self.radius)
        }.with_stroke((self.stroke, self.palette.stroke())).into_bundle(commands);
        let background = frame!(commands {
            z: -0.05,
            anchor: Anchor::TOP_CENTER,
            dimension: Size2::FULL,
            extra: mat,
            extra: IgnoreLayout,
            extra: transition!(Dimension 0.2 Linear default Vec2::ZERO)
        });
        commands.entity(frame).add_child(background);
        if let Some(collapse) = &self.collapse {
            if let Some(banner) = self.banner {
                commands.entity(background)
                    .add_receiver::<ToggleChange>(collapse.clone())
                    .insert((
                    transition!(Dimension 0.2 Linear default Vec2::ONE),
                    WindowCollapse {
                        banner,
                        window: frame,
                        signal: Signal::from_typed(collapse.clone()),
                    }
                ));
            }
            
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
                .compose(EventFlags::LeftDrag)
                .insert(Hitbox::FULL);
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
                    SignalToggleOpacity::new(0.0, 1.0)
                ));
            }
        }

        self.layout = self.layout.or(Some(StackLayout::VSTACK.into()));
        let container = build_frame!(commands, self).id();

        let rest = bevy_rectray::padding!(commands {
            child: container
        });

        if let Some(collapse) = self.collapse {
            commands.entity(rest)
                .add_receiver::<ToggleChange>(collapse.clone())
                .insert((
                    transition!(Opacity 0.2 CubicInOut default 1.0),
                    SignalToggleOpacity::new(0.0, 1.0)
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
