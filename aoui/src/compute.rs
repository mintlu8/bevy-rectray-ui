use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};

use crate::{Anchors, Transform2D, RotatedRect, ScreenSpaceTransform, ParentInfo, FlexContainer, FlexControl, FlexItem, Core, AorsREM, Dimension, SparseContainer, SparsePosition, SparseItem};

type AoUIEntity<'t> = (Entity,
    &'t Anchors, 
    &'t mut Dimension,
    &'t Transform2D,
    &'t mut RotatedRect, 
    Option<&'t mut ScreenSpaceTransform>,
    Option<&'t SparsePosition>
);

/// SAFETY: safe since double mut access is gated by visited
fn propagate(
    parent: ParentInfo,
    entity: Entity,
    rem: Vec2,
    entity_query: &mut Query<AoUIEntity, With<Core>>,
    flex_query: &Query<&FlexContainer>,
    sparse_query: &Query<&SparseContainer>,
    child_query: &Query<&Children>,
    control_query: &Query<&FlexControl>,
    queue: &mut Vec<(Entity, ParentInfo)>,
    visited: &mut HashSet<Entity>) {

    visited.insert(entity);

    let (entity, anchors, mut dimension, transform, mut orig, output, ..) = match unsafe {entity_query.get_unchecked(entity)}{
        Ok(items) => items,
        Err(_) => return,
    };

    let (dimension, em) = dimension.update(parent.dimension, parent.em, rem);
    let offset = transform.offset.as_pixels(parent.dimension, parent.em, rem);

    let (rect, affine) = RotatedRect::construct(
        &parent,
        &anchors.anchor,
        offset,
        dimension,
        anchors.get_center(),
        transform.rotation,
        transform.scale,
        parent.z + transform.z + f32::EPSILON,
    );

    if let Ok(children) = child_query.get(entity) {
        if let Ok(layout) = flex_query.get(entity) {
            let mut entities = Vec::new();
            let mut args = Vec::new();
            for child in children {
                if !visited.insert(*child) { continue; }
                if let Ok((_, child_anc, mut child_dim, ..)) = unsafe { entity_query.get_unchecked(*child) } {
                    entities.push(*child);
                    args.push(FlexItem {
                        anchor: child_anc.anchor.clone(),
                        dimension: child_dim.update(dimension, em, rem).0,
                        flex_control: control_query.get(*child)
                            .map(|x| *x)
                            .unwrap_or(FlexControl::None)
                    });
                }
            }
            let margin = layout.margin.as_pixels(parent.dimension, parent.em, rem);
            queue.extend(entities.into_iter()
                .zip(layout.place_all(&rect, margin, args).into_iter())
                .map(|(entity, anc)| (entity, ParentInfo::from_anchor(&rect, anc, dimension, em))));
        } else if let Ok(sparse) = sparse_query.get(entity) {
            let mut entities = Vec::new();
            let mut args = Vec::new();
            for child in children {
                if !visited.insert(*child) { continue; }
                if let Ok((_, child_anc, mut child_dim, .., pos)) = unsafe { entity_query.get_unchecked(*child) } {
                    entities.push(*child);
                    args.push(SparseItem {
                        anchor: child_anc.anchor.clone(),
                        dimension: child_dim.update(dimension, em, rem).0,
                        position: pos.map(|x| x.0).unwrap_or_else(|| {
                            debug_assert!(false, "You should use `SparsePosition` with SparseLayout.");
                            Vec2::ZERO
                        }),
                    });
                }
            }
            queue.extend(entities.into_iter()
                .zip(sparse.place_all(&rect, args).into_iter())
                .map(|(entity, anc)| (entity, ParentInfo::from_anchor(&rect, anc, dimension, em))));
        } else {
            for child in children {
                if !visited.insert(*child) { continue; }
                if let Ok((_, child_anc, ..)) = unsafe { entity_query.get_unchecked(*child) } {
                    let parent = ParentInfo::new(&rect, &child_anc.anchor, dimension, em);
                    queue.push((*child, parent))
                }
            }
        }
    }

    *orig = rect;
    output.map(|mut x| x.0 = affine);
}

pub fn compute_aoui_root(
    window: Query<&Window, With<PrimaryWindow>>,
    root_query: Query<Entity, (With<Core>, Without<Parent>)>,
    mut entity_query: Query<AoUIEntity, With<Core>>,
    flex_query: Query<&FlexContainer>,
    sparse_query: Query<&SparseContainer>,
    child_query: Query<&Children>,
    control_query: Query<&FlexControl>,
    res_rem: Option<Res<AorsREM>>,
) {
    let window = window.single();
    let dim = Vec2::new(window.width(), window.height());
    let rem = res_rem.map(|x| x.0).unwrap_or(Vec2::new(16.0, 16.0));

    let window_rect = RotatedRect {
        center: Vec2::ZERO,
        dimension: dim,
        rotation: 0.0,
        scale: Vec2::ONE,
        z: 0.0
    };

    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    for (entity, prop, ..) in entity_query.iter_many(root_query.iter()) {
        let parent = ParentInfo::new(&window_rect, &prop.anchor, dim, rem);
        queue.push((entity, parent))
    }

    while !queue.is_empty() {
        for (entity, parent) in std::mem::take(&mut queue) {
            propagate(parent, entity, rem, &mut entity_query, &flex_query, &sparse_query, &child_query, &control_query, &mut queue, &mut visited);
        }
    }
}
