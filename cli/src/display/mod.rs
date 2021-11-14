use std::io::stdout;

use pqsudoku::model::{Cell, CellRelation, Coordinate, Grid};

use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

fn c_upper_left(byte: u8) -> char {
    match byte {
        0b0000_0000 => '┏',
        0b0000_0001 => '┳',
        0b0000_0011 => '┯',
        0b0000_0100 => '┣',
        0b0000_1100 => '┠',
        0b0000_0101 => '╋',
        0b0000_0111 => '┿',
        0b0000_1101 => '╂',
        0b0000_1111 => '┼',
        _ => '?',
    }
}

fn c_lower_left(byte: u8) -> char {
    match byte {
        0b0000_0000 => '┗',
        0b0000_0001 => '┻',
        0b0000_0011 => '┷',
        0b0000_0100 => '┣',
        0b0000_1100 => '┠',
        0b0000_0101 => '╋',
        0b0000_0111 => '┿',
        0b0000_1101 => '╂',
        0b0000_1111 => '┼',
        _ => '?',
    }
}

fn c_upper_right(byte: u8) -> char {
    match byte {
        0b0000_0000 => '┓',
        0b0000_0001 => '┳',
        0b0000_0011 => '┯',
        0b0000_0100 => '┫',
        0b0000_1100 => '┨',
        0b0000_0101 => '╋',
        0b0000_0111 => '┿',
        0b0000_1101 => '╂',
        0b0000_1111 => '┼',
        _ => '?',
    }
}

fn c_lower_right(byte: u8) -> char {
    match byte {
        0b0000_0000 => '┛',
        0b0000_0001 => '┻',
        0b0000_0011 => '┷',
        0b0000_0100 => '┫',
        0b0000_1100 => '┨',
        0b0000_0101 => '╋',
        0b0000_0111 => '┿',
        0b0000_1101 => '╂',
        0b0000_1111 => '┼',
        _ => '?',
    }
}

fn c_row_middle(byte: u8) -> char {
    match byte {
        0b0000_0000 | 0b0000_0001 => '━',
        0b0000_0011 => '─',
        _ => '?',
    }
}

fn c_col_middle(byte: u8) -> char {
    match byte {
        0b0000_0000 | 0b0000_0001 => '┃',
        0b0000_0011 => '│',
        _ => '?',
    }
}

#[derive(Debug)]
struct DCellCoordinate {
    center: Coordinate,
    upper_left: Coordinate,
    upper_middle: [Coordinate; 3],
    upper_right: Coordinate,
    lower_left: Coordinate,
    lower_middle: [Coordinate; 3],
    lower_right: Coordinate,
    left_middle: Coordinate,
    right_middle: Coordinate,
}

#[derive(Debug)]
struct DCellRelations {
    upper: Option<Vec<CellRelation>>,
    left: Option<Vec<CellRelation>>,
    lower: Option<Vec<CellRelation>>,
    right: Option<Vec<CellRelation>>,
}

#[derive(Debug)]
struct DCell {
    coordinates: DCellCoordinate,
    relations: DCellRelations,
}

pub struct DGrid<'a, const N: usize> {
    grid: &'a Grid<N>,
    active: Coordinate,
}

