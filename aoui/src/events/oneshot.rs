use std::{sync::{OnceLock, Arc}, marker::PhantomData};

use bevy::ecs::system::{SystemId, IntoSystem, Command, Commands};

#[derive(Debug)]
pub struct OneShot(Arc<OnceLock<SystemId>>);


struct InsertCommand<T: IntoSystem<(), (), M> + Send + Sync + 'static, M: Send + Sync + 'static=()>{
    sys: T,
    id: Arc<OnceLock<SystemId>>,
    m: PhantomData<M>
}

impl<T: IntoSystem<(), (), M> + Send + Sync + 'static, M: Send + Sync + 'static> Command for InsertCommand<T, M> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let sys_id = world.register_system(self.sys);
        let _ = self.id.set(sys_id);
    }
}

impl OneShot {
    pub fn new<M: Send + Sync + 'static>(commands: &mut Commands, f: impl IntoSystem<(), (), M> + Send + Sync + 'static) -> Self {
        let id = Arc::new(OnceLock::new());
        commands.add(
            InsertCommand {
                sys: f,
                id: id.clone(),
                m: PhantomData
            }
        );
        Self(id)
    }

    pub fn get(&self) -> Option<SystemId> {
        self.0.get().copied()
    }
}