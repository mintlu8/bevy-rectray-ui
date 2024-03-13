use bevy::{render::mesh::Mesh, ecs::entity::Entity};
use bevy::transform::components::GlobalTransform;
use bevy::sprite::{Material2d, Mesh2dHandle};

use crate::{BuildMeshTransform, build_frame, frame_extension, util::mesh_rectangle};

use crate::util::{Widget, RCommands, convert::IntoAsset};

frame_extension!(
    /// Construct a sprite with a custom [`Material2d`](bevy::sprite::Material2d).
    pub struct MaterialSpriteBuilder[M: Material2d] {
        /// Material of the sprite.
        pub material: IntoAsset<M>,
    }
);

impl<M: Material2d> Widget for MaterialSpriteBuilder<M> {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let material = commands.load_or_panic(self.material, "Please specify a material.");
        let mesh = commands.add_asset(mesh_rectangle());
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

frame_extension!(
    /// Construct a [`Mesh2d`](bevy::sprite::Mesh2d) with a custom [`Material2d`](bevy::sprite::Material2d).
    pub struct MaterialMeshBuilder[M: Material2d] {
        /// Mesh of the sprite.
        pub mesh: IntoAsset<Mesh>,
        /// Material of the sprite.
        pub material: IntoAsset<M>,
    }
);

impl<M: Material2d> Widget for MaterialMeshBuilder<M> {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let material = commands.load_or_panic(self.material, "Please specify a material.");
        let mesh = Mesh2dHandle(commands.load_or_panic(self.mesh, "Please specify a mesh."));
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
