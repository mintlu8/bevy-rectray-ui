use bevy::{reflect::TypePath, ecs::entity::Entity};

use crate::{util::WidgetBuilder, dsl::prelude::AouiCommands};

use super::{DslConvert, DslInto};


impl<T, U> DslConvert<U, ' '> for T where T: DslInto<U> {
    fn parse(self) -> U {
        self.dinto()
    }
}

impl<F, T: TypePath> DslConvert<WidgetBuilder<T>, 'W'> for F
        where F: (Fn(&mut AouiCommands, T) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> WidgetBuilder<T> {
        WidgetBuilder::new(self)
    }
}

impl<F, T: TypePath> DslConvert<Option<WidgetBuilder<T>>, 'W'> for F
        where F: (Fn(&mut AouiCommands, T) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> Option<WidgetBuilder<T>> {
        Some(WidgetBuilder::new(self))
    }
}

impl<F> DslConvert<Option<WidgetBuilder<()>>, 'w'> for F
        where F: (Fn(&mut AouiCommands) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> Option<WidgetBuilder<()>> {
        Some(WidgetBuilder::new(self))
    }
}
