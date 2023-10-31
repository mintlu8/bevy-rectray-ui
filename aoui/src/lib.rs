//! Bevy AoUI provides a light-weight Anchor-Offset based 2D sprite rendering system.
//! 
//! AoUI is not a full render pipeline system, but rather a [`GlobalTransform`](bevy::prelude::GlobalTransform) generator.
//! Meaning we only replace bevy's standard transform systems 
//! like [`propagate_transforms`](bevy::transform::systems::propagate_transforms)
//! and [`sync_simple_transforms`](bevy::transform::systems::sync_simple_transforms),
//! while leveraging other parts of bevy's standard library and ecosystem as much as possible.
//!
//! # Core Concepts
//!
//! AoUI offers a refreshingly different paradime from traditional CSS based UI layout.
//! 
//! These are the core components of a AoUI Sprite:
//! * [anchor](Anchors::anchor)
//! * [center](Anchors::center)
//! * [dimension](Dimension::dim)
//! * [offset](Transform2D::offset)
//! * [rotation](Transform2D::rotation)
//! * [scale](Transform2D::scale)
//! 
//! Each sprite is considered a rectangle with a dimension and 
//! has 9 [anchors](bevy::sprite::Anchor): `BottomLeft`, `CenterRight`, `Center`, etc.
//! 
//! We parent each sprite to one of the parent sprite's anchors,
//! and offset it by a `Vec2`. 
//! If offset is `(0, 0)`, the parent and child sprite's 
//! anchors overlap.
//! 

//! <svg width="256px" height="256px" style="margin-left: auto; margin-right: auto; display: block;" viewBox="0 0 128 128" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
//!     <g transform="matrix(0.358906,0,0,0.278019,-25.4441,-14.0335)">
//!         <rect x="70.893" y="50.477" width="356.639" height="460.401" style="fill:none;stroke:rgb(239,239,239);stroke-width:6.23px;"/>
//!     </g>
//!     <g transform="matrix(0.164311,0,0,0.164311,-0.154123,22.9892)">
//!         <g transform="matrix(6.08604,-0,-0,6.08604,0.937997,-139.913)">
//!             <path d="M44.623,48.433L47.218,49.191L45.52,51.296" style="fill:none;stroke:rgb(239,239,239);stroke-width:1px;stroke-linejoin:miter;stroke-miterlimit:10;"/>
//!             <path d="M0,64C0,64 40.751,51.22 47.218,49.191" style="fill:none;stroke:rgb(239,239,239);stroke-width:1px;"/>
//!         </g>
//!     </g>
//!     <g transform="matrix(0.25,0,0,0.535029,0.812428,-19.6745)">
//!         <rect x="188.717" y="54.654" width="148.18" height="148.18" style="fill:none;stroke:rgb(239,239,239);stroke-width:2.39px;"/>
//!     </g>
//! </svg>

//! 
//! For all parentless sprites, 
//! the root parent is the window's rectangle.
//! 
//! To apply `rotation` and `scale`, each sprite uses a `center`
//! independent from anchor. 
//! This is considered to be the canonical [`Transform`](bevy::prelude::Transform) of the sprite.
//! Rotation and scale propagates to all children just like regular bevy.
//! 
//! # FlexContainer
//! 
//! Anchor-Offset works well for isolated UI components, but for laying out
//! multiple UI components in order, we need the [`FlexContainer`].
//! 
//! The FlexContainer is a insertion order only layout 
//! that works with bevy's [`Children`](bevy::prelude::Children) component.
//!
//! ## Span
//! 
//! [`Span`](FlexLayout::Span) is your classic single line `HBox` or `VBox`.
//! 
//! Children are first sorted into 3 buckets based on their anchors along the main axis, 
//! and aligned differently based on their assigned bucket.
//! 
//! For `HBox`, we have sprites laid out like
//! ```
//! # /*
//! (Anchors::*Left ..)       (Anchors::*Center ..)       (Anchors::*Right ..)
//! # */
//! ```
//! with each bucket respecting their insertion order.
//! 
//! Each sprite takes up `dimension * scale` size. 
//! 
//! Currently offset and rotation does not affect how much space a sprite takes up,
//! if you think they should, please file an issue.
//! 
//! ## Paragraph
//! 
//! [`Paragraph`](FlexLayout::Paragraph) is a layout of wrapping spans.
//! 
//! We collect children sequentially until a line fills up,
//! then we render them like a `span`. Due to the alignment feature of `span`, paragraph
//! is capable of creating complex layout on its own with minimal additional support.
//! 
//! To better leverage the paragraph layout, you can use [`FlexControl::Linebreak`] or
//! [`LinebreakBundle`] to perform a linebreak. These are also available in `grid` or `table`.
//! 
//! ## Grid
//! 
//! [`Grid`](FlexLayout::Grid) a layout of evenly subdivided cells.
//! 
//! Features
//! 
//! * Any insertion order.
//! * Both size and count based division.
//! * Re-align incomplete rows.
//! 
//! ## Table
//! 
//! [`Table`](FlexLayout::Table) is a layout of aligned rows and columns.
//! 
//! We support both dynamic sized columns and percentage width based columns.
//! 
//! # SparseContainer
//! 
//! [`SparseContainer`] provides a tile map like layout.
//! Supported [layouts](`SparseLayout`) are 
//! * `Rectangles`
//! * `Isometric` 
//! * `HexGrid`
//! 
//! Children needs to specify a [`SparsePosition`]
//! to be placed accordingly.
//! 
//! # Mental Model
//!
//! The order of rendering goes:
//! * compute parent anchor's final position
//! * offset from parent anchor
//! * scale from parent anchor using parent's scale
//! * rotate from parent anchor using parent's rotation
//! * scale locally from the sprite's center
//! * rotate locally from the sprite's center
//!
//! The result always
//! * is 2D
//! * is a rotated rectangle
//! * is equivalent to a global transform
//! * has the same rotation as the sum of its and its ancestors' rotations
//! * has the same scale as the product of its and its ancestors' scales
//!
//! # Performance
//! 
//! We prioritize ergonomics over performance and our rendering system uses
//! extra steps compared to traditional rendering, this includes:
//! 
//! * Maintaining rectangles instead of points
//! * Two step rotation and scale (from anchor and center)
//! 
//! This means we probably won't be as fast as native bevy transfroms.
//! Nevertheless, our performance is enough to support most UI use cases, including rendering
//! multiline rich text directly with `FlexContainer`. 
//! 
//! Using AoUI as a particle emitter, however, is ill-advised.
//! 
//! Please submit a pull request if you can improve our performance.
//! 
//! 
//! # Widgets
//! 
//! `AoUI` does not provide event handling widgets,
//! like buttons or checkboxes, out of the box, 
//! but they can easily be built using our event system.
//! 
//! Since `AoUI` is a very thin abstraction layer over standard bevy, 
//! it should be relatively easy to integrate widgets from other crates.
//! 
//! Official `AoUI` widgets will live in a separate crate,
//! and `AoUI` DSL will support them in the future.
//! 
//! # Todos
//! 
//! * Add support for `SpriteSheetBundle`.
//! * Better `Mesh2D` support.
//! * Implement change detection.
//! 

pub mod schedule;
mod rect;
mod components;
mod hitbox;
mod compute;
mod scaling;

mod flex;
mod sparse;

pub use rect::*;
pub use components::*;
pub use schedule::AoUIPlugin;
pub use hitbox::*;
pub use flex::*;
pub use sparse::*;

mod bundles;
pub use bundles::*;
mod events;
pub use events::*;
pub use scaling::*;

pub use bevy_aoui_derive::sprite;
