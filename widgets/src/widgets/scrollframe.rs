use bevy::{core_pipeline::{core_2d::{Camera2dBundle, Camera2d}, tonemapping::{Tonemapping, DebandDither}, clear_color::ClearColorConfig}, render::{camera::{Camera, CameraRenderGraph, OrthographicProjection, Viewport}, view::{VisibleEntities, RenderLayers}, primitives::Frustum}, transform::components::GlobalTransform, ecs::{component::Component, bundle::Bundle}};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
pub struct CameraClip;

#[derive(Bundle)]
#[non_exhaustive]
pub struct ClippingBundle {
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
}

impl ClippingBundle {
    pub fn new(layer: u8) -> Self {
        if layer == 0 { panic!("Layer 0 should not be used here.") }
        Self { 
            clip: CameraClip, 
            camera: Camera { 
                viewport: Some(Viewport::default()),
                ..Default::default()
            }, 
            camera_render_graph: CameraRenderGraph::default(), 
            projection: OrthographicProjection::default(), 
            visible_entities: VisibleEntities::default(), 
            frustum: Frustum::default(), 
            camera_2d: Camera2d {clear_color: ClearColorConfig::None}, 
            tonemapping: Tonemapping::None, 
            deband_dither: DebandDither::Disabled, 
            render_layer: RenderLayers::from_layers(&[layer])
        }
    }
}