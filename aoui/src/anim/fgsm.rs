use std::marker::PhantomData;

use bevy::{app::{FixedUpdate, Plugin}, ecs::{component::Component, query::{QueryData, WorldQuery}, system::Query}};

/// Fine-grained state machine.
pub trait Fgsm: Sized {
    type State: QueryData;

    fn from_state(state: &<Self::State as WorldQuery>::Item<'_>) -> Self;
}

/// A single component fine grained state machine.
pub trait ComponentFgsm: Component + Sized + Clone {}

impl<T> Fgsm for T where T: ComponentFgsm {
    type State = &'static Self;

    fn from_state(state: &<Self::State as WorldQuery>::Item<'_>) -> Self {
        (*state).clone()
    }
}

pub trait FgsmPairing: Component + Sized {
    type State: Fgsm;
    type Target: QueryData;
    
    fn system(mut query: Query<(<Self::State as Fgsm>::State, &Self, Self::Target)>) {
        for (state, this, mut target) in query.iter_mut() {
            let item = Self::State::from_state(&state);
            this.write(&mut target, item)
        }
    }

    fn write(&self, target: &mut <Self::Target as WorldQuery>::Item<'_>, state: Self::State);

    /// A plugin that registers a fgsm pairing's associated system.
    fn plugin() -> FgsmPlugin<Self> {
        FgsmPlugin(PhantomData)
    }
}

/// A plugin that registers a fgsm pairing's associated system.
pub struct FgsmPlugin<T: FgsmPairing>(PhantomData<T>);

impl<T: FgsmPairing> Plugin for FgsmPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, T::system);
    }
}

/// Construct a fine grained state machine from a enum that writes to an `Attr`.
#[macro_export]
macro_rules! fgsm_interpolation {
    ($(#[$($attr: tt)*])* $vis: vis struct $struct_name: ident: $fgsm: ty as $ty: ty => $comp: ty {
        $($field: ident: $branch: ident),* $(,)?
    } ) => {
        $(#[$($attr)*])*
        #[derive($crate::bevy::ecs::component::Component)]
        $vis struct $struct_name {
            $(pub $field: $ty),*
        }

        impl $crate::anim::FgsmPairing for $struct_name {
            type State = $fgsm;
            type Target = &'static mut $crate::anim::Interpolate<$comp>;

            fn write(&self, target: &mut $crate::bevy::ecs::change_detection::Mut<'_, $crate::anim::Interpolate<$comp>>, state: Self::State) {
                match state {
                    $(<$fgsm>::$branch => target.interpolate_to(self.$field),)*
                }
            }
        }
    };

    ($(#[$($attr: tt)*])* impl $struct_name: ty: $fgsm: ty as $ty: ty => $comp: ty {
        $($field: ident: $branch: ident),* $(,)?
    } ) => {
        impl $crate::anim::FgsmPairing for $struct_name {
            type State = $fgsm;
            type Target = &'static mut $crate::anim::Interpolate<$comp>;

            fn write(&self, target: &mut $crate::bevy::ecs::change_detection::Mut<'_, $crate::anim::Interpolate<$comp>>, state: Self::State) {
                match state {
                    $(<$fgsm>::$branch => target.interpolate_to(self.$field),)*
                }
            }
        }
    };
}