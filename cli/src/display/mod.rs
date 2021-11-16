mod box_drawing;

use box_drawing::{
    build_corner_char, build_middle_char, CornerPosition, CornerRelative, MiddlePosition,
    MiddleRelative,
};
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetColors, StyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use pqsudoku::model::{Cell, CellRelation, Coordinate, Grid};
use std::io::stdout;

#[derive(Debug)]
struct DCellCoordinate {
    center: Coordinate,
    top_left: Coordinate,
    top_middle: [Coordinate; 3],
    top_right: Coordinate,
    bottom_left: Coordinate,
    bottom_middle: [Coordinate; 3],
    bottom_right: Coordinate,
    left_middle: Coordinate,
    right_middle: Coordinate,
}

#[derive(Debug)]
struct DCellRelations {
    top: Option<Vec<CellRelation>>,
    left: Option<Vec<CellRelation>>,
    bottom: Option<Vec<CellRelation>>,
    right: Option<Vec<CellRelation>>,
}

#[derive(Debug)]
struct DCellStyle {
    bg: Color,
    color: Color,
}

#[derive(Debug)]
struct DCell {
    coordinates: DCellCoordinate,
    relations: DCellRelations,
    // styles: DCellStyle,
}

pub struct DGrid<const N: usize> {
    pub active: Coordinate,
    // d_cells: [[DCell; N]; N],
}

impl<const N: usize> DGrid<N> {
    pub fn new() -> DGrid<N> {
        DGrid {
            active: Coordinate(0, 0),
            // d_cells:
        }
    }

    // move cursor
    // highlight active cell && its relatives

    fn d_cell(&self, grid: &Grid<N>, cell: &Cell) -> DCell {
        let Coordinate(d_x, d_y) = cell_to_d_cell_coor(cell.coordinate);

        let center = Coordinate(d_x, d_y);

        let top_left = Coordinate(d_x - 1, d_y - 2);
        let top_middle = [
            Coordinate(d_x - 1, d_y - 1),
            Coordinate(d_x - 1, d_y),
            Coordinate(d_x - 1, d_y + 1),
        ];
        let top_right = Coordinate(d_x - 1, d_y + 2);

        let bottom_left = Coordinate(d_x + 1, d_y - 2);
        let bottom_middle = [
            Coordinate(d_x + 1, d_y - 1),
            Coordinate(d_x + 1, d_y),
            Coordinate(d_x + 1, d_y + 1),
        ];
        let bottom_right = Coordinate(d_x + 1, d_y + 2);

        let left_middle = Coordinate(d_x, d_y - 2);
        let right_middle = Coordinate(d_x, d_y + 2);

        let coordinates = DCellCoordinate {
            center,
            top_left,
            top_middle,
            top_right,
            bottom_left,
            bottom_middle,
            bottom_right,
            left_middle,
            right_middle,
        };

        let Coordinate(x, y) = cell.coordinate;
        let cell_coors: [(i8, i8); 4] = [
            (x as i8 - 1, y as i8), // top
            (x as i8, y as i8 - 1), // left
            (x as i8 + 1, y as i8), // bottom
            (x as i8, y as i8 + 1), // right
        ];

        let get_relation = |coor: (i8, i8)| -> Option<Vec<CellRelation>> {
            let mut relation: Option<Vec<CellRelation>> = None;
            let (rx, ry) = coor;
            if rx >= 0 && rx < N as i8 && ry >= 0 && ry < N as i8 {
                let relative_coor = Coordinate(rx as u8, ry as u8);
                let relative_cell = grid.get_cell(relative_coor);
                relation = Some(grid.get_cells_relation(cell, relative_cell));
            }

            relation
        };

        DCell {
            coordinates,
            relations: DCellRelations {
                top: get_relation(cell_coors[0]),
                left: get_relation(cell_coors[1]),
                bottom: get_relation(cell_coors[2]),
                right: get_relation(cell_coors[3]),
            },
        }
    }

