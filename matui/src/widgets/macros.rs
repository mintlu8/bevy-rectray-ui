
/// Create a widget builder based on the definition of a matui window `MFrame`.
///
/// Use `build_frame!` to utilize this definition.
#[macro_export]
macro_rules! mframe_extension {
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
            pub anchor: $crate::aoui::Anchor,
            /// Matched parent anchor of the sprite, default is `anchor`.
            /// Usually should not be set in idiomatic use.
            pub parent_anchor: $crate::aoui::dsl::ParentAnchor,
            /// Center of the sprite, default is `center`.
            pub center: $crate::aoui::Anchor,
            /// Propagated opacity.
            pub opacity: $crate::aoui::Opacity,
            /// Offset of the sprite from parent's anchor.
            pub offset: $crate::aoui::Size2,
            /// Rotation of the sprite from `center`.
            pub rotation: f32,
            /// Scale of the sprite from `center`.
            pub scale: $crate::aoui::dsl::Scale,
            /// Z depth of the sprite.
            pub z: f32,
            /// If true, clips its children, currently only affects events.
            pub clipping: Option<bool>,
            /// Owned dimension of the sprite.
            ///
            /// If not set, size is fetched dynamically from various sources.
            ///
            /// The `size` field from `SpriteBuilder` sets the size of the underlying sprite instead.
            pub dimension: $crate::aoui::DimensionType,
            /// Aspect ratio of sprite, default unused.
            pub aspect: $crate::aoui::dsl::Aspect,
            /// Propagated font size.
            pub font_size: $crate::aoui::FontSize,
            /// Sets up which event this receives.
            ///
            /// Due to this being a confusing footgun,
            /// setting event here automatically sets hitbox to `Hitbox::rect(1)` if not set manually.
            pub event: $crate::aoui::events::EventFlags,
            /// The click detection area of the sprite.
            pub hitbox: Option<$crate::aoui::Hitbox>,
            /// The render layer of the sprite.
            pub layer: Option<$crate::bevy::render::view::RenderLayers>,
            /// Layout of the widget's children.
            ///
            /// If this is `Some`, the default `dimension` is `Dynamic` instead of `Copied`.
            pub layout: Option<$crate::aoui::layout::LayoutObject>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub margin: $crate::aoui::dsl::OneOrTwo<$crate::aoui::Size2>,
            /// Margin of the widget's layout, has no effect if widget has no layout.
            pub padding: $crate::aoui::dsl::OneOrTwo<$crate::aoui::Size2>,
            /// Displayed range of children, default is all, has no effect if widget has no layout.
            pub children_range: $crate::aoui::layout::LayoutRange,
            /// Shadow width and color.
            pub shadow: $crate::aoui::dsl::OptionEx<$crate::widgets::ShadowInfo>,
            /// Color of the sprite.
            pub palette: $crate::style::Palette,
            /// Stroke of the sprite.
            pub stroke: f32,
            /// Stroke of the sprite.
            pub radius: f32,
            /// Is this a capsule?
            pub capsule: bool,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }
    };
}

/// Use a `MFrameBuilder` to build a frame, returns an `EntityCommands`.
#[macro_export]
macro_rules! build_mframe {
    ($commands: expr, $this: expr) => {
        {
            let entity = $crate::aoui::util::Widget::spawn($crate::widgets::MFrameBuilder {
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
                shadow: $this.shadow,
                palette: $this.palette,
                stroke: $this.stroke,
                radius: $this.radius,
                capsule: $this.capsule,
            }, $commands);
            $commands.entity(entity.0)
        }
    }
}
