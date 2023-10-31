//! Bevy AoUI provides a light-weight Anchor-Offset based 2D sprite rendering system.
//! 
//! AoUI is not a full render pipeline system, but rather a [`GlobalTransform`](bevy::prelude::GlobalTransform) generator.
//! AoUI replaces bevy's standard transform systems
//! like [`propagate_transforms`](bevy::transform::systems::propagate_transforms)
//! and [`sync_simple_transforms`](bevy::transform::systems::sync_simple_transforms),
//! on structs marked with [AoUI],
//! while leveraging other parts of bevy's standard library and ecosystem whenever possible.
//!
//! # Core Concepts
//!
//! AoUI offers a refreshingly different paradigm from traditional CSS based UI layout.
//! 
//! AoUI Sprites contains these core components:
//! * [anchor](Anchors::anchor)
//! * [center](Anchors::center)
//! * [dimension](Dimension::dim)
//! * [offset](Transform2D::offset)
//! * [rotation](Transform2D::rotation)
//! * [scale](Transform2D::scale)
//! 
//! Each sprite is conceptualized as a rectangle with a dimension and 
//! 9 [anchors](bevy::sprite::Anchor): `BottomLeft`, `CenterRight`, `Center`, etc.
//! 
//! [Custom anchors](bevy::sprite::Anchor::Custom) can be used but not in some layouts.
//! 
//! Sprites are connected to parent sprites via one of the parent's anchors
//! and can be offset by a `Vec2`. When the offset is set to `(0, 0)`, 
//! the anchors of the parent and child sprites overlap.
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
//! In the case of parentless sprites, they are anchored to the window's rectangle.
//! 
//! When applying `rotation` and `scale`, each sprite utilizes a 
//! `center` that operates independently from the anchor. 
//! This `center` is regarded as the definitive [`Transform`](bevy::prelude::Transform) 
//! for the sprite when used with [`BuildTransform`] to integrate with native components.
//! 
//! Similar to standard Bevy behavior, `rotation` and `scale` propagate to all child sprites.
//! 
//! # FlexContainer
//! 
//! Anchor-Offset is well-suited for isolated UI components, but when it comes to arranging
//! multiple UI elements in a specific order, you'll find the [`FlexContainer`] useful.
//! 
//! The `FlexContainer` is a layout system that only depands on insertion order and seamlessly
//! integrates with Bevy's [`Children`](bevy::prelude::Children) component.
//!
//! ## Span
//! 
//! [`Span`](FlexLayout::Span) serves as a classic single-line layout, akin to `HBox` or `VBox`.
//! 
//! Children are first sorted into 3 buckets based on their anchors along the main axis, 
//! and aligned differently based on their assigned bucket.
//! 
//! For `HBox`, we have sprites laid out as follows:
//! ```
//! # /*
//! (Anchors::*Left ..)       (Anchors::*Center ..)       (Anchors::*Right ..)
//! # */
//! ```
//! with each sprite takes up `dimension * scale` size. 
//! 
//! It's important to note that currently offset and rotation 
//! does not affect how much space a sprite takes up,
//! if you think they should, please file an issue.
//! 
//! ## Paragraph
//! 
//! [`Paragraph`](FlexLayout::Paragraph) is a layout for wrapping spans.
//! 
//! Children are collected sequentially until a line is filled,
//! and then they are rendered as a `span`. 
//! Thanks to the alignment features of a `span`, the paragraph
//! layout can create complex arrangements with minimal additional support.
//! 
//! To better leverage the paragraph layout, you can use [`FlexControl::Linebreak`] or
//! [`LinebreakBundle`] to perform a linebreak. 
//! These features are also available in `grid` and `table`.
//! 
//! ## Grid
//! 
//! [`Grid`](FlexLayout::Grid) is a layout of evenly subdivided cells.
//! 
//! Key Features
//! 
//! * Offers both size-based and count-based divisions.
//! * Supports any insertion order.
//! * Supports re-alignment for incomplete rows.
//! 
//! See [`SparseLayout::Rectangles`] for a more flexible version.
//! 
//! ## Table
//! 
//! [`Table`](FlexLayout::Table) provides a layout system for aligned rows and columns.
//! 
//! `Table` supports both dynamic-sized columns and percentage-width-based columns.
//! 
//! # SparseContainer
//! 
//! [`SparseContainer`] provides a tile map like layout.
//! 
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
//! The order of rendering goes as follows:
//! 1. Compute the final position of the parent anchor.
//! 2. Apply an offset from the parent anchor.
//! 3. Scale using the parent anchor and the parent's scale.
//! 4. Rotate based on the parent anchor and the parent's rotation.
//! 5. Apply local scaling relative to the sprite's center.
//! 6. Perform local rotation around the sprite's center.
//!
//! The result always
//! * is a 2D sprite
//! * is a rotated rectangle
//! * is equivalent to a global transform
//! * has the same rotation as the sum of its and its ancestors' rotations
//! * has the same scale as the product of its and its ancestors' scales
//! 
//! # DSL
//! 
//! We offer an optional DSL to streamline widget creation and reduce boilerplate.
//! Check out our book for more details.
//!
//! # Performance
//! 
//! We prioritize ergonomics over performance. 
//! Our rendering system uses extra steps 
//! compared to traditional rendering, this includes:
//! 
//! * Maintaining rectangles instead of points.
//! * Fetching anchor points from rotated rectangles.
//! * Two step rotation and scale (from anchor and from center).
//! 
//! This means we probably won't be as performant as native bevy transfroms.
//! Nevertheless, our performance is enough to support most UI use cases, 
//! including rendering multiline rich text directly with `FlexContainer`. 
//! 
//! Consider submit a pull request if you can improve our performance.
//! 
//! # Widgets
//! 
//! `AoUI` does not provide event handling widgets,
//! such as buttons or checkboxes, out of the box, 
//! but they can easily be built using our event system.
//! 
//! Since `AoUI` is a very thin abstraction layer over standard bevy, 
//! it should be relatively easy to integrate widgets from other crates.
//! 
//! Official `AoUI` widgets might live in a separate crate in the future.
//! 
//! # Todos
//! 
//! * Add support for `SpriteSheetBundle`.
//! * Better `Mesh2D` support.
//! * Implement change detection.
//! * Implement some kind of custom spritesheet.
//! * Better rich text support.

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
