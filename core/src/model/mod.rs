use event_emitter_rs::EventEmitter;
use serde::{Deserialize, Serialize};
use serde_json;

use super::error::SudokuError;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate(pub u8, pub u8);

impl Coordinate {
    pub fn row(&self) -> u8 {
        self.0
    }
    pub fn col(&self) -> u8 {
        self.1
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub value: Option<u8>,
}

impl Cell {
    fn blank() -> Cell {
        Cell {
            coordinate: Coordinate(0, 0),
            value: Option::None,
        }
    }

    pub fn sub_grid(&self, grid_size: u8) -> (usize, usize, Coordinate) {
        let sub_grid_size = (grid_size as f64).sqrt() as u8;
        let Coordinate(x, y) = self.coordinate;
        let sub_grid_x = x / sub_grid_size;
        let sub_grid_y = y / sub_grid_size;

        let sub_grid_index = (sub_grid_x * sub_grid_size + sub_grid_y) as usize;
        let cell_index_in_sub_grid = ((x % sub_grid_size * 3) + (y % sub_grid_size)) as usize;

        (
            sub_grid_index,
            cell_index_in_sub_grid,
            Coordinate(sub_grid_x, sub_grid_y),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct CellRow<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub cells: [Cell; N],
}

impl<const N: usize> CellRow<N> {
    fn blank() -> CellRow<N> {
        CellRow {
            cells: [Cell::blank(); N],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SubGrid<const N: usize> {
    pub coordinate: Coordinate,
    #[serde(with = "serde_arrays")]
    pub cells: [Coordinate; N],
}

impl<const N: usize> SubGrid<N> {
    fn blank() -> SubGrid<N> {
        SubGrid {
            coordinate: Coordinate(0, 0),
            cells: [Coordinate(0, 0); N],
        }
    }

    pub fn has(&self, cell: &Cell) -> bool {
        let (_, index_in_sub_grid, sub_grid_coordinate) = cell.sub_grid(N as u8);

        if sub_grid_coordinate != self.coordinate {
            return false;
        }

        let sub_grid_cell_coor = self.cells[index_in_sub_grid];
        sub_grid_cell_coor == cell.coordinate
    }
}

#[derive(Serialize, Deserialize)]
pub struct Grid<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub rows: [CellRow<N>; N],
    #[serde(with = "serde_arrays")]
    pub sub_grids: [SubGrid<N>; N],

    #[serde(skip_serializing, skip_deserializing)]
    event_emitter: EventEmitter,
}

impl<const N: usize> Grid<N> {
    pub fn new() -> Grid<N> {
        let mut grid = Grid {
            rows: [CellRow::blank(); N],
            sub_grids: [SubGrid::blank(); N],
            event_emitter: EventEmitter::new(),
        };

        for x in 0..grid.rows.len() {
            let row = &mut grid.rows[x];
            for y in 0..row.cells.len() {
                let mut cell = &mut row.cells[y];
                cell.coordinate = Coordinate(x as u8, y as u8);

                let (sub_grid_index, cell_index_in_sub_grid, Coordinate(sub_grid_x, sub_grid_y)) =
                    cell.sub_grid(N as u8);
                let mut sub_grid = &mut grid.sub_grids[sub_grid_index];
                sub_grid.coordinate = Coordinate(sub_grid_x, sub_grid_y);
                sub_grid.cells[cell_index_in_sub_grid] = cell.coordinate;
            }
        }

        grid
    }

    pub fn from_json(serialized: String) -> Grid<N> {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn len(&self) -> u8 {
        N as u8
    }

    pub fn get_sub_grid(&self, cell: &Cell) -> &SubGrid<N> {
        let (sub_grid_index, _, _) = cell.sub_grid(N as u8);
        &self.sub_grids[sub_grid_index]
    }

    pub fn get_cell(&self, coordinate: Coordinate) -> &Cell {
        let Coordinate(x, y) = coordinate;
        &self.rows[x as usize].cells[y as usize]
    }

    pub fn get_cells_relation(&self, cell1: &Cell, cell2: &Cell) -> Vec<CellRelation> {
        if cell1 == cell2 {
            return vec![CellRelation::Same];
        }

        let Coordinate(x_1, y_1) = cell1.coordinate;
        let Coordinate(x_2, y_2) = cell2.coordinate;
        let mut relations: Vec<CellRelation> = Vec::new();
        if x_1 == x_2 {
            relations.push(CellRelation::Row)
        }
        if y_1 == y_2 {
            relations.push(CellRelation::Col)
        }

        if self.get_sub_grid(cell1).has(cell2) {
            relations.push(CellRelation::SubGrid)
        }

        relations
    }

    fn get_cell_mut(&mut self, coordinate: Coordinate) -> &mut Cell {
        let Coordinate(x, y) = coordinate;
        let row = &mut self.rows[x as usize];
        &mut row.cells[y as usize]
    }

    pub fn set_cell_value(&mut self, x: u8, y: u8, value: Option<u8>) -> Result<(), SudokuError> {
        if usize::from(x) >= N || usize::from(y) >= N {
            return Err(SudokuError::CellCoordinateOutOfBound(format!(
                "Coordinate (x: {}, y: {}) is invalid. x,y must be in range [0, {})",
                x, y, N
            )));
        }

        if usize::from(value.unwrap_or_default()) >= N {
            return Err(SudokuError::InvalidCellValue(format!(
                "Cell value must be in range [0, {})",
                N
            )));
        }

        let cell = self.get_cell_mut(Coordinate(x, y));
        cell.value = value;

        self.event_emitter.emit(
            &SudokuEventType::SetCellValue.to_string(),
            (Coordinate(x, y), value),
        );

        Ok(())
    }

    pub fn on<F, T>(&mut self, event: SudokuEventType, callback: F) -> String
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        self.event_emitter.on(&event.to_string(), callback)
    }

    pub fn off(&mut self, id: &str) -> () {
        self.event_emitter.remove_listener(id);
    }
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CellRelation {
    Same,    // same cell
    SubGrid, // cell in same sub_grid
    Row,     // cell in same row
    Col,     // cell in same column
}
