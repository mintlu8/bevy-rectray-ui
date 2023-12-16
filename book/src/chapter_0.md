# Overview

`bevy_aoui` is a rectangular anchor-offset based 2D and UI solution for the bevy engine.

## Design Goals

* Concise, no boilerplate

    `bevy_aoui` is one of the most concise 2D/UI framework currently available,
    depend on the use case.
    we make use of `macro_rules` macros extensively to eliminate boilerplate wherever possible.

    Tired of writing `Vec2::new(1.0, 2.0)`? Try `[1, 2]` instead.

* Syntax magic, no behavior magic

    `bevy_aoui` is built from the ground up with `bevy_ecs`.
    All states are implemented using components in a way
    that enables easy composition and low-risk refactoring.

* Designed for the CSS haters

    `bevy_aoui` uses some CSS features like size units and transition,
    while using a drastically simpler layout system that favors expressing
    style through a hierarchy rather than a single node. This produces
    fine grained nodes that enables the `MarkedSignal` reactive system.

* Fully reactive

    Create a signal, mark the sender with what you wants to send,
    mark the receiver with what you want to change, that's it!

* No editor? No problem

    The anchor-offset layout system is designed to be
    intuitive in a no editor environment, while also being easy to use
    in an editor environment.

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

* Bevy2D as a backend

    `bevy_aoui` exclusively uses bevy's 2D primitives like `Sprite`, `Text2D`,
    `TextureAtlas`, `MaterialMesh` etc to render its contents.
    Meaning we get all the cool features Bevy2D provides
    both now and in the future.

* Bridging the gap between 2D and UI

    `bevy_aoui` is a UI framework with full rotation and scaling support,
    or a 2D framework with widgets, depend on your perspective.

* Versatile

    `bevy_aoui` works with anything that uses `GlobalTransform`,
    any third party widget not dependent on `UI`
    should work with `bevy_aoui` with minimal configuration.

## Limitations

* UI Features

    `bevy_aoui` does not use `bevy_ui`, therefore features available only in
    `bevy_ui` might not be present in `bevy_aoui`.

* No styling
  
    `bevy_ui` and most other UI crate have some kind of styling system,
    but since `bevy_aoui` is render agnostic, there is little we can do to provide
    a generalized implementation of things like `border` in CSS.
    However custom styling can be easily implemented using our widget
    abstraction system.

* Clipping requires a new render target

    All features using clipping, like `ScrollLayer`, `DropDown`, `ComboBox`
    require spawning a new camera and using a new `RenderLayer`.
    This approach has the benefit of being truly render agnostic, while
    supporting almost **all** features available in this crate, including rotation
    and scaling.
    However this will incur a performance cost and might not be the ideal solution
    for every use case.

* No support for drastically different aspect ratios

    `bevy_aoui` is not a html engine, and our Layout systems is
    not suited to handle both desktop and mobile.

* Configuration files

    Rust is the perfect language, why would you want to write in anything else?

    For a more serious answer,
    `bevy_aoui` encourages the use of Rust directly for hierarchy and widget abstractions.
    A decent alternative is to use widget builders through
    serde implementations, however that is currently not directly supported.

* Hot reloading

    Since most of the UI is rust code, we have no control over hot reloading.
