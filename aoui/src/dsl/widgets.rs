use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::render::view::RenderLayers;
use bevy::text::Font;
use bevy::window::CursorIcon;
use crate::widgets::button::{CheckButton, Payload, RadioButton, Button};
use crate::{Dimension, Anchor, Size2, Hitbox};
use crate::bundles::{AoUIBundle, AoUISpriteBundle};
use crate::dsl::prelude::{PropagateFocus, SetCursor};
use crate::events::EventFlags;
use crate::widgets::scroll::Scrolling;
use crate::widgets::scrollframe::ClippingBundle;
use crate::widget_extension;
use crate::signals::{Sender, Receiver};
use crate::signals::types::{SigSubmit, SigChange};
use crate::widgets::TextColor;
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea, InputBoxText};

use super::prelude::SigScroll;
use super::{Widget, get_layer, HandleOrString};
use super::builders::FrameBuilder;
use super::util::OptionX;

widget_extension!(
    pub struct InputBoxBuilder {
        pub text: String,
        pub font: HandleOrString<Font>,
        pub color: Option<Color>,    
        pub cursor_bar: Option<Entity>,
        pub cursor_area: Option<Entity>,
        pub change: OptionX<Sender<SigChange>>,
        pub submit: OptionX<Sender<SigSubmit>>,
    },
    this, commands, assets,
    components: (
        InputBox::new(&this.text),
        TextColor(this.color.expect("color is required.")),
        true => this.event.unwrap_or(EventFlags::Drag)
            |EventFlags::DoubleClick|EventFlags::Drag|EventFlags::ClickOutside,
        this.font.get(assets),
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
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// Sends a signal whenever the button is clicked.
        pub submit: OptionX<Sender<SigSubmit>>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
    },
    this, commands, assets,
    components: (
        PropagateFocus,
        Button,
        SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: this.cursor.unwrap_or(CursorIcon::Hand),
        },
        true => this.event.unwrap_or(EventFlags::Click) | EventFlags::Click | EventFlags::Hover,
        None = this.hitbox => Hitbox::FULL, 
        Some(cursor) = this.cursor => SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: cursor,
        },
        OptionX::Some(submit) = this.submit => submit,
        OptionX::Some(payload) = this.payload => payload,
    )
);

widget_extension!(
    pub struct CheckButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked and its value is `true`.
        /// 
        /// Like button, this sends either `()` or `Payload`.
        pub submit: OptionX<Sender<SigSubmit>>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub change: OptionX<Sender<SigChange>>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,
    },
    this, commands, assets,
    components: (
        PropagateFocus,
        SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: this.cursor.unwrap_or(CursorIcon::Hand),
        },
        CheckButton::from(this.checked),
        true => this.event.unwrap_or(EventFlags::Click) | EventFlags::Click | EventFlags::Hover,
        None = this.hitbox => Hitbox::FULL, 
        Some(cursor) = this.cursor => SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: cursor,
        },
        OptionX::Some(submit) = this.submit => submit,
        OptionX::Some(change) = this.change => change,
        OptionX::Some(payload) = this.payload => payload,
    )
);

widget_extension!(
    pub struct RadioButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// The context for the radio button's value.
        pub context: Option<RadioButton>,
        /// Discriminant for this button's value, must be comparable.
        pub value: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked.
        pub submit: OptionX<Sender<SigSubmit>>,
    },
    this, commands, assets,
    components: (
        PropagateFocus,
        SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: this.cursor.unwrap_or(CursorIcon::Hand),
        },
        this.context.expect("Expected RadioButton context."),
        this.value.expect("Expected RadioButton value."),
        true => this.event.unwrap_or(EventFlags::Click) | EventFlags::Click | EventFlags::Hover,
        None = this.hitbox => Hitbox::FULL, 
        Some(cursor) = this.cursor => SetCursor {
            flags: EventFlags::Hover|EventFlags::Pressed,
            icon: cursor,
        },
        OptionX::Some(submit) = this.submit => submit,
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


#[macro_export]
macro_rules! check_button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::CheckButtonBuilder] {$($tt)*})};
}


#[macro_export]
macro_rules! radio_button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RadioButtonBuilder] {$($tt)*})};
}

widget_extension!(
    pub struct ClippingFrameBuilder {
        /// If set, configure scrolling for this widget.
        pub scroll: Option<Scrolling>,
        /// If set, send the scrolling input to another widget if scrolled to the end.
        pub scroll_send: OptionX<Sender<SigScroll>>,
        /// If set, receive the scrolling input from a signal.
        pub scroll_recv: OptionX<Receiver<SigScroll>>,
        /// If set, set the scrolling position to another widget.
        pub scroll_change: OptionX<Sender<SigChange>>,
        /// Set the size of the buffer this is rendered to, won't be resized dynamically.
        pub buffer: [u32; 2],
        /// Layer of the render target, uses scoped layer if not specified. 
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
        /// Sets the viewport of the camera, note default is `Inherit`, which is dynamic.
        pub camera_dimension: Option<Size2>,
    }
);

impl Widget for ClippingFrameBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> Entity {
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
            assets.expect("Please pass in the asset server."), 
            self.buffer, 
            self.layer.expect("Please specify a render layer.")
        );
        let camera = commands.spawn((
            AoUIBundle::empty(Anchor::Center, self.camera_dimension.unwrap_or(Size2::FULL)),
            clip
        )).id();
        let mut render_target = commands.spawn(AoUISpriteBundle {
            dimension: Dimension::INHERIT,
            texture: image,
            ..Default::default()
        });
        if let Some(layer) = self.original_layer {
            render_target.insert(layer);
        } else if let Some(layer) = get_layer(){
            render_target.insert(RenderLayers::layer(layer.get()));
        }
        let container = self.container.expect("Scrolling requires `container` to be set.");
        let render_target = render_target.id();
        if let Some(scroll) = self.scroll {
            let mut frame = commands.spawn((AoUIBundle {
                    dimension: Dimension::INHERIT,
                    ..Default::default()
                },
                EventFlags::MouseWheel,
                scroll,
                Hitbox::FULL,
            ));
            frame.add_child(container);
            if let OptionX::Some(signal) = self.scroll_send {
                frame.insert(signal);
            }
            if let OptionX::Some(signal) = self.scroll_recv {
                frame.insert(signal);
            }
            if let OptionX::Some(signal) = self.scroll_change {
                frame.insert(signal);
            }
            let frame = frame.id();
            commands.entity(entity).push_children(&[camera, render_target, frame]);
        } else {
            commands.entity(entity).push_children(&[camera, render_target, container]);
        }
        entity
    }
}

#[macro_export]
macro_rules! clipping_layer {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ClippingFrameBuilder] {$($tt)*})};
}
