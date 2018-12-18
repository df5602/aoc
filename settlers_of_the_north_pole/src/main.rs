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

    let grid = Grid::create(&input);
    println!("{}", grid);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Trees,
    Lumberyard,
    OutOfBounds,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Open => write!(f, "."),
            Cell::Trees => write!(f, "|"),
            Cell::Lumberyard => write!(f, "#"),
            Cell::OutOfBounds => write!(f, " "),
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
                '|' => Cell::Trees,
                '#' => Cell::Lumberyard,
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

    fn at(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            return Cell::OutOfBounds;
        }
        self.grid[y * self.width + x]
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
