
/// Construct a one-shot system dynamically as a `Arc<OnceLock<SystemId>>`.
/// This can be used with [`Handler`](crate::events::Handler).
/// 
/// This macro can capture some context as if it's a `impl Fn` closure, and capture
/// some external generics, although syntax for generic bounds are limited.
/// 
/// # Example
/// 
/// Interpolate on hover, captures generic `M` as a marker.
/// 
/// ```
/// # /*
/// Handler::new(Hover, one_shot!(commands => fn on_hover<M: Component>(
///     mut offset: Query<&mut Interpolate<Offset>, With<M>>,
/// ) {
///     offset.single_mut().interpolate_to_or_reverse(Vec2::new(20.0, 0.0));
/// }
/// )),
/// # */
/// ```
#[macro_export]
macro_rules! one_shot {
    ($commands: expr => fn $($name: ident)? $(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}) => {
        {
            use ::std::sync::{Arc, OnceLock};
            use ::bevy::ecs::system::SystemId;
            #[derive(Debug, Default)]
            struct InsertSystem $(<$($generic$(: $ty)?),*>)? (
                Arc<OnceLock<SystemId>>
                $(,::std::marker::PhantomData <$($generic),*>)?
            );

            impl $(<$($generic $(: $ty)?),*>)? $crate::bevy::ecs::system::Command for InsertSystem $(<$($generic),*>)?{
                fn apply(self, world: &mut World) {
                    let _ = self.0.set(world.register_system(move |$($arg)*|{$($tt)*}));
                }
            }
            let arc = Arc::new(OnceLock::new());
            $commands.add(InsertSystem$(::<$($generic),*>)?(arc.clone() $(, ::std::marker::PhantomData::<$($generic),*>)?));
            arc
        }
    };
}

/// Create a handler for a certain event.
/// 
/// This macro cannot capture context and only generates a new `SystemId` on the first call.
/// 
/// Do not use this macro with multiple worlds.
#[macro_export]
macro_rules! handler {
    (($commands: expr $(, $($_tt:tt)*)?) {$flag: expr => fn $($name: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::Handler::new(
            $flag,
            $crate::one_shot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };

    ($commands: tt {$flag: expr => fn $($name: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::Handler::new(
            $flag,
            $crate::one_shot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };
}