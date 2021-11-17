mod box_drawing;

use box_drawing::{
    build_corner_char, build_middle_char, CornerPosition, CornerRelative, MiddlePosition,
    MiddleRelative,
};
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::{Color, ContentStyle, Print, ResetColor, StyledContent, Stylize},
};
use std::{
    collections::{HashMap, HashSet},
    io::stdout,
};
use sudokube::model::{Cell, CellRelation, Coordinate, Grid};

#[derive(Debug)]
struct DCellCoordinates {
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

#[derive(Debug, Clone)]
struct DCellRelations {
    top: Vec<CellRelation>,
    left: Vec<CellRelation>,
    bottom: Vec<CellRelation>,
    right: Vec<CellRelation>,
}

#[derive(Debug, Clone, Copy)]
struct DCellStyle {
    bg: Color,
    color: Color,
}

#[derive(Debug)]
struct DCell {
    coordinates: DCellCoordinates,
    relations: DCellRelations,
    style: DCellStyle,
}

#[derive(Debug)]
struct DConfig {
    context_highlight: bool,
    value_map: HashMap<u8, String>,
}

pub struct DGrid<const N: usize> {
    pub active: Coordinate,
    d_cells: [[DCell; N]; N],
    config: DConfig,
}

impl<const N: usize> DGrid<N> {
    pub fn new(grid: &Grid<N>) -> DGrid<N> {
        let mut d_cell_row: Vec<DCell> = Vec::new();
        let mut d_cells: Vec<[DCell; N]> = Vec::new();
        for row in 0..grid.rows.len() {
            for cell in &grid.rows[row].cells {
                let d_cell = DGrid::build_d_cell(grid, cell);
                d_cell_row.push(d_cell);
            }
            d_cells.push(d_cell_row.try_into().unwrap());
            d_cell_row = Vec::new();
        }

        let value_map: HashMap<u8, String> = HashMap::from([
            (0, String::from("1")),
            (1, String::from("2")),
            (2, String::from("3")),
            (3, String::from("4")),
            (4, String::from("5")),
            (5, String::from("6")),
            (6, String::from("7")),
            (7, String::from("8")),
            (8, String::from("9")),
        ]);

        DGrid {
            active: Coordinate(0, 0),
            d_cells: d_cells.try_into().unwrap(),
            config: DConfig {
                context_highlight: true,
                value_map,
            },
        }
    }

