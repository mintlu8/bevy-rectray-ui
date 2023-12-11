mod dto;
mod signals;
mod extension;
pub use dto::{DataTransfer, Object};
pub use extension::WorldExtension;
pub use signals::{Sender, Receiver, signal};


/// The `Submit` signal, i.e. checkbox press, textbox press enter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub enum Submit {}

/// The `Change` signal, i.e. radio button change, textbox change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub enum Change {}
