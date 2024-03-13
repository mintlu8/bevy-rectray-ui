//! `SystemSets` for `bevy_rectray`.

use bevy::input::InputSystem;
use bevy::text::update_text2d_layout;
use bevy::transform::systems::{propagate_transforms, sync_simple_transforms};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::RectrayRem;

use crate::core::pipeline::{compute_aoui_transforms, compute_aoui_opacity};
use crate::core::systems::*;

/// Fetch info for the tree, happens before `AouiTreeUpdate`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadInputSet;

/// Update the tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PipelineSet;

/// Update data with the tree, happens after `AouiTreeUpdate` and before bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct StoreOutputSet;

/// SystemSet for writing to `GlobalTransform`, after bevy's `propagate_transform`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct FinalizeSet;

/// SystemSet for generating events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct EventSet;

/// SystemSet for cleaning up events and signal in `Last`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct CleanupSet;

/// SystemSet for handling button clicks in `PreUpdate`,
/// has elevated precedence for signal piping.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct PostEventSet;

/// SystemSet for handling widget events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct WidgetEventSet;

/// SystemSet for handling widget events in `PreUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct PostWidgetEventSet;

/// SystemSet for deferred loading assets in `PostUpdate`.
#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct DeferredAssetSet;

/// Core plugin for `bevy_rectray`.
#[derive(Debug)]
pub struct CorePlugin;

impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<RectrayRem>()
            .configure_sets(PreUpdate, EventSet.after(InputSystem))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(EventSet)
                .before(PostEventSet))
            .configure_sets(PreUpdate, PostEventSet.after(EventSet))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(PostEventSet)
                .before(WidgetEventSet))
            .configure_sets(PreUpdate, WidgetEventSet.after(PostEventSet))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(WidgetEventSet)
                .before(PostWidgetEventSet))
            .configure_sets(PreUpdate, PostWidgetEventSet.after(PostEventSet))
            .configure_sets(Last, CleanupSet)
            .configure_sets(PostUpdate, LoadInputSet
                .before(PipelineSet)
                .after(update_text2d_layout))
            .configure_sets(PostUpdate, PipelineSet
                .before(StoreOutputSet))
            .configure_sets(PostUpdate, StoreOutputSet
                .before(propagate_transforms)
                .before(sync_simple_transforms)
            )
            .configure_sets(PostUpdate, FinalizeSet
                .after(propagate_transforms)
                .after(sync_simple_transforms)
            )
            .add_systems(PostUpdate, (
                set_occluded,
                copy_anchor,
                copy_anchor_sprite,
                copy_dimension_sprite,
                copy_dimension_text,
                copy_dimension_atlas,
            ).in_set(LoadInputSet))
            .add_systems(PostUpdate, (
                compute_aoui_transforms::<PrimaryWindow>,
                compute_aoui_opacity
            ).in_set(PipelineSet))
            .add_systems(PostUpdate, (
                sync_dimension_sprite,
                sync_dimension_text_bounds,
                sync_em_text,
                sync_opacity_vis,
                sync_opacity_sprite,
                sync_opacity_text,
            ).in_set(StoreOutputSet))
            .add_systems(PostUpdate, (
                build_mesh_2d_global_transform,
                build_global_transform
            ).in_set(FinalizeSet))
        ;

    }
}
