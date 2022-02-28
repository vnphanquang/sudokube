use super::{Cell, CellRelation, Coordinate, SubGrid, SudokuEventType};
use crate::error::SudokuError;
use event_emitter_rs::EventEmitter;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize)]
pub struct Grid<const N: usize> {
    #[serde(with = "serde_arrays")]
    pub rows: [CellRow<N>; N],
    #[serde(with = "serde_arrays")]
    pub sub_grids: [SubGrid<N>; N],

    pub value_map: HashMap<u8, Vec<Coordinate>>,

    #[serde(skip_serializing, skip_deserializing)]
    event_emitter: EventEmitter,
}

impl<const N: usize> Grid<N> {
    pub fn new() -> Self {
        let mut grid = Self {
            rows: [CellRow::blank(); N],
            sub_grids: [SubGrid::blank(); N],
            value_map: HashMap::new(),
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

    fn from_json(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
    pub fn read(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(json) => Self::from_json(&json),
            Err(_) => Self::new(),
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, self.to_json())?;
        Ok(())
    }
}

impl<const N: usize> Grid<N> {
    pub fn get_col_coors(&self, coordinate: Coordinate) -> [Coordinate; N] {
        let mut coors = [Coordinate(0, 0); N];
        let col = coordinate.col();
        for i in 0..N {
            coors[i] = Coordinate(i as u8, col);
        }
        coors
    }

    pub fn get_row_coors(&self, coordinate: Coordinate) -> [Coordinate; N] {
        let mut coors = [Coordinate(0, 0); N];
        let row = coordinate.row();
        for i in 0..N {
            coors[i] = Coordinate(row, i as u8);
        }
        coors
    }

    pub fn get_sub_grid_coors(&self, coordinate: Coordinate) -> [Coordinate; N] {
        let sub_grid = self.get_sub_grid(self.get_cell(coordinate));
        sub_grid.cells.clone()
    }

    pub fn get_value_coors(&self, value: u8) -> Vec<Coordinate> {
        let default: Vec<Coordinate> = Vec::new();
        self.value_map.get(&value).unwrap_or(&default).to_vec()
    }

    fn map_value_coor(
        &mut self,
        old_value: Option<u8>,
        new_value: Option<u8>,
        coordinate: Coordinate,
    ) -> () {
        if let Some(v) = old_value {
            if let Some(coors) = self.value_map.get_mut(&v) {
                for i in 0..coors.len() {
                    if coors[i] == coordinate {
                        coors.remove(i);
                        break;
                    }
                }
            }
        }

        if let Some(v) = new_value {
            match self.value_map.get_mut(&v) {
                Some(coors) => {
                    coors.push(coordinate);
                }
                None => {
                    let coors = vec![coordinate];
                    self.value_map.insert(v, coors);
                }
            }
        }
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

    pub fn get_cells_relation(&self, coor1: Coordinate, coor2: Coordinate) -> Vec<CellRelation> {
        if coor1 == coor2 {
            return vec![CellRelation::Same];
        }

        let Coordinate(x_1, y_1) = coor1;
        let Coordinate(x_2, y_2) = coor2;
        let mut relations: Vec<CellRelation> = Vec::new();
        if x_1 == x_2 {
            relations.push(CellRelation::Row)
        }
        if y_1 == y_2 {
            relations.push(CellRelation::Col)
        }

        let cell1 = self.get_cell(coor1);
        let cell2 = self.get_cell(coor2);
        if self.get_sub_grid(cell1).has(cell2) {
            relations.push(CellRelation::SubGrid)
        }

        relations
    }

    fn get_cell_mut(&mut self, coordinate: Coordinate) -> &mut Cell {
        let Coordinate(x, y) = coordinate;
        &mut self.rows[x as usize].cells[y as usize]
    }

    pub fn set_cell_value(
        &mut self,
        coordinate: Coordinate,
        value: Option<u8>,
    ) -> Result<(), SudokuError> {
        let Coordinate(x, y) = coordinate;
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

        let old_value = self.get_cell(coordinate).value;
        self.map_value_coor(old_value, value, coordinate);

        let cell = self.get_cell_mut(coordinate);
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
