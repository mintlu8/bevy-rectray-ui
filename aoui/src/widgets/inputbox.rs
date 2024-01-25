use std::mem;
use crate::anim::VisibilityToggle;
use crate::dimension::DimensionMut;
use crate::events::{
    ActiveDetection, CursorAction, CursorClickOutside, CursorFocus, CursorState,
    EventFlags,
};
use crate::sync::{SignalId, SignalSender};
use crate::{RotatedRect, Transform2D, DimensionData, Size, size, AouiRem};
use ab_glyph::{Font as FontTrait, ScaleFont};
use bevy::asset::{Assets, Handle};
use bevy::ecs::query::Or;

use bevy::ecs::{event::EventReader, query::Changed, system::Commands};
use bevy::hierarchy::Children;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::{Component, Entity, Query, Res, With, Without};
use bevy::reflect::Reflect;

use bevy::text::Font;
use bevy::window::ReceivedCharacter;
use super::TextFragment;
use super::text::measure_string;
use super::util::{DisplayIf, BlockPropagation};

#[derive(Debug)]
pub enum TextChange {}

impl SignalId for TextChange {
    type Data = String;
}

#[derive(Debug)]
pub enum TextSubmit {}

impl SignalId for TextSubmit {
    type Data = String;
}

#[derive(Debug, Default, Clone, Copy, Reflect)]
enum LeftRight {
    Left,
    #[default]
    Right,
}

mod sealed {
    use bevy::ecs::component::Component;

    tlbf::tlbf!(
        #[derive(Component)]
        pub InputBoxState: u8 {
            Focus,
            Text,
        }
    );
}

pub use sealed::InputBoxState;


/// If we deny overflowing input or not.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub enum InputOverflow {
    /// Deny overflow.
    #[default]
    Deny,
    /// Allow overflow but not handle it.
    Allow,
    /// Allow n characters
    Characters(usize),
    /// Not implemented.
    Scroll,
}

/// Context for a single line text input.
/// Holds text and cursor information.
///
/// InputBox requires 3 children to function properly:
///
/// * [`InputBoxText`]: empty frame component containing individual glyphs.
/// * [`InputBoxCursorBar`]: vertical bar of the cursor.
/// * [`InputBoxCursorArea`]: select area of the cursor.
///
/// Warning: This widget does not rebuild its glyph entities every frame,
/// might not behave properly if tempered externally.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct InputBox {
    overflow: InputOverflow,
    cursor_start: usize,
    cursor_len: usize,
    text: String,
    focus: bool,
    active: LeftRight,
    max_len: Size,
    em: f32,
}

/// Marker component for a sprite containing renderred glyphs.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct InputBoxText;

/// Marker component for a vertical bar of the cursor.
///
/// This component can be any sprite.
///
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct InputBoxCursorBar;

/// Marker component for the area of the cursor.
///
/// This component sets the `dimension` when area is changed, so anything that
/// updates alongside dimension can be used here.
///
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct InputBoxCursorArea;

impl InputBox {
    pub fn new(s: impl Into<String>, overflow: InputOverflow) -> Self {
        Self {
            text: s.into(),
            overflow,
            max_len: size!(100%),
            ..Default::default()
        }
    }

    pub fn with_width(mut self, s: Size) -> Self {
        self.max_len = s;
        self
    }

    /// Get length of the text in the widget.
    pub fn len(&self) -> usize {
        self.text.chars().count()
    }

    /// Returns true if text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns length of the cursor range.
    pub fn cursor_len(&self) -> usize {
        self.cursor_len
    }

    /// Obtain the string in the textbox.
    pub fn get(&self) -> &str {
        &self.text
    }

    /// Set the range of cursor in this widget.
    pub fn set_cursor(&mut self, start: usize, end: usize) {
        self.cursor_start = start;
        self.cursor_len = end.saturating_sub(start);
    }

    /// Returns true if the widget has focus.
    pub fn has_focus(&self) -> bool {
        self.focus
    }