    fn render_cell(&mut self, grid: &Grid<N>, coordinate: Coordinate) -> () {
        let cell = grid.get_cell(coordinate);
        let d_cell = self.d_cell(grid, cell);

        let coordinates = &d_cell.coordinates;

        let top = d_cell.relations.top.unwrap_or_default();
        let bottom = d_cell.relations.bottom.unwrap_or_default();
        let left = d_cell.relations.left.unwrap_or_default();
        let right = d_cell.relations.right.unwrap_or_default();

        let top_exists = top.len() > 0;
        let top_sub_grid = top.contains(&CellRelation::SubGrid);

        let bottom_exists = bottom.len() > 0;
        let bottom_sub_grid = bottom.contains(&CellRelation::SubGrid);

        let left_exists = left.len() > 0;
        let left_sub_grid = left.contains(&CellRelation::SubGrid);

        let right_exists = right.len() > 0;
        let right_sub_grid = right.contains(&CellRelation::SubGrid);

        //--------------TOP_LEFT----------------
        let top_left = build_corner_char(
            CornerRelative {
                vertical: top_exists,
                vertical_sub_grid: top_sub_grid,
                horizontal: left_exists,
                horizontal_sub_grid: left_sub_grid,
            },
            CornerPosition::TopLeft,
        );
        self.render_at(
            coordinates.top_left,
            &top_left.to_string(),
            RenderVariant::Default,
        );
        //--------------BOTTOM_LEFT---------------
        let bottom_left = build_corner_char(
            CornerRelative {
                vertical: bottom_exists,
                vertical_sub_grid: bottom_sub_grid,
                horizontal: left_exists,
                horizontal_sub_grid: left_sub_grid,
            },
            CornerPosition::BottomLeft,
        );
        self.render_at(
            coordinates.bottom_left,
            &bottom_left.to_string(),
            RenderVariant::Default,
        );
        //--------------TOP_RIGHT---------------
        let top_right = build_corner_char(
            CornerRelative {
                vertical: top_exists,
                vertical_sub_grid: top_sub_grid,
                horizontal: right_exists,
                horizontal_sub_grid: right_sub_grid,
            },
            CornerPosition::TopRight,
        );
        self.render_at(
            coordinates.top_right,
            &top_right.to_string(),
            RenderVariant::Default,
        );
        //--------------BOTTOM_RIGHT---------------
        let bottom_right = build_corner_char(
            CornerRelative {
                vertical: bottom_exists,
                vertical_sub_grid: bottom_sub_grid,
                horizontal: right_exists,
                horizontal_sub_grid: right_sub_grid,
            },
            CornerPosition::BottomRight,
        );
        self.render_at(
            coordinates.bottom_right,
            &bottom_right.to_string(),
            RenderVariant::Default,
        );
        //--------------LEFT_MIDDLE---------------
        let left_middle = build_middle_char(
            MiddleRelative {
                relative: left_exists,
                relative_sub_grid: left_sub_grid,
            },
            MiddlePosition::Horizontal,
        );
        self.render_at(
            coordinates.left_middle,
            &left_middle.to_string(),
            RenderVariant::Default,
        );
        //--------------RIGHT_MIDDLE---------------
        let right_middle = build_middle_char(
            MiddleRelative {
                relative: right_exists,
                relative_sub_grid: right_sub_grid,
            },
            MiddlePosition::Horizontal,
        );
        self.render_at(
            coordinates.right_middle,
            &right_middle.to_string(),
            RenderVariant::Default,
        );
        //--------------TOP_MIDDLE---------------
        let top_middle = build_middle_char(
            MiddleRelative {
                relative: top_exists,
                relative_sub_grid: top_sub_grid,
            },
            MiddlePosition::Vertical,
        );
        for i in 0..coordinates.top_middle.len() {
            let coor = coordinates.top_middle[i];
            self.render_at(coor, &top_middle.to_string(), RenderVariant::Default);
        }
        //--------------BOTTOM_MIDDLE---------------
        let bottom_middle = build_middle_char(
            MiddleRelative {
                relative: bottom_exists,
                relative_sub_grid: bottom_sub_grid,
            },
            MiddlePosition::Vertical,
        );
        for i in 0..coordinates.bottom_middle.len() {
            let coor = coordinates.bottom_middle[i];
            self.render_at(coor, &bottom_middle.to_string(), RenderVariant::Default);
        }
        //----------------CENTER------------------
        self.set_value(grid, cell.coordinate, None, cell.value);
    }

