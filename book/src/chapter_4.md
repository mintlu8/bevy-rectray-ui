# DSL Syntax

For every Aoui widget builder struct, there is a macro with a corresponding name.
e.g. `SpriteBuilder` has `sprite!`

The macro has syntax almost identical to struct construction, that returns an `Entity`:

```rust
sprite!(commands {
    sprite: assets.load("image.png"),
    dimension: [20, 20],
    color: color!(red),
})
```

which translates to

```rust
SpriteBuilder {
    name: assets.load("image.png").dinto(),
    dimension: [20, 20].dinto(),
    color: color!(red).dinto(),
    ..Default::default()
}.spawn_with(commands)
```

## Context Piping

In the previous example, `commands` is the context,
which has to be a `TokenTree` (single variable or in parenthesis).

if you need to pass in the asset server `assets`:

```rust
frame!(commands {
    dimension: [20, 20],
    color: color!(red),
})
```

Inside the macro, any field with syntax

```rust
{
    ...
    // Curly Braces `{}` is required
    field_name: macro! {
        ..,
    },
    ...
}
```

will be passed in the context automatically.
This macro invocation will be rewritten as

```rust
{
    ...
    // Brackets `{}` is required
    field_name: macro!(context {
        ..,
    }),
    ...
}
```

which is useful for chaining children.

## Auto Conversion

In the previous example,
`.dinto()` uses our `DslFrom` and `DslInto` trait to provide some much needed
syntax conversions, like `[i32; 2]` to `Vec2`, for ergonomics.
See the docs for a full list of implementors.

## Syntax

When using our DSL, it is recommended to import the
prelude for syntax consistency.

```rust
use bevy_aoui::dsl::prelude::*;
```

This is a massive prelude with various types, consts, macros and functions.
Importing it in a function scope might be ideal.

```rust
pub fn system(mut commands: Commands) {
    use bevy_aoui::dsl::prelude::*;
    ...
}
```

## Child and Extra

The fields `child` and `extra` are special. They are both repeatable.
`extra` inserts an `impl Bundle` into the entity.
`child` inserts an `Entity` as child.

Evaluation order between normal fields and `child`/`extra` is not guaranteed.

Example: Draggable Sprite.

```rust
sprite! (commands {
    ..
    extra: DragBoth,
    extra: SetCursor { 
        flags: EventFlags::Hover|EventFlags::Drag, 
        icon: CursorIcon::Hand,
    },
    extra: DragSnapBack,
    ..
})
```

Example: Hierarchy

```rust
sprite! (commands {
    dimension: size2!(400, 32),
    color: color!(red),
    child: sprite! {
        color: color!(gold),
        dimension: size2!(2, 1 em),
        child: sprite! {
            color: color!(blue),
            dimension: size2!(100%, 100%),
        }
    },
    child: textbox! {
        color: color!(green) * 0.5,
        dimension: size2!(12, 1 em),
        text: "Hello, World!"
    },
});
```
