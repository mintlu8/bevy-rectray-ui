# Widgets

We automatically do int to float conversion

All values are `TokenTree`s (`tt` in `#[macro_rules]` land)

The default value is **NOT** guaranteed to be specifiable, if you
want the default value, just ignore the field.

## AoUI Core

| `field` | `syntax` | `default` |
| ----- | ----- | ----- |
| `center` | `TopLeft`, `[-0.3, 0.4]` | `=anchor` |
| `anchor` | `TopLeft`, `[-0.3, 0.4]` | `Center` |
| `offset` | `[20, 40]` | `[0, 0]` |
| `rotation` | `25`, `[3 pi/ 2]`, `[90 degrees]` | `0` |
| `scale` | `[2, 2]` | `[1, 1]` |
| `dim` | `[100, 400]`, `parent` | `[0, 0]` or `inherit` |
| `disable` | `rotation`, `rotation_scale` | `none` |

## Signifiers

These fields determine the existance of certain features:

| name | signifies |
| -- | -- |
| `sprite` | `sprite` |
| `text` | `text` |
| `flex` | `flex layout` |
| `sparse` | `sparse layout` |

## AoUI Plugins

These fields can be appended to any AoUI components to compose features.
`child` can be specified multiple times.

| `field` | `syntax` | `default` | `usage` |
| ----- | ----- | ----- | ---- |
| `extras` | `[rust expressions,]`| `[]` | add more bundles |
| `linebreak` | `break`, `self` | `no` | line breaks in `FlexContainer`s |
| `position` | `[1, 2]` | `[0, 0]` | set position in `SparseContainer`s |
| `integration` | `true` | `false` | add a IntoTransformBundle |
| `hitbox` | `[square [0.4, 0.1]]` | `no` | add a hitbox bundle |
| `child`| `{AoUI Widget}`, `(Entity)`| `empty array` | add a child |

## AoUI Sprite

| `field` | `syntax` | `default` |
| ----- | ----- | ----- |
| `sprite` | `"🦀ferris.png"` | `required` |
| `color` | [`Color Syntax`](https://docs.rs/colorthis/latest/colorthis/#color-syntax)| `White` |
| `rect` | `[0, 0, 100, 100]` | `None` |
| `flip` | `[false, true]` | `[false, false]` |

Inherited Fields

| `field` | `as` |
| ---- | ---- |
| `anchor` | `anchor` |
| `dim` | `custom_size` |

## AoUI FlexContainer

Layout Types:

| `FlexLayout` | `implies` |
| -- | -- |
| `Span` | `--` |
| `HBox` | `Span, direction: LeftToRight` |
| `VBox` | `Span, direction: TopToBottom` |
| `WrapBox` | `--` |
| `Paragraph` | `WrapBox, direction: LeftToRight, wrap_to: Down` |
| `Grid` | `--` |
| `Table` | `--` |
| `FixedGrid` | `Grid, cell_count: required` |
| `SizedGrid` | `Grid, cell_size: required` |
| `FixedTable` | `Table, columns: required(array)` |
| `FlexTable` | `Table, columns: required(int)` |

Components:

| `field` | `syntax` | `default` | `implies` |
| ---- | ---- | ---- | ----- |
| `flex` | `FlexLayout` | `--` | `required` |
| `margin` | `4`, `[4, 2]` | `[0, 0]` | `--` |
| `direcion` | `LeftToRight` | `unused or panic` | `Span` or `Wrapbox` |
| `wrap_to` | `Down` | `Down` or `Right` | `WrapBox` |
| `major` | `LeftToRight` | `unused or panic` | `Table` or `Grid` |
| `minor` | `LeftToRight` | `T2B or L2R` | `Table` or `Grid` |
| `cell_count` | `[4, 5]` | `unused or panic` | `FixedGrid` |
| `cell_size` | `[40, 50]` | `unused or panic` | `SizedGrid` |
| `stretch` | `true` | `false` | `SizedGrid` |
| `alignment` | `TopLeft` | `Center` | `--` |
| `pad_align` | `TopLeft` | `Center` | `SizedGrid` or `FlexTable` |
| `columns` | `4, [0.2, 0.5, 0.8]` | `unused or panic` | `Table` |

## AoUI SparseContainer

Layout Types:

| `FlexLayout` | `implies` |
| -- | -- |
| `Rectangles` | `--` |
| `Isometric` | `--` |
| `Hex` | `--` |

Components:

| `field` | `syntax` | `default` |
| ---- | ---- | ---- |
| `sparse` | `SparseLayout` | `required` |
| `size` | `40`, `[40, 40]` | `panic` |
| `x` | `Left` | `panic` |
| `y` | `TopLeft` | `panic` |
| `child_rect` | `[0, 0, 100, 100]` | `size`d rect with `center` at `anchor` |
| `origin` | `[0, 0]` | `[0, 0]` |
| `scene_transform` | `[Mat2]` | `Identity` |

## AoUI Text

| `field` | `syntax` | `default` |
| ----- | ----- | ----- |
| `text` | `"string"`, `r#"raw_string"#` | `required` |
| `font` | `"OpenSans.png"` | `panic` |
| `color` | [`Color Syntax`](https://docs.rs/colorthis/latest/colorthis/#color-syntax)| `panic` |
| `text_bounds` | `[200, 200]` | `infinite` |

Inherited Fields

| `field` | `as` |
| ---- | ---- |
| `anchor` | `anchor` |

## AoUI RichText

This requires a `FlexContainer` setup and compiles to children.

| `field` | `syntax` | `default` |
| ----- | ----- | ----- |
| `format` | `"string"`, `r#"raw_string"#` | `required` |
| `font` | `"OpenSans.png"` | `panic` |
| `color` | [`Color Syntax`](https://docs.rs/colorthis/latest/colorthis/#color-syntax)| `panic` |
| `text-anchor` | `TopLeft` | `TopLeft` |