    /// Set the widget as focused.
    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus
    }

    /// Get the selected portion of the string.
    pub fn selected(&self) -> &str {
        use substring::Substring;
        self.text
            .substring(self.cursor_start, self.cursor_start + self.cursor_len)
    }

    /// Select all text in the widget by setting widget to `[0, len]`.
    pub fn select_all(&mut self) {
        self.cursor_start = 0;
        self.cursor_len = self.text.chars().count();
    }

    /// Clear the text of the widget and reset cursor to `[0, 0]`.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_start = 0;
        self.cursor_len = 0;
        self.focus = false;
    }

    /// Set the text of the widget and reset cursor to `[0, 0]`.
    pub fn set(&mut self, s: impl Into<String>) {
        self.text = s.into();
        self.cursor_start = 0;
        self.cursor_len = 0;
        self.focus = false;
    }

    /// Try push char and obtain the string, may deny based on length.
    pub fn try_push(&self, c: char) -> String {
        self.text
            .chars()
            .take(self.cursor_start)
            .chain(std::iter::once(c))
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect()
    }

    /// Simulate the behavior of typing a char.
    pub fn push(&mut self, c: char) {
        self.text = self
            .text
            .chars()
            .take(self.cursor_start)
            .chain(std::iter::once(c))
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect();
        self.cursor_start += 1;
        self.cursor_len = 0;
    }

    /// Try push str and obtain the string, may deny based on length.
    pub fn try_push_str(&self, c: &str) -> String {
        self.text
            .chars()
            .take(self.cursor_start)
            .chain(c.chars())
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect()
    }

    /// Simulates the behavior of pasting.
    pub fn push_str(&mut self, c: &str) {
        let len = c.chars().count();
        self.text = self
            .text
            .chars()
            .take(self.cursor_start)
            .chain(c.chars())
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect();
        self.cursor_start += len;
        self.cursor_len = 0;
    }

    /// Simulates the behavior of clicking `left`.
    pub fn cursor_left(&mut self) {
        match self.cursor_len {
            0 => self.cursor_start = self.cursor_start.saturating_sub(1),
            _ => self.cursor_len = 0,
        }
    }

    /// Simulates the behavior of clicking `right`.
    pub fn cursor_right(&mut self) {
        match self.cursor_len {
            0 => self.cursor_start = (self.cursor_start + 1).min(self.len()),
            _ => {
                self.cursor_start += self.cursor_len;
                self.cursor_len = 0;
            }
        }
    }

    /// Simulates the behavior of clicking `shift-left`.
    pub fn cursor_select_left(&mut self) {
        match (self.cursor_len, self.active) {
            (0, _) => {
                if self.cursor_start == 0 {
                    return;
                }
                self.active = LeftRight::Left;
                self.cursor_start -= 1;
                self.cursor_len += 1;
            }
            (_, LeftRight::Left) => {
                if self.cursor_start == 0 {
                    return;
                }
                self.cursor_start -= 1;
                self.cursor_len += 1;
            }
            (_, LeftRight::Right) => {
                self.cursor_len -= 1;
            }
        }
    }

    /// Simulate the behavior of clicking `shift-right`.
    pub fn cursor_select_right(&mut self) {
        match (self.cursor_len, self.active) {
            (0, _) => {
                if self.cursor_start + self.cursor_len >= self.len() {
                    return;
                }
                self.active = LeftRight::Right;
                self.cursor_len += 1;
            }
            (_, LeftRight::Left) => {
                self.cursor_start += 1;
                self.cursor_len -= 1;
            }
            (_, LeftRight::Right) => {
                if self.cursor_start + self.cursor_len >= self.len() {
                    return;
                }
                self.cursor_len += 1;
            }
        }
    }

    /// Simulate the behavior of clicking `backspace`.
    pub fn backspace(&mut self) {
        if self.cursor_len > 0 {
            self.swap_selected("");
        } else if self.cursor_start > 0 {
            self.cursor_start -= 1;
            self.cursor_len = 1;
            self.swap_selected("");
        }
    }

    /// Simulate the behavior of clicking `delete`.
    pub fn delete(&mut self) {
        if self.cursor_len > 0 {
            self.swap_selected("");
        } else if self.cursor_start < self.len() - 1 {
            self.cursor_len += 1;
            self.swap_selected("");
        }
    }

    /// Swap the selected area with another string.
    pub fn swap_selected(&mut self, swapped: &str) -> String {
        let len = swapped.chars().count();
        let result = self
            .text
            .chars()
            .skip(self.cursor_start)
            .take(self.cursor_len)
            .collect();
        self.text = self
            .text
            .chars()
            .take(self.cursor_start)
            .chain(swapped.chars())
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect();
        self.cursor_len = len;
        result
    }
}

