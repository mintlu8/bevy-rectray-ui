use std::mem;

use bevy::{ecs::{query::With, entity::Entity, system::{Commands, Query, Res, Resource}, component::Component}, hierarchy::Children, window::{PrimaryWindow, Window, CursorIcon}, reflect::Reflect};

use crate::{dsl::prelude::EventFlags, events::CursorFocus, anim::VisibilityToggle};

use super::button::CheckButtonState;


/// Set the window's [cursor](bevy::window::Window::cursor) value 
/// if the sprite has obtained [`CursorFocus`]
/// and the `CursorFocus` is some [`EventFlags`].
///
/// Call [`register_cursor_default`](crate::WorldExtension::register_cursor_default) 
/// on the `App` if your cursor does not revert. 
/// On the other hand, try remove the [`CursorDefault`] resource
/// if you want to have more control over cursor logic.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct SetCursor {
    pub flags: EventFlags,
    pub icon: CursorIcon,
}

/// Visible only when some conditions are met.
/// 
/// Supported conditions are:
/// 
/// * `EventFlags`: For `CursorFocus`
/// * `CheckButtonState`: For `CheckButton` and `RadioButton`'s status
/// 
/// This component uses `Interpolate<Opacity>` if exists, if not, uses `Visibility`.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub struct DisplayIf<T>(pub T);

pub fn event_conditional_visibility(mut query: Query<(&DisplayIf<EventFlags>, Option<&CursorFocus>, VisibilityToggle)>){
    query.iter_mut().for_each(|(display_if, focus, mut vis)| {
        if focus.is_some() && display_if.0.contains(focus.unwrap().flags()) 
            || focus.is_none() && display_if.0.contains(EventFlags::Idle) {
            vis.set_visible(true)
        } else {
            vis.set_visible(false)
        }
    })
}

pub fn check_conditional_visibility(
    mut query: Query<(&DisplayIf<CheckButtonState>, &CheckButtonState, VisibilityToggle)>
) {
    query.iter_mut().for_each(|(display_if, state, mut vis)| {
        if &display_if.0 == state {
            vis.set_visible(true)
        } else {
            vis.set_visible(false)
        }
    })
}

/// If set, we set the cursor to a default value every frame.
/// 
/// Remove this if custom behavior is desired.
#[derive(Debug, Resource, Clone, Copy, Reflect)]
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
    for (cursor, focus) in query.iter() {
        if cursor.flags.contains(focus.flags()) {
            if let Ok(mut window) = window.get_single_mut() {
                window.cursor.icon = cursor.icon;
            }
            return;
        }
    }
    if let Some(icon) = default_cursor{
        if let Ok(mut window) = window.get_single_mut() {
            window.cursor.icon = icon.0;
        }    
    }
}

/// Marker component for passing `CursorFocus`, 
/// `CursorAction` and `CheckButtonState` to their descendants.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub struct PropagateFocus;

/// Propagate [`CursorFocus`] and [`CursorAction`] down descendants.
pub fn propagate_focus<T: Component + Clone>(
    mut commands: Commands, 
    query: Query<(&T, &Children), With<PropagateFocus>>, 
    descendent: Query<&Children>
) {
    let mut queue = Vec::new();
    for (focus, children) in query.iter() {
        for child in children {
            commands.entity(*child).insert(focus.clone());
            queue.push((child, focus));
        }
    }
    while !queue.is_empty() {
        for (entity, focus) in mem::take(&mut queue) {
            let Ok(children) = descendent.get(*entity) else {return};
            for child in children {
                commands.entity(*child).insert(focus.clone());
                queue.push((child, focus));
            }
        }
    }
}

pub fn remove_all<T: Component>(mut commands: Commands, 
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<T>();
    }
}
