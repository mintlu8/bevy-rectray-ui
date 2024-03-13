use std::mem::discriminant;

use bevy::{ecs::{system::{Query, SystemParam, Res}, query::{With, Without}, component::Component, bundle::Bundle}, render::{camera::Camera, view::Visibility}, transform::components::GlobalTransform, reflect::Reflect, math::Vec2};
use bevy::window::{CursorIcon, Window, PrimaryWindow};
use crate::{Transform2D, util::convert::DslInto, Size2, DimensionData, RectrayRem};

use crate::widgets::clipping::CameraClip;

use super::RectrayCamera;


/// Displays only when the window's CursorIcon is this.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct CustomCursor(pub CursorIcon);

impl CustomCursor {
    pub fn new_bundle(icon: CursorIcon) -> impl Bundle {
        (CustomCursor(icon), TrackCursor(Size2::ZERO))
    }

    pub fn new_offset(icon: CursorIcon, offset: impl DslInto<Size2>) -> impl Bundle {
        (CustomCursor(icon), TrackCursor(offset.dinto()))
    }
}

impl Default for CustomCursor {
    fn default() -> Self {
        Self(CursorIcon::Pointer)
    }
}

/// Make entity track cursor's movement.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub struct TrackCursor(pub Size2);

impl TrackCursor {
    pub const DEFAULT: TrackCursor = TrackCursor(Size2::ZERO);
    pub fn offset(offset: impl DslInto<Size2>) -> Self {
        TrackCursor(offset.dinto())
    }
}

/// A query that finds a camera used for cursor handling.
#[derive(Debug, SystemParam)]
pub struct CameraQuery<'w, 's> {
    marked_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<RectrayCamera>>,
    unmarked_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), (Without<RectrayCamera>, Without<CameraClip>)>,
}

impl CameraQuery<'_, '_> {
    pub fn viewport_to_world(&self, pos: Vec2) -> Option<Vec2> {
        let(camera, camera_transform) = match self.marked_camera.get_single() {
            Ok((cam, transform)) => (cam, transform),
            Err(_) => match self.unmarked_camera.get_single(){
                Ok((cam, transform)) => (cam, transform),
                Err(_) => return None,
            },
        };
        camera
            .viewport_to_world(camera_transform, pos)
            .map(|ray| ray.origin.truncate())
    }
}

pub fn custom_cursor_controller(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&CustomCursor, &mut Visibility)>
) {
    let Ok(window) = windows.get_single() else { return };

    if window.cursor_position().is_some() {
        for (cursor, mut vis) in query.iter_mut() {
            if discriminant(&cursor.0) == discriminant(&window.cursor.icon) {
                *vis = Visibility::Inherited;
            } else {
                *vis = Visibility::Hidden;
            }
        }
    } else {
        for (_, mut vis) in query.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }
}

pub fn track_cursor(
    rem: Res<RectrayRem>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: CameraQuery,
    mut query: Query<(&TrackCursor, &mut Transform2D, &DimensionData)>
) {
    let Ok(window) = windows.get_single() else { return };
    let Some(mouse_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(cursor))
    else {return};
    let dim = Vec2::new(window.width(), window.height());
    for (cursor, mut transform, dimension) in query.iter_mut() {
        transform.offset = (-dim * transform.get_parent_anchor()
            //- dimension.size * transform.anchor
            + cursor.0.as_pixels(dim, dimension.em, rem.get())
            + mouse_pos
        ).into()
    }
}
