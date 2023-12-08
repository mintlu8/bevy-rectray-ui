use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::render::view::RenderLayers;
use bevy::text::Font;
use bevy::asset::Handle;
use bevy::window::CursorIcon;
use bevy_aoui::{Dimension, Anchor, Size2, Hitbox};
use bevy_aoui::bundles::{AoUIBundle, AoUISpriteBundle};
use crate::dsl::prelude::{PropagateFocus, SetCursor};
use crate::events::EventFlags;
use crate::widgets::scroll::Scrolling;
use crate::widgets::scrollframe::ClippingBundle;
use crate::{widget_extension, Submit, Sender, Change};
use crate::widgets::TextColor;
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea, InputBoxText};

use super::AoUIWidget;
use super::builders::FrameBuilder;
use super::util::OptionX;

widget_extension!(
    pub struct InputBoxBuilder {
        pub text: String,
        pub font: Handle<Font>,
        pub color: Option<Color>,    
        pub cursor_bar: Option<Entity>,
        pub cursor_area: Option<Entity>,
        pub change: OptionX<Sender<Change>>,
        pub submit: OptionX<Sender<Submit>>,
    },
    this, commands, assets,
    components: (
        InputBox::new(&this.text),
        TextColor(this.color.expect("color is required.")),
        true => this.event.unwrap_or(EventFlags::Drag)
            |EventFlags::DoubleClick|EventFlags::Drag|EventFlags::ClickOutside,
        this.font,
        OptionX::Some(signal) = this.change => signal,
        OptionX::Some(signal) = this.submit => signal,
    ),
    spawn: (
        commands.spawn ((
            AoUIBundle {
                dimension: Dimension::INHERIT,
                ..Default::default()
            },
            InputBoxText,
        )).id(),
        this.cursor_bar.expect("cursor_bar is required.") => InputBoxCursorBar,
        this.cursor_area.expect("cursor_area is required.") => InputBoxCursorArea,
    )
);

/// Construct a textbox.
#[macro_export]
macro_rules! inputbox {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::InputBoxBuilder] {$($tt)*})};
}

widget_extension!(
    pub struct ButtonBuilder {
        pub cursor: Option<CursorIcon>,
        pub signal: OptionX<Sender<Submit>>,
    },
    this, commands, assets,
    components: (
        PropagateFocus,
        SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: CursorIcon::Hand,
        },
        OptionX::Some(signal) = this.signal => signal,
        true => this.event.unwrap_or(EventFlags::Click) | EventFlags::Click | EventFlags::Hover,
        Some(cursor) = this.cursor => SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: cursor,
        },
    )
);

/// Construct a button.
/// 
/// This doesn't do a whole lot by itself, these are what `button` does:
/// 
/// * Add a event listener for `Hover` and `Click`
/// * If `cursor` is set, change cursor icon when hovering or pressing.
/// * If `signal` is set, change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its direct children.
/// 
/// You can use the `extra: handler!(Click => fn name() {..})` pattern to handle clicks
/// and use [`DisplayIf`](crate::widgets::DisplayIf) for simple UI interaction.
#[macro_export]
macro_rules! button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ButtonBuilder] {$($tt)*})};
}

widget_extension!(
    pub struct ClippingFrameBuilder {
        /// If set, configure scrolling for this widget.
        pub scroll: Option<Scrolling>,
        /// Set the size of the buffer this is rendered to, won't be resized dynamically.
        pub buffer: [u32; 2],
        /// If the layer of the render target is not `0`, 
        /// you need to set is here.
        pub original_layer: Option<RenderLayers>,
        /// Add an entity as the container for scrolling, should usually be
        /// ```
        /// # /*
        /// container: frame! {
        ///     dimension: Size2::FULL,
        ///     child: ..
        ///     child: ..
        ///     ..
        /// }
        /// # */
        /// ```
        pub container: Option<Entity>,
    }
);

impl AoUIWidget for ClippingFrameBuilder {
    fn spawn_with<'w, 's>(self, commands: &mut bevy::prelude::Commands<'w, 's>, assets: Option<&bevy::prelude::AssetServer>) -> Entity {
        if self.buffer[0] == 0 || self.buffer[1] == 0 {
            panic!("Buffer size cannot be 0.")
        }
        let entity = FrameBuilder {
            anchor: self.anchor,
            parent_anchor: self.parent_anchor,
            center: self.center,
            opacity: self.opacity,
            visible: self.visible,
            offset: self.offset,
            rotation: self.rotation,
            scale: self.scale,
            z: self.z,
            dimension: self.dimension,
            font_size: self.font_size,
            event: self.event,
            hitbox: self.hitbox,
            layer: self.original_layer,
        }.spawn_with(commands, assets);
        let (clip, image) = ClippingBundle::new(
            &assets.expect("Please pass in the asset server."), 
            self.buffer, 
            self.layer.expect("Please specify a render layer.")
        );
        let camera = commands.spawn((
            AoUIBundle::empty(Anchor::Center, Size2::FULL),
            clip
        )).id();
        let mut render_target = commands.spawn((
            AoUISpriteBundle {
                dimension: Dimension::INHERIT,
                texture: image,
                ..Default::default()
            },
        ));
        if let Some(scroll) = self.scroll {
            render_target.insert(EventFlags::MouseWheel|self.event.unwrap_or(EventFlags::MouseWheel));
            render_target.insert(scroll);
            render_target.insert(self.hitbox.unwrap_or(Hitbox::FULL));
            render_target.add_child(self.container.expect("Scrolling requires `container` to be set."));
        };
        if let Some(layer) = self.original_layer {
            render_target.insert(layer);
        }
        let render_target = render_target.id();
        commands.entity(entity).push_children(&[camera, render_target]);
        entity
    }
}

#[macro_export]
macro_rules! clipping_frame {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ClippingFrameBuilder] {$($tt)*})};
}
