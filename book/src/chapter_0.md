# Overview

`bevy_aoui` is a rectangular anchor-offset based 2D and UI solution for the bevy engine.

## Design Goals

* Concise, no boilerplate

    `bevy_aoui` is probably the most concise 2D/UI framework currently available,
    we make use `macro_rules` macros extensively to eliminate boilerplate wherever possible.

    Tired of writing `Vec2::new(1.0, 2.0)`? Try `[1, 2]` instead.

* No editor? No problem

    The anchor-offset layout system is extremely intuitive in a no editor environment compared to css
    based layouts or flexbox. This reduces the need for hot-reloading too. 
    If an editor does exist, bevy_aoui can integrate perfectly as well.

* Editor friendly

    `bevy_aoui` does not use proc_macros (aside from the `color_this` color parser),
    everything you write is a rust expression that gets editor support.

* Easy migration

    `bevy_aoui` takes care to make sure it will stay compatible with the latest version of bevy.
    We self impose the following feature restrictions:

  * No shaders, no pbr, no render graph.
  * Limited rendering features:
    * `RenderLayers`, which is needed for clipping.
  * No third party bevy dependencies.

    Features violating one of these rules might live in a separate crate.

* Bridging the gap between 2D and UI

    `bevy_aoui` is a UI framework with full rotation and scaling support,
    or a 2D framework with widgets, depend on your perspective.

* Versatile

    `bevy_aoui` works with anything that uses `GlobalTransform`,
    any third party widget that does not depend on `UI`
    should work with `bevy_aoui` with minimal configuration.

## Non-goals

* One-size-fit-all UIs

    `bevy_aoui` does not have a full flexbox implementation, meaning you might
    work a bit harder porting your desktop UI to mobile.

* Configuration files

    Rust is the perfect language, why would you want to write anything else?
    `bevy_aoui` encourages the use of rust directly for hierarchy and widget abstractions
    A decent alternative to construct a scene is to use widget builders through
    serde implementations, however that is not directly supported.

* Hot reloading

    Since most of the UI is rust code, we have no control over hot reloading.

* Blazingly Fast

    Our transform pipeline trades a small amount of performance for ergonomics,
    which may or may not be a big deal.

    Our implementation for `clipping` uses a camera and a render target,
    which is great for universality, but might not be the best for performance.
    Use an alternative implementation if needed.
