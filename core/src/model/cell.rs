use super::Coordinate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub value: Option<u8>,
    pub editable: bool,
}

impl Cell {
    pub fn blank() -> Cell {
        Cell {
            coordinate: Coordinate(0, 0),
            value: Option::None,
            editable: true,
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