impl<'a, const N: usize> DGrid<'a, N> {
    pub fn new(grid: &'a Grid<N>) -> DGrid<'a, N> {
        DGrid {
            grid,
            active: Coordinate(0, 0),
        }
    }

    // move cursor
    // highlight active cell && its relatives

    fn d_cell(&self, cell: &Cell) -> DCell {
        let Coordinate(d_x, d_y) = cell_to_d_cell_coor(cell.coordinate);

        let center = Coordinate(d_x, d_y);

        let upper_left = Coordinate(d_x - 1, d_y - 2);
        let upper_middle = [
            Coordinate(d_x - 1, d_y - 1),
            Coordinate(d_x - 1, d_y),
            Coordinate(d_x - 1, d_y + 1),
        ];
        let upper_right = Coordinate(d_x - 1, d_y + 2);

        let lower_left = Coordinate(d_x + 1, d_y - 2);
        let lower_middle = [
            Coordinate(d_x + 1, d_y - 1),
            Coordinate(d_x + 1, d_y),
            Coordinate(d_x + 1, d_y + 1),
        ];
        let lower_right = Coordinate(d_x + 1, d_y + 2);

        let left_middle = Coordinate(d_x, d_y - 2);
        let right_middle = Coordinate(d_x, d_y + 2);

        let coordinates = DCellCoordinate {
            center,
            upper_left,
            upper_middle,
            upper_right,
            lower_left,
            lower_middle,
            lower_right,
            left_middle,
            right_middle,
        };

        let Coordinate(x, y) = cell.coordinate;
        let cell_coors: [(i8, i8); 4] = [
            (x as i8 - 1, y as i8), // upper
            (x as i8, y as i8 - 1), // left
            (x as i8 + 1, y as i8), // lower
            (x as i8, y as i8 + 1), // right
        ];

        let get_relation = |coor: (i8, i8)| -> Option<Vec<CellRelation>> {
            let mut relation: Option<Vec<CellRelation>> = None;
            let (rx, ry) = coor;
            if rx >= 0 && rx < N as i8 && ry >= 0 && ry < N as i8 {
                let relative_coor = Coordinate(rx as u8, ry as u8);
                let relative_cell = self.grid.get_cell(relative_coor);
                relation = Some(self.grid.get_cells_relation(cell, relative_cell));
            }

            relation
        };

        DCell {
            coordinates,
            relations: DCellRelations {
                upper: get_relation(cell_coors[0]),
                left: get_relation(cell_coors[1]),
                lower: get_relation(cell_coors[2]),
                right: get_relation(cell_coors[3]),
            },
        }
    }

    fn render_cell(&mut self, coordinate: Coordinate) -> () {
        let cell = self.grid.get_cell(coordinate);
        let d_cell = self.d_cell(cell);

        let coordinates = &d_cell.coordinates;

        let upper = d_cell.relations.upper.unwrap_or_default();
        let lower = d_cell.relations.lower.unwrap_or_default();
        let left = d_cell.relations.left.unwrap_or_default();
        let right = d_cell.relations.right.unwrap_or_default();

        let upper_exists = upper.len() > 0;
        let upper_sub_grid = upper.contains(&CellRelation::SubGrid);

        let lower_exists = lower.len() > 0;
        let lower_sub_grid = lower.contains(&CellRelation::SubGrid);

        let left_exists = left.len() > 0;
        let left_sub_grid = left.contains(&CellRelation::SubGrid);

        let right_exists = right.len() > 0;
        let right_sub_grid = right.contains(&CellRelation::SubGrid);

        let get_corner_byte = |vertical_relative_exists: bool,
                               vertical_relative_sub_grid: bool,
                               horizontal_relative_exists: bool,
                               horizontal_relative_sub_grid: bool|
         -> u8 {
            let mut byte: u8 = 0b0000_0000;

            if horizontal_relative_exists {
                byte |= 0b0000_0001;
            }

            if horizontal_relative_sub_grid {
                byte |= 0b0000_0010;
            }

            if vertical_relative_exists {
                byte |= 0b0000_0100;
            }

            if vertical_relative_sub_grid {
                byte |= 0b0000_1000;
            }

            byte
        };

        let upper_left = c_upper_left(get_corner_byte(
            upper_exists,
            upper_sub_grid,
            left_exists,
            left_sub_grid,
        ));
        self.render_at(coordinates.upper_left, &upper_left.to_string());

        let lower_left = c_lower_left(get_corner_byte(
            lower_exists,
            lower_sub_grid,
            left_exists,
            left_sub_grid,
        ));
        self.render_at(coordinates.lower_left, &lower_left.to_string());

        let upper_right = c_upper_right(get_corner_byte(
            upper_exists,
            upper_sub_grid,
            right_exists,
            right_sub_grid,
        ));
        self.render_at(coordinates.upper_right, &upper_right.to_string());

        let lower_right = c_lower_right(get_corner_byte(
            lower_exists,
            lower_sub_grid,
            right_exists,
            right_sub_grid,
        ));
        self.render_at(coordinates.lower_right, &lower_right.to_string());

        let get_separator_byte = |relative_exists: bool, relative_sub_grid: bool| -> u8 {
            let mut byte: u8 = 0b0000_0000;

            if relative_exists {
                byte |= 0b0000_0001;
            }

            if relative_sub_grid {
                byte |= 0b0000_0010;
            }

            byte
        };

        let left_middle = c_col_middle(get_separator_byte(left_exists, left_sub_grid));
        self.render_at(coordinates.left_middle, &left_middle.to_string());

        let right_middle = c_col_middle(get_separator_byte(right_exists, right_sub_grid));
        self.render_at(coordinates.right_middle, &right_middle.to_string());

        let upper_middle = c_row_middle(get_separator_byte(upper_exists, upper_sub_grid));
        for i in 0..coordinates.upper_middle.len() {
            let coor = coordinates.upper_middle[i];
            self.render_at(coor, &upper_middle.to_string());
        }

        let lower_middle = c_row_middle(get_separator_byte(lower_exists, lower_sub_grid));
        for i in 0..coordinates.lower_middle.len() {
            let coor = coordinates.lower_middle[i];
            self.render_at(coor, &lower_middle.to_string());
        }

        // FIXME: map value to displayed value;
        match cell.value {
            Some(text) => {
                let Coordinate(x, y) = cell.coordinate;
                self.render_at(coordinates.center, &text.to_string());
            }
            None => {}
        }
    }

    pub fn render(&mut self) -> () {
        let mut stdout = stdout();

        execute!(stdout, Clear(ClearType::All)).unwrap();

        for x in 0..N {
            for y in 0..N {
                self.render_cell(Coordinate(x as u8, y as u8));
            }
        }
    }

    fn render_at(&mut self, coordinate: Coordinate, text: &str) {
        let Coordinate(x, y) = coordinate;

        let mut stdout = stdout();
        execute!(
            stdout,
            SavePosition,
            MoveTo(y as u16, x as u16),
            Print(text),
            RestorePosition,
        )
        .unwrap();
    }

    pub fn navigate(&mut self, navigation: Navigation) -> () {
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

        self.navigate_to(Coordinate(row as u8, col as u8))
    }

    pub fn navigate_to(&mut self, coordinate: Coordinate) -> () {
        self.active = coordinate;
        let Coordinate(x, y) = cell_to_d_cell_coor(coordinate);

        let mut stdout = stdout();
        execute!(stdout, Hide, MoveTo(y as u16, x as u16), Show).unwrap();
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

pub enum Navigation {
    Col(i8),
    Row(i8),
    Group(i8),
}
