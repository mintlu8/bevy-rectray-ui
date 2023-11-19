
/// Construct a oneshot event dynamically as a `&'static OnceLock<SystemId>`
/// 
/// This macro cannot capture context and only generates a new `SystemId` on the first call.
/// 
/// Do not use this macro with multiple worlds.
#[macro_export]
macro_rules! oneshot {
    (($commands: expr $(, $ctx:expr)*) {fn $name: ident ($($arg:tt)*){$($tt:tt)*}}) => {
        {
            fn $name($($arg)*) {
                $($tt)*
            }
            static ID: ::std::sync::OnceLock<::bevy::ecs::system::SystemId> = ::std::sync::OnceLock::new();
            static WORLD: ::std::sync::OnceLock<::bevy::ecs::world::WorldId> = ::std::sync::OnceLock::new();
            #[derive(Debug, Default)]
            struct InsertSystem;

            impl ::bevy::ecs::system::Command for InsertSystem {
                fn apply(self, world: &mut World) {
                    assert_eq!(
                        WORLD.get_or_init(||world.id()), &world.id(), 
                        "Cannot reuse SystemId in another World."
                    );
                    let _ = ID.set(world.register_system($name));
                }
            }
            $commands.add(InsertSystem);
            &ID
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
    ($ctx:tt {$flag: expr => fn $name: ident ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::OneShot::new(
            $flag,
            $crate::oneshot!($ctx {fn $name ($($arg)*){$($tt)*}})
        )
    };
}