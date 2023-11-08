# The AoUI DSL

## Core Syntax

Defining a sprite is as simple as.

```rust
sprite! {
    field1: ..,
    field2: ..,
    ...
}
```

To define a child, use the same syntax:

```rust
sprite! {
    ...
    child: {
        field1: ..,
        field2: ..,
        ...
    }
    ...
}
```

## Common Syntax Rules

* We automatically do int to float conversion if applicable.

* `margin`, `scale`, and similar likely positive `Vec2` values
can be written as a single value.

i.e `2` represents `[2, 2]`.

* The default value is **NOT** guaranteed to be specifiable, if you
want the default value, ignore the field.

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
| `z` | relative depth | `f32` | `0.0` (or eps) | `f32` |
| `dimension` | owned size | `Size2` | `Copied` | `Size2` |
| `size` | copied size | `[f32; 2]` | -- | `Vec2` |
| `em` | relative size | `SetEm` | `None` | `SetEm` |

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
| `widget` | name of the widget | `Ident` | -- | -- |
| `extra` | extra components | `rust expression` | -- | `impl Bundle` |
| `child`| add children | `{ dsl syntax }` | -- | -- |
| `background` | see below | `{ dsl syntax }` | -- | -- |
| `linebreak` | add linebreak after this | `bool` | -- | `bool` |
| `position` | set position in a `SparseLayout` | `[f32; 2]` | -- | `Vec2` |
| `build_transform` | build a transform component | `bool` | -- | `bool` |
| `hitbox` | add a cursor event handler | `HitBox` | -- | `HitBox` |
| `hitbox_size` | set hitbox size | `[f32; 2]` | `[1.0, 1.0]` | `Vec2` |
| `hitbox_flag` | set hitbix flag | `u32` | `0` (All) | `u32` |

* `child: Linebreak` inserts a `LinebreakBundle`,
causing a linebreak in a `FlexContainer` without taking up space.

* `background` is a special children that
has default size `[100 %, 100 %]`
and default relative z `-1 eps`

* `widget` will always disable `AoUI Sprite` and `AoUI Text`.

## AoUI Sprite

These fields spawn a widget analogous to `SpriteBundle`.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | --- |
| `sprite` | image handle | `rust expression` | `required` | `Handle<Image>` |
| `color` | sprite color | `Color` | `white` | `Color` |
| `rect` | texture rectangle | `Rect` | `None` | `Rect` |
| `flip` | sprite flips | `[f32; 2]` | `[false, false]` | -- |
| `size` | custom size | `[f32; 2]` | `None` | `Vec2` |

## AoUI Text

These fields spawn a widget analogous to a `Text2dBundle`.

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `text` | text string | `String` | `required` | `impl ToString` |
| `font` | text font | `rust expression` | `required` | `Handle<Font>` |
| `color` | text color | `Color` | `Black` | `Color` |
| `size` | text bounds | `[f32; 2]` | `infinite` | `Vec2` |

## AoUI FlexContainer

Common fields:

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `flex` | FlexLayout | `FlexLayout` | `required` | `FlexLayout` |
| `margin` | margin between cells | `SizeLike` | `required` | `Size2` |

Below are supported FlexLayouts.

Using `alt_target` on `FlexLayout` will bypass everything below.

### Span, HBox and VBox

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `direction` | direction items are laid out | `FlexDir` | -- | `FlexDir` |
| `stretch` | extend margin to fit the span | `bool` | `false` | `bool` |

* `HBox` sets direction to `LeftToRight`.
* `VBox` sets direction to `TopToBottom`.

### Paragraph

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `direction` | direction items are laid out | `FlexDir` | `LeftToRight` | `FlexDir` |
| `stack` | direction lines are laid out | `FlexDir` | `TopToBottom` | `FlexDir` |
| `stretch` | extend margin to fit the span | `bool` | `false` | `bool` |

* `direction` and `stack` either both take default value or both be specified.

### Grid

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `cell_count` | number of cells | `[u32; 2]` | `required` | `UVec2` |
| `cell_size` | size of a single cell | `Size2` | `required` | `Size2` |
| `row_dir` | direction items are laid out | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_dir` | direction lines are laid out | `FlexDir` | `TopToBottom` | `FlexDir` |
| `row_align` | alignement of individual line | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_align` | alignement of all lines | `FlexDir` | `LeftToRight` | `FlexDir` |
| `stretch` | extend `cell_size` to fit the span | `bool` | `false` | `bool` |

* `cell_count` and `cell_size` are mutrally exclusive

### Table

| field | description | syntax | default | alt_target |
| ----- | ----- | ----- | ---- | ---- |
| `cell_count` | number of cells | `[u32; 2]` | `required` | `UVec2` |
| `cell_size` | size of a single cell | `Size2` | `required` | `Size2` |
| `row_dir` | direction items are laid out | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_dir` | direction lines are laid out | `FlexDir` | `TopToBottom` | `FlexDir` |
| `row_align` | alignement of individual line | `FlexDir` | `LeftToRight` | `FlexDir` |
| `column_align` | alignement of all lines | `FlexDir` | `LeftToRight` | `FlexDir` |
| `stretch` | extend `cell_size` to fit the span | `bool` | `false` | `bool` |

## AoUI SparseContainer

| `field` | `syntax` | `default` |
| ---- | ---- | ---- |
| `scene` | layout of the scene | `SparseLayout` | `required` | `SparseLayout` |
| `cell_size` | size of a single cell | `Size2` | `required` | `Size2` |
| `x_axis` | x_axis of the layout | `Ident` | `required` | -- |
| `y_axis` | y_axis of the layout | `Ident` | `required` | -- |
| `origin` | local space `[0, 0]` in child space | `[f32; 2]` | `[0, 0]` | -- |
| `cell_rect` | local rect based anchors | `Rect` | best fit | `Rect` |
| `scene_transform` | transforms scene position | `Affine2` | `identity` |

* Using `alt_target` on `scene` will bypass `cell_size`, `x_axis` and `y_axis`.
