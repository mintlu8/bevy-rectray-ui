use bevy::{ecs::{system::{Query, Resource, Res, Commands}, component::Component, query::With}, render::view::Visibility, window::{Window, PrimaryWindow, CursorIcon}, hierarchy::Children};

use crate::events::{EventFlags, CursorFocus, CursorAction};

// There is no button here, we use events, propagate, oneshot and DisplayIf.


/// Insert resource CursorDefault if your cursor does not revert.
#[derive(Debug, Clone, Copy, Component)]
pub struct SetCursor {
    pub flags: EventFlags,
    pub icon: CursorIcon,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct DisplayIf(pub EventFlags);


pub fn conditional_visibility(mut query: Query<(&DisplayIf, Option<&CursorFocus>, &mut Visibility)>){
    query.par_iter_mut().for_each(|(display, focus, mut vis)| {
        *vis = match focus {
            Some(focus) => if display.0.contains(focus.flags()){
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
            None => if display.0.contains(EventFlags::Idle) {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
        }
    })
}

/// If set, we set the cursor to a default value every frame.
/// 
/// Not a part of the standard plugin, but should be added if you are
/// using `SetCursor`
#[derive(Debug, Resource, Clone, Copy)]
pub struct CursorDefault(pub CursorIcon);

impl Default for CursorDefault {
    fn default() -> Self {
        Self(CursorIcon::Arrow)
    }
}

pub fn set_cursor(
    default_cursor: Option<Res<CursorDefault>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    query: Query<(&SetCursor, &CursorFocus)>,
){
    if let Some(icon) = default_cursor{
        window.single_mut().cursor.icon = icon.0;
    }
    for (cursor, focus) in query.iter() {
        if cursor.flags.contains(focus.flags()){
            window.single_mut().cursor.icon = cursor.icon;
            break;
        }
    }
}


/// Marker component for passing active/passive state down their children.
/// 
/// Does **not** propagate through hierarchy if chained.
#[derive(Debug, Clone, Copy, Component, Default)]
pub struct PropagateFocus;

/// Propagate [`CursorFocus`] and [`CursorAction`] down to children.
pub fn propagate_focus(mut commands: Commands, 
    query1: Query<(&CursorFocus, &Children), With<PropagateFocus>>, 
    query2: Query<(&CursorAction, &Children), With<PropagateFocus>>,
) {
    for (focus, children) in query1.iter() {
        for child in children {
            commands.entity(*child).insert(*focus);
        }
    }
    for (focus, children) in query2.iter() {
        for child in children {
            commands.entity(*child).insert(*focus);
        }
    }
}