# Bevy AoUI Widgets

[![Crates.io](https://img.shields.io/crates/v/bevy_aoui_widgets.svg)](https://crates.io/crates/bevy_aoui_widgets)
[![Docs](https://docs.rs/bevy_aoui_widgets/badge.svg)](https://docs.rs/bevy_aoui_widgets/latest/bevy_aoui_widgets/)

UI, events and dsl for the `bevy_aoui` crate.

This crate does not have a stable API and subject to change.

## Event System

Since AoUI presumably sits on top of the bevy app, we provide an event system
for detecting cursor activity exclusively for AoUI widgets. Cursor events not
catched by our system can be handled by other systems.

We offer a component insertion based core event system for library developers
as well as a oneshot system based event handler system for end users.

### Widgets

We currently offer a few simple widgets.

* `Shape`: a vector shape renderer using `bevy_prototype_lyon`.
* `InputBox`: a single line text input.
* `Button`: a widget that provides click detection and propagation.

## DSL

We offer a DSL for streamlining widget construction.

Before you start, always import the prelude for syntax consistancy.

```rust
use bevy_aoui_widgets::dsl::prelude::*;
```

Each "widget" has a struct and its corresponding macro.

```rust
sprite! ((commands, ..) {
    dim: [400, 400],
    sprite: assets.load("Ferris.png"),
});
```

This translates to

```rust
Sprite {
    dim: [400, 400].dinto(),
    sprite: assets.load("Ferris.png").dinto(),
    ..Default::default(),
}.spawn_with(&mut commands);
```

Where `dinto` is our own `Into`, `DslInto`,
where all the syntax magic happens.

Add children like so, notice you don't need to manually pass in
the context `(commands, ..)`

```rust]
sprite! ((commands, ..) {
    dim: [400, 400],
    sprite: assets.load("Ferris.png"),
    child: textbox! {
        ...
    }
});
```

Check our our book or examples for more info.
