use bevy::ecs::component::Component;

/// Represents a persistent cursor interaction state like hovering, clicking or dragging.
///
/// There should be `0` to `1` entity with `CursorFocus` per frame.
#[derive(Debug, Component, Clone, Copy)]
#[component(storage="SparseSet")]
pub struct CursorFocus(pub(super) EventFlags);

impl CursorFocus {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
    pub fn intersects(&self, flag: EventFlags) -> bool {
        self.0.intersects(flag)
    }
}

/// Represents a single frame cursor event like `LeftClick`.
///
/// There should be `0` to `1` entity with `CursorAction` per frame.
#[derive(Debug, Component, Clone, Copy)]
#[component(storage="SparseSet")]
pub struct CursorAction(pub(super) EventFlags);

impl CursorAction {
    pub fn flags(&self) -> EventFlags {
        self.0
    }
    pub fn is(&self, flag: EventFlags) -> bool {
        self.0 == flag
    }
    pub fn intersects(&self, flag: EventFlags) -> bool {
        self.0.intersects(flag)
    }
}


/// Represents cursor clicking outside the sprite's hitbox.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct CursorClickOutside;

/// Set if some descendant has focus.
#[derive(Debug, Component)]
#[component(storage="SparseSet")]
pub struct DescendantHasFocus;

pub(super) mod sealed {
    use bevy::{ecs::component::Component, reflect::Reflect};

    tlbf::tlbf!(
        /// Flags for cursor events.
        ///
        /// Valid listeners are `Hover`, `*Click`, `*Drag`, `DoubleClick`, `Drop` and `ClickOutside`.
        ///
        /// * `Hover` listens for `Hover`,
        /// * `Click` listens for `Down`, `Up` and `Pressed`
        /// * `Drag` listens for `Down`, `DragEnd` and `Drag`
        /// * `DoubleClick` listens for `DoubleClick`, which replaces `Click` or `DragEnd`
        /// * `Drop` listens for `Drop`
        /// * `ClickOutside` listens for mouse up outside
        ///
        /// Events are emitted as 3 separate components, each frame a sprite can receive at most one of each:
        /// * `CursorFocus`: `Hover`, `Pressed`, `Drag`.
        /// * `CursorAction`: `Down`, `Click`, `DragEnd`, `DoubleClick`, `Drop`.
        /// * `CursorClickOutside`: `ClickOutside`.
        ///
        /// Details:
        /// * `Click` requires mouse up and mouse down be both inside a sprite.
        /// * `ClickOutside` requires mouse up be outside of a sprite and the sprite not being dragged.
        /// * Dragged sprite will receive `Down` from other mouse buttons regardless of their handlers.
        /// * There is in fact no `MouseUp`.
        #[derive(Component, Reflect, Default)]
        pub EventFlags: u32 {
            pub Idle,
            pub Hover,
            pub LeftDrag,
            pub LeftDown,
            pub LeftPressed,
            pub LeftClick,
            pub DoubleClick,
            pub MidDown,
            pub MidPressed,
            pub MidClick,
            pub MidDrag,
            pub RightDown,
            pub RightPressed,
            pub RightClick,
            pub RightDrag,
            pub Drop,
            pub DragEnd,
            pub ClickOutside,
            pub MouseWheel,
        }
    );

    impl EventFlags {
        pub const fn const_or(self, other: EventFlags) -> Self{
            Self(self.0 | other.0)
        }

        #[allow(non_upper_case_globals)]
        pub const All: Self = Self::all();

        /// Blocks all event listeners underneath.
        #[allow(non_upper_case_globals)]
        pub const BlockAll: Self = Self(
            Self::Hover.0 |
            Self::LeftClick.0 |
            Self::MidClick.0 |
            Self::RightClick.0 |
            Self::Drop.0 |
            Self::MouseWheel.0
        );
    }
}

pub use sealed::EventFlags;
/// A dummy indicator for no event is happening.
pub type EvIdle = sealed::Idle;
/// An event sent if widget is being hovered.
pub type EvHover = sealed::Hover;
/// An event sent if widget is being dragged by LMB.
pub type EvLeftDrag = sealed::LeftDrag;
/// An event sent if widget is just pressed by LMB.
pub type EvLeftDown = sealed::LeftDown;
/// An event sent if widget is being hovered and pressed by LMB.
pub type EvLeftPressed = sealed::LeftPressed;
/// An event sent if LMB down and up both happened inside the widget.
pub type EvLeftClick = sealed::LeftClick;
/// An event sent if LMB double clicked inside the widget.
pub type EvDoubleClick = sealed::DoubleClick;
/// An event sent if widget is just pressed by MMB.
pub type EvMidDown = sealed::MidDown;
/// An event sent if widget is being hovered and pressed by MMB.
pub type EvMidPressed = sealed::MidPressed;
/// An event sent if MMB down and up both happened inside the widget.
pub type EvMidClick = sealed::MidClick;
/// An event sent if widget is being dragged by MMB.
pub type EvMidDrag = sealed::MidDrag;
/// An event sent if widget is just pressed by RMB.
pub type EvRightDown = sealed::RightDown;
/// An event sent if widget is being hovered and pressed by RMB.
pub type EvRightPressed = sealed::RightPressed;
/// An event sent if RMB down and up both happened inside the widget.
pub type EvRightClick = sealed::RightClick;
/// An event sent if widget is being dragged by RMB.
pub type EvRightDrag = sealed::RightDrag;
/// An event sent if dragging is release inside the widget.
pub type EvDrop = sealed::Drop;
/// An event sent if dragging is released.
pub type EvDragEnd = sealed::DragEnd;
/// An event sent if some mouse button was release outside the widget.
pub type EvClickOutside = sealed::ClickOutside;
/// An event sent if mouse wheel was scrolled on the widget.
pub type EvMouseWheel = sealed::MouseWheel;

/// An event sent if widget is being dragged.
///
/// Sends the state `Start`, `Continue`, `End`.
/// Delta can be found in the input context.
#[derive(Debug)]
pub enum EvMouseDrag{}

/// An event sent if widget has lost focus (drag, hover, pressed).
#[derive(Debug)]
pub enum EvLoseFocus{}

/// An event sent if widget has obtained focus (drag, hover, pressed).
#[derive(Debug)]
pub enum EvFocusChange{}

/// An event sent if widget has obtained focus (drag, hover, pressed).
#[derive(Debug)]
pub enum EvObtainFocus{}

/// An event that sends the `Payload` value of a widget in the button family.
#[derive(Debug)]
pub enum EvButtonClick{}

/// An event that sends the `bool` value of a Toggle/CheckButton.
#[derive(Debug)]
pub enum EvToggleChange{}

/// An event that sends the key of a spin text
/// on change.
#[derive(Debug)]
pub enum EvSpinChange{}

/// An event that sends the content of a text input
/// on change.
#[derive(Debug)]
pub enum EvTextChange{}

/// An event that sends the content of a text input via
/// pressing enter or receiving [`Invoke`](crate::signals::Invoke).
#[derive(Debug)]
pub enum EvTextSubmit{}

/// An event signal that sends a [`f32`] based on
/// the position of a sprite, generated by either scrolling or dragging.
#[derive(Debug)]
pub enum EvPositionFactor{}
