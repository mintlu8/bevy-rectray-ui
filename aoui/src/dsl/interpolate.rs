


///
/// ```
/// interpolate! [
///     loop position 10 in ([-5, 0], [5, 0]); 
///     Color Linear 0.5 default red,
/// ]
/// ```
#[macro_export]
macro_rules! interpolate {
    ($($tt:tt)*) => {
        $crate::interpolate_impl!({} $($tt)*)
    };
}

#[macro_export]
macro_rules! easing {
    (Linear) => {$crate::anim::Easing::Linear};
    {$ident: ident} => {$crate::anim::Easing::Ease($crate::anim::EaseFunction::$ident)};
    [$($tt: tt)*] => {$crate::anim::Easing::Bezier([$($tt)*])};
}
#[doc(hidden)]
#[macro_export]
macro_rules! interpolate_impl {
    ({$($out: expr),*}) => {($($out),*)};
    ({$($out: expr),*} Color $ease:tt $time:tt default $value:expr $(;$($rest:tt)*)?) => {
        {   
            $($out: expr,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::new(
                $crate::easing!($ease), 
                $crate::colorv4!($value), 
                $time as f32
            )
        }
        $($($rest)*)?
    };
    ({$($out: expr),*} $name:ident $ease:tt $time:tt default $value:expr $(;$($rest:tt)*)?) => {
        {   
            use $crate::dsl::DslInto;
            $($out: expr,)*
            $crate::anim::Interpolate::<$name>::new(
                $crate::easing!($ease),
                $value.dinto(),
                $time as f32
            )
        }
        $($($rest)*)?
    };

    ({$($out: expr),*} Color $ease:tt $time: tt looping [$($range: tt)*] $(;$($rest:tt)*)?) => {
        {   
            $($out: expr,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::looping(
                $crate::easing!($ease), 
                $crate::gradient!($($range)*),
                $time as f32
            )
        }
        $($($rest)*)?
    };
    ({$($out: expr),*} $name:ident $ease:tt $time: tt looping $range: expr $(;$($rest:tt)*)?) => {
        {   
            $($out: expr,)*
            $crate::anim::Interpolate::<$name>::looping(
                $crate::easing!($ease),
                $range,
                $time as f32
            )
        }
        $($($rest)*)?
    };

    ({$($out: expr),*} Color $ease:tt $time: tt repeat [$($range: tt)*]  $(;$($rest:tt)*)?) => {
        {   
            $($out: expr,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::repeat(
                $crate::easing!($ease), 
                $crate::gradient!($($range)*),
                $time as f32
            )
        }
        $($($rest)*)?
    };
    ({$($out: expr),*} $name:ident $ease:tt $time: tt repeat $range: expr $(;$($rest:tt)*)?) => {
        {   
            $($out: expr,)*
            $crate::anim::Interpolate::<$name>::repeat(
                $crate::easing!($ease),
                $range,
                $time as f32
            )
        }
        $($($rest)*)?
    };
} 