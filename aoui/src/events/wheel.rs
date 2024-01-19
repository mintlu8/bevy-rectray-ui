use bevy::{input::mouse::{MouseWheel, MouseScrollUnit}, math::{Vec2, IVec2}, window::{Window, PrimaryWindow}, render::camera::Camera, transform::components::GlobalTransform, ecs::{component::Component, system::{Resource, Local, Res}}};
use bevy::ecs::{system::{Query, Commands}, event::EventReader, query::{With, Without}, entity::Entity};

use crate::{widgets::clipping::CameraClip, sync::{SignalId, StateId}};

use super::{EventFlags, AouiCamera, CursorDetection, ActiveDetection};



/// Resource that determines the direction and magnitude of mouse wheel scrolling.
#[derive(Debug, Clone, Copy, Resource)]
pub struct ScrollScaling{
    pub line_to_pixels: Vec2,
    pub pixel_scale: Vec2,
}

impl Default for ScrollScaling {
    fn default() -> Self {
        Self {
            line_to_pixels: Vec2::new(16.0, 16.0),
            // Satisfies bevy's coordinate system
            pixel_scale: Vec2::new(1.0, -1.0),
        }
    }
}

/// Movement units associated with dragging or scrolling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementUnits{
    pub lines: IVec2,
    pub pixels: Vec2,
}

/// This is relatively independent, as the mousewheel action does not take
/// the drag target and the cursor action target into account.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
#[component(storage="SparseSet")]
pub struct MouseWheelAction(pub MovementUnits);

impl MouseWheelAction {
    pub fn get(&self) -> MovementUnits {
        self.0
    }
}

impl MovementUnits {
    pub const ZERO: Self = Self {
        lines: IVec2::ZERO,
        pixels: Vec2::ZERO
    };
}

impl SignalId for MouseWheelAction {
    type Data = MovementUnits;
}

impl StateId for MouseWheelAction {
    type Data = MovementUnits;
}


pub(crate) fn mousewheel_event(
    mut commands: Commands,
    scaling: Res<ScrollScaling>,
    windows: Query<&Window, With<PrimaryWindow>>,
    marked_camera: Query<(&Camera, &GlobalTransform), With<AouiCamera>>,
    unmarked_camera: Query<(&Camera, &GlobalTransform), (Without<AouiCamera>, Without<CameraClip>)>,
    query: Query<(Entity, &EventFlags, ActiveDetection, CursorDetection)>,
    mut lines: Local<Vec2>,
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
    if let Some(entity) = query.iter()
        .filter(|(_, flags, active, hitbox)| flags.contains(EventFlags::MouseWheel) && active.is_active() && hitbox.contains(mouse_pos))
        .max_by(|(.., a), (.., b)| a.compare(b))
        .map(|(entity,..)| entity) {

        let mut count = 0;
        for event in reader.read() {
            count += 1;
            match event.unit {
                MouseScrollUnit::Line => {
                    let lines = Vec2::new(event.x, event.y);
                    let pixels = lines * scaling.line_to_pixels * scaling.pixel_scale;
                    commands.entity(entity).insert(MouseWheelAction(MovementUnits{
                        lines: lines.as_ivec2(),
                        pixels,
                    }));
                },
                MouseScrollUnit::Pixel => {
                    let pixels = Vec2::new(event.x, event.y) * scaling.pixel_scale;
                    *lines += pixels;
                    let count = (*lines / scaling.line_to_pixels).as_ivec2();
                    *lines %= scaling.line_to_pixels;
                    commands.entity(entity).insert(MouseWheelAction(MovementUnits{
                        lines: count,
                        pixels,
                    }));
                },
            }
        }
        if count == 0 {
            *lines = Vec2::ZERO;
        }
    }
}
