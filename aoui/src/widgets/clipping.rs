use crate::{Anchor, BuildTransform, DimensionData};
use bevy::asset::Handle;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::core_pipeline::{
    clear_color::ClearColorConfig,
    core_2d::Camera2d,
    tonemapping::{DebandDither, Tonemapping},
};
use bevy::ecs::{bundle::Bundle, component::Component, query::With, system::Query};
use bevy::render::camera::{
    Camera, CameraRenderGraph, OrthographicProjection, RenderTarget, ScalingMode,
};
use bevy::render::view::{RenderLayers, VisibleEntities};
use bevy::{
    render::{color::Color, primitives::Frustum, texture::Image},
    transform::components::GlobalTransform,
};

use crate::util::convert::DslInto;

/// Marker component that indicates the camera is used for clipping its contents.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
pub struct CameraClip;

/// A bundle that spawns a camera that draws its enclosed rectangle to a render target.
#[derive(Bundle)]
#[non_exhaustive]
pub struct ScopedCameraBundle {
    pub clip: CameraClip,
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub camera_2d: Camera2d,
    pub tonemapping: Tonemapping,
    pub deband_dither: DebandDither,
    pub render_layer: RenderLayers,
    pub build: BuildTransform,
    pub global: GlobalTransform,
}

impl ScopedCameraBundle {
    /// Create a camera from a render target.
    pub fn new(target: Handle<Image>, layer: impl DslInto<RenderLayers>) -> Self {
        let bun = Camera2dBundle::default();
        Self {
            clip: CameraClip,
            camera: Camera {
                target: RenderTarget::Image(target.clone()),
                ..Default::default()
            },
            camera_render_graph: bun.camera_render_graph,
            projection: bun.projection,
            visible_entities: bun.visible_entities,
            frustum: bun.frustum,
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
            },
            tonemapping: bun.tonemapping,
            deband_dither: bun.deband_dither,
            render_layer: layer.dinto(),
            build: BuildTransform(Anchor::CENTER),
            global: GlobalTransform::default(),
        }
    }
}

pub(crate) fn sync_camera_dimension(
    mut query: Query<(&DimensionData, &mut OrthographicProjection), With<CameraClip>>,
) {
    for (dimension, mut proj) in query.iter_mut() {
        proj.scaling_mode = ScalingMode::Fixed {
            width: dimension.size.x,
            height: dimension.size.y,
        };
    }
}
