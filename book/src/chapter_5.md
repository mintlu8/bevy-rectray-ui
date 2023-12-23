# Primitive Widgets

`bevy_aoui` has a few primitive widgets for `Aoui`
corresponding to bevy2d's primitives.
They only display something and have no additional behaviors.

| name | bundle | widget | macro |
| ---- | ------ | ------ | ----- |
| Frame | `AouiBundle` | `FrameBuilder` | `frame!` |
| Sprite | `AouiSpriteBundle` | `SpriteBuilder` | `sprite!` |
| Rectangle | -- | `RectangleBuilder` | `rectangle!` |
| Text | `AouiText2dBundle` | `TextBoxBuilder` | `textbox!` |
| Atlas | `AouiAtlasSpriteBundle` | `AtlasSpriteBuilder` | `atlas!` |
| MaterialSprite | -- | `MaterialSpriteBuilder` | `material_sprite!` |
| MaterialMesh | `AouiMaterialMesh2dBundle` | `MaterialMeshBuilder` | `material_mesh!` |

When using macros it might be difficult to find documents for it from your editor.
The easiest way is to look for its corresponding builder struct in
[bevy_aoui::dsl::builders].

## Frame

Frames is an empty rectangle.

## Sprite

A simple sprite rendering a texture.

## Rectangle

A simpler sprite with uniform color.

## Text

A 2d text.

## Atlas

A texture atlas sprite.

## MaterialSprite

A rectangle using a custom material

## MaterialMesh

A 2d mesh using a custom material,
bevy_aoui assumes the mesh is in range `-0.5..=0.5`
and scales accordingly.
