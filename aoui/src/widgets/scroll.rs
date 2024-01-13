use bevy::{hierarchy::Children, log::warn, reflect::Reflect};
use bevy::ecs::{bundle::Bundle, entity::Entity};
use bevy::ecs::query::{With, WorldQuery, QueryItem};
use bevy::ecs::system::{Res, SystemParam};
use bevy::math::{Vec2, IVec2};
use bevy::ecs::{component::Component, query::Without};
use bevy::ecs::system::{Query, Commands};
use crate::{Transform2D, anim::Attr, anim::Offset, AouiREM, DimensionData, signals::ReceiveInvoke};
use crate::events::{EvPositionFactor, MouseWheelAction};
use crate::layout::Container;
use crate::events::{EvMouseWheel, Handlers};
use crate::signals::Invoke;
use crate::dsl::DslInto;

use crate::events::MovementUnits;
pub use super::constraints::ScrollConstraint;

use super::constraints::{SharedPosition, PositionChanged};

/// Add mouse wheel scrolling support.
///
/// This component moves children in this sprites
/// bounding area.
///
/// # Setup Requirements
///
/// * add a single child with the `Size2::FULL` and
/// `Anchor::Center`, which acts as a container.
/// * add children to that child.
///
/// # Supporting components
///
/// * [`EventFlags`](crate::events::EventFlags): Requires `MouseWheel` to be set.
/// * [`ScrollConstraint`]: If specified, the sprite cannot go over bounds of its parent.
/// * [`Handlers<EvMouseWheel>`]:
///     A signal that transfers the `being scrolled` status onto another entity.
///     This will trigger if either scrolled to the end or not scrollable to begin with.
/// * [`Invoke<SigScroll>`]:
///     Receives `EvMouseWheel` on another scrollable sprite.
/// * [`SharedPosition`]: Shares relative position in its parent's bounds with another widget.
///     For example synchronizing a scrollbar with a textbox.
/// * [`Handlers<EvPositionFac>`]: A signal that sends a value
///     in `0..=1` in its constraints when being scrolled.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Scrolling {
    pub pos_x: bool,
    pub neg_x: bool,
    pub pos_y: bool,
    pub neg_y: bool,
}

impl Scrolling {
    pub const X: Scrolling = Scrolling {
        pos_x: true,
        neg_x: true,
        pos_y: false,
        neg_y: false,
    };
    pub const Y: Scrolling = Scrolling {
        pos_x: false,
        neg_x: false,
        pos_y: true,
        neg_y: true,
    };
    pub const NEG_X: Scrolling = Scrolling {
        pos_x: false,
        neg_x: true,
        pos_y: false,
        neg_y: false,
    };
    pub const NEG_Y: Scrolling = Scrolling {
        pos_x: false,
        neg_x: false,
        pos_y: false,
        neg_y: true,
    };
    pub const POS_X: Scrolling = Scrolling {
        pos_x: true,
        neg_x: false,
        pos_y: false,
        neg_y: false,
    };
    pub const POS_Y: Scrolling = Scrolling {
        pos_x: false,
        neg_x: false,
        pos_y: true,
        neg_y: false,
    };
    pub const BOTH: Scrolling = Scrolling {
        pos_x: true,
        neg_x: true,
        pos_y: true,
        neg_y: true,
    };

    pub fn x_scroll(&self) -> bool {
        self.neg_x || self.pos_x
    }

    pub fn y_scroll(&self) -> bool {
        self.neg_y || self.pos_y
    }
}

impl Default for Scrolling {
    fn default() -> Self {
        Self::BOTH
    }
}

impl ReceiveInvoke for Scrolling {
    type Type = MovementUnits;
}

pub(crate) fn scrolling_senders(
    mut commands: Commands,
    sender: Query<(Entity, &MouseWheelAction, &Handlers<EvMouseWheel>), Without<MouseWheelAction>>,
) {
    for (entity, action, signal) in sender.iter() {
        signal.handle(&mut commands.entity(entity), action.get());
    }
}