pub(crate) fn text_on_mouse_down(
    state: Res<CursorState>,
    fonts: Res<Assets<Font>>,
    mut query: Query<(&CursorFocus, &mut InputBox, &Handle<Font>, &Children)>,
    child: Query<(&DimensionData, &RotatedRect), With<InputBoxText>>
) {
    for (focus, mut input_box, font, chiildren) in query.iter_mut() {
        if !focus.intersects(EventFlags::LeftDrag) {
            continue;
        };
        let Some(font) = fonts.get(font) else {continue};
        // We only need to care about scale factor in rendering

        let Some((dim, rect)) = child.iter_many(chiildren).next() else {continue};
        let font = font.font.as_scaled(dim.em);

        let curr = rect.local_space(state.cursor_position()).x;
        let down = rect.local_space(state.down_position()).x;

        let mut start = None;
        let mut end = None;
        let mut last_char = font.glyph_id(' ');
        let mut cursor = -dim.size.x / 2.0;
        for (index, char) in input_box.text.chars().enumerate() {
            let id = font.glyph_id(char);
            cursor += font.kern(last_char, id);

            let half = (font.h_advance(id) - font.h_side_bearing(id)) / 2.0;

            if start.is_none() && curr < cursor + half {
                start = Some(index)
            }
            if end.is_none() && down < cursor + half {
                end = Some(index)
            }
            cursor += font.h_advance(id);
            last_char = id;
            if start.is_some() && end.is_some() {
                break;
            }
        }
        let count = input_box.text.chars().count();
        let start = start.unwrap_or(count);
        let end = end.unwrap_or(count);

        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };
        input_box.set_cursor(start, end);
        input_box.set_focus(true);
    }
}

pub(crate) fn text_on_mouse_double_click(mut query: Query<(&mut InputBox, &CursorAction)>) {
    for (mut input_box, action) in query.iter_mut() {
        if action.is(EventFlags::DoubleClick) {
            input_box.select_all();
            input_box.set_focus(true);
        }
    }
}

pub(crate) fn text_propagate_focus(
    mut commands: Commands,
    query: Query<(Entity, &InputBox)>,
    entities: Query<Option<&Children>, Without<BlockPropagation>>,
) {
    let mut queue = Vec::new();
    for (entity, input_box) in query.iter() {
        if input_box.has_focus() && !input_box.text.is_empty() {
            commands.entity(entity).insert(InputBoxState::Focus | InputBoxState::Text);
            queue.push((entity, InputBoxState::Focus | InputBoxState::Text));
        } else if input_box.has_focus() {
            commands.entity(entity).insert(InputBoxState::Focus);
            queue.push((entity, InputBoxState::Focus));
        } else if !input_box.text.is_empty() {
            commands.entity(entity).insert(InputBoxState::Text);
            queue.push((entity, InputBoxState::Text));
        }
    }
    while !queue.is_empty() {
        for (entity, focus) in mem::take(&mut queue) {
            commands.entity(entity).insert(focus);
            if let Ok(Some(children)) = entities.get(entity) {
                for entity in children {
                    queue.push((*entity, focus))
                }
            }
        }
    }
}

pub(crate) fn inputbox_conditional_visibility(
    mut query: Query<(Option<&InputBoxState>, &DisplayIf<InputBoxState>, VisibilityToggle)>,
) {
    query.iter_mut().for_each(|(focus, display_if, mut vis)| {
        if let Some(flag) = focus {
            if flag.contains(display_if.0) {
                vis.set_visible(true)
            } else {
                vis.set_visible(false)
            }
        } else {
            vis.set_visible(false)
        }
    })
}

