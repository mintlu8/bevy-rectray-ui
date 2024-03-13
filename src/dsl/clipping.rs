use bevy::{render::texture::Image,  ecs::entity::Entity, asset::Handle};
use crate::{frame_extension, build_frame};
use crate::widgets::clipping::ScopedCameraBundle;
use crate::util::{Widget, RCommands};

frame_extension!(
    /// A camera with its viewport bound to a sprite's `RotatedRect`.
    pub struct CameraFrameBuilder {
        /// Render target of the camera.
        pub render_target: Option<Handle<Image>>,
    }
);

impl Widget for CameraFrameBuilder {
    fn spawn(self, commands: &mut RCommands) -> (Entity, Entity) {
        let Some(buffer) = self.render_target else  {panic!("Requires \"buffer\"")};
        let entity = build_frame!(commands, self).id();

        let bundle = ScopedCameraBundle::new(
            buffer,
            self.layer.expect("Please specify a render layer.")
        );
        commands.entity(entity).insert(bundle);

        (entity, entity)
    }
}

/// Constructs a camera with its viewport bound to a sprite's `RotatedRect`.
///
/// See [`CameraFrameBuilder`].
#[macro_export]
macro_rules! camera_frame {
    {$commands: tt {$($tt:tt)*}} =>
        {$crate::meta_dsl!($commands [$crate::dsl::builders::CameraFrameBuilder] {$($tt)*})};
}
