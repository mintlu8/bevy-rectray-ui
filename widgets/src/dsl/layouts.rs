
use bevy::math::{Vec2, UVec2};
use bevy_aoui::{Size2, FlexDir, Container, Layout, Alignment};
pub use bevy_aoui::bundles::LinebreakBundle as Linebreak;

/// Construct a dummy entity for linebreak in a layout.
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

use crate::widget_extension;

use super::util::OneOrTwo;


widget_extension! {
    pub struct DynamicFrameBuilder {
        margin: OneOrTwo<Size2>,
        pub x: Option<bool>,
        pub y: Option<bool>,
    },
    this, commands,
    components: (
        Container {
            layout: Layout::Dynamic { 
                x: this.x.unwrap_or(true), 
                y: this.y.unwrap_or(true), 
            },
            margin: this.margin.0,
        }
    ),
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
    FixedTable,
    SizedTable,
}

widget_extension! {
    pub struct SpanContainerBuilder {
        pub r#type: Option<SpanContainerNames>,
        pub direction: Option<FlexDir>,
        pub stack: Option<FlexDir>,
        pub stretch: bool,
        pub margin: OneOrTwo<Size2>,
    },
    this, commands,
    components: (
        Container {
            layout: match this.r#type {
                Some(SpanContainerNames::Compact) => Layout::Compact { 
                    direction: this.direction.unwrap_or(FlexDir::LeftToRight) 
                },
                Some(SpanContainerNames::Span) => Layout::Span { 
                    direction: this.direction.unwrap_or(FlexDir::LeftToRight), 
                    stretch: this.stretch,
                },
                Some(SpanContainerNames::Paragraph) => Layout::Paragraph { 
                    direction: this.direction.unwrap_or(FlexDir::LeftToRight), 
                    stack: this.stack.unwrap_or(match this.direction {
                        Some(FlexDir::BottomToTop|FlexDir::TopToBottom) => FlexDir::LeftToRight,
                        _ => FlexDir::TopToBottom,
                    }), 
                    stretch: this.stretch
                },
                None => panic!("Please specify the container type."),
            },
            margin: this.margin.0
        }
    ),
}

widget_extension! {
    pub struct GridContainerBuilder {
        pub r#type: Option<GridContainerNames>,
        pub cell_count: Option<UVec2>,
        pub cell_size: Option<Vec2>,
        pub column_count: Option<usize>,
        pub columns: Vec<f32>,
        pub row: Option<FlexDir>,
        pub column: Option<FlexDir>,
        pub alignment: Option<Alignment>,
        pub stretch: bool,
        pub margin: OneOrTwo<Size2>,
    },
    this, commands,
    components: (
        {
            let row_dir = this.row.unwrap_or(FlexDir::LeftToRight);
            let column_dir = this.column.unwrap_or(match row_dir {
                FlexDir::LeftToRight | FlexDir::RightToLeft => FlexDir::TopToBottom,
                FlexDir::BottomToTop | FlexDir::TopToBottom => FlexDir::LeftToRight,
            });
            let alignment = this.alignment.unwrap_or(match row_dir {
                FlexDir::LeftToRight | FlexDir::RightToLeft => Alignment::Left,
                FlexDir::BottomToTop | FlexDir::TopToBottom => Alignment::Top,
            });
            Container {
                layout: match this.r#type {
                    Some(GridContainerNames::FixedGrid) => Layout::Grid {
                        cell: bevy_aoui::Cells::Counted(this.cell_count.expect("cell_count must be specified.")),
                        row_dir,
                        column_dir,
                        alignment,
                        stretch: this.stretch,
                    },
                    Some(GridContainerNames::SizedGrid) => Layout::Grid {
                        cell: bevy_aoui::Cells::Sized(this.cell_size.expect("cell_size must be specified.")),
                        row_dir,
                        column_dir,
                        alignment,
                        stretch: this.stretch,
                    },
                    Some(GridContainerNames::FlexTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Dynamic(this.column_count.unwrap_or(usize::MAX)),
                        row_dir,
                        column_dir,
                        stretch: this.stretch,
                    },
                    Some(GridContainerNames::FixedTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Porportions(this.columns),
                        row_dir,
                        column_dir,
                        stretch: this.stretch,
                    },
                    Some(GridContainerNames::SizedTable) => Layout::Table {
                        columns: bevy_aoui::Columns::Sized(this.columns),
                        row_dir,
                        column_dir,
                        stretch: this.stretch,
                    },
                    None => panic!("Please specify the container type."),
                },
                margin: this.margin.0,
            }
        }
    ),
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
            direction: ::bevy_aoui::FlexDir::LeftToRight,
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
            direction: ::bevy_aoui::FlexDir::TopToBottom,
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
            direction: ::bevy_aoui::FlexDir::LeftToRight,
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
            direction: ::bevy_aoui::FlexDir::TopToBottom,
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
macro_rules! fixed_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::FixedTable,
            $($tt)*
        })
    };
}

/// Construct a sized table layout.
#[macro_export]
macro_rules! sized_table {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::GridContainerBuilder] {
            r#type: $crate::dsl::GridContainerNames::SizedTable,
            $($tt)*
        })
    };
}