pub(crate) fn update_inputbox_cursor(
    fonts: Res<Assets<Font>>,
    query: Query<(&InputBox,  &Handle<Font>, ActiveDetection, &Children),
        (Changed<InputBox>, Without<InputBoxText>, Without<InputBoxCursorBar>, Without<InputBoxCursorArea>)>,
    text: Query<(&Children, &DimensionData), With<InputBoxText>>,
    mut bar: Query<(&mut Transform2D, VisibilityToggle),
        (With<InputBoxCursorBar>, Without<InputBoxText>, Without<InputBoxCursorArea>, Without<InputBox>)>,
    mut area: Query<(&mut Transform2D, DimensionMut, VisibilityToggle),
        (With<InputBoxCursorArea>, Without<InputBoxText>, Without<InputBoxCursorBar>, Without<InputBox>)>,
) {
    for (input_box, font_handle, active, children) in query.iter() {
        if !active.is_active() || !input_box.focus {
            let Some((children, _)) = text.iter_many(children).next() else {continue};
            let mut iter = bar.iter_many_mut(children);
            while let Some((.., mut vis)) = iter.fetch_next() {
                vis.set_visible(false)
            }

            let mut iter = area.iter_many_mut(children);
            while let Some((.., mut vis)) = iter.fetch_next() {
                vis.set_visible(false)
            }
            continue;
        }

        let Some((children, dimension)) = text.iter_many(children).next() else {continue};
        let font = match fonts.get(font_handle) {
            Some(font) => font.font.as_scaled(dimension.em),
            None => continue,
        };

        let mut cursor = -dimension.size.x / 2.0;
        let (start_index, end_index) = (
            input_box.cursor_start,
            (input_box.cursor_start + input_box.cursor_len).saturating_sub(1),
        );
        let (mut start, mut end) = (cursor, cursor);
        let mut max = (0, 0.0);
        let mut last = '\0';
        for (index, chara) in input_box.text.chars().enumerate() {
            let glyph = font.scaled_glyph(chara);
            cursor += font.kern(font.glyph_id(last), font.glyph_id(chara));
            last = chara;
            let bounds = font.glyph_bounds(&glyph);
            if index == start_index {
                start = cursor + bounds.min.x;
            }
            if index == end_index {
                end = cursor + bounds.max.x;
            }
            max = (index, end);
            cursor += font.h_advance(font.glyph_id(chara));
        }

        if start_index == max.0 + 1 {
            start = max.1;
        }

        if end_index == max.0 + 1 {
            end = max.1;
        }

        if input_box.cursor_start + input_box.cursor_len == 0 {
            end = start;
        }
        if input_box.cursor_len == 0 {
            let mut iter = bar.iter_many_mut(children);
            while let Some((mut transform, mut vis)) = iter.fetch_next() {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                vis.set_visible(true)
            }

            let mut iter = area.iter_many_mut(children);
            while let Some((.., mut vis)) = iter.fetch_next() {
                vis.set_visible(false)
            }
        } else {
            let mut iter = bar.iter_many_mut(children);
            while let Some((.., mut vis)) = iter.fetch_next() {
                vis.set_visible(false)
            }
            let mut iter = area.iter_many_mut(children);
            while let Some((mut transform, mut dimension, mut vis)) = iter.fetch_next() {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                dimension.edit_raw(|v| v.x = end - start);
                vis.set_visible(true)
            }
        }
    }
}
#[cfg(not(target_os = "macos"))]
const CONTROL: [KeyCode; 2] = [KeyCode::ControlLeft, KeyCode::ControlRight];
#[cfg(target_os = "macos")]
const CONTROL: [KeyCode; 2] = [KeyCode::SuperLeft, KeyCode::SuperLeft];

