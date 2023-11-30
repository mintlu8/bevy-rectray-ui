use bevy::{ecs::{system::Query, component::Component}, hierarchy::Children};
use bevy_aoui::{Dimension, Transform2D, Anchor};

use crate::events::MouseWheelAction;

#[derive(Debug, Clone, Copy, Component)]
pub struct Scrolling {
    x: bool,
    y: bool,
}

impl Default for Scrolling {
    fn default() -> Self {
        Self { x: true, y: true }
    }
}

pub fn scrolling(
    scroll: Query<(&Scrolling, &Dimension, &Children, &MouseWheelAction)>,
    child: Query<(&Dimension, &mut Transform2D)>
) {
    for (scroll, dimension, children, action) in scroll.iter() {
        if scroll.x {
            let value = if scroll.y {
                action.get().x
            } else {
                action.get().x + action.get().y
            };
            for entity in children {
            }
        }
    }
}