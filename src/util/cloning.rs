
/// Allow a struct to create many clones of itself as either
/// itself `T`, an array `[T; N]` or a tuple `(T, T, T, ...)`.
pub trait CloneSplit<T: Clone> {
    fn clone_split(item: T) -> Self;
}

impl<T: Clone> CloneSplit<T> for T {
    fn clone_split(item: T) -> Self {
        item
    }
}


impl<T: Clone, const N: usize> CloneSplit<T> for [T; N] {
    fn clone_split(item: T) -> Self {
        std::array::from_fn(|_| item.clone())
    }
}

macro_rules! impl_clone_split {
    () => {};
    ($first: ident $(,$rest: ident)*) => {
        impl<$first: Clone> CloneSplit<$first> for ($first, $($rest),*) {
            fn clone_split(item: T) -> Self {
                (
                    $({
                        let v: $rest = item.clone();
                        v
                    },)*
                    item,
                )
            }
        }
        impl_clone_split!($($rest),*);
    };
}

impl_clone_split!(
    T,T,T,T,T,
    T,T,T,T,T,
    T,T,T,T,T
);
