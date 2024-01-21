use std::{iter::Copied, slice::Iter};

use bevy::{ecs::{entity::Entity, query::WorldQuery}, hierarchy::Children};

#[derive(WorldQuery)]
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
