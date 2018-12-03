extern crate util;

use std::cmp::max;
use std::env;
use std::str::FromStr;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    // Parse input
    let input: Vec<Rectangle> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    // Create grid
    let (max_width, max_height) = input
        .iter()
        .map(|rect| (rect.pos_x + rect.width, rect.pos_y + rect.height))
        .fold((0, 0), |m, dims| (max(m.0, dims.0), max(m.1, dims.1)));
    let mut grid = Grid::new(max_width, max_height);

    // Fill grid
    for rect in input.iter() {
        for x in rect.pos_x..(rect.pos_x + rect.width) {
            for y in rect.pos_y..(rect.pos_y + rect.height) {
                grid.increment_at(x, y);
            }
        }
    }

    println!(
        "Square inches within two or more claims: {}",
        grid.count_eq_or_above(2)
    );

    let mut non_overlapping_claim = None;
    for a in input.iter() {
        let mut collision_detected = false;
        for b in input.iter() {
            if a.owner == b.owner {
                continue;
            }
            if a.collides_with(&b) {
                collision_detected = true;
            }
        }
        if !collision_detected {
            non_overlapping_claim = Some(a.owner);
        }
    }

    match non_overlapping_claim {
        Some(id) => println!("Only non-overlapping claim: {}", id),
        None => println!("No non-overlapping claim found!"),
    }
}

#[derive(Debug)]
struct Rectangle {
    owner: usize,
    pos_x: usize,
    pos_y: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn collides_with(&self, other: &Rectangle) -> bool {
        other.pos_x < self.pos_x + self.width
            && other.pos_x + other.width > self.pos_x
            && other.pos_y < self.pos_y + self.height
            && other.pos_y + other.height > self.pos_y
    }
}

#[derive(Debug)]
enum RectangleParseError {
    ParseIntError(std::num::ParseIntError),
    ParseError(String),
}

impl std::fmt::Display for RectangleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RectangleParseError::ParseIntError(e) => write!(f, "Error parsing int: {}", e),
            RectangleParseError::ParseError(s) => write!(f, "Error parsing rectangle: {}", s),
        }
    }
}

impl From<std::num::ParseIntError> for RectangleParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        RectangleParseError::ParseIntError(error)
    }
}

impl FromStr for Rectangle {
    type Err = RectangleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrings: Vec<_> = s
            .split(|c| c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();
        if substrings.len() != 5 {
            return Err(RectangleParseError::ParseError(String::from(
                "input does not match format",
            )));
        }
        Ok(Self {
            owner: substrings[0].parse()?,
            pos_x: substrings[1].parse()?,
            pos_y: substrings[2].parse()?,
            width: substrings[3].parse()?,
            height: substrings[4].parse()?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collisions() {
        let rect1 = Rectangle {
            owner: 0,
            pos_x: 3,
            pos_y: 2,
            width: 7,
            height: 5,
        };
        let mut rect2 = Rectangle {
            owner: 0,
            pos_x: 7,
            pos_y: 3,
            width: 7,
            height: 7,
        };

        assert!(rect1.collides_with(&rect2));

        rect2.pos_x = 9;
        assert!(rect1.collides_with(&rect2));

        rect2.pos_x = 10;
        assert!(!rect1.collides_with(&rect2));

        rect2.pos_x = 3;
        assert!(rect1.collides_with(&rect2));

        rect2.pos_x = 2;
        assert!(rect1.collides_with(&rect2));

        rect2.pos_x = 1;
        rect2.width = 2;
        assert!(!rect1.collides_with(&rect2));

        rect2.pos_x = 7;
        rect2.width = 7;

        rect2.pos_y = 6;
        assert!(rect1.collides_with(&rect2));

        rect2.pos_y = 7;
        assert!(!rect1.collides_with(&rect2));

        rect2.pos_y = 1;
        rect2.height = 2;
        assert!(rect1.collides_with(&rect2));

        rect2.height = 1;
        assert!(!rect1.collides_with(&rect2));
    }
}
