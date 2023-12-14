# Overview

`bevy_aoui` is a rectangular anchor-offset based 2D and UI solution for the bevy engine.

## Design Goals

* Concise, no boilerplate

    `bevy_aoui` is one of the most concise 2D/UI framework currently available,
    depend on the use case.
    we make use of `macro_rules` macros extensively to eliminate boilerplate wherever possible.

    Tired of writing `Vec2::new(1.0, 2.0)`? Try `[1, 2]` instead.

* Syntax magic, no behavior magic.

    `bevy_aoui` is built from the ground up with `bevy_ecs`.
    All states are implemented using components in a way
    that enables easy composition and low-risk refactoring.

* Designed for the CSS haters

    `bevy_aoui` does not fully reject the ideas from CSS, in fact we have CSS inspired systems
    like `em` and percentage size. However our `Transform2D` is much simpler than a `Style` node,
    and our `Container` is much simpler than `FlexBox`. A lot of weird CSS attributes can be emulated
    by adding more rectangles, using `Container` or abstracting with a custom widget.

* No editor? No problem

    The anchor-offset layout system is intuitive in a no editor environment.
    If an editor does exist, bevy_aoui can integrate with it perfectly as well.

* IDE friendly

    `bevy_aoui` does not use proc_macros (aside from the `color_this` color parser),
    macros used closely mimics rust syntax and
    everything you write is a rust expression that gets editor support.

* Easy migration

    `bevy_aoui` takes care to make sure it will stay compatible with the latest version of bevy.
    We self impose the following feature restrictions:

  * No shaders, no render pipelines.
  * Limited rendering features that can be opt-out:
    * `RenderLayers`, which is needed for clipping.
  * No third party bevy dependencies.

    Features violating one of these rules might live in a separate crate.

* Bridging the gap between 2D and UI

    `bevy_aoui` is a UI framework with full rotation and scaling support,
    or a 2D framework with widgets, depend on your perspective.

* Versatile

    `bevy_aoui` works with anything that uses `GlobalTransform`,
    any third party widget not dependent on `UI`
    should work with `bevy_aoui` with minimal configuration.

## Non-goals

* One-size-fit-all UIs

    `bevy_aoui` is not a html engine, meaning you might
    work a bit harder porting your desktop UI to mobile.

* Styling
  
    `bevy_aoui` is render agnostic, so there is little we can do to provide
    a generalized implementation of things like `border` in CSS.

* Style inheritance

    `bevy_aoui` has a custom widget system you can use to group
    widgets with similar properties. But there likely won't be support for
    directly inheriting style attributes like color from parent to child.

* Configuration files

    Rust is the perfect language, why would you want to write in anything else?

    For a more serious answer,
    `bevy_aoui` encourages the use of rust directly for hierarchy and widget abstractions.
    A decent alternative is to use widget builders through
    serde implementations, however that is currently not directly supported.

* Hot reloading

    Since most of the UI is rust code, we have no control over hot reloading.
