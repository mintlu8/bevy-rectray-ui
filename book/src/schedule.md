# The Aoui Tree and Scheduling

By default the Aoui Schedule has the following `SystemSets`:

* AouiLoadInput: Gather information from external resources
  * Get dimension from `Sprite` or `Image`
  * Get dimension from `Text2dLayout`

* AouiTreeUpdate: Propagate information through the `Aoui` tree.
  * Generate `RotatedRect` for general purpose.
  * Generate `BuildTransform` for `Aoui` widgets.
  * Generate `Transform` for integrating bevy children.
  * Update `dimension` and `em`

The root is always a rectangle representing the window.

For example, a sprite with no parent and has `Anchor::TopLeft`
should be placed on the topleft of the window, with the default 2D camera.

This step is marked by the `Aoui` marker component.
You can use `compute_aoui_transforms` with custom queries for alternative build order.

* AouiStoreOutput: Synchronize information owned by the `Aoui` tree.
  * Update `transform` with `BuildTransform`
  * Update `custom_size` of a sprite.
  * Update `size` of `FontStyle`.

* Run Bevy's `propagate_transoform`.

* AouiFinalize: Finalize by updating `GlobalTransform`.
