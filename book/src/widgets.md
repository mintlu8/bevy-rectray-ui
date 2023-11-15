# Widgets (draft)

Widgets provide functionalites that requires external crates.

## Rich Text

Build a rich text component at compile time using markdown + html like syntax.

This builds a Layout::Paragraph.

Example:

```rust
sprite!(commands, RichText {
    segment: r#"this text is <red>red</red> and **bold**.
    <center/>Awesome, right?"#,
    segment: CheckBox {
        ...
    },
    segment: Linebreak,
    segment: Right,
    segment: "Say yes."
    ...
})
```
