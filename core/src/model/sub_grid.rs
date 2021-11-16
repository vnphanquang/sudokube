use super::{Cell, Coordinate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SubGrid<const N: usize> {
    pub coordinate: Coordinate,
    #[serde(with = "serde_arrays")]
    pub cells: [Coordinate; N],
}

impl<const N: usize> SubGrid<N> {
    pub fn blank() -> SubGrid<N> {
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
