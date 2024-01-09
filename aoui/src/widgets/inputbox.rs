use std::mem;
use crate::anim::VisibilityToggle;
use crate::dimension::DimensionMut;
use crate::events::{
    ActiveDetection, CursorAction, CursorClickOutside, CursorFocus, CursorState, EvTextChange,
    EvTextSubmit, EventFlags, Handlers,
};
use crate::signals::{Invoke, KeyStorage, ReceiveInvoke};
use crate::{RotatedRect, Transform2D, DimensionData};
use ab_glyph::{Font as FontTrait, ScaleFont, point};
use bevy::asset::{Assets, Handle};
use bevy::ecs::query::Has;
use bevy::ecs::system::ResMut;
use bevy::ecs::{event::EventReader, query::Changed, system::Commands};
use bevy::hierarchy::Children;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::{Component, Entity, Parent, Query, Res, Visibility, With, Without};
use bevy::reflect::Reflect;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::Image;
use bevy::text::{Font, Text};
use bevy::window::ReceivedCharacter;
use super::util::DisplayIf;

#[derive(Debug, Default, Clone, Copy, Reflect)]
enum LeftRight {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Default, Clone, Copy, Component, Reflect)]
pub struct InputBoxFocus;

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
#[derive(Debug, Component, Default, Reflect)]
pub struct InputBox {
    overflow: InputOverflow,
    cursor_start: usize,
    cursor_len: usize,
    text: String,
    focus: bool,
    active: LeftRight,
    em: f32,
}


impl ReceiveInvoke for InputBox {
    type Type = ();
}

/// Marker component for a empty frame containing individual glyphs.
#[derive(Debug, Component, Default)]
pub struct InputBoxText;

/// Marker component for a vertical bar of the cursor.
///
/// This component can be any sprite.
///
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Component, Default)]
pub struct InputBoxCursorBar;

/// Marker component for the area of the cursor.
///
/// This component sets the `dimension` when area is changed, so anything that
/// updates alongside dimension can be used here.
///
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Component, Default)]
pub struct InputBoxCursorArea;

impl InputBox {
    pub fn new(s: impl Into<String>, overflow: InputOverflow) -> Self {
        Self {
            text: s.into(),
            overflow,
            ..Default::default()
        }
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

pub fn text_on_mouse_down(
    state: Res<CursorState>,
    fonts: Res<Assets<Font>>,
    mut query: Query<(&DimensionData, &CursorFocus, &mut InputBox, &Handle<Font>, &RotatedRect)>,
) {
    for (dim, focus, mut input_box, font, rect) in query.iter_mut() {
        if !focus.intersects(EventFlags::LeftDrag) {
            continue;
        };
        let Some(font) = fonts.get(font) else {continue};
        // We only need to care about scale factor in rendering
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

pub fn text_on_mouse_double_click(mut query: Query<(&mut InputBox, &CursorAction)>) {
    for (mut input_box, action) in query.iter_mut() {
        if action.is(EventFlags::DoubleClick) {
            input_box.select_all();
            input_box.set_focus(true);
        }
    }
}

pub fn text_propagate_focus(
    mut commands: Commands,
    query: Query<(Entity, &InputBox)>,
    entities: Query<&Children>,
) {
    let mut queue = Vec::new();
    for (entity, input_box) in query.iter() {
        if input_box.has_focus() {
            commands.entity(entity).insert(InputBoxFocus);
            queue.push(entity);
        }
    }
    while !queue.is_empty() {
        for children in entities.iter_many(mem::take(&mut queue)) {
            queue.extend(children.iter().map(|entity| {
                commands.entity(*entity).insert(InputBoxFocus);
                *entity
            }))
        }
    }
}

pub fn inputbox_conditional_visibility(
    mut query: Query<(Has<InputBoxFocus>, VisibilityToggle), With<DisplayIf<InputBoxFocus>>>,
) {
    query.iter_mut().for_each(|(has_focus, mut vis)| {
        if has_focus {
            vis.set_visible(true)
        } else {
            vis.set_visible(false)
        }
    })
}

pub fn measure_string<F: ab_glyph::Font>(
    font: &impl ab_glyph::ScaleFont<F>,
    string: &str,
) -> f32 {
    let mut cursor = 0.0;
    let mut last = '\0';
    for c in string.chars() {
        cursor += font.kern(font.glyph_id(last), font.glyph_id(c));
        cursor += font.h_advance(font.glyph_id(c));
        last = c
    }
    cursor
}

pub fn update_inputbox_cursor(
    fonts: Res<Assets<Font>>,
    query: Query<(Entity, &InputBox, &DimensionData,
            &Handle<Font>, ActiveDetection),
        (Changed<InputBox>, Without<InputBoxCursorArea>,  Without<Text>)>,
    mut bar: Query<(&Parent, &mut Transform2D, &mut Visibility),
        (With<InputBoxCursorBar>, Without<InputBoxText>, Without<InputBoxCursorArea>, 
            Without<Text>, Without<InputBox>,)>,
    mut area: Query<(&Parent, &mut Transform2D, DimensionMut, &mut Visibility),
        (With<InputBoxCursorArea>, Without<InputBoxText>, Without<InputBoxCursorBar>,
            Without<Text>, Without<InputBox>,)>
) {
    use bevy::prelude::*;
    for (entity, input_box, dimension, font_handle, active) in query.iter() {
        if !active.is_active() {
            continue;
        }
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
        if !input_box.focus {
            for (.., mut vis) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            }
            for (.., mut vis) in area.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            }
            continue;
        }
        if input_box.cursor_len == 0 {
            for (_, mut transform, mut vis) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                *vis = Visibility::Inherited;
            }
            for (.., mut vis) in area.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            }
        } else {
            for (.., mut vis) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            }
            for (.., mut transform, mut dimension, mut vis) in
                area.iter_mut().filter(|(p, ..)| p.get() == entity)
            {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                dimension.edit_raw(|v| v.x = end - start);
                *vis = Visibility::Inherited;
            }
        }
    }
}
#[cfg(not(target_os = "macos"))]
const CONTROL: [KeyCode; 2] = [KeyCode::ControlLeft, KeyCode::ControlRight];
#[cfg(target_os = "macos")]
const CONTROL: [KeyCode; 2] = [KeyCode::SuperLeft, KeyCode::SuperLeft];

