
/// Construct a one-shot system dynamically as a `&'static OnceLock<SystemId>`
/// 
/// This macro cannot capture context and only generates a new `SystemId` on the first call.
/// 
/// The function can capture external generics, which works through static type mapping.
/// 
/// Do not use this macro with multiple worlds.
#[macro_export]
macro_rules! oneshot {
    ($commands: expr => fn $($name: ident)? ($($arg:tt)*){$($tt:tt)*}) => {
        {
            static ID: ::std::sync::OnceLock<$crate::bevy::ecs::system::SystemId> = ::std::sync::OnceLock::new();
            static WORLD: ::std::sync::OnceLock<$crate::bevy::ecs::world::WorldId> = ::std::sync::OnceLock::new();
            #[derive(Debug, Default)]
            struct InsertSystem;

            impl $crate::bevy::ecs::system::Command for InsertSystem {
                fn apply(self, world: &mut World) {
                    assert_eq!(
                        WORLD.get_or_init(||world.id()), &world.id(), 
                        "Cannot reuse SystemId in another World."
                    );
                    let _ = ID.set(world.register_system(|$($arg)*|{$($tt)*}));
                }
            }
            $commands.add(InsertSystem);
            &ID
        }
    };
    ($commands: expr => fn $($name: ident)? <$($generic: ident$(: $ty: ident)?),*> ($($arg:tt)*){$($tt:tt)*}) => {
        {
            static ID: ::std::sync::OnceLock<$crate::StaticTypeMap<::std::sync::OnceLock<$crate::bevy::ecs::system::SystemId>>> = ::std::sync::OnceLock::new();
            static WORLD: ::std::sync::OnceLock<$crate::bevy::ecs::world::WorldId> = ::std::sync::OnceLock::new();
            #[derive(Debug, Default)]
            struct InsertSystem <$($generic$(: $ty)?),*> (::std::marker::PhantomData <$($generic),*>);

            impl<$($generic $(: $ty)?),*> $crate::bevy::ecs::system::Command for InsertSystem <$($generic),*>{
                fn apply(self, world: &mut World) {
                    assert_eq!(
                        WORLD.get_or_init(||world.id()), &world.id(), 
                        "Cannot reuse SystemId in another World."
                    );
                    let map = ID.get_or_init(|| $crate::StaticTypeMap::new());
                    let cell = map.call_once::<($($generic),*), _>(|| ::std::sync::OnceLock::new());
                    let _ = cell.set(world.register_system(|$($arg)*|{$($tt)*}));
                }
            }
            $commands.add(InsertSystem::<$($generic),*>(::std::marker::PhantomData));
            let map = ID.get_or_init(|| $crate::StaticTypeMap::new());
            map.call_once::<($($generic),*), _>(|| ::std::sync::OnceLock::new())
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
        $crate::events::OneShot::new(
            $flag,
            $crate::oneshot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };

    ($commands: tt {$flag: expr => fn $($name: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::OneShot::new(
            $flag,
            $crate::oneshot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };
}