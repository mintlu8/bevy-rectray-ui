use std::ops::BitOr;

use bevy::{prelude::*, window::{Window, PrimaryWindow}, ecs::system::EntityCommands};
use bevy_aoui::{RotatedRect, Hitbox};

#[derive(Debug, Resource)]
pub struct DoubleClickThreshold(f32);

impl Default for DoubleClickThreshold {
    fn default() -> Self {
        Self(0.3)
    }
}

impl DoubleClickThreshold {
    pub fn new(timespan: f32) -> Self {
        Self(timespan)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

/// State of the cursor system.
#[derive(Debug, Resource)]
pub struct CursorState{
    last_lmb_down_time: [f32; 2],
    cursor_pos: Vec2,
    up_pos: Vec2,
    down_pos: Vec2,
    blocked: bool,
    catched: bool,
    dragging: bool,
    drag_button: MouseButton,
    drag_target: Option<Entity>,
    drag_dbl_click: bool,
}

impl Default for CursorState {
    fn default() -> Self {
        Self { 
            last_lmb_down_time: [0.0, 0.0], 
            cursor_pos: Vec2::ZERO, 
            up_pos: Vec2::ZERO, 
            down_pos: Vec2::ZERO, 
            blocked: false, 
            dragging: false, 
            drag_button: MouseButton::Left, 
            drag_target: None, 
            catched: false,
            drag_dbl_click: false,
        }
    }
}

/// Represents hovering, clicking or dragging.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct CursorFocus(EventFlags);

impl CursorFocus {
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
}

/// Represents a cursor event like `OnMouseDown`.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct CursorAction(EventFlags);

impl CursorAction {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
}


/// Represents cursor clicking outside the sprite's hitbox.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct CursorClickOutside;

/// Captures a mouse up outside event.
pub struct MouseUpSubscriber(bool);

impl CursorState {

    /// Check if mouse event is catched by AoUI this frame.
    pub fn catched(&self) -> bool {
        self.catched
    }

    /// Call if some external system catched mouse events this frame before this.
    /// 
    /// Does not cancel dragging.
    pub fn block(&mut self) {
        if self.drag_target != None {
            self.last_lmb_down_time = [0.0, 0.0];
            self.blocked = true;
            self.dragging = false;
        }
    }

    /// Call if some external system catched mouse events this frame before this.
    pub fn block_force(&mut self) {
        self.last_lmb_down_time = [0.0, 0.0];
        self.blocked = true;
        self.drag_target = None;
        self.dragging = false;
    }

    /// This guarantees the existance of the entity.
    pub fn drag_target<'w, 's, 't>(&self, commands: &'t mut Commands<'w, 's>) -> Option<EntityCommands<'w, 's, 't>> {
        commands.get_entity(self.drag_target?)
    }

    pub fn down_position(&self) -> Vec2 {
        self.down_pos
    }

    pub fn up_position(&self) -> Vec2 {
        self.up_pos
    }

    pub fn cursor_position(&self) -> Vec2 {
        self.cursor_pos
    }

    pub fn dragging(&self) -> bool {
        self.dragging
    }

    pub fn drag_button(&self) -> MouseButton {
        self.drag_button
    }
}

/// `CLICK`, `DRAG`, `HOVER`, `DROP`, `CLICK_OUTSIDE` are valid component flags.
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct EventFlags(u16);

impl EventFlags{
    pub const DRAG: Self = Self(1);
    pub const DOWN: Self = Self(2);
    pub const CLICK: Self = Self(4);
    pub const DOUBLE_CLICK: Self = Self(8);
    pub const HOVER: Self = Self(16);

    pub const MID_DOWN: Self = Self(32);
    pub const MID_CLICK: Self = Self(64);
    pub const MID_DRAG: Self = Self(128);

    pub const RIGHT_DOWN: Self = Self(256);
    pub const RIGHT_CLICK: Self = Self(512);
    pub const RIGHT_DRAG: Self = Self(1024);

    pub const DROP: Self = Self(2048);
    pub const DRAG_END: Self = Self(4096);

    pub const CLICK_OUTSIDE: Self = Self(8192);

    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}
impl BitOr for EventFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs. 0)
    }
}

/// Remove [`CursorFocus`] and [`CursorAction`];
pub fn remove_focus(mut commands: Commands, 
    query1: Query<Entity, With<CursorFocus>>, 
    query2: Query<Entity, With<CursorAction>>,
    query3: Query<Entity, With<CursorClickOutside>>,
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
}

