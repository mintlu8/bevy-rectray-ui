use bevy::{render::{primitives::Frustum, texture::Image, color::Color}, transform::components::GlobalTransform};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::asset::{AssetServer, Handle};
use bevy::ecs::{component::Component, bundle::Bundle, system::Query, query::With};
use bevy::render::render_resource::{Extent3d, TextureUsages, TextureDescriptor, TextureDimension, TextureFormat};
use bevy::render::view::{VisibleEntities, RenderLayers};
use bevy::render::camera::{Camera, CameraRenderGraph, OrthographicProjection, RenderTarget, ScalingMode};
use bevy::core_pipeline::{core_2d::Camera2d, tonemapping::{Tonemapping, DebandDither}, clear_color::ClearColorConfig};
use crate::{BuildTransform, Anchor, DimensionData};

use crate::dsl::DslInto;

/// Marker component that indicates the camera is used for clipping its contents.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Component)]
pub struct CameraClip;

/// Allow a frame to clip its out-of-bounds children.
/// 
/// This spawns a camera that draws its enclosed rectangle, which uses a new render layer.
/// you are responsible to ensure the render layers used don't overlap.
/// 
/// The entity this bundle is attached to should not be rendered, since the entity
/// has to use `Anchor::Center` as its `GlobalTransform`
/// and hold it's children's `RenderLayers` for the camera.
/// 
/// In idiomatic usage, attach the generated `Handle<Image>` to a child `Sprite` on the 
/// main render layer `0` with dimension `[100%, 100%]`
/// and add children to it on a new render layer.
/// 
/// ```
/// # /*
/// let (clip, sprite) = ClippingBundle::new(&assets, [800, 800], 1);
/// frame!(commands {
///     dimension: [400, 400],
///     extra: clip,
///     child: sprite! {
///         sprite: sprite,
///         dimension: Size2::FULL,
///         child: sprite! {
///             layer: 1
///         }
///     }
/// }
/// # */
/// ```
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


/// Create an image suitable as render target.
pub fn new_render_target(assets: &AssetServer, [width, height]: [u32; 2]) -> Handle<Image> {
    assets.add(Image {
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
    })
}

impl ScopedCameraBundle {

    /// Create a camera and its render target. 
    /// 
    /// You have to set the size of the target image here, which will not be resized.
    /// This might change in the future.
    pub fn new(assets: &AssetServer, dimension: [u32; 2], layer: impl DslInto<RenderLayers>) -> (Self, Handle<Image>) {
        let target = new_render_target(assets, dimension);
        (Self::from_image(target.clone(), layer), target)
    }

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

pub fn clipping_layer(
    mut query: Query<(&DimensionData, &mut OrthographicProjection), With<CameraClip>>,
) {
    for (dimension, mut proj) in query.iter_mut() {
        proj.scaling_mode = ScalingMode::Fixed { 
            width: dimension.size.x, 
            height: dimension.size.y 
        };
    }
}
