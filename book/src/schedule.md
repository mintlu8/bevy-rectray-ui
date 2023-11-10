# The AoUI DOM and Scheduling

By default the AoUI Schedule has the following `SystemSets`:

* AoUISyncRead: Gather information from external resources
  * Get dimension from `Sprite` or `Image`
  * Get dimension from `Text2dLayout`

* AoUIUpdateDom: Propagate information through the `AoUI` DOM.
  * Generate `RotatedRect` for general purpose.
  * Generate `BuildGlobal` for `AoUI` widgets.
  * Generate `Transform` for integrating bevy children.
  * Update `dimension` and `em`

Currently the DOM root is always a rectangle representing the window.

For example, a sprite with no parent and has `Anchor::TopLeft`
should be placed on the topleft of the window, with the default 2D camera.

This step is marked by the `AoUI` marker component.
You can use `compute_aoui_transforms` with custom queries for alternative build order.

* AoUISyncWrite: Synchronize information owned by the `AoUI` DOM.
  * Update `transform` with `BuildTransform`
  * Update `custom_size` of a sprite.
  * Update `size` of `FontStyle`.

* Run Bevy's `propagate_transoform`.

* AoUIFinalize: Finalize by updating `GlobalTransform`.
