
use bevy::math::UVec2;
use crate::{Size2, layout::*, SizeUnit, build_frame};
pub use crate::bundles::LinebreakBundle as Linebreak;

/// Construct a dummy entity for linebreak in a layout.
#[macro_export]
macro_rules! linebreak {
    (($commands: expr $(, $tt:expr)*) $({})? $(,)?) => {
        $commands.spawn($crate::bundles::LinebreakBundle::default()).id()
    };
    (($commands: expr $(, $tt:expr)*), $size: expr $(,)?) => {
        {
            use $crate::dsl::DslInto;            
            let OneOrTwo(size) = DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    (($commands: expr $(, $tt:expr)*) {$size: expr}) => {
        {
            use $crate::dsl::DslInto;            
            let OneOrTwo(size) = DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    ($commands: tt $({})? $(,)?) => {
        $commands.spawn($crate::bundles::LinebreakBundle::default()).id()
    };
    ($commands: tt $size: expr $(,)?) => {
        {
            use $crate::dsl::DslInto;
            let OneOrTwo(size) = DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
    ($commands: tt {$size: expr}) => {
        {
            use $crate::dsl::DslInto;
            let OneOrTwo(size) = DslInto::<OneOrTwo<Size2>>::dinto($size);
            $commands.spawn($crate::bundles::LinebreakBundle::new(size)).id()
        }
    };
}

use crate::widget_extension;

use super::{util::OneOrTwo, Widget};


widget_extension! {
    pub struct PaddingBuilder {
        pub padding: OneOrTwo<Size2>,
        pub x: Option<bool>,
        pub y: Option<bool>,
    }
}

impl Widget for PaddingBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let entity = build_frame!(commands, self).insert(
            Container {
                layout: Box::new(FitLayout { 
                    x: self.x.unwrap_or(true), 
                    y: self.y.unwrap_or(true), 
                }),
                margin: Size2::ZERO,
                padding: self.padding.0,
            }
        ).id();
        (entity, entity)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpanContainerNames {
    Compact,
    Span,
    Paragraph,
}

#[derive(Debug, Clone, Copy)]
pub enum GridContainerNames {
    FixedGrid,
    SizedGrid,
    FlexTable,
    Table,
}

widget_extension! {
    pub struct SpanContainerBuilder {
        pub r#type: Option<SpanContainerNames>,
        pub direction: Option<FlexDir>,
        pub stack: Option<FlexDir>,
        pub stretch: bool,
        pub margin: OneOrTwo<Size2>,
        pub padding: OneOrTwo<Size2>,
    }
}

impl Widget for SpanContainerBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let entity = build_frame!(commands, self).insert(
            Container {
                layout: match self.r#type {
                    Some(SpanContainerNames::Compact) => Box::new(CompactLayout { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight) 
                    }),
                    Some(SpanContainerNames::Span) => Box::new(SpanLayout { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight), 
                        stretch: self.stretch,
                    }),
                    Some(SpanContainerNames::Paragraph) => Box::new(ParagraphLayout { 
                        direction: self.direction.unwrap_or(FlexDir::LeftToRight), 
                        stack: self.stack.unwrap_or(match self.direction {
                            Some(FlexDir::BottomToTop|FlexDir::TopToBottom) => FlexDir::LeftToRight,
                            _ => FlexDir::TopToBottom,
                        }), 
                        stretch: self.stretch
                    }),
                    None => panic!("Please specify the container type."),
                },
                margin: self.margin.0,
                padding: self.padding.0,
            }
        ).id();
        (entity, entity)
        
    }
}

widget_extension! {
    pub struct GridContainerBuilder {
        pub r#type: Option<GridContainerNames>,
        pub cell_count: Option<UVec2>,
        pub cell_size: Option<Size2>,
        pub column_count: Option<usize>,
        pub columns: Vec<(SizeUnit, f32)>,
        pub row: Option<FlexDir>,
        pub column: Option<FlexDir>,
        pub alignment: Option<Alignment>,
        pub stretch: bool,
        pub margin: OneOrTwo<Size2>,
        pub padding: OneOrTwo<Size2>,
    }
}

impl Widget for GridContainerBuilder {
    fn spawn_with(self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        let row_dir = self.row.unwrap_or(FlexDir::LeftToRight);
        let column_dir = self.column.unwrap_or(match row_dir {
            FlexDir::LeftToRight | FlexDir::RightToLeft => FlexDir::TopToBottom,
            FlexDir::BottomToTop | FlexDir::TopToBottom => FlexDir::LeftToRight,
        });
        let alignment = self.alignment.unwrap_or(match row_dir {
            FlexDir::LeftToRight | FlexDir::RightToLeft => Alignment::Left,
            FlexDir::BottomToTop | FlexDir::TopToBottom => Alignment::Top,
        });
        let entity = build_frame!(commands, self).insert(
            Container {
                layout: match self.r#type {
                    Some(GridContainerNames::FixedGrid) => Box::new(FixedGridLayout {
                        cells: self.cell_count.expect("cell_count must be specified."),
                        row_dir,
                        column_dir,
                        alignment,
                    }),
                    Some(GridContainerNames::SizedGrid) => Box::new(SizedGridLayout {
                        cell_size: self.cell_size.expect("cell_size must be specified."),
                        row_dir,
                        column_dir,
                        alignment,
                        stretch: self.stretch,
                    }),
                    Some(GridContainerNames::FlexTable) => Box::new(DynamicTableLayout {
                        columns: self.column_count.expect("column_count must be specified."),
                        row_dir,
                        column_dir,
                        stretch: self.stretch,
                    }),
                    Some(GridContainerNames::Table) => Box::new(TableLayout {
                        columns: self.columns,
                        row_dir,
                        column_dir,
                        stretch: self.stretch,
                    }),
                    None => panic!("Please specify the container type."),
                },
                margin: self.margin.0,
                padding: self.padding.0,
            }
        ).id();
        (entity, entity)
    }
}