pub(crate) fn text_on_click_outside(mut query: Query<&mut InputBox, With<CursorClickOutside>>) {
    for mut input in query.iter_mut() {
        input.focus = false;
    }
}
pub(crate) fn inputbox_keyboard(
    rem: Res<AouiRem>,
    fonts: Res<Assets<Font>>,
    mut events: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&DimensionData, &mut InputBox, &Handle<Font>,
        &Children,
        SignalSender<TextChange>,
        SignalSender<TextSubmit>,
        ActiveDetection)>,
    text: Query<&Children, With<InputBoxText>>,
    mut bar: Query<VisibilityToggle,
        (With<InputBoxCursorBar>, Without<InputBoxCursorArea>, Without<InputBox>)>,
    mut area: Query<VisibilityToggle,
        (With<InputBoxCursorArea>, Without<InputBoxCursorBar>, Without<InputBox>)>,
) {
    // Since the order is input -> draw text -> propagate -> move cursor
    // We can't resolve dimension on the same frame
    // therefore we hide cursor area for a frame here if it is removed 
    // to prevent an off-by-1-frame error.
    // keeping bar is fine since it moves at most half a glyph to the target on this frame
    let mut temp_set_invisible = |children: &Children| {
        let Some(children) = text.iter_many(children).next() else {return};
        let mut iter = bar.iter_many_mut(children);
        while let Some(mut vis) = iter.fetch_next() {
            vis.set_visible(false)
        }
        let mut iter = area.iter_many_mut(children);
        while let Some(mut vis) = iter.fetch_next() {
            vis.set_visible(false)
        }
    };

    for (dimension, mut inputbox, font_handle, children, change, submit, active) in
        query.iter_mut().filter(|(_, input, ..)| input.has_focus())
    {
        let em = dimension.em;
        let dimension = inputbox.max_len.as_pixels(dimension.size.x, dimension.em, rem.get());
        if !active.is_active() {
            inputbox.focus = false;
            continue;
        }
        let mut changed = false;
        let is_area = inputbox.cursor_len() > 0;
        if keys.any_pressed(CONTROL) {
            if keys.just_pressed(KeyCode::C) {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(inputbox.selected());
                }
            } else if keys.just_pressed(KeyCode::V) {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    if let Ok(text) = clipboard.get_text() {
                        if inputbox.overflow == InputOverflow::Deny {
                            let string = inputbox.try_push_str(&text);
                            let font = match fonts.get(font_handle) {
                                Some(font) => font.font.as_scaled(em),
                                None => continue,
                            };
                            let len = measure_string(&font, &string);
                            if len > dimension {
                                continue;
                            }
                        } else if let InputOverflow::Characters(c) = inputbox.overflow {
                            let string = inputbox.try_push_str(&text);
                            if string.chars().count() > c {
                                continue;
                            }
                        }
                        inputbox.push_str(&text);
                        changed = true;
                    }
                }
            } else if keys.just_pressed(KeyCode::X) {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(inputbox.swap_selected(""));
                } else {
                    inputbox.swap_selected("");
                }
                changed = true;
            } else if keys.just_pressed(KeyCode::A) {
                inputbox.select_all()
            }
        } else if keys.just_pressed(KeyCode::Left) {
            if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
                inputbox.cursor_select_left()
            } else {
                inputbox.cursor_left()
            }
        } else if keys.just_pressed(KeyCode::Right) {
            if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
                inputbox.cursor_select_right()
            } else {
                inputbox.cursor_right()
            }
        } else {
            for char in events.read() {
                match char.char {
                    '\t' => (),
                    '\r' | '\n' => {
                        submit.send(inputbox.get().to_owned())
                    }
                    '\x08' | '\x7f' => inputbox.backspace(),
                    _ => {
                        if inputbox.overflow == InputOverflow::Deny {
                            let string = inputbox.try_push(char.char);
                            let font = match fonts.get(font_handle) {
                                Some(font) => font.font.as_scaled(em),
                                None => continue,
                            };
                            let len = measure_string(&font, &string);
                            if len > dimension {
                                continue;
                            }
                        } else if let InputOverflow::Characters(c) = inputbox.overflow {
                            let string = inputbox.try_push(char.char);
                            if string.chars().count() > c {
                                continue;
                            }
                        }
                        inputbox.push(char.char)
                    }
                }
                changed = true;
            }
        }
        if changed {
            if is_area{
                temp_set_invisible(children);
            }
            change.send(inputbox.get().to_owned())
        }
    }
}

/// Copy em as text size.
pub(crate) fn sync_em_inputbox(mut query: Query<(&mut InputBox, &DimensionData)>) {
    query.iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().em != dimension.em {
            sp.em = dimension.em;
        }
    })
}

pub(crate) fn draw_input_box(
    query: Query<(&Children, &Handle<Font>, &InputBox), Or<(Changed<InputBox>, Changed<Handle<Font>>)>>,
    mut child: Query<&mut TextFragment, With<InputBoxText>>,
) {
    for (children, font, input_box) in query.iter() {
        for entity in children {
            let Ok(mut fragment) = child.get_mut(*entity) else {continue};
            TextFragment::set_text(&mut fragment, &input_box.text);
            TextFragment::set_font(&mut fragment, font);
            break
        }
    }
}
