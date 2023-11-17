use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow, ecs::query::{ReadOnlyWorldQuery, WorldQuery}};
use itertools::Itertools;

use crate::*;

type AoUIEntity<'t> = (
    Entity,
    &'t mut Dimension,
    &'t Transform2D,
    &'t mut RotatedRect,
);

fn propagate<TAll: ReadOnlyWorldQuery>(
    parent: ParentInfo,
    entity: Entity,
    rem: f32,
    entity_query: &mut Query<AoUIEntity, TAll>,
    flex_query: &Query<&Container>,
    scene_query: &Query<&SceneLayout>,
    child_query: &Query<&Children>,
    control_query: &Query<&LayoutControl>,
    queue: &mut Vec<(Entity, ParentInfo)>,
    visited: &mut HashSet<Entity>) {

    // SAFETY: safe since double mut access is gated by visited
    let Ok((entity, mut dim, transform, mut orig, ..)) = (unsafe {entity_query.get_unchecked(entity)}) else {return};

    let (dimension, em) = dim.update(parent.dimension, parent.em, rem);
    let offset = transform.offset.as_pixels(parent.dimension, em, rem);

    if let Ok(layout) = flex_query.get(entity) {
        let children = child_query.get(entity).map(|x| x.iter()).into_iter().flatten();
        let mut layout_entities = Vec::new();
        let mut other_entities = Vec::new();
        let mut args = Vec::new();
        for child in children {
            if !visited.insert(*child) { continue; }
            // SAFETY: safe since double mut access is gated by visited
            if let Ok((_, mut child_dim, child_transform, ..)) = unsafe { entity_query.get_unchecked(*child) } {
                match control_query.get(*child) {
                    Ok(LayoutControl::IgnoreLayout) => other_entities.push((
                        *child, 
                        child_transform.get_parent_anchor()
                    )),
                    control => {
                        layout_entities.push(*child);
                        args.push(LayoutItem {
                            anchor: child_transform.get_parent_anchor().clone(),
                            dimension: child_dim.update(dimension, em, rem).0 * child_transform.scale,
                            control: control.copied().unwrap_or_default(),
                        })
                    }
                };
            }
        }
        let margin = layout.margin.as_pixels(parent.dimension, em, rem);
        let (placements, size) = layout.place_all(dimension, margin, args);

        dim.size = size;
        let rect = RotatedRect::construct(
            &parent,
            &transform.anchor,
            offset,
            size,
            transform.get_center(),
            transform.rotation,
            transform.scale,
            parent.z + transform.z + f32::EPSILON * 8.0,
        );
        
        queue.extend(layout_entities.into_iter()
            .zip_eq(placements.into_iter().map(|x| x / size - Vec2::new(0.5, 0.5)))
            .map(|(entity, anc)| (entity, ParentInfo::from_anchor(&rect, anc, dimension, em))));
        *orig = rect;
        for (child, anchor) in other_entities {
            let parent = ParentInfo::new(&rect, anchor, size, em);
            queue.push((child, parent))
        }
        return;
    } 

    let (offset, z) = if let Ok(scene) = scene_query.get(entity) {
        let v3 = scene.transform(offset);
        (v3.truncate(), transform.z + v3.z)
    } else {
        (offset, transform.z)
    };

    let rect = RotatedRect::construct(
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
                let parent = ParentInfo::new(&rect, t.get_parent_anchor(), dimension, em);
                queue.push((*child, parent))
            }
        }
    }

    *orig = rect;
}

/// Query for finding the root rectangle of a `compute_aoui_transforms` pass.
/// 
/// Usually `PrimaryWindow`.
pub trait RootQuery<'t> {
    type Query: WorldQuery;
    type ReadOnly: ReadOnlyWorldQuery;

    fn as_rect(query: &Query<Self::Query, Self::ReadOnly>) -> RotatedRect;
}

impl<'t> RootQuery<'t> for PrimaryWindow {
    type Query= &'t Window;
    type ReadOnly = With<PrimaryWindow>;

    fn as_rect(query: &Query<Self::Query, Self::ReadOnly>) -> RotatedRect {
        let window = match query.get_single(){
            Ok(w) => w,
            Err(_) => return Default::default(), 
        };
        let dim = Vec2::new(window.width(), window.height());
        RotatedRect {
            center: Vec2::ZERO,
            dimension: dim,
            rotation: 0.0,
            scale: Vec2::ONE,
            z: 0.0
        }
    }
}

pub(crate) type TRoot = (With<AoUI>, Without<Parent>);
pub(crate) type TAll = With<AoUI>;

/// The main computation step.
/// 
/// For custom usage,
/// 
/// R: Get root rectangle,
/// 
/// TRoot: Readonly query for child of root rectangle.
/// 
/// TAll: Readonly query for all children, including TRoot.
pub fn compute_aoui_transforms<'t, R: RootQuery<'t>, TRoot: ReadOnlyWorldQuery, TAll: ReadOnlyWorldQuery>(
    root: Query<R::Query, R::ReadOnly>,
    root_entities: Query<Entity, TRoot>,
    mut entity_query: Query<AoUIEntity, TAll>,
    flex_query: Query<&Container>,
    sparse_query: Query<&SceneLayout>,
    child_query: Query<&Children>,
    control_query: Query<&LayoutControl>,
    res_rem: Option<Res<AoUIREM>>,
) {
    let rem = res_rem.map(|x| x.0).unwrap_or(16.0);

    let window_rect = R::as_rect(&root);
    let dim = window_rect.dimension;

    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    for (entity, _, t, ..) in entity_query.iter_many(root_entities.iter()) {
        if !visited.insert(entity) { return; }
        let parent = ParentInfo::new(&window_rect, t.get_parent_anchor(), dim, rem);
        queue.push((entity, parent))
    }

    while !queue.is_empty() {
        for (entity, parent) in std::mem::take(&mut queue) {
            propagate(parent, entity, rem, &mut entity_query, &flex_query, &sparse_query, &child_query, &control_query, &mut queue, &mut visited);
        }
    }
}
