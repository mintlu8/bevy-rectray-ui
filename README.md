# Bevy Aoui

[![Crates.io](https://img.shields.io/crates/v/bevy_aoui.svg)](https://crates.io/crates/bevy_aoui)
[![Docs](https://docs.rs/bevy_aoui/badge.svg)](https://docs.rs/bevy_aoui/latest/bevy_aoui/)

Bevy Aoui is a component based 2D and UI solution for the bevy engine.

## Getting Started

First add the Aoui Plugin:

```rust
app.add_plugins(AouiPlugin)
```

Import the DSL prelude in the function scope

```rust
fn spawn(mut commands: AouiCommands) {
    use bevy_aoui::dsl::prelude::*;
    ...
}
```

Create a sprite:

```rust
sprite!(commands {
    sprite: "Ferris.png",
    anchor: Left,
    offset: [40, 0],
    dimension: [200, 200],
})
```

This spawns a "Ferris.png" to the center left of the screen,
moved to the right by 40 px, with dimension 200 px * 200 px,
and returns an `Entity`.

Create a stack of words:

```rust
vstack!(commands {
    font_size: em(2),
    child: text! {
        text: "Hello"
    },
    child: text! {
        text: "rust"
    },
    child: text! {
        text: "and"
    },
    child: text! {
        text: "bevy"
    },
});
```

## How this works?

`bevy_aoui` is all about rectangles!

Each sprite is a rectangle, and placed relative to the parent
rectangle.

You might want to

```text
Place a sprite to the center right of the parent sprite,
move left by 10 px, 
with 20% of parent's width as width
2x font size as height
and rotate by 45 degrees.
```

In `aoui` this is incredibly simple:

```rust
sprite!(commands {
    anchor: Right,
    offset: [-10, 0],
    dimension: size2!(20 %, 2 em),
    rotation: degrees(45),
    ...
})
```

Use `Transform2D` and `Dimension` to manipulate `aoui` widgets directly.

## What `bevy_aoui` provides

* Fine grained low level anchor-offset layout system.
* First class support for rotation and scaling.
* Simple and intuitive layouts.
* Decentralized ECS components with no central state.
* Complete support of bevy's 2D primitives.
* Input handling system for mouse and cursor.
* Building blocks for most common widgets.
* Event handling through closures.
* Reactivity and animation through signals.
* `macro_rules` based DSL that annihilates boilerplate.
* Easy migration to future bevy versions.

## What `bevy_aoui` is not

* Not a renderer.

    `bevy_aoui` has minimal rendering features and no third party bevy dependencies,
    this ensures maintainability and easy migration to future bevy versions,
    at the cost of not having out of the box widget styles.

* Not `bevy_ui` compatible.

    `bevy_aoui` is not dependent on `bevy_ui` in any way. This means `bevy_ui` exclusive
    features won't be available in `bevy_aoui` as is.

* No ui script or serialization.

    `bevy_aoui` uses rust closures for a lot of things, including events and reactivity,
    those are unfortunately not serializable.

* No styling

   Styling is outside the scope of this crate.

## Layouts

Vanilla `bevy_aoui` gives you an experience akin to a traditional 2D game framework,
this is great for keeping things simple at first, like placing something
at the corner of a window.
But for more complicated UI you might find using `Layout` with `Container` more attractive.

With `Container`, you get access to CSS like properties `padding` and `margin`,
reverse dimension propagation in `BoundsLayout`,
and common layouts like `hbox`, `paragraph` and `grid`.

You can also implement `Layout` yourself to create a custom layout.

## Widget Abstractions

Widget builders are used to empower our DSL.
Widget builders implements `Widget` and `Default` and can be used in general like so:

```rust
FrameBuilder {
    offset: [121, 423].dinto(),
    anchor: Center.dinto(),
    color: color!(red).dinto()
    ..Default::default()
}.build(commands)
```

This returns an `Entity`.

`dinto` is implemented in `DslFrom` or `DslInto`.
which gives us nice conversion like `[i32; 2] -> Vec2`, which can save us a lot of typing!

When using the dsl macro, this becomes

```rust
frame! (commands {
    offset: [121, 423],
    anchor: Center,
    color: color!(red),
});
```

much nicer, right?

`commands` is the context, if `AssetServer` is needed
we can put `commands` there, which should be the
case most of the time.

## DSL Syntax

The DSL have a few special fields that makes it much more powerful than
a simple struct constructor.

### child and children

`child:` is a special field that can be repeated, it accepts an `Entity`
and inserts it as a child.

```rust
frame! (commands {
    ...
    child: rectangle! {
        dimension: [40, 40]
    },
    child: text! {
        text: "Hello, World!!"
    },
});
```

This syntax, notice the use of braces `{}`,

```rust
field: macro! { .. },
```

Will be automatically rewritten as

```rust
field: macro!(commands { .. }),
```

Which serves as context propagation.

`children:` adds an iterator as children to the entity.
Iterators of `Entity` and `&Entity` are both accepted.
Child and children guarantees insertion order.

### extra

Extra adds a component or a bundle to a widget,
which is the idiomatic pattern to compose behaviors.

```rust
// Example: Add dragging support to a `Sprite`.
sprite! (commands {
    ...
    extra: DragX,
    extra: DragConstraint,
    extra: DragSnapBack,
});
```

### entity

`entity` lets us fetch the `Entity`
directly from a nested macro invocation.

```rust
let sprite_entity: Entity;
sprite! (commands {
    child: sprite! {
        entity: sprite_entity,
    }
});
```

## Next Step

See documentation on individual modules for more information.

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
