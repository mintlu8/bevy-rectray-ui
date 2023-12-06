use bevy::ecs::entity::Entity;
use bevy::render::color::Color;
use bevy::text::Font;
use bevy::asset::Handle;
use bevy::window::CursorIcon;
use bevy_aoui::Dimension;
use bevy_aoui::bundles::AoUIBundle;
use crate::dsl::prelude::{PropagateFocus, SetCursor};
use crate::events::EventFlags;
use crate::{widget_extension, Submit, Sender, Change};
use crate::widgets::TextColor;
use crate::widgets::inputbox::{InputBox, InputBoxCursorBar, InputBoxCursorArea, InputBoxText};

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
    this, commands,
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
    this, commands,
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
