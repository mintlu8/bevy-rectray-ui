
/// Construct a oneshot event dynamically as a `&'static OnceLock<SystemId>`
/// 
/// This macro should only be used exactly once, as this requires static
/// resource to work.
#[macro_export]
macro_rules! oneshot {
    (($commands: expr $(, $ctx:expr)*) {fn $name: ident ($($arg:tt)*){$($tt:tt)*}}) => {
        {
            fn $name($($arg)*) {
                $($tt)*
            }
            static ID: ::std::sync::OnceLock<::bevy::ecs::system::SystemId> = ::std::sync::OnceLock::new();
            #[derive(Debug, Default)]
            struct InsertSystem;

            impl bevy::ecs::system::Command for InsertSystem {
                fn apply(self, world: &mut World) {
                    match ID.set(world. register_system($name)) {
                        Ok(_) => (),
                        Err(_) => eprintln!(
                            "OnceLock for oneshot system {} is already set.",
                            stringify!($name)
                        ),
                    }
                }
            }
            $commands.add(InsertSystem);
            &ID
        }
    };
}

/// Create a handler for a certain event.
///
/// This macro should only be used exactly once, as this requires static
/// resource to work.
#[macro_export]
macro_rules! handler {
    ($ctx:tt {mouse($($flag: ident)|*) => fn $name: ident ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::OneShot::new(
            $(crate::EventFlags::$flag)|*,
            $crate::oneshot!($ctx {fn $name ($($arg)*){$($tt)*}})
        )
    };
}