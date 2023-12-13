
#[macro_export]
macro_rules! signals {
    ($($(#[$($attr:tt)*])* $name: ident),* $(,)?) => {
        $(
            $(#[$($attr)*])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
            pub enum $name {}
            impl $crate::signals::SignalMarker for $name {}
        )*
    };
}

// Note `SigSubmit`, `SigChange`, `Sent`, `SigScroll` are the only valid senders.
signals!(
    /// The canonical output of the widget, including `Button` press,
    /// `enter` press on `InputBox`, etc. 
    /// 
    /// When receiving a submit signal, widgets like `InputBox` will emit
    /// a `Submit` signal of its own. 
    /// Thus enabling the `Button -> Text -> Output` chain.
    SigSubmit, 
    /// Send a signal whenever the output of a widget is changed.
    SigChange, 
    /// Sent if a non-draggable sprite is being dragged, 
    /// `Draggable` sprite will be dragged if receiving this signal.
    /// 
    /// This is useful for creating a draggable banner for a non-draggable parent sprite.
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
    /// Modifies the recipient's scale x.
    SigScaleX,
    /// Modifies the recipient's scale y.
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
