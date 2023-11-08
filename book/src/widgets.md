# Widgets (draft)

Widgets provide functionalites that requires external crates.

## Rich Text

Build a rich text component at compile time using html like syntax.

Example:

```rust
sprite! {
    widget: "RichText",
    text: r#"this is <Red>red</Red> **bold** text.
    Awesome, right? {
        widget: CheckBox,
        ...
    }"#,
    ...
}
```