    fn build_d_cell(grid: &Grid<N>, cell: &Cell) -> DCell {
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

        let coordinates = DCellCoordinates {
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

        let get_relation = |coor: (i8, i8)| -> Vec<CellRelation> {
            let mut relation: Vec<CellRelation> = Vec::new();
            let (rx, ry) = coor;
            if rx >= 0 && rx < N as i8 && ry >= 0 && ry < N as i8 {
                let relative_coor = Coordinate(rx as u8, ry as u8);
                relation = grid.get_cells_relation(cell.coordinate, relative_coor);
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
            style: DCellStyle {
                bg: Color::Reset,
                color: Color::Red,
            },
        }
    }

    pub fn toggle_context_highlight(&mut self, grid: &Grid<N>) {
        self.config.context_highlight = !self.config.context_highlight;
        if self.config.context_highlight {
        } else {
            self.render(grid);
        }
    }

    fn d_cell(&self, coordinate: Coordinate) -> &DCell {
        let Coordinate(row, col) = coordinate;
        &self.d_cells[row as usize][col as usize]
    }

    fn set_d_style(&mut self, coordinate: Coordinate, d_style: DCellStyle) {
        let Coordinate(row, col) = coordinate;
        self.d_cells[row as usize][col as usize].style = d_style;
    }

    fn render_cell(&mut self, grid: &Grid<N>, coordinate: Coordinate) -> () {
        let cell = grid.get_cell(coordinate);
        let d_cell = self.d_cell(coordinate);

        let coordinates = &d_cell.coordinates;

        let top = &d_cell.relations.top;
        let bottom = &d_cell.relations.bottom;
        let left = &d_cell.relations.left;
        let right = &d_cell.relations.right;

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
        render_plain_at(coordinates.top_left, &top_left.to_string());
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
        render_plain_at(coordinates.bottom_left, &bottom_left.to_string());
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
        render_plain_at(coordinates.top_right, &top_right.to_string());
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
        render_plain_at(coordinates.bottom_right, &bottom_right.to_string());
        //--------------LEFT_MIDDLE---------------
        let left_middle = build_middle_char(
            MiddleRelative {
                relative: left_exists,
                relative_sub_grid: left_sub_grid,
            },
            MiddlePosition::Horizontal,
        );
        render_plain_at(coordinates.left_middle, &left_middle.to_string());
        //--------------RIGHT_MIDDLE---------------
        let right_middle = build_middle_char(
            MiddleRelative {
                relative: right_exists,
                relative_sub_grid: right_sub_grid,
            },
            MiddlePosition::Horizontal,
        );
        render_plain_at(coordinates.right_middle, &right_middle.to_string());
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
            render_plain_at(coor, &top_middle.to_string());
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
            render_plain_at(coor, &bottom_middle.to_string());
        }
        //----------------CENTER------------------
        self.set_value(grid, cell.coordinate, None, cell.value);
    }

    pub fn render(&mut self, grid: &Grid<N>) -> () {
        for x in 0..N {
            for y in 0..N {
                self.render_cell(grid, Coordinate(x as u8, y as u8));
            }
        }
    }

    pub fn navigate(&mut self, grid: &Grid<N>, navigation: Navigation) -> () {
        let mut row = self.active.row() as i8;
        let mut col = self.active.col() as i8;

        let i8_n = N as i8;

        match navigation {
            Navigation::Row(step) => {
                row += step;
                if row >= i8_n {
                    row = row % i8_n;
                }
                if row < 0 {
                    row = i8_n + (row % i8_n);
                }
            }
            Navigation::Col(step) => {
                col += step;
                if col >= i8_n {
                    col = col % i8_n;
                }
                if col < 0 {
                    col = i8_n + (col % i8_n);
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

        self.render_relative_cells(grid, old_cell.coordinate, new_cell.coordinate);
        self.rerender_same_value_cells(grid, new_cell.coordinate, old_cell.value, new_cell.value);
    }

    fn rerender_same_value_cells(
        &mut self,
        grid: &Grid<N>,
        coordinate: Coordinate,
        old_value: Option<u8>,
        new_value: Option<u8>,
    ) {
        if old_value != new_value {
            // removing old styles
            if let Some(v) = old_value {
                let value_coors = grid.get_value_coors(v);
                for coor in value_coors {
                    self.render_cell_value(grid, coor, RenderVariant::Default);
                }
            }
        }

        // applying new styles
        if let Some(v) = new_value {
            let value_coors = grid.get_value_coors(v);
            for coor in value_coors {
                if coor != coordinate {
                    let relations = grid.get_cells_relation(coordinate, coor);
                    let mut variant = RenderVariant::SameValue;
                    if relations.len() != 0 {
                        variant = RenderVariant::Error;
                    }
                    self.render_cell_value(grid, coor, variant);
                }
            }
        }
    }

    fn render_relative_cells(
        &mut self,
        grid: &Grid<N>,
        old_coor: Coordinate,
        new_coor: Coordinate,
    ) {
        if old_coor != new_coor {
            // remove old styles
            let col_coors = grid.get_col_coors(old_coor);
            let row_coors = grid.get_row_coors(old_coor);
            let sub_grid_coors = grid.get_sub_grid_coors(old_coor);

            let coors = [col_coors, row_coors, sub_grid_coors].concat();
            let mut set: HashSet<Coordinate> = HashSet::new();

            for coor in coors {
                if set.insert(coor) && coor != self.active {
                    self.render_cell_value(grid, coor, RenderVariant::Default);
                }
            }

            // applying new styles
            let col_coors = grid.get_col_coors(new_coor);
            let row_coors = grid.get_row_coors(new_coor);
            let sub_grid_coors = grid.get_sub_grid_coors(new_coor);

            let coors = [col_coors, row_coors, sub_grid_coors].concat();
            let mut set: HashSet<Coordinate> = HashSet::new();

            for coor in coors {
                if set.insert(coor) && coor != self.active {
                    self.render_cell_value(grid, coor, RenderVariant::DirectionalRelative);
                }
            }
        }
    }

    fn render_cell_value(
        &mut self,
        grid: &Grid<N>,
        coordinate: Coordinate,
        mut variant: RenderVariant,
    ) {
        let cell = grid.get_cell(coordinate);
        let d_value = self.d_value(cell.value);
        let d_cell = self.d_cell(coordinate);

        let value_coordinate = d_cell.coordinates.center;
        let mut d_style = d_cell.style;

        let mut color: Option<Color> = None;
        let mut bg: Option<Color> = None;

        if !cell.editable {
            // if value is fixed, overwrite render variant
            variant = RenderVariant::Fixed;
        }

        match variant {
            RenderVariant::Fixed => {
                color = Some(Color::Green);
            }
            RenderVariant::Error => {
                bg = Some(Color::Red);
                color = Some(Color::White);
            }
            RenderVariant::DirectionalRelative => {
                bg = Some(Color::Rgb {
                    r: 78,
                    g: 78,
                    b: 78,
                })
            }
            RenderVariant::SameValue => {
                color = Some(Color::Rgb {
                    r: 0,
                    g: 215,
                    b: 255,
                });
            }
            RenderVariant::Default => {
                color = Some(Color::Reset);
                bg = Some(Color::Reset);
            }
        }
        if let Some(c) = color {
            d_style.color = c;
        }
        if let Some(c) = bg {
            d_style.bg = c;
        }
        self.set_d_style(coordinate, d_style);

        let styled: StyledContent<String>;
        if self.config.context_highlight {
            styled = d_value.with(d_style.color).on(d_style.bg);
        } else {
            styled = d_value.with(Color::Reset).on(Color::Reset);
        }
        render_at(value_coordinate, styled);
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
        self.render_cell_value(grid, coordinate, RenderVariant::Default);
        self.rerender_same_value_cells(grid, coordinate, old_value, new_value);
    }

    fn d_value(&self, value: Option<u8>) -> String {
        let mut text = String::from(" ");
        if let Some(num) = value {
            if let Some(str) = self.config.value_map.get(&num) {
                text = String::from(str);
            }
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
    Fixed, // cell not editable
    DirectionalRelative, // for cells in same col/row
    SameValue, // for cells with same value
}

fn render_at(coordinate: Coordinate, styled: StyledContent<String>) {
    let Coordinate(x, y) = coordinate;
    execute!(
        stdout(),
        SavePosition,
        MoveTo(y as u16, x as u16),
        Print(styled),
        ResetColor,
        RestorePosition,
    )
    .unwrap();
}

fn render_plain_at(coordinate: Coordinate, text: &str) {
    render_at(
        coordinate,
        StyledContent::new(ContentStyle::new(), String::from(text)),
    );
}
