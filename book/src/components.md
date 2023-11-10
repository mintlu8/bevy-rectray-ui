# Components

## AoUI

`AoUI` is a marker component that enables our render pipeline.

## Transform2D

- `anchor` is used to inherit the parent's position, rotation and scale.
- `center` is used to apply the sprite's own rotation and scale.
- `offset` is the offset from parent's anchor.
- `rotation` is the rotation around the sprite's center.
- `scale` is the scaling from the sprite's center.
- `z` is the z offset from parent.

The Z formula is `parent_z + child_z + eps * 8.0`,
so you can ignore it most of the time.

## Dimension

`Dimension` provides the dimension of the sprite.

- `dim` determines the size of the sprite.
- `set_em` modifies the relative font size.
- `size` and `em` are set dynamically at runtime.

## RotatedRect

A rectangle with center, dimension, rotation and z.

The main output of `AoUI`,
used for transform generation and click detection.

## BuildGlobal

Marker for generating [`GlobalTransform`] directly.

## BuildTransform

Marker for generating [`Transform`].

Note this should only be placed on leaf nodes.

## HitBox

Provides cursor detection for `RotatedRect`.

- `shape`: shape of the hitbox.
- `size`: scale of the hitbox.

## Container

A container that lays out children sequentially.

See `Layout` for more information.
