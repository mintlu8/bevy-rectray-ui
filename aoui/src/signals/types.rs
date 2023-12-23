use bevy::math::Vec2;
use bevy::render::color::Color;

use crate::widgets::drag::DragState;

#[macro_export]
macro_rules! signal_receivers {
    ($($(#[$($attr:tt)*])* $name: ident: $ty: ty),* $(,)?) => {
        $(
            $(#[$($attr)*])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
            pub enum $name {}
            impl $crate::signals::SignalReceiver for $name {
                type Type = $ty;
            }
        )*
    };
}

#[macro_export]
macro_rules! signal_both {
    ($($(#[$($attr:tt)*])* $name: ident: $ty: ty),* $(,)?) => {
        $(
            $(#[$($attr)*])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
            pub enum $name {}
            impl $crate::signals::SignalReceiver for $name {
                type Type = $ty;
            }
        )*
    };
}

// signal_senders!(
//     /// The canonical output of the widget, including `Button` press,
//     /// `enter` press on `InputBox`, etc. 
//     /// 
//     /// When receiving a submit signal, widgets like `InputBox` will emit
//     /// a `Submit` signal of its own. 
//     /// Thus enabling the `Button -> Text -> Output` chain.
//     SigSubmit, 
//     /// Send a signal whenever the output of a widget is changed.
//     SigChange, 
// );

signal_both!(
    /// Sent if a non-draggable sprite is being dragged, 
    /// `Draggable` sprite will be dragged if receiving this signal.
    /// 
    /// This is useful for creating a draggable banner for a non-draggable parent sprite.
    SigDrag: DragState,
    /// Sent if being scrolled on, `Scroll` sprite will be scrolled if receiving this signal.
    SigScroll: Vec2,

    /// Triggers some behavior when sent to another widget.
    SigInvoke: (),

    /// Modifies the recipient's text.
    SigText: String,
    /// Modifies the recipient's raw offset.
    SigOffset: Vec2,
    /// Modifies the recipient's raw dimension.
    SigDimension: Vec2,
    /// Modifies the recipient's rotation.
    SigRotation: f32,
    /// Modifies the recipient's scale.
    SigScale: Vec2,

    /// Modifies the recipient's raw offset x.
    SigOffsetX: f32,
    /// Modifies the recipient's raw offset y.
    SigOffsetY: f32,
    /// Modifies the recipient's scale x.
    SigScaleX: f32,
    /// Modifies the recipient's scale y.
    SigScaleY: f32,
    /// Modifies the recipient's raw dimension X.
    SigDimensionX: f32,
    /// Modifies the recipient's raw dimension y.
    SigDimensionY: f32,

    /// Modifies the recipient's color.
    SigColor: Color,
    /// Modifies the recipient's opacity.
    SigOpacity: f32,
    /// Modifies the recipient's disabled status.
    SigDisable: bool,
    /// Modifies the recipient's opacity, if 0, disables the target.
    SigOpacityDisable: f32,

    /// Modifies the recipient layout's margin.
    SigMargin: Vec2,
);
