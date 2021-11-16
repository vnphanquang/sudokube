#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CellRelation {
    Same,    // same cell
    SubGrid, // cell in same sub_grid
    Row,     // cell in same row
    Col,     // cell in same column
}

#[derive(Debug, Clone)]
pub enum SudokuEventType {
    SetCellValue,
}

impl std::fmt::Display for SudokuEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
