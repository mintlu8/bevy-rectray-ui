mod dto;
mod signals;
mod extension;
pub use dto::{DataTransfer, Object};
pub use extension::WorldExtension;
pub use signals::{Sender, Receiver, signal, SignalMarker};


macro_rules! signals {
    ($($(#[$($attr:tt)*])* $name: ident),* $(,)?) => {
        $(
            $(#[$($attr)*])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
            pub enum $name {}
            impl SignalMarker for $name {}
        )*
    };
}

signals!(
    /// The canonical output of the widget, including `Button` press,
    /// `enter` press on `InputBox`, etc. 
    /// 
    /// When receiving a submit signal, widgets like `InputBox` will emit
    /// a `Submit` signal of its own. 
    /// Thus enabling the `Button -> Text -> Output` chain.
    SigSubmit, 
    /// Send a signal whenever the output of a widget would be changed.
    SigChange, 
    /// Sent if being dragged, `Draggable` sprite will be dragged if receiving this signal.
    /// 
    /// This is useful for creating a draggable banner for a sprite.
    SigDrag,
    /// Sent if being scrolled on, `Scroll` sprite will be scrolled if receiving this signal.
    SigScroll,

    /// Modifies the recipient's text.
    SigText,
    /// Modifies the recipient's raw offset.
    SigOffset,
    /// Modifies the recipient's raw dimension.
    SigDimension,
    /// Modifies the recipient's rotation.
    SigRotation,
    /// Modifies the recipient's scale.
    SigScale,

    /// Modifies the recipient's raw offset x.
    SigOffsetX,
    /// Modifies the recipient's raw offset y.
    SigOffsetY,
    /// Modifies the recipient's raw scale x.
    SigScaleX,
    /// Modifies the recipient's raw scale y.
    SigScaleY,
    /// Modifies the recipient's raw dimension X.
    SigDimensionX,
    /// Modifies the recipient's raw dimension y.
    SigDimensionY,

    /// Modifies the recipient's color.
    SigColor,
    /// Modifies the recipient's opacity.
    SigOpacity,

    /// Modifies the recipient layout's margin.
    SigMargin,
);

