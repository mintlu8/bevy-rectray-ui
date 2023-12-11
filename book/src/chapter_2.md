# Core Concepts

## Rectangles

All AoUI entities are in fact rectangles. You can expect
an entity and its descendants to remain consistent in the rectangle's local space
through translation, rotation and scaling. This even applies
to functionalities of widgets like `InputBox` or `Dropdown`

## Anchor, Center, Offset, Rotation, Scale

* `anchor` is the `(0,0)` position on this sprite.

    By default, this sets the `Anchor` component of various bevy components
    that interacts with anchor, like `Sprite` or `Text2d`.

* `parent_anchor` is the corresponding point of `anchor` on the parent,
by default and in idiomatic use cases, it is the same as `anchor`.

* `center` is the center of the sprite's local `rotation` and `scale`.
If specified as `Inherit`, it is always the same as `anchor`.

* `offset` is the offset from the parent's anchor.

    Note: `offset` is not affected by local `rotation` and `scale`.

* `rotation` is the local rotation of the sprite.

* `scale` is the local scaling of the sprite.

## Dimension

Each sprite has a dimension which is crucial for constructing the rectangles.
AoUI offers two modes to determine the size of a sprite:

* Owned
* Copied

When size is Owned, AoUI maintains the size of the rect, and tries to update the size of
its corresponding data, i.e. `custom_size`.

When size is Copied, AoUI will dynamically obtain the size of the rect from `Sprite`, `Text2dLayout`,
`Handle<Image>` etc, and update the size of its rect.

It is recommended to use `Copied` for text in particular,
and place it in a larger rectangle for layout.

Owned dimension supports these modes:

* pixels
* percent
* em
* rem
* margin-px, margin-em, margin-rem: `100% - n px`

## Font Size

We have a font size system, similar to Html and CSS.
`AoUI` propagates an `em` value that controls the size
of fonts in the `Text` widget.

You can use the `FontSize` component to change the size of
`em` on a widget and its children.
Additionally, none-text widget can use `em` or `rem` to align their size with text.

## Dynamic Layouts

When using `Layout`, owned size instead serves as a suggestion for the dimension
of the layout, which will always produce the smallest best fit.
If any child in the layout uses `percent`,
their size will be relative to the parent's original dimension.
Children using `LayoutControl::IgnoreLayout` will use the computed dimension instead.

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