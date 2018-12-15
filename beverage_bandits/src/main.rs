extern crate util;

use std::env;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<String> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let game = Game::create(&input);
    println!("{}", game);
}

struct Game {
    grid: Grid,
}

impl Game {
    fn create(input: &[String]) -> Self {
        Self {
            grid: Grid::create(&input),
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Unit {
    kind: UnitType,
}

impl Unit {
    fn is_goblin(&self) -> bool {
        match self.kind {
            UnitType::Elf => false,
            UnitType::Goblin => true,
        }
    }

    fn is_elf(&self) -> bool {
        match self.kind {
            UnitType::Elf => true,
            UnitType::Goblin => false,
        }
    }
}

#[derive(Debug)]
enum Cell {
    Open,
    Wall,
    Unit(Unit),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Open => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Unit(unit) => {
                if unit.is_elf() {
                    write!(f, "E")
                } else {
                    write!(f, "G")
                }
            }
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl Grid {
    fn create(input: &[String]) -> Self {
        let (width, height) = if !input.is_empty() {
            (input[0].len(), input.len())
        } else {
            (0, 0)
        };

        let mut grid = Vec::with_capacity(width * height);
        input.iter().flat_map(|s| s.chars()).for_each(|c| {
            let cell = match c {
                '.' => Cell::Open,
                '#' => Cell::Wall,
                'E' => Cell::Unit(Unit {
                    kind: UnitType::Elf,
                }),
                'G' => Cell::Unit(Unit {
                    kind: UnitType::Goblin,
                }),
                _ => panic!("unexpected input!"),
            };
            grid.push(cell);
        });

        Self {
            width,
            height,
            grid,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, c) in self.grid.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
