use bevy::render::{mesh::{Indices, Mesh}, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology};



/// Construct a mesh rectangle use in `material_sprite!`.
pub fn mesh_rectangle() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION,
            vec![[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, 0.5, 0.0], [0.5, 0.5, 0.0]]
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
        )
        .with_inserted_indices(Indices::U32(vec![
            0, 1, 2,
            1, 2, 3
        ]))
}
