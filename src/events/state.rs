use bevy::{ecs::entity::Entity, input::mouse::MouseButton, math::Vec2, reflect::Reflect};
use bevy::ecs::system::{Resource, Commands, EntityCommands};

/// Time threshold in seconds for double click.
#[derive(Debug, Resource, Reflect)]
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
#[derive(Debug, Resource, Reflect)]
pub struct CursorState{
    pub(super) last_lmb_down_time: [f32; 2],
    pub(super) cursor_pos: Vec2,
    pub(super) up_pos: Vec2,
    pub(super) down_pos: Vec2,
    pub(super) blocked: bool,
    pub(super) caught: bool,
    pub(super) dragging: bool,
    pub(super) drag_button: MouseButton,
    pub(super) drag_target: Option<Entity>,
    pub(super) focused: Option<Entity>,
    pub(super) drag_dbl_click: bool,
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
            focused: None,
            caught: false,
            drag_dbl_click: false,
        }
    }
}

impl CursorState {

    /// Check if mouse event is handled by us this frame.
    pub fn is_handled_this_frame(&self) -> bool {
        self.caught
    }

    /// Call if some external system caught mouse events this frame before this.
    ///
    /// Does not cancel dragging.
    pub fn block(&mut self) {
        if !self.dragging {
            self.last_lmb_down_time = [0.0, 0.0];
            self.blocked = true;
        }
    }

    /// Call if some external system caught mouse events this frame before this.
    ///
    /// Force dragging to end.
    pub fn block_force(&mut self) {
        self.last_lmb_down_time = [0.0, 0.0];
        self.blocked = true;
        self.drag_target = None;
        self.dragging = false;
    }

    /// Cancels dragging of the current entity, does not reset mouse state.
    pub fn cancel_drag(&mut self) {
        self.drag_target = None;
    }

    /// Cancels dragging of the current entity, does not reset mouse state.
    pub fn clear_dbl_click(&mut self) {
        self.last_lmb_down_time = [0.0, 0.0];
    }

    /// This guarantees the existence of the entity.
    pub fn drag_target<'t>(&self, commands: &'t mut Commands) -> Option<EntityCommands<'t>> {
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
