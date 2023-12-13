use bevy::{app::{Plugin, PostUpdate}, ecs::schedule::IntoSystemConfigs};
use bevy_aoui::schedule::AoUIStoreOutputSet;
mod systems;
mod shapes;

pub use shapes::ShapeBuilder;
pub use systems::{Shapes, ShapeDimension};
/// Plugin for widgets that do not depend on events.
pub struct AoUIShapesPlugin;

impl Plugin for AoUIShapesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, systems::sync_shape_size.in_set(AoUIStoreOutputSet))
            .add_systems(PostUpdate, systems::rebuild_shapes.in_set(AoUIStoreOutputSet).after(systems::sync_shape_size))
        ;
    }
}