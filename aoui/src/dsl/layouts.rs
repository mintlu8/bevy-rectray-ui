use crate::{layout::*, build_frame};

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

use super::Widget;


widget_extension! {
    pub struct PaddingBuilder {}
}

impl Widget for PaddingBuilder {
    fn spawn_with(mut self, commands: &mut bevy::prelude::Commands, _: Option<&bevy::prelude::AssetServer>) -> (bevy::prelude::Entity, bevy::prelude::Entity) {
        self.layout = Some(Box::new(BoundsLayout::PADDING));
        let entity = build_frame!(commands, self).id();
        (entity, entity)
    }
}

/// Construct a fit layout, commonly used for padding.
#[macro_export]
macro_rules! padding {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::PaddingBuilder] {
            $($tt)*
        })
    };
}

/// Construct a horizontal left to right compact layout.
#[macro_export]
macro_rules! hbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::CompactLayout::HBOX,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom compact layout.
#[macro_export]
macro_rules! vbox {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::CompactLayout::VBOX,
            $($tt)*
        })
    };
}

/// Construct a horizotal left to right layout with fixed dimension.
#[macro_export]
macro_rules! hspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::SpanLayout::HSPAN,
            $($tt)*
        })
    };
}

/// Construct a vertical top to bottom layout with fixed dimension.
#[macro_export]
macro_rules! vspan {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::SpanLayout::VSPAN,
            $($tt)*
        })
    };
}

/// Construct a paragraph layout.
#[macro_export]
macro_rules! paragraph {
    {$commands: tt {$($tt:tt)*}} => {
        $crate::meta_dsl!($commands [$crate::dsl::builders::FrameBuilder] {
            layout: $crate::layout::ParagraphLayout::default(),
            $($tt)*
        })
    };
}
