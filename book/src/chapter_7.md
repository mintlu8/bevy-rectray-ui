# Events

`bevy_aoui` handles cursor event and mouse wheel scrolling.
Keyboard events are usually not handled by `bevy_aoui` unless
you are typing to an inputbox.

In general, `bevy_aoui` won't poll bevy events unless it's certain
it has to. For example, if you don't have an active scrolling handler, we won't poll
the `MouseWheel` event from bevy, meaning you can use it somewhere else.

## Event Model

`bevy_aoui` currently only considers left, mid and right mouse buttons.
Most events are in 2 categories: `CursorFocus` and `CursorAction`
where `CursorFocus` handles a persistent state, like `Hover`,
`CursorAction` handles an instant state, like `Click`.

* CursorFocus
  * Hover: Mouseover without pressing any buttons.
  * *Click: Mouse button `*` is pressed, and nothing is being dragged.
  * *Drag: Something is being dragged using mouse button `*`.
* CursorAction
  * *Down: Mouse button `*` is just pressed.
  * *Click: Mouse button down and up both on the listener.
  * *DragEnd: Dragged ends, for the sprite being dragged.
  * Drop: Dragged ends, on top of another sprite.
  * DoubleClick: Left mouse button clicked twice in a short time on the listener.

    Note `DoubleClick` replaces a `Click` and `DragEnd` if subscribed to.
* CursorClickOutside
  
  Mouse up outside of the sprite's boundary.
* MouseWheel
  
  Mouse wheel is being scrolled while sprite is being hovered.

## Event Listener Groups

Adding an `EventFlags` and a `Hitbox` component allows a sprite to receive events.

Listeners are simplified.

* `Hover` listens for `Hover`,
* `*Click` listens for `*Down`, `*Click` and `*Pressed`
* `*Drag` listens for `*Down`, `*DragEnd` and `*Drag`
* `DoubleClick` listens for `DoubleClick`
* `Drop` listens for `Drop`
* `MouseWheel` listens for mouse wheel scrolling while being hovered.
* `ClickOutside` listens for mouse up outside.

## Handlers

Events allow you to attach a one-shot system to an event.
