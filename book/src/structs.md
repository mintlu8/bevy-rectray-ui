# Structs

We support a few pseudo-classes that accepts various froms of data.

## Size2

Syntax: `[x, y]` or `[x em, y px]`

Where `px`, `em`, `rem`, `%` denotes relative size.

The default is `px`.

## Rect

Syntax: `[x, y, w, h]`

This is different from the bevy `[min, max]` representation.

## Anchor

Syntax: `TopLeft`, `Center`, `[0.4, 0.5]`

Where `[0.4, 0.5]` is a custom anchor.

## Color

Supported Syntax:

* Bracketed numbers: `[0.3, 0.72, 0.98]`, `[124, 54, 87, 255]`
* Repeat syntax: `[0.3; 3]`, `[0.7; 4]`
* Hex strings: `"AABBCC"`, `"AABBCCFF"`, `"#AABBCC"`, `"#AABBCCFF"`
* Hex number literals: `0xAABBCC`, `0xAABBCCFF`
* CSS color names: `Red`, `Blue`
* TailwindCSS color names: `Red100`, `Sky400`

Note `[0, 1, 0, 1]` means `[0.0, 1.0, 0.0, 1.0]`
since all values are in `0..=1`.

See [macroex_extras](https://docs.rs/macroex-extras/latest/macroex_extras/struct.Rgba.html).

## Angle

Supported Syntax:

* `4.2`
* `pi`
* `45 degrees` or `45 deg`
* `2.1 radians` or `2.1 rad`
* `pi / 2`
* `2 pi`
* `2/3 pi`
* `[2 pi]`

See [macroex_extras](https://docs.rs/macroex-extras/latest/macroex_extras/struct.Angle.html).

## SetEm

* `12` or `12 px`

set em to 12px.

* `1.2 em`

set em to 1.2x of parent's em.

* `1.4 rem`

set em to 1.4 rem.
