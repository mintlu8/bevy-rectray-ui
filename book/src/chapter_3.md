# Core Components

These are the components used in the core pipeline.

## AoUI

`AoUI` is a marker component that enables our render pipeline.

## Transform2D

- `anchor` is used to inherit the parent's position, rotation and scale.
- `center` is used to apply the sprite's own rotation and scale.
- `offset` is the offset from parent's anchor.
- `rotation` is the rotation around the sprite's center.
- `scale` is the scaling from the sprite's center.
- `z` is the z offset from parent.

The Z formula is `parent_z + child_z` if child_z is not 0,
otherwise it's `parent_z.next_after()`,
so you can ignore it most of the time.

## Dimension

`Dimension` provides the dimension of the sprite.

- `dim` determines the size of the sprite.
- `set_em` modifies the font size.
- `size` and `em` are set dynamically at runtime.

## RotatedRect

A 2D rectangle with a `z` value,
This is the canonical output of `AoUI`.
Used for transform generation, mesh generation,
cursor detection, local space transformation, etc.

## BuildTransform

Marker for generating `GlobalTransform` with `RotatedRect`.

## HitBox

Provides cursor detection for `RotatedRect`.

- `shape`: shape of the hitbox.
- `size`: scale of the hitbox.
