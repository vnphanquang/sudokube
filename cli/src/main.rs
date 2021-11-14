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

use pqsudoku::model::{Coordinate, Grid};

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
            grid.set_cell_value(0, 0, Some(8)).unwrap();
            let mut d_grid: DGrid<GRID_SIZE> = DGrid::new(&grid);

            enable_raw_mode().unwrap();

            let mut stdout = stdout();
            execute!(
                stdout,
                EnableMouseCapture,
                EnterAlternateScreen,
                DisableBlinking,
            )
            .unwrap();

            d_grid.render();
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
                        d_grid.navigate(Navigation::Col(-1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('j'),
                    }) => {
                        d_grid.navigate(Navigation::Row(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('k'),
                    }) => {
                        d_grid.navigate(Navigation::Row(-1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('l'),
                    }) => {
                        d_grid.navigate(Navigation::Col(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::NONE,
                        code: KeyCode::Char('g'),
                    }) => {
                        d_grid.navigate(Navigation::Group(1));
                    }
                    Event::Key(KeyEvent {
                        modifiers: KeyModifiers::SHIFT,
                        code: KeyCode::Char('G'),
                    }) => {
                        d_grid.navigate(Navigation::Group(-1));
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
