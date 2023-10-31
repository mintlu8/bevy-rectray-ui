use macroex::*;

#[derive(Debug, FromMacro)]
pub enum FlexLayout {
    Span,
    HBox,
    VBox,
    WrapBox,
    Paragraph,
    Grid,
    Table,
    FixedGrid,
    SizedGrid,
    FixedTable,
    FlexTable,
}
