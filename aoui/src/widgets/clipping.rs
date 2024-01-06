use bevy::{render::{primitives::Frustum, texture::Image, color::Color}, transform::components::GlobalTransform};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::asset::{AssetServer, Handle};
use bevy::ecs::{component::Component, bundle::Bundle, system::Query, query::With};
use bevy::render::render_resource::{Extent3d, TextureUsages, TextureDescriptor, TextureDimension, TextureFormat};
use bevy::render::view::{VisibleEntities, RenderLayers};
use bevy::render::camera::{Camera, CameraRenderGraph, OrthographicProjection, RenderTarget, ScalingMode};
use bevy::core_pipeline::{core_2d::Camera2d, tonemapping::{Tonemapping, DebandDither}, clear_color::ClearColorConfig};
use crate::{BuildTransform, Anchor, DimensionData, dsl::CloneSplit};

use crate::dsl::DslInto;

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


/// Create an image suitable as a render target.
pub fn render_target<T: CloneSplit<Handle<Image>>>(assets: impl AsRef<AssetServer>, [width, height]: [u32; 2]) -> T {
    let handle = assets.as_ref().add(Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width,
                height,
                ..Default::default()
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        data: vec![0; width as usize * height as usize * 4],
        ..Default::default()
    });
    CloneSplit::clone_split(handle)
}

impl ScopedCameraBundle {

    /// Create a camera and its render target. 
    pub fn new(assets: impl AsRef<AssetServer>, dimension: [u32; 2], layer: impl DslInto<RenderLayers>) -> (Self, Handle<Image>) {
        let (cam, texture) = render_target(assets, dimension);
        (Self::from_image(cam, layer), texture)
    }

    /// Create a camera from a render target.
    pub fn from_image(target: Handle<Image>, layer: impl DslInto<RenderLayers>) -> Self {
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
            camera_2d: Camera2d {clear_color: ClearColorConfig::Custom(Color::NONE)}, 
            tonemapping: bun.tonemapping,
            deband_dither: bun.deband_dither,
            render_layer: layer.dinto(),
            build: BuildTransform(Anchor::Center),
            global: GlobalTransform::default(),
        }
    }
}

pub fn sync_camera_dimension(
    mut query: Query<(&DimensionData, &mut OrthographicProjection), With<CameraClip>>,
) {
    for (dimension, mut proj) in query.iter_mut() {
        proj.scaling_mode = ScalingMode::Fixed { 
            width: dimension.size.x, 
            height: dimension.size.y 
        };
    }
}
