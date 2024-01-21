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