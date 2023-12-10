use bevy::{ecs::{system::{Resource, Commands, EntityCommands}, entity::Entity}, math::Vec2, input::mouse::MouseButton};

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
    pub(super) last_lmb_down_time: [f32; 2],
    pub(super) cursor_pos: Vec2,
    pub(super) up_pos: Vec2,
    pub(super) down_pos: Vec2,
    pub(super) blocked: bool,
    pub(super) catched: bool,
    pub(super) dragging: bool,
    pub(super) drag_button: MouseButton,
    pub(super) drag_target: Option<Entity>,
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
            catched: false,
            drag_dbl_click: false,
        }
    }
}

impl CursorState {

    /// Check if mouse event is catched by AoUI this frame.
    pub fn catched(&self) -> bool {
        self.catched
    }

    /// Call if some external system catched mouse events this frame before this.
    /// 
    /// Does not cancel dragging.
    pub fn block(&mut self) {
        if !self.dragging {
            self.last_lmb_down_time = [0.0, 0.0];
            self.blocked = true;
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
    pub fn cancel_drag(&mut self) {
        self.drag_target = None;
    }
    
    /// Cancels dragging of the current entity, does not reset mouse state.
    pub fn clear_dbl_click(&mut self) {
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