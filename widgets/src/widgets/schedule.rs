use bevy::{prelude::{Plugin, Query, Changed, PostUpdate, IntoSystemConfigs, Update}, sprite::Anchor};
use bevy_aoui::{Dimension, schedule::AoUISyncWrite, Transform2D};
use bevy_prototype_lyon::prelude::Path;

use super::{shape::{Shapes, ShapeDimension}, inputbox};

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Update, inputbox::text_on_mouse_down)
            .add_systems(Update, inputbox::update_inputbox_cursor.after(inputbox::text_on_mouse_down))
            .add_systems(Update, inputbox::text_on_click_outside)
            .add_systems(Update, inputbox::text_on_mouse_double_click)
            .add_systems(Update, inputbox::inputbox_keyboard)
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUISyncWrite))
            .add_systems(PostUpdate, sync_shape_size.in_set(AoUISyncWrite))
            .add_systems(PostUpdate, rebuild_shapes.in_set(AoUISyncWrite).after(sync_shape_size))
        ;
    }
}

// TODO: wait for bevy impl
fn anchor_eq(left: &Anchor, right: &Anchor) -> bool{
    use std::mem::discriminant;
    discriminant(left) == discriminant(right) && 
        match (left, right) {
            (Anchor::Custom(a), Anchor::Custom(b)) => a == b,
            _ => true,
        }
}

pub fn sync_shape_size(mut query: Query<(&Transform2D, &Dimension, &mut ShapeDimension)>) {
    for (transform, dimension, mut shape) in query.iter_mut() {
        if !anchor_eq(&transform.anchor, &shape.as_ref().anchor) {
            shape.anchor = transform.anchor
        }

        if dimension.size != shape.as_ref().size {
            shape.size = dimension.size
        }
    }
}

pub fn rebuild_shapes(mut query: Query<(&Shapes, &ShapeDimension, &mut Path), Changed<ShapeDimension>>) {
    for (shape, cache, mut path) in query.iter_mut() {
        *path = shape.build_path(cache.anchor, cache.size)
    }
}