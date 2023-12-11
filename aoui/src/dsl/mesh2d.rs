use bevy::{math::Vec2, render::{view::RenderLayers, mesh::Mesh, render_resource::PrimitiveTopology}, sprite::{Material2d, Mesh2dHandle}, transform::components::GlobalTransform};

use crate::{Anchor, Opacity, Size2, FontSize, events::EventFlags, Hitbox, map_builder, dsl::builders::FrameBuilder, OwnedRectangleMesh};

use super::{OneOrTwo, Widget};

#[derive(Default)]
/// A `MaterialMesh2d` with a managed rectangle mesh.
pub struct MaterialRectangleBuilder<M: Material2d> {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub opacity: Opacity,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: FontSize,
    pub event: Option<EventFlags>,
    pub hitbox: Option<Hitbox>,
    pub layer: Option<RenderLayers>,
    pub material: Option<M>,
}

impl<M: Material2d> Widget for MaterialRectangleBuilder<M> {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> bevy::prelude::Entity {
        let entity = map_builder!(self => FrameBuilder move (
            anchor, parent_anchor, center, opacity, visible,
            offset, rotation, scale, z, dimension, hitbox,
            layer, font_size, event
        )).spawn_with(commands, assets);
        let assets = assets.expect("Please pass in the asset server.");
        let mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, 
                vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, 
                vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, 
                vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
            )
            .with_indices(Some(bevy::render::mesh::Indices::U32(vec![
                0, 1, 2,
                1, 2, 3
            ])));
        commands.entity(entity).insert((
            assets.add(self.material.expect("Please specify a material.")),
            Mesh2dHandle(assets.add(mesh)),
            GlobalTransform::IDENTITY,
            OwnedRectangleMesh,
        )).id()
    }
}


/// Construct a compact layout.
#[macro_export]
macro_rules! material_rect {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::MaterialRectangleBuilder] {
            $($tt)*
        })
    };
}
