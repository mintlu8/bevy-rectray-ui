use bevy::{app::{Plugin, PostUpdate}, ecs::schedule::IntoSystemConfigs};
use bevy_rectray::schedule::AouiStoreOutputSet;
mod systems;
mod shapes;

pub use shapes::ShapeBuilder;
pub use systems::{Shapes, ShapeDimension};
/// Plugin for widgets that do not depend on events.
pub struct AouiShapesPlugin;

impl Plugin for AouiShapesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, systems::sync_shape_size.in_set(AouiStoreOutputSet))
            .add_systems(PostUpdate, systems::rebuild_shapes.in_set(AouiStoreOutputSet).after(systems::sync_shape_size))
        ;
    }
}