use bevy::ecs::system::ResMut;
use bevy::hierarchy::Children;
use bevy::asset::{Handle, Assets};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::ecs::{query::Changed, event::EventReader, system::Commands};
use bevy::render::color::Color;
use bevy::window::{Window, PrimaryWindow, ReceivedCharacter};
use bevy::text::{Text, Font};
use bevy::prelude::{Component, Query, Entity, With, Parent, Visibility, Without, Res};
use crate::signals::KeyStorage;
use crate::{RotatedRect, Transform2D, Dimension, bundles::AouiTextBundle};
use crate::signals::{Receiver, types::SigInvoke};
use crate::events::{CursorState, CursorFocus, CursorClickOutside, EventFlags, CursorAction, ActiveDetection, TextChange, TextSubmit, Handlers};
use ab_glyph::Font as FontTrait;

#[derive(Debug, Default, Clone, Copy)]
enum LeftRight {
    Left, #[default] Right,
}


/// Color of text.
#[derive(Debug, Clone, Copy, Component)]
pub struct TextColor(pub Color);

impl TextColor {
    pub fn get(&self) -> Color {
        self.0
    }
    pub fn set(&mut self, color: Color) {
        self.0 = color
    }
}

/// Single line text input.
/// 
/// InputBox requires 3 children to function properly:
/// 
/// * `InputBoxText`: empty frame component for individual glyphs.
/// * `InputBoxCursorBar`: vertical bar of the cursor.
/// * `InputBoxCursorArea`: select area of the cursor, should be scalable with owned dimension.
/// 
/// `InputBoxCursorBar` and `InputBoxCursorArea` requires 
/// `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
/// 
/// Warning: This widget does not rebuild its glyphs every frame,
/// might not behave properly if tempered externally.
#[derive(Debug, Component, Default)]
pub struct InputBox {
    cursor_start: usize,
    cursor_len: usize,
    text: String,
    focus: bool,
    active: LeftRight,
    em: f32,
}

/// Empty frame component for individual glyphs.
#[derive(Debug, Component, Default)]
pub struct InputBoxText;

/// Vertical bar of the cursor.
/// 
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Component, Default)]
pub struct InputBoxCursorBar;

/// Select area of the cursor, should be scalable.
/// 
/// Requires `Center`, `TopCenter` or `BottomCenter` Anchor to function properly.
#[derive(Debug, Component, Default)]
pub struct InputBoxCursorArea;

impl InputBox {
    pub fn new(s: impl Into<String>) -> Self{
        Self {
            text: s.into(),
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.text.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }


    pub fn cursor_len(&self) -> usize {
        self.cursor_len
    }

    pub fn get(&self) -> &str {
        &self.text
    }

    pub fn set_cursor(&mut self, start: usize, end: usize) {
        self.cursor_start = start;
        self.cursor_len = end.saturating_sub(start);
    }

    pub fn has_focus(&self) -> bool {
        self.focus
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus
    }

    pub fn selected(&self) -> &str {
        use substring::Substring;
        self.text.substring(self.cursor_start, self.cursor_start + self.cursor_len)
    }

    pub fn select_all(&mut self) {
        self.cursor_start = 0;
        self.cursor_len = self.text.chars().count();
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_start = 0;
        self.cursor_len = 0;
        self.focus = false;
    }

    pub fn set(&mut self, s: impl Into<String>) {
        self.text = s.into();
        self.cursor_start = 0;
        self.cursor_len = 0;
        self.focus = false;
    }

    /// Simulate the behavior of typing a char.
    pub fn push(&mut self, c: char) {
        self.text = self.text.chars().take(self.cursor_start)
            .chain(std::iter::once(c))
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect();
        self.cursor_start += 1;
        self.cursor_len = 0;
    }

    /// Simulates the behavior of pasting.
    pub fn push_str(&mut self, c: &str) {
        let len = c.chars().count();
        self.text = self.text.chars().take(self.cursor_start)
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
            },
        }
    }

    /// Simulates the behavior of clicking `shift-left`.
    pub fn cursor_select_left(&mut self) {
        match (self.cursor_len, self.active) {
            (0, _) => {
                if self.cursor_start == 0 { return }
                self.active = LeftRight::Left;
                self.cursor_start -= 1;
                self.cursor_len += 1;
            },
            (_, LeftRight::Left) => {
                if self.cursor_start == 0 { return }
                self.cursor_start -= 1;
                self.cursor_len += 1;
            },
            (_, LeftRight::Right) => {
                self.cursor_len -= 1;
            },
        }
    }

    /// Simulate the behavior of clicking `shift-right`.
    pub fn cursor_select_right(&mut self) {
        match (self.cursor_len, self.active) {
            (0, _) => {
                if self.cursor_start + self.cursor_len >= self.len() { return }
                self.active = LeftRight::Right;
                self.cursor_len += 1;
            },
            (_, LeftRight::Left) => {
                self.cursor_start += 1;
                self.cursor_len -= 1;
            },
            (_, LeftRight::Right) => {
                if self.cursor_start + self.cursor_len >= self.len() { return }
                self.cursor_len += 1;
            },
        }
    }


    /// Simulate the behavior of clicking `backspace`.
    pub fn backspace(&mut self) {
        if self.cursor_len > 0 {
            self.swap_selected("");
        } else if self.cursor_start > 0{
            self.cursor_start -= 1;
            self.cursor_len = 1;
            self.swap_selected("");
        }
    }

    /// Simulate the behavior of clicking `delete`.
    pub fn delete(&mut self) {
        if self.cursor_len > 0 {
            self.swap_selected("");
        } else if self.cursor_start < self.len() - 1{
            self.cursor_len += 1;
            self.swap_selected("");
        } 
    }

    /// Swap the selected area with another string.
    pub fn swap_selected(&mut self, swapped: &str) -> String {
        let len = swapped.chars().count();
        let result = self.text.chars()
            .skip(self.cursor_start)
            .take(self.cursor_len)
            .collect();
        self.text = self.text.chars().take(self.cursor_start)
            .chain(swapped.chars())
            .chain(self.text.chars().skip(self.cursor_start + self.cursor_len))
            .collect();
        self.cursor_len = len;
        result
    }
}

