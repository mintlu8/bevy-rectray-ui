//! Bevy AoUI provides a light-weight rectangular anchor-offset based 2D sprite layout, 
//! UI layout and skeletal animation system.
//! 
//! Similar to the philosophy of Rust, AoUI provides low level control through the 
//! anchor-offset system and high level ergonomics through its layout system.
//! 
//! # The AoUI Pipeline
//! 
//! AoUI is not a full render pipeline system, but rather a 
//! [`Transform`](bevy::prelude::Transform) and
//! [`GlobalTransform`](bevy::prelude::GlobalTransform) generator.
//! 
//! AoUI replaces bevy's standard transform systems
//! like [`propagate_transforms`](bevy::transform::systems::propagate_transforms)
//! and [`sync_simple_transforms`](bevy::transform::systems::sync_simple_transforms)
//! on structs marked with [AoUI],
//! while leveraging other parts of bevy's standard library and ecosystem whenever possible.
//! 
//! AoUI provides 2 rendering methods: 
//! * [`BuildTransform`] generates `GlobalTransform` directly.
//! * [`BuildTransform`] generates `Transform`.
//! 
//! AoUI propagates translation, rotation, scale and font size down its tree.
//! 
//! In the default pipeline, the root node of the AoUI tree is the window. 
//! meaning orphaned sprites will be placed against the window's rectangle.
//!
//! # Getting Started
//! 
//! Before you start you should check out `bevy_aoui`'s examples if you like shapes or DSL.
//! 
//! First add the AoUI Plugin:
//! 
//! ```
//! # /*
//! app.add_plugins(AoUIPlugin)
//! # */
//! ```
//! 
//! Create a sprite:
//! 
//! ```
//! # /*
//! commands.spawn(AoUISpriteBundle {
//!     sprite: Sprite { 
//!         color: Color::RED,
//!         ..Default::default()
//!     },
//!     transform: Transform2D { 
//!         center: Some(Anchor::Center),
//!         anchor: Anchor::TopCenter,
//!         offset: Vec2::new(20.0, 0.0),
//!         rotation: 1.21,
//!         scale: Vec2::new(4.0, 1.0),
//!         ..Default::default()
//!     },
//!     dimension: Dimension::pixels(Vec2::new(50.0, 50.0)),
//!     texture: assets.load("sprite.png"),
//!     ..Default::default()
//! });
//! # */
//! ```
//! 
//! Create some text:
//! 
//! ```
//! # /*
//! commands.spawn(AoUITextBundle {
//!     text: Text::from_section(
//!         "Hello, World!!", 
//!         style(Color::WHITE)
//!     ),
//!     font: assets.load::<Font>("OpenSans.ttf"),
//!     transform: Transform2D { 
//!         center: Some(Anchor::Center),
//!         anchor: Anchor::TopCenter,
//!         offset: Vec2::new(20.0, 0.0),
//!         rotation: 1.21,
//!         scale: Vec2::new(4.0, 1.0),
//!         ..Default::default()
//!     },
//!     dimension: Dimension::COPIED.with_em(SetEM::Pixels(24.0)),
//!     ..Default::default()
//! });
//! # */
//! ```
//! 
//! # Core Concepts
//!
//! AoUI offers a refreshingly different paradigm from traditional CSS based UI layout.
//! 
//! AoUI Sprites contains these core components:
//! * [anchor](Transform2D::anchor)
//! * [center](Transform2D::center)
//! * [offset](Transform2D::offset)
//! * [rotation](Transform2D::rotation)
//! * [scale](Transform2D::scale)
//! * [dimension](Dimension::dim)
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
//! When applying `rotation` and `scale`, sprites can use a 
//! `center` that operates independently from the anchor.
//! 
//! # Container
//! 
//! Anchor-Offset is well-suited for isolated UI components, but when it comes to arranging
//! multiple UI elements in a specific order, you'll find the `Container` useful.
//! 
//! The `Container` is a layout system that only depends on insertion order and works
//! with Bevy's [`Children`](bevy::prelude::Children) component.
//! 
//! Check out the book for more information.
//! 
//! # Advantages of AoUI
//! 
//! There are many awesome UI libraries in the bevy ecosystem
//! that you should definitely use over AoUI in
//! many use cases. However, AoUI offers some unique advantages:
//! 
//! * Full ECS support with easy feature composition.
//! 
//! AoUI is built fully embracing bevy's ecosystem. 
//! You can mix and match our modularized components
//! and add, remove or edit any system you want to change.
//! 
//! * Relative size system.
//! 
//! Full support for web like size units: `em`, `rem`, `%`, etc.
//! 
//! * First class rotation and scaling support.
//! 
//! You are can rotate and scale any sprite from any position on it with ease.
//! 
//! * Simple but versatile layout system.
//! 
//! Simple layouts that work out of the box with minimal configuration.
//! 
//! * High level abstractions with low level control.
//! 
//! You can mix and match anchoring and layouts to best suit your needs.
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_match)]

pub mod layout;
pub(crate) mod core;
pub mod dsl;
pub mod widgets;
pub mod events;
pub mod anim;
pub mod signals;
pub use core::*;

#[doc(hidden)]
pub use bevy;

pub mod schedule;
mod extension;
pub use extension::WorldExtension;

pub use schedule::CorePlugin;

/// Plugin for both widgets and events.
#[derive(Debug)]
pub struct AoUIPlugin;

impl bevy::prelude::Plugin for AoUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(schedule::CorePlugin)
            .add_plugins(signals::SignalsPlugin)
            .add_plugins(events::CursorEventsPlugin)
            .add_plugins(anim::AnimationPlugin)
            .add_plugins(widgets::schedule::WidgetsPlugin)
        ;
    }
}