pub(crate) fn scrolling_system<'t, T: Movement>(
    mut fetched: T::Ctx<'_, '_>,
    mut scroll: Query<(T::Query<'t>, &MouseWheelAction)>,
    mut receiver: Query<(T::Query<'t>, &Invoke<Scrolling>), Without<MouseWheelAction>>,
) {
    let iter = scroll.iter_mut()
        .map(|(query, action)|
            (query, action.get()))
        .chain(receiver.iter_mut().filter_map(|(query, receiver)|
            Some((query, receiver.poll()?)))); {
    }
    for (mut query, input) in iter {
        T::run(&mut query, &mut fetched, input);
    }
}

pub trait Movement {
    type Query<'t>: WorldQuery;
    type Ctx<'w, 's>: SystemParam;
    fn run(this: &mut QueryItem<Self::Query<'_>>, ctx: &mut Self::Ctx::<'_, '_>, amount: MovementUnits);
}

impl Movement for Scrolling {
    type Query<'t> = (Entity, &'t Scrolling, &'t DimensionData, &'t Children);
    type Ctx<'w, 's> = (Commands<'w, 's>, Res<'w, AouiREM>, Query<'w, 's, Attr<Transform2D, Offset>, With<Children>>);

    fn run(this: &mut QueryItem<Self::Query<'_>>, ctx: &mut Self::Ctx::<'_, '_>, delta: MovementUnits) {
        let (entity, scroll, dim, children) = this;
        let (commands, rem, child_query) = ctx;
        let delta_scroll = match (scroll.x_scroll(), scroll.y_scroll()) {
            (true, true) => delta.pixels,
            (true, false) => Vec2::new(delta.pixels.x + delta.pixels.y, 0.0),
            (false, true) => Vec2::new(0.0, delta.pixels.x + delta.pixels.y),
            (false, false) => return,
        };
        if children.len() != 1 {
            warn!("Component 'Scrolling' requires exactly one child as a buffer.");
            return;
        }
        let container = children[0];
        if let Ok(mut transform) = child_query.get_mut(container){
            transform.force_set_pixels(transform.get_pixels(dim.size, dim.em, rem.get()) + delta_scroll);
        }
        commands.entity(*entity).insert(PositionChanged);
    }
}


/// Marker component for making scrolling affect
/// the `range` value on a layout.
///
/// This implementation has the benefit of not requiring clipping.
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub enum ScrollDiscrete {
    XPos,
    XNeg,
    YPos,
    #[default]
    YNeg,
}

impl ScrollDiscrete {
    pub fn new() -> Self {
        Self::YNeg
    }

    pub fn get(&self) -> IVec2 {
        match self {
            ScrollDiscrete::XPos => IVec2::new(1, 0),
            ScrollDiscrete::XNeg => IVec2::new(-1, 0),
            ScrollDiscrete::YPos => IVec2::new(0, 1),
            ScrollDiscrete::YNeg => IVec2::new(0, -1),
        }
    }
}

impl Movement for ScrollDiscrete {
    type Query<'t> = (Entity, &'t ScrollDiscrete, &'t mut Container);

    type Ctx<'w, 's> = Commands<'w, 's>;

    fn run(this: &mut QueryItem<Self::Query<'_>>, commands: &mut Self::Ctx::<'_, '_>, delta: MovementUnits) {
        let (entity, scroll, container) = this;
        let delta = delta.lines.dot(scroll.get());
        match delta {
            ..=-1 => {
                container.decrement();
            }
            1.. => {
                container.increment();
            }
            0 => return,
        };
        commands.entity(*entity).insert(PositionChanged);
    }
}


/// For a texture allow scrolling on the sprite's UV.
/// [`Sprite::rect`](bevy::sprite::Sprite::rect).
///
/// ## Experimental
#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
pub enum ScrollUV {
    X, Y, #[default] XY
}

// impl Movement for ScrollUV {
//     type Query<'t> = (Entity, &'t mut Sprite, &'t ScrollUV);

//     type Ctx<'w, 's> = (Commands<'w, 's>, Res<'w, Assets<Image>>);

//     fn run(this: &mut QueryItem<Self::Query<'_>>, ctx: &mut Self::Ctx::<'_, '_>, amount: MovementUnits) {
//         let (entity, sprite, scroll) = this;

//     }
// }

/// Builder trait for a scrollable widget.
pub trait IntoScrollingBuilder: Bundle + Default {

    fn with_constraints(self) -> impl IntoScrollingBuilder {
        (ScrollConstraint, self)
    }

    fn with_shared_position(self, position: impl DslInto<SharedPosition>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), position.dinto())
    }

    fn with_handler(self, handler: impl DslInto<Handlers<EvPositionFactor>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_invoke(self, handler: impl DslInto<Handlers<EvMouseWheel>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }

    fn with_recv(self, handler: impl DslInto<Invoke<Scrolling>>) -> impl IntoScrollingBuilder {
        (self.with_constraints(), handler.dinto())
    }
}

impl IntoScrollingBuilder for Scrolling {}

impl<T, A> IntoScrollingBuilder for (T, A) where T: IntoScrollingBuilder, A: Bundle + Default {
    fn with_constraints(self) -> impl IntoScrollingBuilder {
        (T::with_constraints(self.0), self.1)
    }
}

impl<T> IntoScrollingBuilder for (ScrollConstraint, T) where T: IntoScrollingBuilder {
    fn with_constraints(self) -> impl IntoScrollingBuilder { self }
}
