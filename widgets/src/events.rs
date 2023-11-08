use std::{ops::{BitAnd, BitOr}, mem::replace};

use bevy::{prelude::*, input::mouse::MouseWheel, window::PrimaryWindow};
use bevy_aoui::{query_hitbox, RotatedRect, Hitbox, AoUI};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EventFlags<const MB: char='L'>(u32);

impl<const MB: char> EventFlags<MB> {
    pub const CLICK: Self = Self(1);
    pub const HOVER: Self = Self(2);
    pub const DOWN: Self = Self(4);
    pub const UP: Self = Self(8);
    pub const DRAG: Self = Self(16);
    pub const PRESSED: Self = Self(32);
}

impl<const MB: char> BitAnd for EventFlags<MB> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const MB: char> BitOr for EventFlags<MB> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}


/// Handle order:
/// down -> up
/// (Click, DragEnd) -> Up
#[derive(Debug, Event)]
pub struct MouseDown<const MB: char='L'>{
    position: Vec2,
    entity: Option<Entity>,
}

/// Handle order:
/// down -> up
/// (Click, DragEnd) -> Up
#[derive(Debug, Event)]
pub struct MouseDrag<const MB: char='L'>{
    position: Vec2,
    delta: Vec2,
    down: Vec2,
    entity: Entity,
}

#[derive(Debug, Event)]
pub struct MousePressed<const MB: char='L'>{
    position: Vec2,
    delta: Vec2,
    down: Vec2,
    entity: Option<Entity>,
}

#[derive(Debug, Event)]
pub struct MouseHover<const MB: char='L'>{
    position: Vec2,
    delta: Vec2,
    entity: Option<Entity>,
}
/// Handle order:
/// down -> up
/// (Click, DragEnd) -> Up
#[derive(Debug, Event)]
pub struct MouseDragUp<const MB: char='L'>{
    position: Vec2,
    down: Vec2,
    entity: Entity,
}


/// Handle order:
/// down -> up
/// (Click, DragEnd) -> Up
#[derive(Debug, Event)]
pub struct MouseUp<const MB: char='L'>{
    position: Vec2,
    down: Vec2,
    entity: Option<Entity>,
}


#[derive(Debug, Event)]
pub struct MouseClick<const MB: char='L'>{
    position: Vec2,
    down: Vec2,
    entity: Entity,
}

#[derive(Debug, Default)]
#[doc(hidden)]
pub struct MouseHistory {
    pub last: Vec2,
    pub down_at: Vec2,
}

pub fn fetch_events<const MB: char>(
    window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    wheel: EventReader<MouseWheel>,
    mut down_event: EventWriter<MouseDown<MB>>,
    mut up_event: EventWriter<MouseUp<MB>>,
    mut press_event: EventWriter<MousePressed<MB>>,
    mut hover_event: EventWriter<MouseHover<MB>>,
    mut click_event: EventWriter<MouseClick<MB>>,
    mut drag_event: EventWriter<MouseDrag<MB>>,
    mut drag_up_event: EventWriter<MouseDragUp<MB>>,
    mut cache: Local<MouseHistory>,
    query: Query<(Entity, &RotatedRect, &Hitbox), With<AoUI>>
) {
    let mut cursor_in_window = true;
    let position = match window.get_single() {
        Ok(x) => x.cursor_position()
            .unwrap_or_else(|| {
                cursor_in_window = false;
                cache.last
            }),
        Err(_) => return,
    };
    let mouse_button = match MB {
        'L' => MouseButton::Left,
        'M' => MouseButton::Middle,
        'R' => MouseButton::Right,
        '0' => MouseButton::Other(0),
        '1' => MouseButton::Other(1),
        '2' => MouseButton::Other(2),
        '3' => MouseButton::Other(3),
        '4' => MouseButton::Other(4),
        '5' => MouseButton::Other(5),
        '6' => MouseButton::Other(6),
        '7' => MouseButton::Other(7),
        '8' => MouseButton::Other(8),
        '9' => MouseButton::Other(9),
        c => panic!("Unsupported mouse button {c}.")
    };
    let down = cache.down_at;
    let delta = position - replace(&mut cache.last, position);
    if mouse.just_pressed(mouse_button) {
        cache.down_at = position;
        let entity = query_hitbox(query.iter(), [position]);
        down_event.send(MouseDown { 
            position, 
            entity,
        })
    } else if mouse.just_released(mouse_button) {
        if let Some(entity) = query_hitbox(query.iter(), [down]) {
            drag_up_event.send(MouseDragUp { 
                position, 
                down,
                entity,
            });
            up_event.send(MouseUp { 
                position, 
                down,
                entity: Some(entity),
            });
        } else if let Some(entity) = query_hitbox(query.iter(), [position, down]) {
            click_event.send(MouseClick { 
                position, 
                down,
                entity,
            });
            up_event.send(MouseUp { 
                position, 
                down,
                entity: Some(entity),
            });
        } else {
            let entity = query_hitbox(query.iter(), [position]);
            up_event.send(MouseUp { 
                position, 
                down,
                entity,
            })
        }
    } else if mouse.pressed(mouse_button) {
        if let Some(entity) = query_hitbox(query.iter(), [down]) {
            drag_event.send(MouseDrag { 
                position, 
                delta,
                down,
                entity,
            })
        }
        let entity = query_hitbox(query.iter(), [position]);
        press_event.send(MousePressed { 
            position, 
            delta,
            down,
            entity,
        })
    } else if cursor_in_window && MB == 'L' 
            && !mouse.any_pressed([MouseButton::Left, MouseButton::Middle, MouseButton::Right]) {
        let entity = query_hitbox(query.iter(), [position]);
        hover_event.send(MouseHover { 
            position, 
            delta,
            entity,
        })
    }
}

// pub fn fetch_events_touches<const ONE: bool>(
//     inputs: Res<Touches>,
//     mut touches: EventReader<TouchInput>,
//     mut hover: EventWriter<RawMouseHover>,
//     mut up: EventWriter<RawMouseUp>,
//     mut down: EventWriter<RawMouseDown>,
//     mut click: EventWriter<RawMouseClick>,
//     mut click2: EventWriter<MouseClick>,

// ) {
//     for input in inputs.iter() {
//         hover.send(RawMouseHover(input.position()));
//         if ONE { break; }
//     }
//     for touch in touches.iter() {
//         match touch.phase {
//             TouchPhase::Started => {
//                 down.send(RawMouseDown(touch.position));
//             },
//             TouchPhase::Ended => {
//                 up.send(RawMouseUp(touch.position));
//                 click.send(RawMouseClick(touch.position));
//             },
//             TouchPhase::Canceled => {
//                 // Send a "click outside" event.
//                 click2.send(MouseClick { 
//                     position: touch.position, 
//                     entity: None 
//                 })
//             },
//             TouchPhase::Moved => (),
//         }
//     }
// }