use macroex::*;

#[derive(Debug, FromMacro)]
pub enum FlexLayout {
    Span,
    HBox,
    VBox,
    Paragraph,
    Grid,
    Table,
}

#[derive(Debug, FromMacro)]
pub enum SparseLayout {
    Rectangles,
    Isometric,
    HexGrid,
}
