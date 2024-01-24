use super::{DslInto, DslFrom};


impl<T, U> DslInto<U> for T where U: DslFrom<T> {
    fn dinto(self) -> U {
        U::dfrom(self)
    }
}

impl<T> DslFrom<T> for T {
    fn dfrom(value: T) -> Self {
        value
    }
}

impl<T> DslFrom<T> for Option<T> {
    fn dfrom(value: T) -> Self {
        Some(value)
    }
}

impl<'t, T> DslFrom<&'t T> for T where T: Clone{
    fn dfrom(value: &'t T) -> Self {
        value.clone()
    }
}

impl<'t, T> DslFrom<&'t mut T> for T where T: Clone{
    fn dfrom(value: &'t mut T) -> Self {
        value.clone()
    }
}

impl DslFrom<&str> for Option<String> {
    fn dfrom(value: &str) -> Self {
        Some(value.to_owned())
    }
}

impl DslFrom<i32> for Option<f32> {
    fn dfrom(value: i32) -> Self {
        Some(value as f32)
    }
}

impl DslFrom<i32> for f32 {
    fn dfrom(value: i32) -> Self {
        value as f32
    }
}

impl DslFrom<usize> for f32 {
    fn dfrom(value: usize) -> Self {
        value as f32
    }
}

impl DslFrom<char> for String {
    fn dfrom(value: char) -> Self {
        value.to_string()
    }
}

impl DslFrom<&str> for String {
    fn dfrom(value: &str) -> Self {
        value.to_string()
    }
}

impl<T, const N: usize> DslFrom<[T; N]> for Vec<T> {
    fn dfrom(value: [T; N]) -> Self {
        value.into()
    }
}

impl<T> DslFrom<&[T]> for Vec<T> where T: Clone {
    fn dfrom(value: &[T]) -> Self {
        value.to_vec()
    }
}

impl<const N: usize> DslFrom<[i32; N]> for Vec<f32> {
    fn dfrom(value: [i32; N]) -> Self {
        value.into_iter().map(|x| x as f32).collect()
    }
}

impl DslFrom<&[i32]> for Vec<f32> {
    fn dfrom(value: &[i32]) -> Self {
        value.iter().map(|x| *x as f32).collect()
    }
}

impl<const N: usize> DslFrom<[i32; N]> for [f32; N] {
    fn dfrom(value: [i32; N]) -> Self {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = value[i] as f32
        }
        result
    }
}
