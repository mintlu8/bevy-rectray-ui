use bevy::ecs::entity::Entity;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::Font;
use bevy::window::CursorIcon;
use crate::widgets::button::{Payload, Button, CheckButton, RadioButton, RadioButtonCancel, SetCursor, PropagateFocus};
use crate::{build_frame, Dimension};
use crate::bundles::AouiBundle;
use crate::events::{EventFlags, Handlers, EvButtonClick, EvToggleChange, EvTextChange, EvTextSubmit};
use crate::widget_extension;
use crate::widgets::inputbox::{TextColor, InputOverflow};
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea, InputBoxText};

use super::{Widget, HandleOrString, AouiCommands};
use super::converters::OptionX;

#[doc(hidden)]
#[macro_export]
macro_rules! inject_event {
    ($this: expr, $flags: expr) => {
        match &mut $this {
            Some(event) => *event |= $flags,
            None => $this = Some($flags),
        }
    };
}

widget_extension!(
    pub struct InputBoxBuilder {
        pub text: String,
        pub font: HandleOrString<Font>,
        pub color: Option<Color>,    
        pub cursor_bar: Option<Entity>,
        pub cursor_area: Option<Entity>,
        pub on_change: Handlers<EvTextChange>,
        pub on_submit: Handlers<EvTextSubmit>,
        pub overflow: InputOverflow,
        /// Sets the CursorIcon when hovering this button, default is `Text`
        pub cursor_icon: Option<CursorIcon>,
    }
);

impl Widget for InputBoxBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        inject_event!(self.event, EventFlags::Hover|EventFlags::DoubleClick|EventFlags::LeftDrag|EventFlags::ClickOutside);
        let font = self.font.get(&commands);
        let mut entity = build_frame!(commands, self);
        entity.insert((
            InputBox::new(&self.text, self.overflow),
            TextColor(self.color.expect("color is required.")),
            font,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftDrag,
                icon: self.cursor_icon.unwrap_or(CursorIcon::Text),
            },
        ));
        if !self.on_submit.is_empty()  {
            entity.insert(self.on_submit);
        }
        if !self.on_change.is_empty()  {
            entity.insert(self.on_change);
        }
        let entity = entity.id();
        let children = [
            commands.spawn_bundle((
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
/// Construct a `input_box`. The underlying struct is [`InputBoxBuilder`].
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
        pub on_click: Handlers<EvButtonClick>,
        /// If set, `submit` sends its contents.
        pub payload: OptionX<Payload>,
    }
);

impl Widget for ButtonBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        inject_event!(self.event, EventFlags::Hover|EventFlags::LeftClick);
        let mut entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            Button,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
        ));
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
        pub on_checked: Handlers<EvButtonClick>,
        /// Sends a `bool` signal whenever the button is clicked.
        pub on_change: Handlers<EvToggleChange>,
        /// Sets whether the default value is checked or not.
        pub checked: bool,
    }
);

impl Widget for CheckButtonBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        inject_event!(self.event, EventFlags::Hover|EventFlags::LeftClick);
        let mut  entity = build_frame!(commands, self);
        entity.insert((
            PropagateFocus,
            CheckButton::from(self.checked),
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
        ));
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
        /// If true, behave like a `CheckButton` and set context to `None` if already checked.
        pub cancellable: bool,
        /// Discriminant for this button's value, must be comparable.
        pub value: OptionX<Payload>,
        /// Sends a signal whenever the button is clicked.
        pub on_click: Handlers<EvButtonClick>,
    }
);

impl Widget for RadioButtonBuilder {
    fn spawn(mut self, commands: &mut AouiCommands) -> (Entity, Entity) {
        inject_event!(self.event, EventFlags::Hover|EventFlags::LeftClick);
        let mut entity = build_frame!(commands, self);

        entity.insert((
            PropagateFocus,
            SetCursor {
                flags: EventFlags::Hover|EventFlags::LeftPressed,
                icon: self.cursor.unwrap_or(CursorIcon::Hand),
            },
            self.context.expect("Expected RadioButton context."),
            self.value.expect("Expected RadioButton value."),
        ));
        if self.cancellable {
            entity.insert(RadioButtonCancel);
        }
        if !self.on_click.is_empty()  {
            entity.insert(self.on_click);
        }
        (entity.id(), entity.id())
    }
}

/// Construct a button. The underlying struct is [`ButtonBuilder`].
/// 
/// # Features
/// 
/// `button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `button` function properly.
/// 
/// These are what `button` does compared to `frame`:
/// 
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Allow usage of `EvButtonClick` event. Which uses the button's [`Payload`].
/// 
/// You can use [`Handlers`] to handle clicks
/// and use [`DisplayIf`](crate::widgets::button::DisplayIf) 
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
/// 
/// # Common Pitfall
/// 
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::ButtonBuilder] {$($tt)*})};
}


/// Construct a `check_button`. The underlying struct is [`CheckButtonBuilder`].
/// 
/// # Features
/// 
/// `check_button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `check_button` function properly.
/// 
/// These are what `check_button` does compared to `frame`:
/// 
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Hold a boolean context value for if the button is checked or not. 
/// * Generate `CheckButtonState` based on the context. 
/// * Allow usage of `EvButtonClick` event. Which uses the button's [`Payload`].
/// 
/// You can use [`Handlers`] to handle clicks
/// and use [`DisplayIf`](crate::widgets::button::DisplayIf) 
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
/// 
/// # Common Pitfall
/// 
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! check_button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::CheckButtonBuilder] {$($tt)*})};
}


/// Construct a `radio_button`. The underlying struct is [`RadioButtonBuilder`].
/// 
/// This is in fact very versatile and can be used for any exclusive UI elements
/// like a dropdown select or an accordion.
/// 
/// # Features
/// 
/// `radio_button` is a widget primitive with no default look. You need to nest
/// `sprite` or `text` as children to make `radio_button` function properly.
/// 
/// These are what `radio_button` does compared to `frame`:
/// 
/// * Add event listeners for `Hover` and `Click`
/// * Change cursor icon when hovering or pressing.
/// * Propagate its status `Down`, `Click`, `Hover`, `Pressed` to its descendants.
/// * Hold a [`Payload`] value as a discriminant. 
/// * Generate `CheckButtonState` based on the context and payload. 
/// * Send payload value through `EvButtonClick`.
/// 
/// You can use [`Handlers`] to handle clicks
/// and use [`DisplayIf`](crate::widgets::button::DisplayIf) 
/// or [`Interpolate`](crate::anim::Interpolate) for simple UI interaction.
/// 
/// # Common Pitfall
/// 
/// Do not nest `button`, `check_button` or `radio_button` inside a button.
/// Button propagates its state to all its descendants and can inject unwanted state.
/// Introduce a common parent instead.
#[macro_export]
macro_rules! radio_button {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::builders::RadioButtonBuilder] {$($tt)*})};
}
