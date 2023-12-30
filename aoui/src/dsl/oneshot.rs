
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
            use $crate::bevy::ecs::system::SystemId;
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

#[doc(hidden)]
#[macro_export]
macro_rules! complex_handler {
    ($commands: expr, $flag: ty => {$($tt: tt)*}) => {
        {
            let mut empty = $crate::events::Handlers::<$flag>::new_empty();
            $crate::complex_handler!(empty, $commands, {$($tt)*});
            empty
        }
    };
    ($name: ident, $commands: expr, {fn $($_: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*} $(,$($rest: tt)*)?}) => {
        {
            $name = $name.with($crate::events::Handler::OneShotSystem($crate::one_shot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})));
            $crate::complex_handler!($name, $commands, {$($($rest)*)?});
        }
    };
    ($name: ident, $commands: expr, {$key: literal = $value: expr $(,$($rest: tt)*)?}) => {
        {
            $name = $name.with($crate::events::Handler::GlobalKey($key.to_string(), $crate::signal_mapper!($value)));
            $crate::complex_handler!($name, $commands, {$($($rest)*)?});
        }
    };
    ($name: ident, $commands: expr, {$expr: expr $(,$($rest: tt)*)?}) => {
        {
            $name = $name.with($expr);
            $crate::complex_handler!($name, $commands, {$($($rest)*)?});
        }
    };
    ($name: ident, $commands: expr, {}) => {}
}
/// Construct a one-shot system dynamically for an event.
/// 
/// See also [`one_shot`].
#[macro_export]
macro_rules! handler {
    (($commands: expr $(, $($_tt:tt)*)?) {$flag: ty => fn $($name: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::Handlers::<$flag>::oneshot(
            $crate::one_shot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };

    ($commands: tt {$flag: ty => fn $($name: ident)?$(<$($generic: ident$(: $ty: ident)?),*>)? ($($arg:tt)*){$($tt:tt)*}})  => {
        $crate::events::Handlers::<$flag>::oneshot(
            $flag,
            $crate::one_shot!($commands => fn $(<$($generic$(: $ty)?),*>)? ($($arg)*){$($tt)*})
        )
    };

    (($commands: expr $(, $($_tt:tt)*)?) {$flag: ty => {$($tt:tt)*}})  => {
        $crate::complex_handler!($commands, $flag => {$($tt)*})
    };

    ($commands: tt {$flag: ty => {$($tt:tt)*}}) => {
        $crate::complex_handler!($commands, $flag => {$($tt)*})
    };

    (($commands: expr $(, $($_tt:tt)*)?) {$flag: ty => $signal: expr})  => {
        $crate::events::Handlers::<$flag>::signal(
            $flag,
            $signal
        )
    };

    ($commands: tt {$flag: ty => $signal: expr})  => {
        $crate::events::Handlers::<$flag>::signal(
            $flag,
            $signal
        )
    };
}