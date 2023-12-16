//! Schedules for the `core` and `layout` modules, 
//! which are considered the core features of `aoui`.

use bevy::input::InputSystem;
use bevy::text::update_text2d_layout;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::AoUIREM;

use crate::core::compute::{compute_aoui_transforms, TRoot, TAll};
use crate::core::systems::*;

/// Fetch info for the tree, happens before `AoUITreeUpdate`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUILoadInputSet;

/// Update the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUITreeUpdateSet;

/// Update data with the tree, happens after `AoUITreeUpdate` and before bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUIStoreOutputSet;

/// SystemSet for writing to `GlobalTransform`, after bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AoUIFinalizeSet;

/// SystemSet for generating events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIEventSet;

/// SystemSet for cleaning up events and signal in `Last`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUICleanupSet;

/// SystemSet for handling button clicks in `PreUpdate`, 
/// has elevated precedence for signal piping.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIButtonEventSet;

/// SystemSet for handling widget events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIWidgetsEventSet;

/// Core plugin for AoUI Rendering.
#[derive(Debug)]
pub struct CorePlugin;

impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AoUIREM>()
            .configure_sets(PreUpdate, AoUIEventSet.after(InputSystem))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(AoUIEventSet)
                .before(AoUIButtonEventSet))
            .configure_sets(PreUpdate, AoUIButtonEventSet.after(AoUIEventSet))
            .configure_sets(PreUpdate, AoUIWidgetsEventSet.after(AoUIButtonEventSet))
            .configure_sets(Last, AoUICleanupSet)
            .configure_sets(PostUpdate, AoUILoadInputSet
                .before(AoUITreeUpdateSet)
                .after(update_text2d_layout))
            .configure_sets(PostUpdate, AoUITreeUpdateSet
                .before(AoUIStoreOutputSet))
            .configure_sets(PostUpdate, AoUIStoreOutputSet
                .before(propagate_transforms)
                .before(sync_simple_transforms)
            )
            .configure_sets(PostUpdate, AoUIFinalizeSet
                .after(propagate_transforms)
                .after(sync_simple_transforms)
            )
            .add_systems(PostUpdate, (
                copy_anchor, 
                copy_anchor_sprite, 
                copy_anchor_atlas,
                copy_dimension_sprite,
                copy_dimension_text,
                copy_dimension_atlas,
            ).in_set(AoUILoadInputSet))
            .add_systems(PostUpdate,
                compute_aoui_transforms::<PrimaryWindow, TRoot, TAll>
            .in_set(AoUITreeUpdateSet))
            .add_systems(PostUpdate, (
                sync_dimension_atlas,
                sync_dimension_sprite,
                sync_dimension_text_bounds,
                sync_em_text,
                sync_opacity_sprite,
                sync_opacity_atlas,
                sync_opacity_text,
            ).in_set(AoUIStoreOutputSet))
            .add_systems(PostUpdate, 
                (
                    build_mesh_2d_global_transform,
                    build_global_transform
                ).in_set(AoUIFinalizeSet)
            );
    }
}