pub fn text_on_click_outside(mut query: Query<&mut InputBox, With<CursorClickOutside>>) {
    for mut input in query.iter_mut() {
        input.focus = false;
    }
}
pub fn inputbox_keyboard(
    mut commands: Commands,
    fonts: Res<Assets<Font>>,
    storage: Res<KeyStorage>,
    mut query: Query<(
        Entity,
        &mut InputBox,
        &DimensionData,
        &Handle<Font>,
        Option<&Handlers<EvTextChange>>,
        Option<&Handlers<EvTextSubmit>>,
        Option<&Invoke<InputBox>>,
        ActiveDetection,
    )>,
    mut events: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
) {
    for (entity, mut inputbox, dimension, font_handle, change, submit, invoke, active) in
        query.iter_mut().filter(|(_, input, ..)| input.has_focus())
    {
        let mut commands = commands.entity(entity);
        if !active.is_active() {
            inputbox.focus = false;
            continue;
        }
        let mut changed = false;
        if keys.any_pressed(CONTROL) {
            if keys.just_pressed(KeyCode::C) {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(inputbox.get());
                    changed = true;
                }
            } else if keys.just_pressed(KeyCode::V) {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    if let Ok(text) = clipboard.get_text() {
                        if inputbox.overflow == InputOverflow::Deny {
                            let string = inputbox.try_push_str(&text);
                            let font = match fonts.get(font_handle) {
                                Some(font) => font.font.as_scaled(dimension.em),
                                None => continue,
                            };
                            let len = measure_string(&font, &string);
                            if len > dimension.size.x {
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
                        if let Some(submit) = submit {
                            submit.handle(&mut commands, &storage, inputbox.get().to_owned())
                        }
                    }
                    '\x08' | '\x7f' => inputbox.backspace(),
                    _ => {
                        if inputbox.overflow == InputOverflow::Deny {
                            let string = inputbox.try_push(char.char);
                            let font = match fonts.get(font_handle) {
                                Some(font) => font.font.as_scaled(dimension.em),
                                None => continue,
                            };
                            let len = measure_string(&font, &string);
                            if len > dimension.size.x {
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
        if let Some(invoke) = invoke {
            if invoke.poll().is_some() {
                if let Some(submit) = submit {
                    submit.handle(&mut commands, &storage, inputbox.get().to_owned())
                }
            }
        }
        if changed {
            if let Some(change) = change {
                change.handle(&mut commands, &storage, inputbox.get().to_owned())
            }
        }
    }
}

/// Copy em as text size.
pub fn sync_em_inputbox(mut query: Query<(&mut InputBox, &DimensionData)>) {
    query.iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().em != dimension.em {
            sp.em = dimension.em;
        }
    })
}

pub fn draw_input_box(
    mut images: ResMut<Assets<Image>>,
    fonts: Res<Assets<Font>>,
    query: Query<(&Handle<Image>, &Handle<Font>, &DimensionData, &InputBox), Changed<InputBox>> 
) {
    for (image, font, dim, input_box) in query.iter() {
        let Some(font) = fonts.get(font) else {continue};
        let font = font.font.as_scaled(dim.em * 2.0);
        let Some(image) = images.get_mut(image) else {continue};
        let dimension = measure_string(&font, &input_box.text);
        let height = font.height().ceil();
        let width = (dimension.ceil() as usize).max(1);
        let height = (height.ceil() as usize).max(1);
        let mut buffer = vec![0u8; width * height * 4];

        let mut cursor = 0.0;
        let mut last = '\0';
        for c in input_box.text.chars() {
            let mut glyph = font.scaled_glyph(c);
            glyph.position = point(cursor, 0.0 + font.ascent());
            cursor += font.kern(font.glyph_id(last), font.glyph_id(c));
            cursor += font.h_advance(font.glyph_id(c));
            last = c;
            if let Some(glyph) = font.outline_glyph(glyph) {
                let bounds = glyph.px_bounds();
                glyph.draw(|x, y, v| {
                    let x = x as usize + bounds.min.x as usize;
                    let y = y as usize + bounds.min.y as usize;
                    if x < width && y < height {
                        buffer[(x + y * width) * 4] = 255;
                        buffer[(x + y * width) * 4 + 1] = 255;
                        buffer[(x + y * width) * 4 + 2] =  255;
                        buffer[(x + y * width) * 4 + 3] += (v * 255.0) as u8;
                    }
                })
            }
        }

        *image = Image::new(Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        }, TextureDimension::D2, buffer, TextureFormat::Rgba8Unorm)
    }
}