extern crate util;

use std::cmp::max;
use std::env;
use std::str::FromStr;

use util::input::{FileReader, FromFile};
use util::rectangle::Rectangle;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Claim> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let (max_width, max_height) = input
        .iter()
        .map(|claim| &claim.rectangle)
        .map(|rect| (rect.x() + rect.width(), rect.y() + rect.height()))
        .fold((0, 0), |m, dims| (max(m.0, dims.0), max(m.1, dims.1)));
    let mut grid = Grid::new(max_width, max_height);

    for claim in input.iter() {
        grid.add_rectangle(&claim.rectangle);
    }

    println!(
        "Square inches within two or more claims: {}",
        grid.count_eq_or_above(2)
    );

    let mut non_overlapping_claim = None;
    for a in input.iter() {
        if input
            .iter()
            .filter(|&b| a.owner != b.owner)
            .all(|b| !a.rectangle.collides_with(&b.rectangle))
        {
            non_overlapping_claim = Some(a.owner);
            break;
        }
    }

    match non_overlapping_claim {
        Some(id) => println!("Only non-overlapping claim: {}", id),
        None => println!("No non-overlapping claim found!"),
    }
}

#[derive(Debug)]
struct Claim {
    owner: usize,
    rectangle: Rectangle,
}

#[derive(Debug)]
enum ClaimParseError {
    ParseIntError(std::num::ParseIntError),
    ParseError(String),
}

impl std::fmt::Display for ClaimParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClaimParseError::ParseIntError(e) => write!(f, "Error parsing int: {}", e),
            ClaimParseError::ParseError(s) => write!(f, "Error parsing claim: {}", s),
        }
    }
}

impl From<std::num::ParseIntError> for ClaimParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ClaimParseError::ParseIntError(error)
    }
}

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrings: Vec<_> = s
            .split(|c| c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();
        if substrings.len() != 5 {
            return Err(ClaimParseError::ParseError(String::from(
                "input does not match format",
            )));
        }
        Ok(Self {
            owner: substrings[0].parse()?,
            rectangle: Rectangle::new(
                substrings[1].parse()?,
                substrings[2].parse()?,
                substrings[3].parse()?,
                substrings[4].parse()?,
            ),
        })
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<u32>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![0; width * height],
        }
    }

    fn add_rectangle(&mut self, r: &Rectangle) {
        for x in r.x()..(r.x() + r.width()) {
            for y in r.y()..(r.y() + r.height()) {
                self.increment_at(x, y);
            }
        }
    }

    fn increment_at(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("access out of bounds");
        }
        let idx = x + y * self.width;
        self.grid[idx] = self.grid[idx].saturating_add(1);
    }

    fn count_eq_or_above(&self, val: u32) -> usize {
        self.grid.iter().filter(|&v| *v >= val).count()
    }
}
