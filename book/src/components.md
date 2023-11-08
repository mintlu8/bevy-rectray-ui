# Components

## AoUI

`AoUI` is a marker component that enables our render pipeline.

## Anchors

`Anchors` provides anchor and center of a sprite.

- `anchor` is used to inherit the parent's position, rotation and scale.
This also governs the anchor of `Sprite` and `Text2d`.

- `center` is used to apply the sprite's own rotation and scale.

If `center` is None, `center` is always the same as `anchor`.

## Dimension

`Dimension` provides the dimension of the sprite.

- `dim` determines the size of the sprite.
  - `DimensionSize::Copied` means the size is synced with
a sprite's `custom_size` a texture's `size` or a text's `LayoutInfo::size`.
  - `DimensionSize::Owned` means we supply the size with
our relative size system and update fields like `custom_size` accordingly.

- `set_em` modifies our relative size system.
- `size` and `em` these are set dynamically at runtime.

## Transform2D

- `offset` offset from parent's anchor.
- `rotation` rotation around the sprite's center.
- `scale` scale around the sprite's center.
- `z` z offset from parent.

The Z formula is `parent_z + child_z + eps * 8.0`,
so you can ignore it most of the time.

## RotatedRect

The definitive output of AoUI, used by components like `HitBox` or `IntoTransform`

## ScreenSpaceTransform

Cache for `GlobalTransform`

## IntoTransform

Convert `RotatedRect` into a parentless `Transform` on `Anchors::center`.

This is useful for integrating with native bevy (especially 3D) objects.

Keep in mind you should only put this on leaf nodes.

## HitBox

Provides mouse detection for `RotatedRect`.

- `shape`: shape of the hitbox.
- `size`: scale of the hitbox.
- `flag`: bitmask for events in our event system.

Check out `EventPipe` for how to use our simple event system.

## FlexControl

Controls the behavior of being inserted into a `FlexContainer`.
Currently exclusively controls linebreaks.

- `FlexControl::Linebreak`
Forces a linebreak in a supported `FlexContainer` **AFTER** this sprite.

- `FlexControl::LinebreakMarker`
Forces a linebreak in a supported `FlexContainer` without taking up space
in the current line.
The sprite's width or height might be used in some layouts to determine line gap.
Using the sprite for rendering is unspecified behavior.

## FlexContainer

A container that lays out children sequentially. Given a ordered list of
`Children`, these containers should work out of the box.

### `Span`

A single line container that respects its contents's anchors.

- `direction`: The order children are laid out.
This does not affect anchors.
  
For a `LeftToRight` layout `1,2,3  4,5,6  7,8,9`

The `RightToLeft` layout is `3,2,1  6,5,4  9,8,7`

### `Paragraph`

A multiline wrapping layout that respects its contents's anchors.

- `direction`: The order children are laid out in its `Span`s.
- `alignment`: Where spans are places relative to the parent box.
- `wrap_to`: Direction of wrapping.

### `SizedGrid`

### `FixedGrid`

### `SizedTable`

### `FlexTable`

## SparseContainer

A container that hosts a coordinate system, or a "scene".
Each children needs a `SparseIndex` to be inserted correctly.
