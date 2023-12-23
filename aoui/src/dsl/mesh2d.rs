use bevy::{math::Vec2, render::{view::RenderLayers, mesh::Mesh, render_resource::PrimitiveTopology}, sprite::{Material2d, Mesh2dHandle}, transform::components::GlobalTransform, ecs::entity::Entity};

use crate::{Anchor, Opacity, Size2, FontSize, events::EventFlags, Hitbox, BuildMeshTransform, build_frame};

use super::{OneOrTwo, Widget, converters::HandleOrAsset, Aspect};

#[derive(Debug, Default)]
/// A `MaterialMesh2d` with a rectangle mesh.
pub struct MaterialSpriteBuilder<M: Material2d> {
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
    pub aspect: Aspect,
    pub font_size: FontSize,
    pub event: Option<EventFlags>,
    pub hitbox: Option<Hitbox>,
    pub clipping: bool,
    pub layer: Option<RenderLayers>,
    pub material: HandleOrAsset<M>,
}

pub fn mesh_rectangle() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, 
            vec![[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, 0.5, 0.0], [0.5, 0.5, 0.0]]
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
        ])))
}

impl<M: Material2d> Widget for MaterialSpriteBuilder<M> {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (Entity, Entity) {
        let mut entity = build_frame!(commands, self);
        let assets = assets.expect("Please pass in the asset server.");
        let mesh = mesh_rectangle();
        let e = entity.insert((
            self.material.expect(Some(assets), "Please specify a material."),
            Mesh2dHandle(assets.add(mesh)),
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        )).id();
        (e, e)
    }
}


/// Construct a compact layout.
#[macro_export]
macro_rules! material_sprite {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::MaterialSpriteBuilder] {
            $($tt)*
        })
    };
}


#[derive(Debug, Default)]
/// A `MaterialMesh2d` with a managed rectangle mesh.
pub struct MaterialMeshBuilder<M: Material2d> {
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
    pub aspect: Aspect,
    pub font_size: FontSize,
    pub event: Option<EventFlags>,
    pub hitbox: Option<Hitbox>,
    pub layer: Option<RenderLayers>,
    pub clipping: bool,
    pub mesh: HandleOrAsset<Mesh>,
    pub material: HandleOrAsset<M>,
}

impl<M: Material2d> Widget for MaterialMeshBuilder<M> {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (Entity, Entity) {
        let mut entity = build_frame!(commands, self);
        let e = entity.insert((
            self.material.expect(assets, "Please specify a material."),
            Mesh2dHandle(self.mesh.expect(assets, "Please specify a mesh.")),
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        )).id();
        (e, e)
    }
}


/// Construct a compact layout.
#[macro_export]
macro_rules! material_mesh{
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::MaterialMeshBuilder] {
            $($tt)*
        })
    };
}
