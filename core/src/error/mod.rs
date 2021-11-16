#[derive(Debug, Clone)]
pub enum SudokuError {
    InvalidCellValue(String),
    CellCoordinateOutOfBound(String),
}
