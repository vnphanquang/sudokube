use clap::{crate_authors, crate_description, crate_license, crate_name, crate_version, App, Arg};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveLeft, MoveTo},
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use merge::Merge;
use std::io::stdout;

use sudokube::model::{Coordinate, Grid};

pub mod config;
pub mod display;
pub mod enums;
pub mod game;
pub mod lib;

use crate::display::DGrid;
use crate::enums::Navigation;
use crate::{config::Config, display::render_coordinate_guide};

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

pub struct Sudokube {
    mode: Mode,
    command_input: String,
}

impl Sudokube {
    pub fn new() -> Self {
        Sudokube {
            mode: Mode::Normal,
            command_input: String::new(),
        }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        let mut stdout = stdout();

        if self.mode == Mode::Command && mode == Mode::Normal {
            self.command_input = String::new();
            execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        } else if self.mode == Mode::Normal && mode == Mode::Command {
            let (_, rows) = size().unwrap();
            execute!(stdout, MoveTo(0, rows), Print(String::from(':'))).unwrap();
        }

        self.mode = mode;
    }

    pub fn command_input(&mut self, c: char) {
        self.command_input.push_str(&c.to_string());
        execute!(stdout(), Print(c.to_string())).unwrap();
    }

    pub fn command_backspace(&mut self) {
        if self.command_input.len() > 0 {
            execute!(stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine)).unwrap();
            let mut chars = self.command_input.chars();
            chars.next_back();
            self.command_input = chars.as_str().to_string();
        }
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .license(crate_license!())
        .arg(
            Arg::new("config")
                .about("config file to use")
                .takes_value(true)
                .short('c')
                .long("config")
                .multiple_occurrences(false)
                .default_value(&Config::default_path())
                .required(false),
        )
        .subcommand(
            App::new("make").about("create/edit a game").arg(
                Arg::new("path")
                    .about("filepath of game to edit or create")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("solve")
                .about("attempt to solve a given game")
                .arg(
                    Arg::new("path")
                        .about("filepath of game to solve")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("play").about("play a sudoku game").arg(
                Arg::new("path")
                    .about("filepath of custom game to load, otherwise will be generated")
                    .index(1)
                    .required(false),
            ),
        )
        .get_matches();

    let mut config = Config::default();
    if let Some(ref file) = matches.value_of("config") {
        config.merge(Config::read(file));

        // debug
        config.write(file).unwrap();
    }

    let mut sudokube = Sudokube::new();

    match matches.subcommand() {
        Some(("make", clone_matches)) => {
            let game_path = match clone_matches.value_of("path") {
                Some(game) => game,
                None => "<no_game_path_provided>",
            };
            println!("Create/Edit game at {}", game_path);

            const GRID_SIZE: usize = 9;

            let mut grid: Grid<GRID_SIZE> = Grid::read(&game_path);

            // let mut grid: Grid<GRID_SIZE> = Grid::new();
            let mut d_grid: DGrid<GRID_SIZE> = DGrid::new(&grid, Coordinate(1, 2));

            enable_raw_mode().unwrap();

            let mut stdout = stdout();
            execute!(
                stdout,
                EnableMouseCapture,
                EnterAlternateScreen,
                DisableBlinking,
            )
            .unwrap();

            render_coordinate_guide(Coordinate(0, 0), 9);

            d_grid.render(&grid, &config);
            d_grid.navigate_to(
                &grid,
                &config,
                Coordinate(GRID_SIZE as u8 / 2, GRID_SIZE as u8 / 2),
            );

            let key_binding = &config.key_binding();
            let navigation_key_events = &key_binding.navigation();

            loop {
                let event = read().unwrap();

                match sudokube.mode {
                    Mode::Normal => {
                        // navigation event;
                        if event == navigation_key_events.left().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Col(-1));
                        } else if event == navigation_key_events.up().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Row(-1));
                        } else if event == navigation_key_events.right().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Col(1));
                        } else if event == navigation_key_events.down().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Row(1));
                        } else if event == navigation_key_events.next_group().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Group(1));
                        } else if event == navigation_key_events.previous_group().crossterm() {
                            d_grid.navigate(&grid, &config, Navigation::Group(-1));
                        } else if event == key_binding.toggle_context_highlight().crossterm() {
                            config.toggle_context_highlight();
                            d_grid.rerender(&grid, &config);
                        } else if event == key_binding.delete().crossterm() {
                            let old_value = grid.get_cell(d_grid.active).value;
                            grid.set_cell_value(d_grid.active, None).unwrap();
                            d_grid.set_value(&grid, &config, d_grid.active, old_value, None);
                        } else if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Char(':'),
                        }) = event
                        {
                            sudokube.switch_mode(Mode::Command);
                        } else if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Char(c),
                        }) = event
                        {
                            let mut value: Option<u8> = None;
                            for i in 0..GRID_SIZE {
                                let ii = i + 1;
                                let char = ii.to_string().chars().nth(0).unwrap();
                                if char == c {
                                    value = Some(i as u8);
                                }
                            }
                            if let Some(_) = value {
                                let old_value = grid.get_cell(d_grid.active).value;
                                grid.set_cell_value(d_grid.active, value).unwrap();
                                d_grid.set_value(&grid, &config, d_grid.active, old_value, value);
                            }
                        }
                    }
                    Mode::Command => {
                        if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Esc,
                        }) = event
                        {
                            sudokube.switch_mode(Mode::Normal);
                            d_grid.navigate_to(&grid, &config, d_grid.active);
                        }
                        if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Enter,
                        }) = event
                        {
                            if sudokube.command_input == "q" {
                                break;
                            } else if sudokube.command_input == "w" {
                                grid.write(&game_path).unwrap();
                                sudokube.switch_mode(Mode::Normal);
                                d_grid.navigate_to(&grid, &config, d_grid.active);
                            } else if sudokube.command_input == "wq"
                                || sudokube.command_input == "qw"
                            {
                            } else {
                                sudokube.switch_mode(Mode::Normal);
                                d_grid.navigate_to(&grid, &config, d_grid.active);
                            }
                        }
                        if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Backspace,
                        }) = event
                        {
                            sudokube.command_backspace();
                        } else if let Event::Key(KeyEvent {
                            modifiers: KeyModifiers::NONE,
                            code: KeyCode::Char(c),
                        }) = event
                        {
                            sudokube.command_input(c);
                        }
                    }
                }
            }

            execute!(
                stdout,
                DisableMouseCapture,
                LeaveAlternateScreen,
                EnableBlinking,
            )
            .unwrap();
            // execute!(stdout, DisableMouseCapture).unwrap();

            disable_raw_mode().unwrap();
            // TODO: sudoku generation
        }
        Some(("solve", clone_matches)) => {
            let game_path = match clone_matches.value_of("path") {
                Some(path) => path,
                None => panic!("Path to game must be provided!"),
            };
            println!("Solving game at {}", game_path);
            // TODO: sudoku solving
        }
        Some(("play", clone_matches)) => {
            // TODO: sudoku playing
            // - error checking
            // - type tracking

            let game_path = match clone_matches.value_of("path") {
                Some(game) => game,
                None => "<no_game_path_provided:should_auto_generate>",
            };
            println!("Playing game at {}", game_path);
        }
        None => println!("Doing nothing..."),
        _ => unreachable!(),
    }
}
