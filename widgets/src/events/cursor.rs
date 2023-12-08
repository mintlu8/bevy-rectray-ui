use bevy::{math::Vec2, ecs::{system::Query, query::{With, Without}, component::Component}, render::{camera::Camera, view::Visibility}, transform::components::GlobalTransform};
use bevy::window::{CursorIcon, Window, PrimaryWindow};
use bevy_aoui::Transform2D;

use crate::widgets::scrollframe::CameraClip;

use super::AoUICamera;


/// Must be unparented to work correctly.
/// Disabling system cursor is outside the scope of this crate
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct CustomCursor {
    icon: CursorIcon,
    offset: Vec2,
}

impl CustomCursor {
    pub fn new(icon: CursorIcon, offset: Vec2) -> Self {
        CustomCursor {
            icon, offset
        }
    }
}

pub fn custom_cursor_controller(
    windows: Query<&Window, With<PrimaryWindow>>,
    marked_camera: Query<(&Camera, &GlobalTransform), With<AoUICamera>>,
    unmarked_camera: Query<(&Camera, &GlobalTransform), (Without<AoUICamera>, Without<CameraClip>)>,
    mut query: Query<(&CustomCursor, &mut Transform2D, &mut Visibility)>
) {

    let(camera, camera_transform) = match marked_camera.get_single() {
        Ok((cam, transform)) => (cam, transform),
        Err(_) => match unmarked_camera.get_single(){
            Ok((cam, transform)) => (cam, transform),
            Err(_) => return,
        },
    };
    let Ok(window) = windows.get_single() else { return };       
    let mouse_pos = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    match mouse_pos {
        Some(pos) => {
            for (cursor, mut transform, mut vis) in query.iter_mut() {
                if std::mem::discriminant(&cursor.icon) == 
                        std::mem::discriminant(&window.cursor.icon) {
                    transform.offset = (pos + cursor.offset).into();
                    *vis = Visibility::Inherited;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        },
        None => {
            for (_, _, mut vis) in query.iter_mut() {
                *vis = Visibility::Hidden;
            }
        },
    }
}