use bevy::{prelude::*, window::{Window, PrimaryWindow}, ecs::system::EntityCommands};
use bevy_aoui::{RotatedRect, Hitbox};

use crate::dto::Submit;

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
#[derive(Debug, Component, Clone, Copy)]
#[component(storage="SparseSet")]
pub struct CursorFocus(EventFlags);

impl CursorFocus {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
}

/// Represents a cursor event like `OnMouseDown`.
#[derive(Debug, Component, Clone, Copy)]
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

    /// Cancels dragging of the current entity, does not reset mouse state.
    fn cancel_drag(&mut self) {
        self.drag_target = None;
    }
    
    /// Cancels dragging of the current entity, does not reset mouse state.
    fn clear_dbl_click(&mut self) {
        self.last_lmb_down_time = [0.0, 0.0];
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

tlbf::tlbf!(
    #[derive(Component)]
    /// Flags for cursor events.
    /// 
    /// Valid listeners are `Hover`, `*Click`, `*Drag`, `DoubleClick`, `Drop` and `ClickOutside`.
    /// 
    /// * `Hover` listens for `Hover`,
    /// * `Click` listens for `Down`, `Up` and `Pressed`
    /// * `Drag` listens for `Down`, `DragEnd` and `Drag`
    /// * `DoubleClick` listens for `DoubleClick`, which replaces `Click` or `DragEnd`
    /// * `Drop` listens for `Drop`
    /// * `ClickOutside` listens for mouse up outside.
    /// 
    /// Events are emitted as 3 separate components, each frame a sprite can receive at most one of each:
    /// * `CursorFocus`: `Hover`, `Down`, `Drag`
    /// * `CursorAction`: `Down`, `Click`, `DragEnd`, `DoubleClick`, `Drop`
    /// * `CursorClickOutside`: `ClickOutside`
    /// 
    /// Details:
    /// * `Click` requires mouse up and mouse down be both inside a sprite.
    /// * `ClickOutside` requires mouse up be outside of a sprite and the sprite not being dragged.
    /// * Dragged sprite will receive `Down` from other mouse buttons regardless of their handlers.
    /// * There is in fact no `MouseUp`.
    pub EventFlags: u32 {
        Idle,
        Hover,
        Drag,
        Down,
        Pressed,
        Click,
        DoubleClick,
        MidDown,
        MidPressed,
        MidClick,
        MidDrag,
        RightDown,
        RightPressed,
        RightClick,
        RightDrag,
        Drop,
        DragEnd,
        ClickOutside,
    }
);

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
                query.iter().filter(|(.., flags)| flags.contains(Drop))
                    .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                    .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                    .map(|(entity,..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::Drop))));
                query.iter().filter(|(.., flags)| { flags.contains(ClickOutside) })
                    .filter(|(e, ..)| e != &dragged_id)
                    .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
                    .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
            } else {
                if state.drag_button != MouseButton::Left && buttons.just_pressed(MouseButton::Left) {
                    entity.insert(CursorAction(EventFlags::Down));
                } else if state.drag_button != MouseButton::Right && buttons.just_pressed(MouseButton::Right) {
                    entity.insert(CursorAction(EventFlags::RightDown));
                } else if state.drag_button != MouseButton::Middle && buttons.just_pressed(MouseButton::Middle) {
                    entity.insert(CursorAction(EventFlags::MidDown));
                }
                entity.insert(CursorFocus(match state.drag_button {
                    MouseButton::Left => EventFlags::Drag,
                    MouseButton::Right => EventFlags::RightDrag,
                    MouseButton::Middle => EventFlags::MidDrag,
                    MouseButton::Other(_) => EventFlags::Drag,
                }));
            }
        } else if !buttons.pressed(state.drag_button) {
            state.dragging = false;
            state.drag_target = None;
            query.iter().filter(|(.., flags)| { flags.contains(ClickOutside) })
                .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
                .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
        }
    } else if buttons.pressed(MouseButton::Left) {
        if buttons.just_pressed(MouseButton::Left) { 
            state.down_pos = mouse_pos;
            let [_, last] = state.last_lmb_down_time;
            state.last_lmb_down_time = [last, time.elapsed_seconds()];
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)| flags.intersects(Drag|Click))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(entity).insert(CursorAction(EventFlags::Down));
                if flag.contains(Drag) {
                    state.drag_target = Some(entity);
                    state.dragging = true;
                    state.drag_button = MouseButton::Left;
                    state.drag_dbl_click = flag.contains(DoubleClick);
                    commands.entity(entity).insert(CursorFocus(EventFlags::Drag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::Pressed));
                }
            } else if flag.contains(Click) {
                commands.entity(entity).insert(CursorFocus(EventFlags::Pressed));
            }
        }
    } else if buttons.pressed(MouseButton::Right) {
        if buttons.just_pressed(MouseButton::Right) { 
            state.down_pos = mouse_pos
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)| flags.intersects(RightDrag|RightClick))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Right) {
                commands.entity(entity).insert(CursorAction(EventFlags::RightDown));
                if flag.contains(RightDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Right;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightDrag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
                }
            } else if flag.contains(RightClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::RightPressed));
            }
        }
    } else if buttons.pressed(MouseButton::Middle) {
        if buttons.just_pressed(MouseButton::Middle) { 
            state.down_pos = mouse_pos 
        }
        if let Some((entity, flag)) = query.iter().filter(|(.., flags)|flags.intersects(MidDrag|MidClick))
                .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
                .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
                .map(|(entity, _, _, flags)| (entity, flags)
        ) {
            state.catched = true;
            if buttons.just_pressed(MouseButton::Middle) {
                state.down_pos = mouse_pos;
                commands.entity(entity).insert(CursorAction(EventFlags::MidDown));
                if flag.contains(MidDrag) {
                    state.drag_target = Some(entity);
                    state.drag_button = MouseButton::Middle;
                    state.drag_dbl_click = false;
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidDrag));
                } else {
                    commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
                }
            } else if flag.contains(MidClick) {
                commands.entity(entity).insert(CursorFocus(EventFlags::MidPressed));
            }
        }
    } else if buttons.just_released(MouseButton::Left) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(Click))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, .., flags)| 
                if flags.contains(DoubleClick) && time.elapsed_seconds() - state.last_lmb_down_time[0] <= double_click.get() {
                    commands.entity(entity).insert(CursorAction(EventFlags::DoubleClick));
                    state.clear_dbl_click();
                } else {
                    commands.entity(entity).insert(CursorAction(EventFlags::Click));
                }
            )
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| flags.contains(ClickOutside))
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else if buttons.just_released(MouseButton::Right) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(RightClick))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::RightClick))))
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| flags.contains(ClickOutside))
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else if buttons.just_released(MouseButton::Middle) {
        let down = state.down_pos;
        query.iter().filter(|(.., flags)| flags.contains(MidClick))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos) && hitbox.contains(rect, down))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorAction(EventFlags::MidClick))))
            .map(|_| state.catched = true);
        query.iter().filter(|(.., flags)| flags.contains(ClickOutside))
            .filter(|(_, rect, hitbox, _)| !hitbox.contains(rect, mouse_pos))
            .for_each(|(entity, ..)| drop(commands.entity(entity).insert(CursorClickOutside)));
    } else {
        query.iter().filter(|(.., flags)| flags.intersects(Hover))
            .filter(|(_, rect, hitbox, _)| hitbox.contains(rect, mouse_pos))
            .max_by(|(_, a, ..), (_, b, ..)| a.z.total_cmp(&b.z))
            .map(|(entity, ..)| drop(commands.entity(entity).insert(CursorFocus(EventFlags::Hover))))
            .map(|_| state.catched = true);
    }
}

/// Remove [`CursorFocus`], [`CursorAction`] and [`CursorClickOutside`];
pub fn remove_focus(mut commands: Commands, 
    query1: Query<Entity, With<CursorFocus>>, 
    query2: Query<Entity, With<CursorAction>>,
    query3: Query<Entity, With<CursorClickOutside>>,
    query4: Query<Entity, With<Submit>>,
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
        commands.entity(entity).remove::<Submit>();
    }
}
