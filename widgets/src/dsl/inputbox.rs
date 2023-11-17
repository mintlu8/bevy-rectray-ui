use bevy::{sprite::Anchor, math::Vec2, asset::Handle, text::{Font, BreakLineOn}, render::color::Color, ecs::{system::Commands, entity::Entity}, hierarchy::BuildChildren};
use bevy_aoui::{Size2, SetEM, Hitbox, bundles::AoUIBundle, Dimension};

use crate::{widgets::{inputbox::{InputBox, InputBoxCursorBar, InputBoxText, InputBoxCursorArea}, TextColor}, events::EventFlags};

use super::{util::OneOrTwo, AoUIWidget, core::{transform2d, dimension}, DslInto};


/// A text box.
#[derive(Debug, Default)]
pub struct InputBoxDsl {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,

    pub text: String,
    pub font: Handle<Font>,
    /// Note if not specified this is `UNBOUNDED`.
    pub color: Option<Color>,

    pub cursor_bar: Option<Entity>,
    pub cursor_area: Option<Entity>,
}

impl AoUIWidget for InputBoxDsl {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                ..Default::default()
            },
            InputBox::new(&self.text),
            TextColor(self.color.expect("color is required.")),
            EventFlags::DOUBLE_CLICK|EventFlags::DRAG|EventFlags::CLICK_OUTSIDE,
            self.font,
            self.hitbox.unwrap_or_default()
        ));
        let entity = base.id();
        let text = commands.spawn((
            AoUIBundle {
                dimension: Dimension::INHERIT,
                ..Default::default()
            },
            InputBoxText,
        )).id();
        let cursor_bar = commands.entity(self.cursor_bar.expect("cursor_bar is required."))
            .insert(InputBoxCursorBar)
            .id();
        let cursor_area = commands.entity(self.cursor_area.expect("cursor_bar is required."))
            .insert(InputBoxCursorArea)
            .id();
        commands.entity(entity).push_children(&[
            text, 
            cursor_bar,
            cursor_area,
        ]);
        entity
    }
}


/// Construct a textbox.
#[macro_export]
macro_rules! inputbox {
    {$commands: tt {$($tt:tt)*}} => 
        {$crate::meta_dsl!($commands [$crate::dsl::InputBoxDsl] {$($tt)*})};
}