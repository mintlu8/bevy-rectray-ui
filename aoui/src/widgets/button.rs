use std::sync::{Mutex, Arc};

use bevy::{render::view::Visibility, window::{Window, PrimaryWindow, CursorIcon}, hierarchy::Children};
use bevy::ecs::{system::{Query, Resource, Res, Commands}, component::Component, query::With};
use crate::Opacity;

use crate::{events::{EventFlags, CursorFocus, CursorAction}, util::Dto, dsl::prelude::Interpolate, util::Sender};

/// Set cursor if [`CursorFocus`] is some [`EventFlags`].
///
/// Insert resource [`CursorDefault`] if your cursor does not revert.
#[derive(Debug, Clone, Copy, Component)]
pub struct SetCursor {
    pub flags: EventFlags,
    pub icon: CursorIcon,
}

/// Visible only when some conditions are met.
/// 
/// Supported conditions are:
/// 
/// * `EventFlags`: For `CursorFocus`
/// * `CheckButtonState`: For `CheckButton`
/// 
/// This uses `Interpolate<Opacity>` if exists, if not, uses `Visibility`.
#[derive(Debug, Clone, Copy, Component)]
pub struct DisplayIf<T>(pub T);

pub fn event_conditional_visibility(mut query: Query<(&DisplayIf<EventFlags>, Option<&CursorFocus>, &mut Visibility, Option<&mut Interpolate<Opacity>>)>){
    query.par_iter_mut().for_each(|(display_if, focus, mut vis, mut opacity)| {
        if focus.is_some() && display_if.0.contains(focus.unwrap().flags()) 
            || focus.is_none() && display_if.0.contains(EventFlags::Idle) {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(0.0);
            } else {
                *vis = Visibility::Hidden;
            }
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub enum CheckButtonState{
    #[default]
    Unchecked,
    Checked,
}

pub struct RadioButtonState(Arc<Dto>);

pub fn state_conditional_visibility(mut query: Query<(&DisplayIf<CheckButtonState>, &CheckButtonState, &mut Visibility, Option<&mut Interpolate<Opacity>>)>){
    query.par_iter_mut().for_each(|(display_if, state, mut vis, mut opacity)| {
        if &display_if.0 == state {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(1.0);
            } else {
                *vis = Visibility::Inherited;
            }
        } else {
            if let Some(opacity) = opacity.as_mut() {
                opacity.interpolate_to_or_reverse(0.0);
            } else {
                *vis = Visibility::Hidden;
            }
        }
    })
}

/// If set, we set the cursor to a default value every frame.
/// 
/// Not a part of the standard plugin, but
/// can be used if you are using `SetCursor`.
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
        if cursor.flags.contains(focus.flags()) {
            window.single_mut().cursor.icon = cursor.icon;
            break;
        }
    }
}


/// Marker component for passing `CursorFocus`/`CursorAction` to their children.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Component)]
pub struct CheckBox;

pub struct RadioButtonContext(Arc<Mutex<Dto>>);

/// When attached to a widget in the button family,
/// the submit signal will send the containing data.
/// 
/// # Submit signal behavior:
/// 
/// * Button OnClick: send `Payload` or `()`.
/// * RadioButton OnClick: send `Payload` or `()`.
/// * CheckButton OnClick: If `Payload` exists, send `Payload` or `()`, 
/// If payload doesn't exist, send `true` or `false`.
/// 
/// For radio buttons, you need to make sure the binary
/// serializations of each branch.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)] 
pub struct Payload(Dto);

impl Payload {
    pub const fn empty() -> Self {
        Self(Dto(Vec::new()))
    }

    pub fn new(value: &impl serde::Serialize) -> Result<Self, postcard::Error> {
        Ok(Self(Dto::new(value)?))
    }

    pub fn send<M>(&self, sender: &Sender<M>) {
        sender.send_bytes(&self.0.0)
    }
}