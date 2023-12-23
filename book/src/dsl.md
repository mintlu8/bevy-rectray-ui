# DSL

`bevy_aoui` provides a simple DSL syntax for UI generation.

For every Aoui widget struct, there is a macro with a corresponding name
`e.g.` `SpriteBuilder` has `sprite!`

The macro has syntax almost identical to struct construction:

```rust
widget!((commands) {
    name: "Hello Aoui!",
    dimension: [20, 20],
    color: color!(red),
})
```

which translates to

```rust
Widget {
    name: "Hello Aoui!".dinto(),
    dimension: [20, 20].dinto(),
    color: color!(red).dinto(),
    ..Default::default()
}.spawn_with(commands)
```

This returns an `Entity`.

`dinto` uses our `DslInto` trait to provide some nice
syntax conversions for ergonomics. See our docs for a list
of implementors.

## Syntax

When using our DSL, it is recommended to import the
prelude for syntax consistancy.

```rust
use bevy_aoui::dsl::prelude::*;
```

This provides constants, functions and macros that you
may find useful in streamlining the user experience of
the DSL.

## Context

At the root level of the macro, you need to pass in a context,
wrapped in parenthesis.

By default the context is `commands`, which is required.
You can pass down additional context through a tuple syntax.

To pass in the `AssetServer`:

```rust
widget!((commands, asset_server, ..) {
    ..
})
```

This is required by a few widgets like `Shape`.

### Context Propagation

All fields using this exact syntax

```rust
field_name: macro_name! { .. },
```

will be passed in the context, this becomes

```rust
field_name: macro_name! ((command, ..) { .. })
```

This is especially useful for spawining children,
as you can avoid writing `context` multiple times.

## Special Syntax

* `extra`: repeatable, insert a custom bundle to the widget.

```rust
sprite! ( (commands) {
    ..
    extra: SpacialBundle::default(),
    extra: DraggableMarker,
    extra: handler!{LeftDrag => 
        fn handle_drag(mut query: Query<&mut Transform2D, With<DraggableMarker>>, res: Res<CursorState>) {
            query.single_mut().offset.edit_raw(|x| *x = res.cursor_position())
        }
    },
    ..
})
```

* `child`: repeatable, given an `Entity`, insert them as children.

```rust
inputbox! (commands {
    dimension: size2!(400, 32),
    color: color!(red),
    child: shape! {
        shape: Shapes::Rectangle,
        fill: color!(gold),
        dimension: size2!(2, 1 em),
    },
    child: shape! {
        shape: Shapes::Rectangle,
        fill: color!(green) * 0.5,
        dimension: size2!(12, 1 em),
    },
});
```
