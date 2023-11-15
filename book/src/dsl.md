# DSL

AoUI provides a simple DSL syntax for UI generation.

For every AoUI widget struct, there is a macro with a corressponding name
`e.g.` `Sprite` has `sprite!`

The macro has syntax almost identical to struct construction:

```rust
widget!(commands {
    name: "Hello AoUI!",
    dimension: [20, 20],
    color: color!(red),
})
```

which translates to

```rust
Widget(commands {
    name: "Hello AoUI!".dinto(),
    dimension: [20, 20].dinto(),
    color: color!(red).dinto(),
    ..Default::default()
})
```

`dinto` uses our `DslInto` trait to provide some nice
syntax conversions for ergonomics. See our docs for a list
of implementors.

## Syntax

When using our DSL, it is recommended to import the
prelude for syntax consistancy.

```rust
use bevy_aoui_widgets::dsl::prelude::*;
```

This provides constants, functions and macros that you
may find useful in streamlining the user experience of
the DSL.

## Context

By default the context is `commands`, which is required.
You can pass down additional context through a tuple syntax.

To pass down the `AssetServer`:

```rust
widget!((commands, asset_server, ..) {
    ..
})
```

This is required for a few widgets like `Shape`.

## Special Syntax

* `extra`: repeatable, inserts an extra custom bundle to the widget.

```rust
sprite! ( commands {
    ..
    extra: ClickHandler {
        buttom: MouseButton::Left,
    },
    extra: SpacialBundle::default(),
    ..
})
```

* `child`: repeatable, collect all `child` entries and insert them as
children of this sprite.

When using macro call syntax with braces `{}`,

```rust
parent!(ctx {
    ..
    child: button! {
        field: ..
        ..
    }
})
```

We automatically pass down the context, transforming the macro invocation into:

```rust
child: button! (ctx {
    field: ...
    ...
})
```

* `texture` and `font`: if `AssetServer` is in context, you can specify a
string literal instead of a handle.

```rust
{
    texture: "ferris.png",
}
```
