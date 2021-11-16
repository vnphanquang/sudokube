use clap::{crate_authors, crate_version, App, Arg};

use crossterm::{
    cursor::position,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::stdout;

use sudokube::model::{Coordinate, Grid, SudokuEventType};

mod display;

use display::{DGrid, Navigation};

fn main() {
    let matches = App::new("pgsudoku")
        .about("pgSudoku playground CLI")
        .author(crate_authors!())
        .version(crate_version!())
        .license("MIT")
        .arg(
            Arg::new("config")
                .about("config file to use")
                .takes_value(true)
                .short('c')
                .long("config")
                .multiple_occurrences(false)
                .default_value("./config.toml")
                .required(false),
        )
        .subcommand(
            App::new("play").about("play a sudoku game").arg(
                Arg::new("game")
                    .about("custom game to load, otherwise will be generated")
                    .index(1)
                    .required(false),
            ),
        )
        .get_matches();

    if let Some(ref file) = matches.value_of("config") {
        println!("Using config file: {}", file);
    }

    match matches.subcommand() {
        Some(("play", clone_matches)) => {
            let game = match clone_matches.value_of("game") {
                Some(game) => game,
                None => "should generate",
            };
            println!("Playing game {}", game);

            const GRID_SIZE: usize = 9;
            let mut grid: Grid<GRID_SIZE> = Grid::new();
            let mut d_grid: DGrid<GRID_SIZE> = DGrid::new();

            enable_raw_mode().unwrap();

            let mut stdout = stdout();
            execute!(
                stdout,
                EnableMouseCapture,
                EnterAlternateScreen,
                DisableBlinking,
            )
            .unwrap();

            d_grid.render(&grid);
            d_grid.navigate_to(Coordinate(0, 0));

            loop {
                let event = read().unwrap();

                match event {
                    // Event::Key(KeyEvent {
                    //     modifiers: KeyModifiers::CONTROL,
                    //     code: KeyCode::Char('d'),
                    // }) => {
                    //     execute!(stdout, ScrollDown(2),).unwrap();
                    // }
                    // Event::Key(KeyEvent {
                    //     modifiers: KeyModifiers::CONTROL,
                    //     code: KeyCode::Char('u'),
                    // }) => {
                    //     execute!(stdout, ScrollUp(2),).unwrap();
                    // }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('c'),
                    }) => {
                        execute!(
                            stdout,
                            SavePosition,
                            Print(format!("Cursor position: {:?}\r", position())),
                            RestorePosition,
                        )
                        .unwrap();
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('q'),
                    }) => {
                        let (cols, rows) = size().unwrap();
                        execute!(stdout, MoveTo(0, rows), Print("Quitting...".to_string()))
                            .unwrap();
                        println!("Terminal Size ({}, {})", cols, rows);
                        break;
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('h'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Col(-1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('j'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Row(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('k'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Row(-1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('l'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Col(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('g'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Group(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::SHIFT,
                        code: KeyCode::Char('G'),
                    }) => {
                        d_grid.navigate(&grid, Navigation::Group(-1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('x'),
                    }) => {
                        let old_value = grid.get_cell(d_grid.active).value;
                        grid.set_cell_value(d_grid.active, None).unwrap();
                        d_grid.set_value(&grid, d_grid.active, old_value, None);
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char(c),
                    }) => {
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
                            d_grid.set_value(&grid, d_grid.active, old_value, value);
                        }
                    }
                    _ => {}
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
        }
        None => println!("Doing nothing..."),
        _ => unreachable!(),
    }
}
