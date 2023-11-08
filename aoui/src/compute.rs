use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use itertools::Itertools;

use crate::{Transform2D, RotatedRect, ScreenSpaceTransform, ParentInfo, Container, LayoutControl, LayoutItem, AoUI, AouiREM, Dimension, SceneLayout};

type AoUIEntity<'t> = (
    Entity,
    &'t mut Dimension,
    &'t Transform2D,
    &'t mut RotatedRect, 
    Option<&'t mut ScreenSpaceTransform>,
);

fn propagate(
    parent: ParentInfo,
    entity: Entity,
    rem: f32,
    entity_query: &mut Query<AoUIEntity, With<AoUI>>,
    flex_query: &Query<&Container>,
    scene_query: &Query<&SceneLayout>,
    child_query: &Query<&Children>,
    control_query: &Query<&LayoutControl>,
    queue: &mut Vec<(Entity, ParentInfo)>,
    visited: &mut HashSet<Entity>) {

    visited.insert(entity);

    // SAFETY: safe since double mut access is gated by visited
    let (entity, mut dim, transform, mut orig, output, ..) = match unsafe {entity_query.get_unchecked(entity)}{
        Ok(items) => items,
        Err(_) => return,
    };

    let (dimension, em) = dim.update(parent.dimension, parent.em, rem);
    let offset = transform.offset.as_pixels(parent.dimension, parent.em, rem);

    if let Ok(layout) = flex_query.get(entity) {
        let children = child_query.get(entity).map(|x| x.iter()).into_iter().flatten();
        let mut entities = Vec::new();
        let mut args = Vec::new();
        for child in children {
            if !visited.insert(*child) { continue; }
            // SAFETY: safe since double mut access is gated by visited
            if let Ok((_, mut child_dim, child_transform, ..)) = unsafe { entity_query.get_unchecked(*child) } {
                entities.push(*child);
                args.push(LayoutItem {
                    anchor: child_transform.anchor.clone(),
                    dimension: child_dim.update(dimension, em, rem).0,
                    control: control_query.get(*child)
                        .map(|x| *x)
                        .unwrap_or(LayoutControl::None)
                });
            }
        }
        let margin = layout.margin.as_pixels(parent.dimension, parent.em, rem);
        let (placements, size) = layout.place_all(dimension, margin, args);

        #[cfg(debug_assertions)]{
            if transform.rotation != 0.0 || transform.scale != Vec2::ONE {
                eprintln!("Warning: anchors of a FlexLayout is unreliable. Rotate or scale its parent is recommended.")
            }
        }
        dim.size = size;
        let (rect, affine) = RotatedRect::construct(
            &parent,
            &transform.anchor,
            offset,
            size,
            transform.get_center(),
            transform.rotation,
            transform.scale,
            parent.z + transform.z + f32::EPSILON * 8.0,
        );
        
        queue.extend(entities.into_iter()
            .zip_eq(placements.into_iter() .map(|x| x / size - Vec2::new(0.5, 0.5)))
            .map(|(entity, anc)| (entity, ParentInfo::from_anchor(&rect, anc, dimension, em))));
        *orig = rect;
        output.map(|mut x| x.0 = affine);
        return;
    } 

    let (offset, z) = if let Ok(scene) = scene_query.get(entity) {
        let v3 = scene.transform(offset);
        (v3.truncate(), transform.z + v3.z)
    } else {
        (offset, transform.z)
    };

    let (rect, affine) = RotatedRect::construct(
        &parent,
        &transform.anchor,
        offset,
        dimension,
        transform.get_center(),
        transform.rotation,
        transform.scale,
        parent.z + z + f32::EPSILON * 8.0,
    );

    if let Ok(children) = child_query.get(entity) {
        for child in children {
            if !visited.insert(*child) { continue; }
            // SAFETY: safe since double mut access is gated by visited
            if let Ok((_, _, t, ..)) = unsafe { entity_query.get_unchecked(*child) } {
                let parent = ParentInfo::new(&rect, &t.anchor, dimension, em);
                queue.push((*child, parent))
            }
        }
    }

    *orig = rect;
    output.map(|mut x| x.0 = affine);
}

/// The main computation step.
pub fn compute_aoui_transforms(
    window: Query<&Window, With<PrimaryWindow>>,
    root_query: Query<Entity, (With<AoUI>, Without<Parent>)>,
    mut entity_query: Query<AoUIEntity, With<AoUI>>,
    flex_query: Query<&Container>,
    sparse_query: Query<&SceneLayout>,
    child_query: Query<&Children>,
    control_query: Query<&LayoutControl>,
    res_rem: Option<Res<AouiREM>>,
) {
    let window = window.single();
    let dim = Vec2::new(window.width(), window.height());
    let rem = res_rem.map(|x| x.0).unwrap_or(16.0);

    let window_rect = RotatedRect {
        center: Vec2::ZERO,
        dimension: dim,
        rotation: 0.0,
        scale: Vec2::ONE,
        z: 0.0
    };

    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    for (entity, _, t, ..) in entity_query.iter_many(root_query.iter()) {
        let parent = ParentInfo::new(&window_rect, &t.anchor, dim, rem);
        queue.push((entity, parent))
    }

    while !queue.is_empty() {
        for (entity, parent) in std::mem::take(&mut queue) {
            propagate(parent, entity, rem, &mut entity_query, &flex_query, &sparse_query, &child_query, &control_query, &mut queue, &mut visited);
        }
    }
}
