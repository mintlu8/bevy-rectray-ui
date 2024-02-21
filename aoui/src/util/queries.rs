use std::{iter::Copied, slice::Iter};

use bevy::{hierarchy::Children, math::Vec2, window::{PrimaryWindow, Window}};
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Query, Res, SystemParam};
use bevy::ecs::query::{QueryData, With};

use crate::RectrayRem;

/// Query for scaling factor from [`Window`].
#[derive(SystemParam)]
pub struct ScalingFactor<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
}

impl ScalingFactor<'_, '_> {
    pub fn get(&self) -> f32 {
        self.window
            .get_single()
            .map(|x| x.scale_factor() as f32)
            .unwrap_or(2.0)
    }
}

/// Query for size from [`Window`].
#[derive(SystemParam)]
pub struct WindowSize<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
}

impl WindowSize<'_, '_> {
    pub fn get(&self) -> Vec2 {
        self.window
            .get_single()
            .map(|x| Vec2::new(x.width(), x.height()))
            .unwrap_or(Vec2::new(0.0, 0.0))
    }
}

/// Query for `rem` from [`RectrayRem`].
#[derive(SystemParam)]
pub struct Rem<'w> {
    rem: Option<Res<'w, RectrayRem>>,
}

impl Rem<'_> {
    pub fn get(&self) -> f32 {
        self.rem.as_ref()
            .map(|x| x.get())
            .unwrap_or(16.0)
    }
}

/// Query for children that can also be empty.
#[derive(QueryData)]
pub struct ChildIter {
    children: Option<&'static Children>
}

impl ChildIterItem<'_> {

    pub fn is_empty(&self) -> bool {
        self.children.map(|x| x.is_empty()).unwrap_or(true)
    }

    pub fn len(&self) -> usize {
        self.children.map(|x| x.len()).unwrap_or(0)
    }

    pub fn iter(&self) -> Copied<Iter<Entity>> {
        match &self.children {
            Some(children) => children.iter().copied(),
            None => [].iter().copied(),
        }
    }

    pub fn get_single(&self) -> Option<Entity> {
        self.children
            .and_then(|x| if x.len() == 1 {
                x.first().copied()
            } else {
                None
            })
    }
}
