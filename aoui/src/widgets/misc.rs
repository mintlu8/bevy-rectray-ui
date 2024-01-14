use bevy::{ecs::{system::{Query, In}, query::With, component::Component, entity::Entity}, hierarchy::Children};

use crate::{Opacity, layout::LayoutControl};

/// Limit opacity in a layout based on insertion order.
///
/// Useful for a simple dropdown fade implementation.
#[derive(Debug, Clone, Copy, Component)]
pub struct LayoutOpacityLimiter;

pub fn layout_opacity_limit(
    parent: Query<(Entity, &Opacity), (With<LayoutOpacityLimiter>, With<Children>)>,
) -> Vec<(Entity, f32)> {
    parent.iter()
        .map(|(entity, opacity)| (entity, opacity.computed_opacity))
        .collect()
}

pub fn set_layout_opactiy_limit(
    In(input): In<Vec<(Entity, f32)>>,
    parent: Query<&Children>,
    mut children_opacity: Query<(&mut Opacity, &LayoutControl)>,
) {
    for (entity, parent_opactiy) in input {
        let Ok(children) = parent.get(entity) else {continue};
        let mut count = 0.0;
        for child in children.iter() {
            let Ok((_, layout)) = children_opacity.get (*child) else {continue};
            if !matches!(layout, LayoutControl::IgnoreLayout) {
                count += 1.0;
            }
        }
        if count == 0.0 {continue}
        let mut index = 0.0;
        for child in children.iter() {
            let Ok((mut opacity, layout)) = children_opacity.get_mut (*child) else {continue};
            if matches!(layout, LayoutControl::IgnoreLayout) {continue}
            let fac = index / count;
            opacity.style_opacity = ((parent_opactiy - fac) / (1.0 - fac)).clamp(0.0, 1.0);
            index += 1.0;
        }
    }
}
