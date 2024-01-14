//! `SystemSets` for `bevy_aoui`.

use bevy::input::InputSystem;
use bevy::text::update_text2d_layout;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::AouiREM;

use crate::core::pipeline::{compute_aoui_transforms, compute_aoui_opacity};
use crate::core::systems::*;

/// Fetch info for the tree, happens before `AouiTreeUpdate`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AouiLoadInputSet;

/// Update the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AouiTreeUpdateSet;

/// Update data with the tree, happens after `AouiTreeUpdate` and before bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AouiStoreOutputSet;

/// SystemSet for writing to `GlobalTransform`, after bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct AouiFinalizeSet;

/// SystemSet for generating events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AouiEventSet;

/// SystemSet for cleaning up events and signal in `Last`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AouiCleanupSet;

/// SystemSet for handling button clicks in `PreUpdate`,
/// has elevated precedence for signal piping.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AouiButtonEventSet;

/// SystemSet for handling widget events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AouiWidgetEventSet;

/// SystemSet for deferred loading assets in `PostUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AouiDeferredAssetSet;

/// Core plugin for Aoui Rendering.
#[derive(Debug)]
pub struct CorePlugin;

impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<AouiREM>()
            .configure_sets(PreUpdate, AouiEventSet.after(InputSystem))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(AouiEventSet)
                .before(AouiButtonEventSet))
            .configure_sets(PreUpdate, AouiButtonEventSet.after(AouiEventSet))
            .configure_sets(PreUpdate, AouiWidgetEventSet.after(AouiButtonEventSet))
            .configure_sets(Last, AouiCleanupSet)
            .configure_sets(PostUpdate, AouiLoadInputSet
                .before(AouiTreeUpdateSet)
                .after(update_text2d_layout))
            .configure_sets(PostUpdate, AouiTreeUpdateSet
                .before(AouiStoreOutputSet))
            .configure_sets(PostUpdate, AouiStoreOutputSet
                .before(propagate_transforms)
                .before(sync_simple_transforms)
            )
            .configure_sets(PostUpdate, AouiFinalizeSet
                .after(propagate_transforms)
                .after(sync_simple_transforms)
            )
            .add_systems(PostUpdate, (
                set_occluded,
                copy_anchor,
                copy_anchor_sprite,
                copy_anchor_atlas,
                copy_dimension_sprite,
                copy_dimension_text,
                copy_dimension_atlas,
            ).in_set(AouiLoadInputSet))
            .add_systems(PostUpdate, (
                compute_aoui_transforms::<PrimaryWindow>,
                compute_aoui_opacity
            ).in_set(AouiTreeUpdateSet))
            .add_systems(PostUpdate, (
                sync_dimension_atlas,
                sync_dimension_sprite,
                sync_dimension_text_bounds,
                sync_em_text,
                sync_opacity_vis,
                sync_opacity_sprite,
                sync_opacity_atlas,
                sync_opacity_text,
            ).in_set(AouiStoreOutputSet))
            .add_systems(PostUpdate, (
                build_mesh_2d_global_transform,
                build_global_transform
            ).in_set(AouiFinalizeSet))
        ;

    }
}