/// We hand out component [`CursorFocus`] for persistant states,
/// [`CursorAction`] for active events.
/// and [`CursorClickOutside`] for cancelling.
/// These should be handled on this frame during [`Update`].
pub fn mouse_button_input(
    mut commands: Commands,
    mut state: ResMut<CursorState>,
    time: Res<Time>,
    double_click: Res<DoubleClickThreshold>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    query: Query<(Entity, &RotatedRect, &Hitbox, &EventFlags)>,
) {
    fn drop<T>(_: T) {}
    state.catched = false;
    if state.blocked { return; }
    let Ok((camera, camera_transform)) = camera.get_single() else { return };    
    let Ok(window) = windows.get_single() else { return };       
    let Some(mouse_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) else {return;};
    state.cursor_pos = mouse_pos;
    if buttons.pressed(MouseButton::Left) {
        if state.dragging { 
            if let Some(mut target) = state.drag_target(&mut commands){
                target.insert(CursorFocus(EventFlags::DRAG));
                state.catched = true; 
                return;
            } else {
                state.dragging = false;
                state.drag_target = None;
            }
        }
        if buttons.just_pressed(MouseButton::Left) { 
            state.down_pos = mouse_pos;
            let [_, last] = state.last_lmb_down_time;
            state.last_lmb_down_time = [last, time.elapsed_seconds()];
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)| flags.intersects(EventFlags::DRAG | EventFlags::CLICK))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(entity).insert(CursorAction(EventFlags::DOWN));
                if flag.contains(EventFlags::DRAG) {
                    state.drag_target = Some(entity);
                    state.dragging = true;
                    state.drag_button = MouseButton::Left;
                    state.drag_dbl_click = flag.contains(EventFlags::DOUBLE_CLICK);
                    commands.entity(entity).insert(CursorFocus(EventFlags::DRAG));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::DOWN));
                }
            } else if flag.contains(EventFlags::CLICK) {
                commands.entity(entity).insert(CursorFocus(EventFlags::CLICK));
            }
        }
    } else if buttons.pressed(MouseButton::Right) {
        if state.dragging { 
            if let Some(mut target) = state.drag_target(&mut commands){
                target.insert(CursorFocus(EventFlags::RIGHT_DRAG));
                state.catched = true; 
                return;
            } else {
                state.dragging = false;
                state.drag_target = None;
            }
        }
        if buttons.just_pressed(MouseButton::Right) { 
            state.down_pos = mouse_pos
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)| flags.intersects(EventFlags::RIGHT_DRAG | EventFlags::RIGHT_CLICK))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Right) {
                commands.entity(entity).insert(CursorAction(EventFlags::RIGHT_DOWN));
                if flag.contains(EventFlags::RIGHT_DRAG) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Right;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::RIGHT_DRAG));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::RIGHT_DOWN));
                }
            } else if flag.contains(EventFlags::RIGHT_CLICK) {
                commands.entity(entity).insert(CursorFocus(EventFlags::RIGHT_CLICK));
            }
        }
    } else if buttons.pressed(MouseButton::Middle) {
        if state.dragging { 
            if let Some(mut target) = state.drag_target(&mut commands){
                target.insert(CursorFocus(EventFlags::MID_DRAG));
                state.catched = true; 
                return;
            } else {
                state.dragging = false;
                state.drag_target = None;
            }
        }
        if buttons.just_pressed(MouseButton::Middle) { 
            state.down_pos = mouse_pos 
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)|flags.intersects(EventFlags::MID_DRAG | EventFlags::MID_CLICK))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Middle) {
                state.down_pos = mouse_pos;
                commands.entity(entity).insert(CursorAction(EventFlags::MID_DOWN));
                if flag.contains(EventFlags::MID_DRAG) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Middle;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::MID_DRAG));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::MID_DOWN));
                }
            } else if flag.contains(EventFlags::MID_CLICK) {
                commands.entity(entity).insert(CursorFocus(EventFlags::MID_CLICK));
            }
        }
    } else if state.dragging {
        state.catched = true;
        state.dragging = false;
        if let Some(drag) = state.drag_target { 
            if let Some(mut entity) = commands.get_entity(drag) {
                if state.drag_dbl_click && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                    entity.insert(CursorAction(EventFlags::DOUBLE_CLICK));
                } else {
                    entity.insert(CursorAction(EventFlags::DRAG_END));
                }
            }
            query.iter().filter(|(.., flags)| flags.contains(EventFlags::DROP))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity,..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::DROP))));
        }
    } else if buttons.just_released(MouseButton::Left) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(EventFlags::CLICK))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, .., flags)| 
                if flags.contains(EventFlags::DOUBLE_CLICK) && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                    commands.entity(entity).insert(CursorAction(EventFlags::DOUBLE_CLICK));
                } else {
                    commands.entity(entity).insert(CursorAction(EventFlags::CLICK));
                }
            )
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| { flags.contains(EventFlags::CLICK_OUTSIDE) })
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else if buttons.just_released(MouseButton::Right) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(EventFlags::RIGHT_CLICK))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::RIGHT_CLICK))))
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| { flags.contains(EventFlags::CLICK_OUTSIDE) })
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else if buttons.just_released(MouseButton::Middle) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(EventFlags::MID_CLICK))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::MID_CLICK))))
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| { flags.contains(EventFlags::CLICK_OUTSIDE) })
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else {
        query.iter().filter(|(.., flags)| flags.intersects(EventFlags::HOVER))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorFocus(EventFlags::HOVER))))
            .map(|_| state.catched = true);
    }
}
