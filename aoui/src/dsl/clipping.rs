use bevy::{render::texture::Image, hierarchy::BuildChildren, ecs::entity::Entity, asset::Handle};
use crate::{widget_extension, build_frame, Clipping, frame, Size2, events::{EventFlags, ESigCoveragePx, Handlers, ESigCoveragePercent}};
use crate::widgets::{scroll::IntoScrollingBuilder, clipping::ScopedCameraBundle};
use super::Widget;

widget_extension!(
    /// A camera with its viewport bound to a sprite's `RotatedRect`.`
    pub struct CameraFrameBuilder {
        pub render_target: Option<Handle<Image>>,
    }
);

impl Widget for CameraFrameBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (Entity, Entity) {
        let Some(buffer) = self.render_target else  {panic!("Requires \"buffer\"")};
        let entity = build_frame!(commands, self).id();

        let bundle = ScopedCameraBundle::from_image(
            buffer,
            self.layer.expect("Please specify a render layer.")
        );
        commands.entity(entity).insert(bundle);

        (entity, entity)
    }
}

widget_extension!(
    pub struct ScrollingFrameBuilder[B: IntoScrollingBuilder] {
        /// If set, configure scrolling for this widget.
        pub scroll: Option<B>,
        pub coverage_px: Handlers<ESigCoveragePx>,
        pub coverage_percent: Handlers<ESigCoveragePercent>,
    }
);

impl<B: IntoScrollingBuilder> Widget for ScrollingFrameBuilder<B> {
    fn spawn_with(mut self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (Entity, Entity) {
        match &mut self.event {
            Some(flag) => *flag |= EventFlags::MouseWheel,
            None => self.event = Some(EventFlags::MouseWheel),
        }
        let entity = build_frame!(commands, self).id();
        commands.entity(entity).insert(self.scroll.expect("Expect `scroll`").with_constraints());
        if self.clipping.is_none(){
            commands.entity(entity).insert(Clipping::new(true));
        }
        let container = frame!(commands {
            dimension: Size2::FULL,
        });
        if !self.coverage_px.is_empty() {
            commands.entity(container).insert(self.coverage_px);
        }
        if !self.coverage_percent.is_empty() {
            commands.entity(container).insert(self.coverage_percent);
        }
        commands.entity(entity).add_child(container);
        (entity, container)
    }
}


/// Constructs a camera with its viewport bound to a sprite's `RotatedRect`.`
/// 
/// See [`CameraFrameBuilder`].
#[macro_export]
macro_rules! camera_frame {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::CameraFrameBuilder] {$($tt)*})};
}


/// Constructs a layer that clips its inner content.
/// 
/// See [`ScrollingFrameBuilder`].
/// 
/// This spawns a camera, uses a new render layer
/// and renders to a new render target. 
#[macro_export]
macro_rules! scrolling {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ScrollingFrameBuilder] {$($tt)*})};
}
