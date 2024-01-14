
/// Create a widget builder based on the definition of a primitive widget `Frame`.
///
/// Use `build_frame!` to utilize this definition.
#[macro_export]
macro_rules! frame_extension {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident $([$($generics: tt)*])? {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Default)]
        $(#[$($parent_attr)*])*
        $vis0 struct $name $(<$($generics)*>)? {
            /// Anchor of the sprite.
            pub anchor: $crate::Anchor,
            /// Matched parent anchor of the sprite, default is `anchor`.
            /// Usually should not be set in idiomatic use.
            pub parent_anchor: $crate::dsl::ParentAnchor,
            /// Center of the sprite, default is `center`.
            pub center: $crate::Anchor,
            /// Propagated opacity.
            pub opacity: $crate::Opacity,
            /// Offset of the sprite from parent's anchor.
            pub offset: $crate::Size2,
            /// Rotation of the sprite from `center`.
            pub rotation: f32,
            /// Scale of the sprite from `center`.
            pub scale: $crate::dsl::Scale,
            /// Z depth of the sprite.
            pub z: f32,
            /// If true, clips its children, currently only affects events.
            pub clipping: Option<bool>,
            /// Owned dimension of the sprite.
            ///
            /// If not set, size is fetched dynamically from various sources.
            ///
            /// The `size` field from `SpriteBuilder` sets the size of the underlying sprite instead.
            pub dimension: $crate::DimensionType,
            /// Aspect ratio of sprite, default unused.
            pub aspect: $crate::dsl::Aspect,
            /// Propagated font size.
            pub font_size: $crate::FontSize,
            /// Sets up which event this receives.
            ///
            /// Due to this being a confusing footgun,
            /// setting event here automatically sets hitbox to `Hitbox::rect(1)` if not set manually.
            pub event: $crate::events::EventFlags,
            /// The click detection area of the sprite.
            pub hitbox: Option<$crate::Hitbox>,
            /// The render layer of the sprite.
            pub layer: Option<$crate::bevy::render::view::RenderLayers>,
            /// Layout of the widget's children.
            ///
            /// If this is `Some`, the default `dimension` is `Dynamic` instead of `Copied`.
            pub layout: Option<$crate::layout::LayoutObject>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub margin: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub padding: $crate::dsl::OneOrTwo<$crate::Size2>,
            /// Displayed range of children, default is all, has no effect if widget has no layout.
            pub children_range: $crate::layout::LayoutRange,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }
    };
}

/// Use a `FrameBuilder` to build a frame, returns an `EntityCommands`.
#[macro_export]
macro_rules! build_frame {
    ($commands: expr, $this: expr) => {
        {
            let entity = $crate::util::Widget::spawn($crate::dsl::builders::FrameBuilder {
                anchor: $this.anchor,
                parent_anchor: $this.parent_anchor,
                center: $this.center,
                opacity: $this.opacity,
                offset: $this.offset,
                rotation: $this.rotation,
                scale: $this.scale,
                z: $this.z,
                dimension: $this.dimension,
                font_size: $this.font_size,
                event: $this.event,
                hitbox: $this.hitbox,
                layer: $this.layer,
                aspect: $this.aspect,
                clipping: $this.clipping,
                layout: $this.layout,
                margin: $this.margin,
                padding: $this.padding,
                children_range: $this.children_range,
            }, $commands);
            $commands.entity(entity.0)
        }
    }
}
