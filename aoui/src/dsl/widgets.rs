use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::render::view::RenderLayers;
use bevy::text::Font;
use bevy::window::CursorIcon;
use crate::widgets::button::{Payload, Button, CheckButton, RadioButton};
use crate::widgets::scrollframe::ClippingBundle;
use crate::{Dimension, Anchor, Size2, Hitbox, build_frame};
use crate::bundles::{AouiBundle, AouiSpriteBundle};
use crate::dsl::prelude::{PropagateFocus, SetCursor};
use crate::events::{EventFlags, Handlers, ButtonClick, ToggleChange, TextChange, TextSubmit, MouseWheel};
use crate::widgets::scroll::Scrolling;
use crate::widget_extension;
use crate::signals::Receiver;
use crate::widgets::inputbox::TextColor;
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea, InputBoxText};

use super::context::with_layer;
use super::prelude::SigScroll;
use super::{Widget, get_layer, HandleOrString};
use super::converters::OptionX;

widget_extension!(
    pub struct InputBoxBuilder {
        pub text: String,
        pub font: HandleOrString<Font>,
        pub color: Option<Color>,    
        pub cursor_bar: Option<Entity>,
        pub cursor_area: Option<Entity>,
        pub on_change: Handlers<TextChange>,
        pub on_submit: Handlers<TextSubmit>,
    }
);

impl Widget for InputBoxBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut entity = build_frame!(commands, self);
        entity.insert((
            InputBox::new(&self.text),
            TextColor(self.color.expect("color is required.")),
            self.font.get(assets),
            self.event.unwrap_or(EventFlags::LeftDrag)|EventFlags::DoubleClick|EventFlags::LeftDrag|EventFlags::ClickOutside,
        ));
        if self.hitbox.is_none() {
            entity.insert(Hitbox::FULL);
        }
        if !self.on_submit.is_empty()  {
            entity.insert(self.on_submit);
        }
        if !self.on_change.is_empty()  {
            entity.insert(self.on_change);
        }
        let entity = entity.id();
        let children = [
            commands.spawn ((
                AouiBundle {
                    dimension: Dimension::INHERIT,
                    ..Default::default()
                },
                InputBoxText,
            )).id(),
            commands.entity(self.cursor_bar.expect("cursor_bar is required."))
                .insert(InputBoxCursorBar)
                .id(),
            commands.entity(self.cursor_area.expect("cursor_bar is required."))
                .insert(InputBoxCursorArea)
                .id()
        ];
        commands.entity(entity).push_children(&children);
        (entity, entity)
    }
}
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
        pub on_click: Handlers<ButtonClick>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
    }
);

impl Widget for ButtonBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            Button,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
        ));
        if self.hitbox.is_none() {
            entity.insert(Hitbox::FULL);
        }
        if !self.on_click.is_empty()  {
            entity.insert(self.on_click);
        }
        if let OptionX::Some(payload) = self.payload  {
            entity.insert(payload);
        }
        (entity.id(), entity.id())
    }
}

widget_extension!(
    pub struct CheckButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked and its value is `true`.
        /// 
        /// Like button, this sends either `()` or `Payload`.
        pub on_checked: Handlers<ButtonClick>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub on_change: Handlers<ToggleChange>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,
    }
);

impl Widget for CheckButtonBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut  entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            CheckButton::from(self.checked),
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
        ));
        if self.hitbox.is_none() {
            entity.insert(Hitbox::FULL);
        }
        if !self.on_checked.is_empty() {
            entity.insert(self.on_checked);
        }
        if !self.on_change.is_empty() {
            entity.insert(self.on_change);
        }
        if let OptionX::Some(payload) = self.payload  {
            entity.insert(payload);
        }
        (entity.id(), entity.id())
    }
}

widget_extension!(
    pub struct RadioButtonBuilder {
        /// Sets the CursorIcon when hovering this button, default is `Hand`
        pub cursor: Option<CursorIcon>,
        /// The context for the radio button's value.
        pub context: Option<RadioButton>,
        /// Discriminant for this button's value, must be comparable.
        pub value: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked.
        pub on_click: Handlers<ButtonClick>,
    }
);

impl Widget for RadioButtonBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let mut entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            self.context.expect("Expected RadioButton context."),
            self.value.expect("Expected RadioButton value."),
            self.event.unwrap_or(EventFlags::LeftClick) | EventFlags::LeftClick | EventFlags::Hover,
        ));
        if self.hitbox.is_none() {
            entity.insert(Hitbox::FULL);
        }
        if !self.on_click.is_empty()  {
            entity.insert(self.on_click);
        }
        (entity.id(), entity.id())
    }
}

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
        pub scroll_send: Handlers<MouseWheel>,
        /// If set, receive the scrolling input from a signal.
        pub scroll_recv: Option<Receiver<SigScroll>>,
        /// Set the size of the buffer this is rendered to, won't be resized dynamically.
        pub buffer: [u32; 2],
        /// Layer of the render target, uses scoped layer if not specified. 
        pub original_layer: Option<RenderLayers>,
        /// Sets the viewport of the camera, note default is `Inherit`, which is dynamic.
        pub camera_dimension: Option<Size2>,
    }
);

impl Widget for ClippingFrameBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, assets: Option<&bevy::prelude::AssetServer>) -> (Entity, Entity) {
        if self.buffer[0] == 0 || self.buffer[1] == 0 {
            panic!("Buffer size cannot be 0.")
        };
        let entity = build_frame!(commands, self).id();
        let (clip, image) = ClippingBundle::new(
            assets.expect("Please pass in the asset server."), 
            self.buffer, 
            self.layer.expect("Please specify a render layer.")
        );
        let camera = commands.spawn((
            AouiBundle::empty(Anchor::Center, self.camera_dimension.unwrap_or(Size2::FULL)),
            clip
        )).id();
        let mut render_target = commands.spawn(AouiSpriteBundle {
            dimension: Dimension::INHERIT,
            texture: image,
            ..Default::default()
        });
        if let Some(layer) = self.original_layer {
            render_target.insert(layer);
        } else if let Some(layer) = get_layer(){
            render_target.insert(layer);
        }
        let render_target = render_target.id();
        let container = if let Some(scroll) = self.scroll {
            let container = commands.spawn(AouiBundle {
                dimension: Dimension::INHERIT,
                ..Default::default()
            }).id();
            let mut frame = commands.spawn((AouiBundle {
                    dimension: Dimension::INHERIT,
                    ..Default::default()
                },
                EventFlags::MouseWheel,
                scroll,
                Hitbox::FULL,
            ));
            frame.add_child(container);
            if !self.scroll_send.is_empty() {
                frame.insert(self.scroll_send);
            }
            if let Some(signal) = self.scroll_recv {
                frame.insert(signal);
            }
            let frame = frame.id();
            commands.entity(entity).push_children(&[camera, render_target, frame]);
            container
        } else {
            let container = commands.spawn(AouiBundle {
                dimension: Dimension::INHERIT,
                ..Default::default()
            }).id();
            commands.entity(entity).push_children(&[camera, render_target, container]);
            container
        };
        (entity, container)
    }

    fn scope_fn<T>(&self, f: impl FnOnce() -> T) -> T {
        with_layer(self.layer.expect("Expected layer"), f)
    }
}

#[macro_export]
macro_rules! clipping_layer {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ClippingFrameBuilder] {$($tt)*})};
}
