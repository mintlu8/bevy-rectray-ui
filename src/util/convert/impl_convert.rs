use bevy::ecs::entity::Entity;

use crate::{util::WidgetBuilder, dsl::prelude::RCommands};

use super::{DslConvert, DslInto, SealToken};


impl<T, U> DslConvert<U, ' '> for T where T: DslInto<U> {
    fn parse(self) -> U {
        self.dinto()
    }
    fn sealed(_: SealToken) {}
}

impl<F, T> DslConvert<WidgetBuilder<T>, 'W'> for F
        where F: (Fn(&mut RCommands, T) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> WidgetBuilder<T> {
        WidgetBuilder::new(self)
    }
    fn sealed(_: SealToken) {}
}

impl<F, T> DslConvert<Option<WidgetBuilder<T>>, 'W'> for F
        where F: (Fn(&mut RCommands, T) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> Option<WidgetBuilder<T>> {
        Some(WidgetBuilder::new(self))
    }
    fn sealed(_: SealToken) {}
}

impl<F> DslConvert<Option<WidgetBuilder<()>>, 'w'> for F
        where F: (Fn(&mut RCommands) -> Entity) + Send + Sync + 'static {
    fn parse(self) -> Option<WidgetBuilder<()>> {
        Some(WidgetBuilder::new(self))
    }
    fn sealed(_: SealToken) {}
}
