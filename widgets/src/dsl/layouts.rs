
use bevy::{sprite::Anchor, prelude::{Vec2, Commands, Entity}};
use bevy_aoui::{Size2, SetEM, Hitbox, FlexDir, bundles::AoUIBundle, Container, Layout};
pub use bevy_aoui::bundles::LinebreakBundle as Linebreak;

#[macro_export]
macro_rules! linebreak {
    (($commands: expr $(, $tt:expr)*) $(,)?) => {
        $commands.spawn(::bevy_aoui::bundles::LinebreakBundle::default()).id()
    };
    (($commands: expr $(, $tt:expr)*), $size: expr $(,)?) => {
        {
            use $crate::dsl::DslInto;
            let OneOrTwo(size) = $size.dinto();
            $commands.spawn(::bevy_aoui::bundles::LinebreakBundle::new(size)).id()
        }
    };
    (($commands: expr $(, $tt:expr)*) {$size: expr}) => {
        {
            use $crate::dsl::DslInto;
            let size: bevy_aoui::Size2;
            OneOrTwo(size) = $size.dinto();
            $commands.spawn(::bevy_aoui::bundles::LinebreakBundle::new(size)).id()
        }
    };
}

pub use linebreak;

use crate::dsl::core::{transform2d, dimension, common_plugins};

use super::{util::OneOrTwo, AoUIWidget, Frame};

#[derive(Debug, Default)]
pub struct Compact {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,
    pub direction: Option<FlexDir>,
    pub margin: OneOrTwo<Size2>,
}

impl AoUIWidget for Compact {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                ..Default::default()
            },
            Container {
                layout: Layout::Compact { 
                    direction: self.direction.expect("Expected `Direction`.") 
                },
                margin: self.margin.0,
            }
        ));
        common_plugins!(self, base);
        base.id()
    }
}