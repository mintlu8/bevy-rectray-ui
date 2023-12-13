
use std::marker::PhantomData;

use super::{Sender, Receiver, mpmc::Signal};

pub trait SignalCreate {
    fn new() -> Self;
}

macro_rules! signal_create {
    ($sender: ident, $first: ident) => {
        impl SignalCreate for ($sender, $first) {
            fn new() -> Self {
                let signal = Signal::new();
                (
                    $sender{
                        signal: signal.clone(), 
                        map: None,
                        p: PhantomData
                    },
                    $first{
                        signal: signal, 
                        map: None,
                        p: PhantomData
                    }, 
                )
            }
        }
    };
    ($sender: ident, $first: ident, $($receivers: ident),*) => {
        impl
            SignalCreate for ($sender, $($receivers),* , $first) {
            fn new() -> Self {
                let signal = Signal::new();
                (
                    $sender{
                        signal: signal.clone(), 
                        map: None,
                        p: PhantomData
                    }, 
                    $($receivers{
                        signal: signal.clone(), 
                        map: None,
                        p: PhantomData
                    },)*
                    $first{
                        signal: signal, 
                        map: None,
                        p: PhantomData
                    },
                )
            }
        }

        signal_create!($sender, $($receivers),*);
    };
}

signal_create!(Sender, 
    Receiver, Receiver, Receiver, Receiver,
    Receiver, Receiver, Receiver, Receiver,
    Receiver, Receiver, Receiver, Receiver
);   


/// Create a spmc signal that can be polled. 
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
pub fn signal<S: SignalCreate>() -> S {
    S::new()
}
