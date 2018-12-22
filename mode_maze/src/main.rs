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

    let depth = input[0]
        .split("depth: ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .nth(0)
        .unwrap();

    let mut target = input[1]
        .split("target: ")
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap());
    let target = (target.next().unwrap(), target.next().unwrap());

    println!("depth = {}", depth);
    println!("target = ({},{})", target.0, target.1);

    let grid = Grid::new(depth, target);
    let total_risk_level = grid.total_risk_level();
    println!("Total risk level: {}", total_risk_level);
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<usize>,
}

impl Grid {
    fn new(depth: usize, target: (usize, usize)) -> Self {
        let width = target.0 + 1;
        let height = target.1 + 1;
        let mut grid = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let gi = if (x == 0 && y == 0) || (x == target.0 && y == target.1) {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    let el_left = grid[y * width + x - 1];
                    let el_above = grid[(y - 1) * width + x];
                    el_left * el_above
                };
                let el = (gi + depth) % 20183;
                grid.push(el);
            }
        }

        Self {
            width,
            height,
            grid,
        }
    }

    fn total_risk_level(&self) -> usize {
        self.grid.iter().map(|el| el % 3).sum()
    }
}
