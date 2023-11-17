
use bevy::{sprite::Anchor, prelude::{Vec2, Commands, Entity}, math::UVec2};
use bevy_aoui::{Size2, SetEM, Hitbox, FlexDir, bundles::AoUIBundle, Container, Layout, Alignment};
pub use bevy_aoui::bundles::LinebreakBundle as Linebreak;

#[macro_export]
macro_rules! linebreak {
    (($commands: expr $(, $tt:expr)*) $(,)?) => {
        $commands.spawn(::bevy_aoui::bundles::LinebreakBundle::default()).id()
    };
    (($commands: expr $(, $tt:expr)*) {}) => {
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
            let size: ::bevy_aoui::Size2;
            OneOrTwo(size) = $size.dinto();
            $commands.spawn(::bevy_aoui::bundles::LinebreakBundle::new(size)).id()
        }
    };
}

use crate::dsl::{core::{transform2d, dimension}, DslInto};

use super::{util::OneOrTwo, AoUIWidget};

pub use linebreak;

/// A Sized AoUI Component with no rendering.
#[derive(Debug, Default)]
pub struct DynFrame {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub x: Option<bool>,
    pub y: Option<bool>,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,
}


impl AoUIWidget for DynFrame {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                ..Default::default()
            },
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }
        base.id()
    }
}


#[derive(Debug, Clone, Copy)]
pub enum SpanContainerNames {
    Compact,
    Span,
    Paragraph,
}

#[derive(Debug, Default)]
pub struct SpanContainer {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub r#type: Option<SpanContainerNames>,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,
    pub direction: Option<FlexDir>,
    pub stack: Option<FlexDir>,
    pub stretch: bool,
    pub margin: OneOrTwo<Size2>,
}

impl AoUIWidget for SpanContainer {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),                
                vis: self.visible.dinto(),
                ..Default::default()
            },
            Container {
                layout: match self.r#type {
                    Some(SpanContainerNames::Compact) => Layout::Compact { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight) 
                    },
                    Some(SpanContainerNames::Span) => Layout::Span { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight), 
                        stretch: self.stretch,
                    },
                    Some(SpanContainerNames::Paragraph) => Layout::Paragraph { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight), 
                        stack: self.stack.unwrap_or(match self.direction {
                            Some(FlexDir::BottomToTop|FlexDir::TopToBottom) => FlexDir::LeftToRight,
                            _ => FlexDir::TopToBottom,
                        }), 
                        stretch: self.stretch
                    },
                    None => panic!("Please specify the container type."),
                },
                margin: self.margin.0,
            }
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }
        base.id()
    }
}

#[macro_export]
macro_rules! compact {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! hbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            direction: ::bevy_aoui::FlexDir::LeftToRight,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! vbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            direction: ::bevy_aoui::FlexDir::TopToBottom,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! span {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! hspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            direction: ::bevy_aoui::FlexDir::LeftToRight,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! vspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            direction: ::bevy_aoui::FlexDir::TopToBottom,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! paragraph {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::SpanContainer] {
            r#type: $crate::dsl::SpanContainerNames::Paragraph,
            $($tt)*
        })
    };
}

#[derive(Debug, Clone, Copy)]
pub enum GridContainerNames {
    FixedGrid,
    SizedGrid,
    FlexTable,
    FixedTable,
    SizedTable,
}
#[derive(Debug, Default)]
pub struct GridContainer {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub r#type: Option<GridContainerNames>,
    pub cell_size: Option<Vec2>,
    pub cell_count: Option<UVec2>,
    pub column_count: Option<usize>,
    pub columns: Vec<f32>,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    pub font_size: SetEM,
    pub hitbox: Option<Hitbox>,
    pub row: Option<FlexDir>,
    pub column: Option<FlexDir>,
    pub alignment: Option<Alignment>,
    pub stretch: bool,
    pub margin: OneOrTwo<Size2>,
}


impl AoUIWidget for GridContainer {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let row_dir = self.row.unwrap_or(FlexDir::LeftToRight);
        let column_dir = self.column.unwrap_or(match row_dir {
            FlexDir::LeftToRight | FlexDir::RightToLeft => FlexDir::TopToBottom,
            FlexDir::BottomToTop | FlexDir::TopToBottom => FlexDir::LeftToRight,
        });
        let alignment = self.alignment.unwrap_or(match row_dir {
            FlexDir::LeftToRight | FlexDir::RightToLeft => Alignment::Left,
            FlexDir::BottomToTop | FlexDir::TopToBottom => Alignment::Top,
        });
        let mut base = commands.spawn((
            AoUIBundle {
                transform: transform2d!(self),
                dimension: dimension!(self),
                vis: self.visible.dinto(),
                ..Default::default()
            },
            Container {
                layout: match self.r#type {
                    Some(GridContainerNames::FixedGrid) => Layout::Grid {
                        cell: bevy_aoui::Cells::Counted(self.cell_count.expect("cell_count must be specified.")),
                        row_dir,
                        column_dir,
                        alignment,
                        stretch: self.stretch,
                    },
                    Some(GridContainerNames::SizedGrid) => Layout::Grid {
                        cell: bevy_aoui::Cells::Sized(self.cell_size.expect("cell_size must be specified.")),
                        row_dir,
                        column_dir,
                        alignment,
                        stretch: self.stretch,
                    },
                    Some(GridContainerNames::FlexTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Dynamic(self.column_count.unwrap_or(usize::MAX)),
                        row_dir,
                        column_dir,
                        stretch: self.stretch,
                    },
                    Some(GridContainerNames::FixedTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Porportions(self.columns),
                        row_dir,
                        column_dir,
                        stretch: self.stretch,
                    },
                    Some(GridContainerNames::SizedTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Sized(self.columns),
                        row_dir,
                        column_dir,
                        stretch: self.stretch,
                    },
                    None => panic!("Please specify the container type."),
                },
                margin: self.margin.0,
            }
        ));
        if let Some(hitbox) = self.hitbox {
            base.insert(hitbox);
        }
        base.id()
    }
}


#[macro_export]
macro_rules! fixed_grid {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::GridContainer] {
            r#type: $crate::dsl::GridContainerNames::FixedGrid,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! sized_grid {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::GridContainer] {
            r#type: $crate::dsl::GridContainerNames::SizedGrid,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! flex_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::GridContainer] {
            r#type: $crate::dsl::GridContainerNames::FlexTable,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! fixed_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::GridContainer] {
            r#type: $crate::dsl::GridContainerNames::FixedTable,
            $($tt)*
        })
    };
}

#[macro_export]
macro_rules! sized_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::GridContainer] {
            r#type: $crate::dsl::GridContainerNames::SizedTable,
            $($tt)*
        })
    };
}