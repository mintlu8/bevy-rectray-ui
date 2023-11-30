use bevy::{input::mouse::MouseWheel, math::Vec2, window::{Window, PrimaryWindow}, render::camera::Camera, transform::components::GlobalTransform, ecs::component::Component};
use bevy::ecs::{system::{Query, Commands}, event::EventReader, query::{With, Without}, entity::Entity};
use bevy_aoui::{RotatedRect, Hitbox};

use super::{EventFlags, AoUICamera};


/// This is relatively independent, as the mousewheel action does not take
/// the drag target and the cursor action target into account.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct MouseWheelAction(Vec2);

impl MouseWheelAction {
    pub fn get(&self) -> Vec2 {
        self.0
    }
}


pub fn mousewheel_event(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    marked_camera: Query<(&Camera, &GlobalTransform), With<AoUICamera>>,
    unmarked_camera: Query<(&Camera, &GlobalTransform), Without<AoUICamera>>,
    query: Query<(Entity, &RotatedRect, &Hitbox, &EventFlags)>,
    mut reader: EventReader<MouseWheel>,
) {
    let(camera, camera_transform) = match marked_camera.get_single() {
        Ok((cam, transform)) => (cam, transform),
        Err(_) => match unmarked_camera.get_single(){
            Ok((cam, transform)) => (cam, transform),
            Err(_) => return,
        },
    };
    let Ok(window) = windows.get_single() else { return };       
    let Some(mouse_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) else {return;};

    if let Some(entity) = query.iter().filter(|(.., flags)| flags.contains(crate::events::MouseWheel))
        .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
        .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
        .map(|(entity,..)| entity) {
            
        for event in reader.read() {
            commands.entity(entity).insert(MouseWheelAction(Vec2::new(event.x, event.y)));
        }
    }
}