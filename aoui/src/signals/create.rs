
use super::{DataTransfer, Signal, mpmc::SignalBuilder};

pub trait SignalCreate<T> {
    fn new() -> Self;
}

macro_rules! signal_create {
    ($first: ident) => {
        impl<T: DataTransfer> SignalCreate<T> for ($first<T>,) {
            fn new() -> Self {
                ($first::new(Signal::new()), )
            }
        }
    };
    ($first: ident, $($receivers: ident),*) => {
        impl<T: DataTransfer> SignalCreate<T> for ($($receivers<T>),* , $first<T>) {
            fn new() -> Self {
                let signal = Signal::new();
                (
                    $($receivers::new(signal.clone()),)*
                    $first::new(signal),
                )
            }
        }

        signal_create!($($receivers),*);
    };
}

signal_create!(SignalBuilder, 
    SignalBuilder, SignalBuilder, SignalBuilder, SignalBuilder,
    SignalBuilder, SignalBuilder, SignalBuilder, SignalBuilder,
    SignalBuilder, SignalBuilder, SignalBuilder, SignalBuilder
);

impl<T: DataTransfer, const N: usize> SignalCreate<T> for [SignalBuilder<T>; N] {
    fn new() -> Self {
        let signal = Signal::new();
        core::array::from_fn(|_|SignalBuilder::new(signal.clone()))
    }
}


/// Create a mpmc signal that can be polled. 
/// 
/// The result can be inferred as either a tuple or an array of `SignalBuilders`.
/// 
/// # Writing
/// 
/// Signals are dynamic and type erased.
/// All types meeting their requirement can be sent.
/// They are usually written in `PreUpdate` and cleaned up in `Last`
/// 
/// # Reading
/// 
/// `poll()` returns `Some` only if type matches 
/// and treats type mismatch as if no value exists.
/// 
/// `poll_any()` returns `true` as long as something exists.
/// 
/// # Usage
///  
/// ```
/// # /*
/// let (sender, recv_a, recv_b, ...) = signal();
/// # */
/// ```
/// 
/// To have multiple senders or receiver on the same entity,
/// mark them.
/// 
/// ```
/// # /*
/// let sender = sender.mark::<ButtonClick>()
/// # */
/// ```
/// 
/// To map the value of a signal, supply a mapping function.
/// 
/// 
/// ```
/// # /*
/// sender.map(|x: f32| format!("{:.2}", f))
/// # */
/// ```
/// 
/// If registered, this signal is cleared at the end of the frame.
/// 
/// ```
/// # /*
/// app.register_aoui_signal::<ButtonClick>()
/// # */
/// ```
pub fn signal<T, S: SignalCreate<T>>() -> S {
    S::new()
}
