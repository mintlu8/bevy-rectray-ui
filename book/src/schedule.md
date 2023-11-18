# The AoUI Tree and Scheduling

By default the AoUI Schedule has the following `SystemSets`:

* AoUILoadInput: Gather information from external resources
  * Get dimension from `Sprite` or `Image`
  * Get dimension from `Text2dLayout`

* AoUITreeUpdate: Propagate information through the `AoUI` tree.
  * Generate `RotatedRect` for general purpose.
  * Generate `BuildGlobal` for `AoUI` widgets.
  * Generate `Transform` for integrating bevy children.
  * Update `dimension` and `em`

The root is always a rectangle representing the window.

For example, a sprite with no parent and has `Anchor::TopLeft`
should be placed on the topleft of the window, with the default 2D camera.

This step is marked by the `AoUI` marker component.
You can use `compute_aoui_transforms` with custom queries for alternative build order.

* AoUIStoreOutput: Synchronize information owned by the `AoUI` tree.
  * Update `transform` with `BuildTransform`
  * Update `custom_size` of a sprite.
  * Update `size` of `FontStyle`.

* Run Bevy's `propagate_transoform`.

* AoUIFinalize: Finalize by updating `GlobalTransform`.
