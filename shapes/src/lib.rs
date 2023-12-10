
/// Plugin for widgets that do not depend on events.
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(PostUpdate, shape::sync_shape_size.in_set(AoUIStoreOutput))
            .add_systems(PostUpdate, shape::rebuild_shapes.in_set(AoUIStoreOutput).after(shape::sync_shape_size))
        ;
    }
}


fn main() {
    println!("Hello, world!");
}
