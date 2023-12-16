# Layouts

For simplicity we assume the items are laid out horizontally, (i.e. `LeftToRight`),
and rows are placed vertically (i.e. `TopToBottom`), when referring to
"row", "column", "width" or "height".

## Single

Single is a dynamic container that always has the maximum size of its underlying sprites + margin.
If the hierarchy can be reversed, consider using `SizeUnit::MarginPx` as a simpler alternative.

## Compact

Compact is a simple horizontal or vertical layout that is size agnostic.

The height is always the maximum height of its children and the width is always the
sum of the widths of children, plus margin.

## Span

Span is an alignment based fixed sized row layout.

Row alignments of sprites is used to sort items into buckets.

When given a child with custom anchor, the layout panics.

## Paragraph

Paragraph is a multiline version of span.

## Grid

Grid is a layout with unifrom, pre-subdivided cells.

Unlike table, grid supports alternative alignment on incomplete rows.

* `Fixed`: subdivided by count
* `Sized`: subdivided by cell size

## Table

Table is a grid with uneven cells.

* `Dynamic`: subdivide columns by maximum width of children.
* `Porportion`: subdivide columns by porportions.
* `Sized`: subdivide columns by size.
