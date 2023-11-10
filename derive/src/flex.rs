use macroex::*;

#[derive(Debug, FromMacro)]
pub enum Layout {
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
