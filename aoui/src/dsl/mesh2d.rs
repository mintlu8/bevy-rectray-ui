use bevy::{render::{mesh::Mesh, render_resource::PrimitiveTopology}, ecs::entity::Entity};
use bevy::transform::components::GlobalTransform;
use bevy::sprite::{Material2d, Mesh2dHandle};

use crate::{BuildMeshTransform, build_frame, widget_extension};

use super::{Widget, converters::HandleOrAsset, AouiCommands};

widget_extension!(
    /// Construct a sprite with a custom [`Material2d`](bevy::sprite::Material2d).
    pub struct MaterialSpriteBuilder[M: Material2d] {
        /// Material of the sprite.
        pub material: HandleOrAsset<M>,
    }
);

/// Construct a mesh rectangle use in `material_sprite!`.
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
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let material = self.material.expect(&commands, "Please specify a material.");
        let mesh = commands.add(mesh_rectangle());
        let mut entity = build_frame!(commands, self);
        let e = entity.insert((
            material, Mesh2dHandle(mesh),
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        )).id();
        (e, e)
    }
}


/// Construct a sprite with a custom [`Material2d`](bevy::sprite::Material2d).
/// 
/// The underlying struct is [`MaterialSpriteBuilder`].
#[macro_export]
macro_rules! material_sprite {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::MaterialSpriteBuilder] {
            $($tt)*
        })
    };
}

widget_extension!(
    /// Construct a [`Mesh2d`](bevy::sprite::Mesh2d) with a custom [`Material2d`](bevy::sprite::Material2d).
    pub struct MaterialMeshBuilder[M: Material2d] {
        /// Mesh of the sprite.
        pub mesh: HandleOrAsset<Mesh>,
        /// Material of the sprite.
        pub material: HandleOrAsset<M>,
    }
);

impl<M: Material2d> Widget for MaterialMeshBuilder<M> {
    fn spawn(self, commands: &mut AouiCommands) -> (Entity, Entity) {
        let material = self.material.expect(&commands, "Please specify a material.");
        let mesh = Mesh2dHandle(self.mesh.expect(&commands, "Please specify a mesh."));
        let mut entity = build_frame!(commands, self);
        let e = entity.insert((
            material, mesh,
            GlobalTransform::IDENTITY,
            BuildMeshTransform,
        )).id();
        (e, e)
    }
}



/// Construct a [`Mesh2d`](bevy::sprite::Mesh2d) with a custom [`Material2d`](bevy::sprite::Material2d).
/// 
/// The underlying struct is [`MaterialMeshBuilder`].
#[macro_export]
macro_rules! material_mesh{
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::MaterialMeshBuilder] {
            $($tt)*
        })
    };
}