    pub fn render(&mut self, grid: &Grid<N>) -> () {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All)).unwrap();

        for x in 0..N {
            for y in 0..N {
                self.render_cell(grid, Coordinate(x as u8, y as u8));
            }
        }
    }

    fn render_at(&mut self, coordinate: Coordinate, text: &str, variant: RenderVariant) {
        let Coordinate(x, y) = coordinate;

        let mut foreground_c = Color::Reset;
        let mut background_c = Color::Reset;

        match variant {
            RenderVariant::Error => {
                foreground_c = Color::White;
                background_c = Color::Red;
            }
            RenderVariant::DirectionalRelative => {
                foreground_c = Color::White;
                background_c = Color::Rgb {
                    r: 78,
                    g: 78,
                    b: 78,
                };
            }
            RenderVariant::SameValue => {
                foreground_c = Color::White;
                background_c = Color::Rgb {
                    r: 0,
                    g: 95,
                    b: 175,
                };
            }
            RenderVariant::Default => {}
        }

        let styled = text.with(foreground_c).on(background_c);

        let mut stdout = stdout();
        execute!(
            stdout,
            SavePosition,
            MoveTo(y as u16, x as u16),
            Print(styled),
            ResetColor,
            RestorePosition,
        )
        .unwrap();
    }

    pub fn navigate(&mut self, grid: &Grid<N>, navigation: Navigation) -> () {
        let mut row = self.active.row() as i8;
        let mut col = self.active.col() as i8;

        let i8_n = N as i8;

        match navigation {
            Navigation::Row(step) => {
                row += step;
                if row >= i8_n {
                    row = i8_n - 1;
                }
                if row < 0 {
                    row = 0;
                }
            }
            Navigation::Col(step) => {
                col += step;
                if col >= i8_n {
                    col = i8_n - 1;
                }
                if col < 0 {
                    col = 0;
                }
            }
            Navigation::Group(mut step) => {
                let sub_grid_size = (i8_n as f64).sqrt() as i8;
                step *= sub_grid_size;
                col += step;

                if step < 0 {
                    if col < 0 {
                        row += step / i8_n * sub_grid_size;
                        col += i8_n * (-step / i8_n);
                        if -step < i8_n {
                            col += i8_n;
                            row -= sub_grid_size;
                        }
                    }

                    if row < 0 {
                        row += i8_n;
                    }
                } else {
                    row += col / i8_n * sub_grid_size;
                    if row >= i8_n {
                        row -= i8_n;
                    }
                    col = col % i8_n;
                }
            }
        }
        let old_cell = grid.get_cell(self.active);

        self.navigate_to(Coordinate(row as u8, col as u8));

        let new_cell = grid.get_cell(self.active);

        self.rerender_directional_relative_cells(grid, old_cell.coordinate, new_cell.coordinate);
        self.rerender_same_value_cells(grid, old_cell.value, new_cell.value);
    }

    fn rerender_same_value_cells(
        &mut self,
        grid: &Grid<N>,
        old_value: Option<u8>,
        new_value: Option<u8>,
    ) {
        if old_value != new_value {
            // removing old styles
            if let Some(v) = old_value {
                let value_coors = grid.get_value_coors(v);
                for coor in value_coors {
                    self.rerender_cell(grid, coor, RenderVariant::Default);
                }
            }
        }

        // applying new styles
        if let Some(v) = new_value {
            let value_coors = grid.get_value_coors(v);
            for coor in value_coors {
                self.rerender_cell(grid, coor, RenderVariant::SameValue);
            }
        }
    }

    fn rerender_directional_relative_cells(
        &mut self,
        grid: &Grid<N>,
        old_coor: Coordinate,
        new_coor: Coordinate,
    ) {
        if old_coor != new_coor {
            // remove old styles
            let col_coors = grid.get_col_coors(old_coor);
            let row_coors = grid.get_row_coors(old_coor);
            for coor in [col_coors, row_coors].concat() {
                if coor != self.active {
                    self.rerender_cell(grid, coor, RenderVariant::Default);
                }
            }

            // applying new styles
            let col_coors = grid.get_col_coors(new_coor);
            let row_coors = grid.get_row_coors(new_coor);
            for coor in [col_coors, row_coors].concat() {
                if coor != self.active {
                    self.rerender_cell(grid, coor, RenderVariant::DirectionalRelative);
                }
            }
        }
    }

    fn rerender_cell(&mut self, grid: &Grid<N>, coordinate: Coordinate, variant: RenderVariant) {
        let cell = grid.get_cell(coordinate);
        let d_cell_coor = cell_to_d_cell_coor(coordinate);
        let d_value = self.d_value(cell.value);
        self.render_at(d_cell_coor, &d_value, variant);
    }

    pub fn navigate_to(&mut self, coordinate: Coordinate) -> () {
        self.active = coordinate;
        let Coordinate(x, y) = cell_to_d_cell_coor(coordinate);

        execute!(stdout(), Hide, MoveTo(y as u16, x as u16), Show).unwrap();
    }

    pub fn set_value(
        &mut self,
        grid: &Grid<N>,
        coordinate: Coordinate,
        old_value: Option<u8>,
        new_value: Option<u8>,
    ) -> () {
        // FIXME: map value:
        let d_cell_coor = cell_to_d_cell_coor(coordinate);
        let d_value = self.d_value(new_value);
        self.render_at(d_cell_coor, &d_value, RenderVariant::Default);

        self.rerender_same_value_cells(grid, old_value, new_value);
    }

    fn d_value(&self, value: Option<u8>) -> String {
        let mut text = String::from(" ");
        if let Some(num) = value {
            text = num.to_string();
        }

        text
    }
}

// fn d_cell_to_cell_coor(coordinate: Coordinate) -> Coordinate {
//     let Coordinate(x, y) = coordinate;
//     Coordinate((x - 1) / 2, (y - 2) / 4)
// }

fn cell_to_d_cell_coor(coordinate: Coordinate) -> Coordinate {
    let Coordinate(x, y) = coordinate;
    Coordinate(2 * x + 1, 2 * (2 * y + 1))
}

#[derive(Debug)]
pub enum Navigation {
    Col(i8),
    Row(i8),
    Group(i8),
}

#[derive(Debug)]
pub enum RenderVariant {
    Default,
    Error, // for cells with same value but invalid position (same row / col / subgrid)
    DirectionalRelative, // for cells in same col/row
    SameValue, // for cells with same value
}