pub fn text_on_mouse_down(
    state: Res<CursorState>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut InputBox, &RotatedRect), With<CursorFocus>>,
    text: Query<(&Parent, &Children), With<InputBoxText>>,
    glyphs: Query<&Transform2D>,
) {
    let scale = window.get_single().map(|x| x.scale_factor() as f32).unwrap_or(1.0);
    for (entity, mut input_box, rect) in query.iter_mut() {
        let Some((_, children)) = text.into_iter().find(|(p, ..)| p.get() == entity) else {continue;};

        let cursor = rect.local_space(state.cursor_position()) * scale / 2.0; 
        let down = rect.local_space(state.down_position()) * scale / 2.0;

        let start = glyphs.iter_many(children)
            .position(|x| x.offset.raw().x > cursor.x)
            .unwrap_or(children.len());
        let end = glyphs.iter_many(children)
            .position(|x| x.offset.raw().x > down.x)
            .unwrap_or(children.len());

        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };
        input_box.set_cursor(start, end);
        input_box.set_focus(true);
    }
}

pub fn text_on_mouse_double_click(
    mut query: Query<(&mut InputBox, &CursorAction)>
) {
    for (mut input_box, action) in query.iter_mut() {
        if action.is(EventFlags::DoubleClick) {
            input_box.select_all();
            input_box.set_focus(true);
        }
    }
}

