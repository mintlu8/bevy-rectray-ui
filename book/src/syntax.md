# The AoUI DSL

## Core Syntax

Defining a sprite is as simple as.

```rust
sprite!(commands, [asset_server,] [WidgetName] {
    field1: ..,
    field2: ..,
    ...
})
```

* asset server is required to load assets like sprite with a string literal.

```rust
{
    sprite: "Ferris.png" 
    ...
}
```

* WidgetName is optional, if not specified it is considered `Frame`.

To define a child, use the same syntax:

```rust
sprite!(commands, {
    ...
    child: {
        field1: ...,
        field2: ...,
        ...
    }
    child: RichText {
        field1: ...,
        field2: ...,
        ...
    }
    ...
})
```

## Common Syntax Rules

* Ints are automatically converted to floats if applicable.

* `margin` and `scale` can be written as a single value.

i.e `2` represents `[2, 2]`.

* The default value is **NOT** guaranteed to be specifiable,
it is ideomatic to ignore the field.

* You can use an alternative syntax `{rust expression}` returning `alt_target`.
This syntax is not available if the standard syntax is already `rust expression`.

## AoUI Core

These are present on all AoUI Widgets.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ----- | ----- |
| `center` | center of the item | `Anchor` | `None` (=anchor) | `Anchor` |
| `anchor` | anchor of the item | `Anchor` | `Center` | `Anchor` |
| `offset` | offset from parent | `Size2` | `[0 px, 0 px]` | `Size2` |
| `rotation` | rotation of the item | `Angle` | `0.0` | `f32` |
| `scale` | scale of the item | `[f32; 2]` | `[1.0, 1.0]` | `Vec2` |
| `z` | relative depth | `f32` | `0.0` | `f32` |
| `dimension` | owned size | `Size2` | `Copied` | `Size2` |
| `size` | copied size | `[f32; 2]` | -- | `Vec2` |
| `em` | font size | `SetEm` | `None` | `SetEm` |

* `dimension` and `size` are mutually exclusive.

## Feature Signifiers

These fields determine the existance of certain features:

| name | signifies |
| -- | -- |
| `sprite` | `sprite` |
| `text` | `text` |
| `flex` | `flex layout` |
| `sparse` | `sparse layout` |

## Misc Fields

These fields can be appended to any AoUI components to compose features.
`child` and `extra` can be specified multiple times.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | --- |
| `extra` | extra components | `rust expression` | -- | `impl Bundle` |
| `child`| add children | `{ dsl syntax }` | -- | -- |
| `linebreak` | add linebreak after this | `bool` | -- | `bool` |
| `position` | set position in a `SparseLayout` | `[f32; 2]` | -- | `Vec2` |
| `hitbox` | add a cursor event handler | `HitBox` | -- | `HitBox` |
| `hitbox_size` | set hitbox size | `[f32; 2]` | `[1.0, 1.0]` | `Vec2` |

* `child: Linebreak` inserts a `LinebreakBundle`,
causing a linebreak in a `Container` without taking up space.

* `extra: Transform` adds `Transform` and `BuildTransform`.

* `extra: ScreenSpace` adds `ScreenSpaceTransform` and `GlobalTransfrom`, although auto inserted in most cases.

## AoUI Sprite

These fields act like a `SpriteBundle` in most widgets.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | --- |
| `sprite` | image handle | `rust expression` | `required` | `Handle<Image>` |
| `color` | sprite color | `Color` | `white` | `Color` |
| `rect` | texture rectangle | `Rect` | `None` | `Rect` |
| `flip` | flip sprite  | `[bool; 2]` | `[false, false]` | -- |
| `size` | custom size | `[f32; 2]` | `None` | `Vec2` |

## AoUI Text

These fields act like a `Text2dBundle` in most widgets.

Font size is set by `em`.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `text` | text string | `String` | `required` | `impl ToString` |
| `font` | text font | `rust expression` | `required` | `Handle<Font>` |
| `color` | text color | `Color` | `Black` | `Color` |
| `size` | text bounds | `[f32; 2]` | `infinite` | `Vec2` |

## AoUI Container

The Container is determined by the `widget` name:

`Background`(single), `Conpact`, `HBox`, `VBox`, `Span`, `HSpan`, `VSpan`,
`Paragraph`, `Grid`, `Table`

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `margin` | margin between cells | `Size2` | `required` | `Size2` |
| `direction` | direction items are laid out | `FlexDir` | `LeftToRight` | `FlexDir` |
| `stack` | direction lines are laid out | `FlexDir` | `TopToBottom` | `FlexDir` |
| `stretch` | extend margin to fit the span | `bool` | `false` | `bool` |
| `row_dir` | direction items are laid out | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_dir` | direction lines are laid out | `FlexDir` | `TopToBottom` | `FlexDir` |
| `row_align` | alignement of individual line | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_align` | alignement of all lines | `FlexDir` | `LeftToRight` | `FlexDir` |
| `stretch` | extend `cell_size` to fit the span | `bool` | `false` | `bool` |
| `cell_count` | number of cells | `[u32; 2]` | `required` | `UVec2` |
| `cell_size` | size of a single cell | `Size2` | `required` | `Size2` |
| `columns` | number of columns | `usize` | `required` | `usize` |
| `columns` | proportion of columns | `[f32]` | `required` | `--` |
| `column_proportion` | proportion of columns | `[f32]` | `required` | `Vec<f32>` |
| `column_sizes` | sizes of columns | `[f32]` | `required` | `Vec<f32>` |

Alternatively you can use widget `Container` and set

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `Layout` | Layout | `rust expression` | `required` | `Layout` |
| `margin` | margin between cells | `Size2` | `required` | `Size2` |
