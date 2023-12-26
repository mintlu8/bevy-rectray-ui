use bevy::{prelude::*, window::{Window, PrimaryWindow}};

use crate::widgets::scrollframe::CameraClip;

use super::*;


/// Remove [`CursorFocus`], [`CursorAction`], [`CursorClickOutside`] and [`Submit`];
pub fn remove_focus(mut commands: Commands, 
    query1: Query<Entity, With<CursorFocus>>, 
    query2: Query<Entity, With<CursorAction>>,
    query3: Query<Entity, With<CursorClickOutside>>,
    query4: Query<Entity, With<MouseWheelAction>>,
) {
    for entity in query1.iter() {
        commands.entity(entity).remove::<CursorFocus>();
    }
    for entity in query2.iter() {
        commands.entity(entity).remove::<CursorAction>();
    }
    for entity in query3.iter() {
        commands.entity(entity).remove::<CursorClickOutside>();
    }
    for entity in query4.iter() {
        commands.entity(entity).remove::<MouseWheelAction>();
    }
}


/// We hand out component [`CursorFocus`] for persistant states,
/// [`CursorAction`] for active events.
/// and [`CursorClickOutside`] for cancelling.
/// These should be handled on this frame during [`Update`].
#[allow(clippy::too_many_arguments)]
#[allow(clippy::option_map_unit_fn)]
pub fn mouse_button_input(
    mut commands: Commands,
    mut state: ResMut<CursorState>,
    time: Res<Time>,
    double_click: Res<DoubleClickThreshold>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    marked_camera: Query<(&Camera, &GlobalTransform), With<AouiCamera>>,
    unmarked_camera: Query<(&Camera, &GlobalTransform), (Without<AouiCamera>, Without<CameraClip>)>,
    query: Query<(Entity, &EventFlags, CursorDetection, ActiveDetection)>,
) {
    fn drop<T>(_: T) {}
    let iter = |f: EventFlags|query.iter().filter_map(move |(entity, flag, cursor, detection)| {
        if detection.is_active() && flag.intersects(f) {
            Some((entity, flag, cursor))
        } else {
            None
        }
    });
    state.catched = false;
    if state.blocked { return; }
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
    state.cursor_pos = mouse_pos;
    if state.dragging {
        state.catched = true;
        if let Some(mut entity) = state.drag_target(&mut commands) {
            if !buttons.pressed(state.drag_button) {
                if state.drag_dbl_click && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                    entity.insert(CursorAction(EventFlags::DoubleClick));
                    state.clear_dbl_click();
                } else {
                    entity.insert(CursorAction(EventFlags::DragEnd));
                }
                state.dragging = false;
                state.drag_target = None;
                let dragged_id = entity.id();
                iter(EventFlags::Drop)
                    .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
                    .max_by(|(.., a), (.., b)| a.z().total_cmp(&b.z()))
                    .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::Drop))));
                iter(EventFlags::ClickOutside)
                    .filter(|(e, ..)| e != &dragged_id)
                    .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                    .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
            } else {
                if state.drag_button != MouseButton::Left && buttons.just_pressed(MouseButton::Left) {
                    entity.insert(CursorAction(EventFlags::LeftDown));
                } else if state.drag_button != MouseButton::Right && buttons.just_pressed(MouseButton::Right) {
                    entity.insert(CursorAction(EventFlags::RightDown));
                } else if state.drag_button != MouseButton::Middle && buttons.just_pressed(MouseButton::Middle) {
                    entity.insert(CursorAction(EventFlags::MidDown));
                }
                entity.insert(CursorFocus(match state.drag_button {
                    MouseButton::Left => EventFlags::LeftDrag,
                    MouseButton::Right => EventFlags::RightDrag,
                    MouseButton::Middle => EventFlags::MidDrag,
                    MouseButton::Other(_) => EventFlags::LeftDrag,
                }));
            }
        } else if !buttons.pressed(state.drag_button) {
            state.dragging = false;
            state.drag_target = None;
            iter(EventFlags::ClickOutside)
            .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
        }
    } else if buttons.pressed(MouseButton::Left) {
        if buttons.just_pressed(MouseButton::Left) { 
            state.down_pos = mouse_pos;
            let [_, last] = state.last_lmb_down_time;
            state.last_lmb_down_time = [last, time.elapsed_seconds()];
        }
        if let Some((entity, flag)) = iter(EventFlags::LeftDrag|EventFlags::LeftClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, flags, _)| (entity, flags)
            ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(entity).insert(CursorAction(EventFlags::LeftDown));
                if flag.contains(EventFlags::LeftDrag) {
                    state.drag_target = Some(entity);
                    state.dragging = true;
                    state.drag_button = MouseButton::Left;
                    state.drag_dbl_click = flag.contains(EventFlags::DoubleClick);
                    commands.entity(entity).insert(CursorFocus(EventFlags::LeftDrag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::LeftPressed));
                }
            } else if flag.contains(EventFlags::LeftClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::LeftPressed));
            }
        }
    } else if buttons.pressed(MouseButton::Right) {
        if buttons.just_pressed(MouseButton::Right) { 
            state.down_pos = mouse_pos
        }
        if let Some((entity, flag)) = iter(EventFlags::RightDrag|EventFlags::RightClick)
            .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
            .max_by(|(.., a), (.., b)| a.compare(b))
            .map(|(entity, flags, _)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Right) {
                commands.entity(entity).insert(CursorAction(EventFlags::RightDown));
                if flag.contains(EventFlags::RightDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Right;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightDrag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
                }
            } else if flag.contains(EventFlags::RightClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
            }
        }
    } else if buttons.pressed(MouseButton::Middle) {
        if buttons.just_pressed(MouseButton::Middle) { 
            state.down_pos = mouse_pos 
        }
        if let Some((entity, flag)) = iter(EventFlags::MidDrag|EventFlags::MidClick)
            .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
            .max_by(|(.., a), (.., b)| a.compare(b))
            .map(|(entity, flags, _)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Middle) {
                state.down_pos = mouse_pos;
                commands.entity(entity).insert(CursorAction(EventFlags::MidDown));
                if flag.contains(EventFlags::MidDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Middle;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidDrag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
                }
            } else if flag.contains(EventFlags::MidClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
            }
        }
    } else {
        if buttons.just_released(MouseButton::Left) {
            let down = state.down_pos;
            iter(EventFlags::LeftClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, flags, _)| 
                    if flags.contains(EventFlags::DoubleClick) && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                        commands.entity(entity).insert(CursorAction(EventFlags::DoubleClick));
                        state.clear_dbl_click();
                    } else {
                        commands.entity(entity).insert(CursorAction(EventFlags::LeftClick));
                    }
                )
                .map(|_| state.catched = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
        } else if buttons.just_released(MouseButton::Right) {
            let down = state.down_pos;
            iter(EventFlags::RightClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::RightClick))))
                .map(|_| state.catched = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
        } else if buttons.just_released(MouseButton::Middle) {
            let down = state.down_pos;
            iter(EventFlags::MidClick)
                .filter(|(.., hitbox)| hitbox.contains(mouse_pos) && hitbox.contains(down))
                .max_by(|(.., a), (.., b)| a.compare(b))
                .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::MidClick))))
                .map(|_| state.catched = true);
            iter(EventFlags::ClickOutside)
                .filter(|(.., hitbox)| !hitbox.contains(mouse_pos))
                .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
        }
        iter(EventFlags::Hover)
            .filter(|(.., hitbox)| hitbox.contains(mouse_pos))
            .max_by(|(.., a), (.., b)| a.compare(b))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorFocus(EventFlags::Hover))))
            .map(|_| state.catched = true);
    }   
}