pub fn update_inputbox_cursor(
    mut commands: Commands,
    fonts: Res<Assets<Font>>,
    query: Query<(Entity, &InputBox, &Dimension, &Handle<Font>, &TextColor, ActiveDetection), (Changed<InputBox>, Without<InputBoxCursorArea>, Without<Text>)>,
    mut text: Query<(Entity, &Parent, Option<&Children>), (With<InputBoxText>, Without<InputBoxCursorBar>, Without<InputBoxCursorArea>, Without<Text>, Without<InputBox>)>,
    mut bar: Query<(&Parent, &mut Transform2D, &mut Visibility), (With<InputBoxCursorBar>, Without<InputBoxText>, Without<InputBoxCursorArea>, Without<Text>, Without<InputBox>)>,
    mut area: Query<(&Parent, &mut Transform2D, &mut Dimension, &mut Visibility), (With<InputBoxCursorArea>, Without<InputBoxText>, Without<InputBoxCursorBar>, Without<Text>, Without<InputBox>)>,
    mut letters: Query<(Entity, &mut Transform2D, &mut Dimension, &mut Text, &mut Visibility), (Without<InputBoxText>, Without<InputBoxCursorBar>, Without<InputBoxCursorArea>)>
) {
    use ab_glyph::ScaleFont as FontTrait;
    use bevy::prelude::*;
    for (entity, input_box, dimension, font_handle, color, active) in query.iter() {
        if !active.is_active() { continue; }
        let font = match fonts.get(font_handle){
            Some(font) => font.font.as_scaled(dimension.em),
            None => continue,
        };
        let Some((glyph_container, _, children)) = text.iter_mut().find(|(_, p, ..)| p.get() == entity) else {continue;};
        
        let font_size = dimension.em;
        let mut added = Vec::new();
        let mut cursor = -dimension.size.x / 2.0;
        let mut entities = match children {
            Some(children) => children.iter(),
            None => [].iter(),
        };
        let (start_index, end_index) = (input_box.cursor_start, (input_box.cursor_start + input_box.cursor_len).saturating_sub(1));
        let (mut start, mut end) = (cursor, cursor);
        let mut max = (0, 0.0);
        let mut last = ' ';
        for (index, chara) in input_box.text.chars().enumerate() {
            let glyph = font.scaled_glyph(chara);
            cursor += font.kern(font.glyph_id(last), font.glyph_id(chara));            
            last = chara;
            let bounds = font.glyph_bounds(&glyph);
            let center = (bounds.min.x + bounds.max.x) / 2.0;
            if index == start_index {
                start = cursor + bounds.min.x;
            }
            if index == end_index {
                end = cursor + bounds.max.x;
            }
            max = (index, end);
            if let Some(entity) = entities.next(){
                if let Ok((_, mut transform, mut dimension, mut text, mut vis)) = letters.get_mut(*entity){
                    transform.offset.edit_raw(|v| v.x = cursor + center);
                    dimension.edit_raw(|v| v.x = bounds.width());
                    *vis = Visibility::Inherited;
                    match text.sections.first_mut() {
                        Some(first) => first.value = chara.to_string(),
                        None => text.sections.push(TextSection { 
                            value: chara.to_string(), 
                            style: TextStyle { 
                                font: font_handle.clone(), 
                                font_size, 
                                color: color.get(),
                            } 
                        }),
                    }
                } else {
                    // fixing broken state makes little sense here.
                    warn!("Glyph entity invalidated in textbox, aborting.");
                    continue;
                }
            } else {
                added.push(commands.spawn({
                    AouiTextBundle {
                        transform: Transform2D::UNIT.with_offset(Vec2::new(cursor + center, 0.0)),
                        dimension: Dimension::owned(crate::size2!([{bounds.width()} px, 1 em])),
                        text: Text::from_section(chara, TextStyle { 
                            font: font_handle.clone(), 
                            font_size, 
                            color: color.get(),
                        }),
                        ..Default::default()
                    }
                }).id())
            }
            cursor += font.h_advance(font.glyph_id(chara));
        }
        
        if start_index == max.0 + 1{
            start = max.1;
        }

        if end_index == max.0 + 1{
            end = max.1;
        }

        if input_box.cursor_start + input_box.cursor_len == 0 {
            end = start;
        }
        entities.for_each(|entity| if let Ok((.., mut vis)) = letters.get_mut(*entity){
            *vis = Visibility::Hidden;
        });
        commands.entity(glyph_container).push_children(&added);
        if !input_box.focus {
            for (.., mut vis) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            };
            for (.., mut vis) in area.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            };
            continue;
        }
        if input_box.cursor_len == 0 {
            for (_, mut transform, mut vis, ) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                *vis = Visibility::Inherited;
            };
            for (.., mut vis) in area.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            };
        } else {
            for (.., mut vis) in bar.iter_mut().filter(|(p, ..)| p.get() == entity) {
                *vis = Visibility::Hidden;
            };
            for (.., mut transform, mut dimension, mut vis) in area.iter_mut().filter(|(p, ..)| p.get() == entity) {
                transform.offset.edit_raw(|v| v.x = (start + end) / 2.0);
                dimension.edit_raw(|v| v.x = end - start);
                *vis = Visibility::Inherited;
            };

        }
    }
}
#[cfg(not(target_os="macos"))]
const CONTROL: [KeyCode; 2] = [KeyCode::ControlLeft, KeyCode::ControlRight];
#[cfg(target_os="macos")]
const CONTROL: [KeyCode; 2] = [KeyCode::SuperLeft, KeyCode::SuperLeft]; 

pub fn text_on_click_outside(
    mut query: Query<&mut InputBox, With<CursorClickOutside>>,
) {
    for mut input in query.iter_mut() {
        input.focus = false;
    }
}
pub fn inputbox_keyboard(
    mut commands: Commands,
    mut storage: ResMut<KeyStorage>,
    mut query: Query<(&mut InputBox, Option<&Handlers<TextChange>>, Option<&Handlers<TextSubmit>>, Option<&Receiver<SigInvoke>>, ActiveDetection)>,
    mut events: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut inputbox, change, submit, invoke, active) in query.iter_mut().filter(|(input, ..)| input.has_focus()) {
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
            if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]){
                inputbox.cursor_select_left()
            } else {
                inputbox.cursor_left()
            }
        } else if keys.just_pressed(KeyCode::Right) {
            if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]){
                inputbox.cursor_select_right()
            } else {
                inputbox.cursor_right()
            }
        } else {
            for char in events.read() {
                match char.char {
                    '\t' => (),
                    '\r'|'\n' => {
                        if let Some(submit) = submit {
                            submit.handle(&mut commands, &mut storage, inputbox.get().to_owned())
                        }
                    },
                    '\x08'|'\x7f' => inputbox.backspace(),
                    _ => inputbox.push(char.char)
                }
                changed = true;
            }
        }
        if let Some(invoke) = invoke {
            if invoke.poll().is_some() {
                if let Some(submit) = submit {
                    submit.handle(&mut commands, &mut storage, inputbox.get().to_owned())
                }
            }
        }
        if changed {
            if let Some(change) = change {
                change.handle(&mut commands, &mut storage, inputbox.get().to_owned())
            }
        }
    }
}


/// Copy em as text size.
pub fn sync_em_inputbox(mut query: Query<(&mut InputBox, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().em != dimension.em {
            sp.em = dimension.em;
        }
    })
}
