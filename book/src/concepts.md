
# Core Concepts

## Anchor, Center, Offset, Rotation, Scale

* `anchor` is the shared `(0,0)` position on the parent rect and this sprite.

By default, this sets the `Anchor` component of various bevy components
that interacts with anchor, like `Sprite` or `Text2d`.

* `center` is the center of the sprite's local `rotation` and `scale`.
If specified as `None`, it is always the same as `anchor`.

* `offset` is the distance of offset from the parent's anchor.
Note this is not affected by local rotation and scale.

* `rotation` is the local rotation of the sprite.

* `scale` is the local scaling of the sprite.

### Examples

* Docking at `CenterLeft` with no offset:

```rust
Transform2D {
    anchor: Anchor::CenterLeft,
    ..Default::default(),
}
```

* Render a 2d mesh of size 1.0,
and scale it up properly at `CenterLeft`:

```rust
// This is a naive approach, better alternatives exist
Transform2D {
    anchor: Anchor::CenterLeft,
    // We use the center as the underlying mesh's local origin point.
    center: Anchor::Center,
    // Move the sprite to reserve space for centered scaling.
    // Note local scaling does not affect offset.
    offset: Vec2::new(200.0, 0.0),
    // Scale by a factor of 200.
    scale: Vec2::new(200.0, 200.0),
    ..Default::default(),
}
```

* Put a card at `BottomCenter`
and rotate it from its local `BottomCenter`  by 30 degrees:

```rust
Transform2D {
    anchor: Anchor::BottomCenter,
    // Note this is the local, offseted BottomCenter
    center: Anchor::BottomCenter,
    // Move the card up a little bit
    offset: Vec2::new(0.0, 50.0),
    // Rotate right by 30 degrees from origin point center
    rotate: 0.52,
    ..Default::default(),
}
```

## Dimension, Dynamic vs Static, em, rem

Each sprite has a dimension which is crucial to constructed the rectangles containing the anchors.
AoUI offers these setting models:

* Owned
* Copied

When size is Owned, AoUI maintains the size of the rect, and tries to update the size of
its corresponding data, i.e. `custom_size`.

When size is Copied, AoUI will dynamically obtain the size of the rect from `Sprite`, `Text2dLayout`,
`Handle<Image>` etc, and update the size of its rect.

When using `Layout`, owned size instead serve as a suggestion for the dimension
of the layout, which might be dynamic.

Owned dimension currently supports 4 different modes:

* pixels
* percent of parent dimension
* em
* rem

`em` is our relative size system, similar to Html and CSS.
The `AoUI` DOM propagates an `em` value that controls
the size of our text widgets.
You can use the `SetEm` component to change the size of
`em` on a widget and its children.
Additionally, none-text widget can use `em` or `rem` to align their size with text.