/// Construct a compact layout.
#[macro_export]
macro_rules! padding {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::PaddingBuilder] {
            $($tt)*
        })
    };
}

/// Construct a compact layout.
#[macro_export]
macro_rules! compact {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            $($tt)*
        })
    };
}

/// Construct a horizotal left to right compact layout.
#[macro_export]
macro_rules! hbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            direction: $crate::layout::FlexDir::LeftToRight,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom compact layout.
#[macro_export]
macro_rules! vbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Compact,
            direction: $crate::layout::FlexDir::TopToBottom,
            $($tt)*
        })
    };
}

/// Construct a span layout.
#[macro_export]
macro_rules! span {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            $($tt)*
        })
    };
}

/// Construct a horizontal left to right span layout.
#[macro_export]
macro_rules! hspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            direction: $crate::layout::FlexDir::LeftToRight,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom span layout.
#[macro_export]
macro_rules! vspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Span,
            direction: $crate::layout::FlexDir::TopToBottom,
            $($tt)*
        })
    };
}

/// Construct a paragtaph layout.
#[macro_export]
macro_rules! paragraph {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::SpanContainerBuilder] {
            r#type: $crate::dsl::SpanContainerNames::Paragraph,
            $($tt)*
        })
    };
}

/// Construct a fixed grid layout.
#[macro_export]
macro_rules! fixed_grid {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::FixedGrid,
            $($tt)*
        })
    };
}

/// Construct a sized grid layout.
#[macro_export]
macro_rules! sized_grid {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::SizedGrid,
            $($tt)*
        })
    };
}

/// Construct a flex table layout.
#[macro_export]
macro_rules! flex_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::FlexTable,
            $($tt)*
        })
    };
}

/// Construct a fixed table layout.
#[macro_export]
macro_rules! table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::Table,
            $($tt)*
        })
    };
}
