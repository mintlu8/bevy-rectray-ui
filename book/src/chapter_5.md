# Primitives

There are the primitive widgets for `AoUI`, they display something and has no additional behaviors.
Some of them corresponds to bundles in `bevy_sprite` and `bevy_text`.

| name | bundle | widget | macro |
| ---- | ------ | ------ | ----- |
| Frame | `AoUIBundle` | `FrameBuilder` | `frame!` |
| Sprite | `AoUISpriteBundle` | `SpriteBuilder` | `sprite!` |
| Rectangle | -- | `RectangleBuilder` | `rectangle!` |
| Text | `AoUIText2dBundle` | `TextBoxBuilder` | `textbox!` |
| TextureAtlasSprite | `AoUIAtlasSpriteBundle` | `AtlasSpriteBuilder` | `atlas!` |
| MaterialRectangle | -- | `MaterialRectangleBuilder` | `material_rect!` |
| MaterialMesh | `AoUIMaterialMesh2dBundle` | `MaterialMeshBuilder` | `material_mesh!` |

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

## MaterialRectangle

A rectangle using a custom material

## MaterialMesh

A 2d mesh using a custom material,
the mesh should be in range `-0.5..=0.5`
and let `bevy_aoui` handle scaling.
