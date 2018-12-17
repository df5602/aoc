extern crate regex;
extern crate util;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::str::FromStr;

use regex::Regex;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<ScanLine> = match FileReader::new().read_from_file(input_file) {
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
    Clay,
    Sand,
    Spring,
    StillWater,
    DriedWater,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Clay => write!(f, "#"),
            Cell::Sand => write!(f, "."),
            Cell::Spring => write!(f, "+"),
            Cell::StillWater => write!(f, "~"),
            Cell::DriedWater => write!(f, "|"),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    offset_x: usize,
    grid: Vec<Cell>,
}

impl Grid {
    fn create(scan_lines: &[ScanLine]) -> Self {
        let (x_min, x_max, _y_min, y_max) = Grid::calculate_dimensions(&scan_lines);

        let width = x_max - x_min + 1;
        let height = y_max + 1;
        let mut grid = Self {
            width,
            height,
            offset_x: x_min,
            grid: Vec::with_capacity(width * height),
        };

        for _ in 0..width * height {
            grid.grid.push(Cell::Sand);
        }

        grid.fill_grid(&scan_lines);
        grid.set_at(500, 0, Cell::Spring);
        grid
    }

    fn set_at(&mut self, x: usize, y: usize, v: Cell) {
        let idx = y * self.width + (x - self.offset_x);
        if idx >= self.grid.len() {
            panic!("out of bounds: x={}, y={} => idx={}", x, y, idx);
        }
        self.grid[y * self.width + (x - self.offset_x)] = v;
    }

    fn fill_grid(&mut self, scan_lines: &[ScanLine]) {
        for line in scan_lines {
            match line {
                ScanLine::VerticalLine(line) => {
                    for y in line.y1..=line.y2 {
                        self.set_at(line.x, y, Cell::Clay);
                    }
                }
                ScanLine::HorizontalLine(line) => {
                    for x in line.x1..=line.x2 {
                        self.set_at(x, line.y, Cell::Clay);
                    }
                }
            }
        }
    }

    fn calculate_dimensions(scan_lines: &[ScanLine]) -> (usize, usize, usize, usize) {
        let mut x_min = usize::max_value();
        let mut x_max = usize::min_value();
        let mut y_min = usize::max_value();
        let mut y_max = usize::min_value();
        for line in scan_lines {
            match line {
                ScanLine::VerticalLine(line) => {
                    if line.x > x_max {
                        x_max = line.x;
                    }
                    if line.x < x_min {
                        x_min = line.x;
                    }
                    if line.y1 < y_min {
                        y_min = line.y1;
                    }
                    if line.y2 > y_max {
                        y_max = line.y2;
                    }
                }
                ScanLine::HorizontalLine(line) => {
                    if line.y > y_max {
                        y_max = line.y;
                    }
                    if line.y < y_min {
                        y_min = line.y;
                    }
                    if line.x1 < x_min {
                        x_min = line.x1;
                    }
                    if line.x2 > x_max {
                        x_max = line.x2;
                    }
                }
            }
        }
        println!(
            "Dimensions: x = {}..{}, y= {}..{}",
            x_min, x_max, y_min, y_max
        );
        (x_min, x_max, y_min, y_max)
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

#[derive(Debug, Copy, Clone)]
enum ScanLine {
    VerticalLine(VerticalLine),
    HorizontalLine(HorizontalLine),
}

#[derive(Debug, Copy, Clone)]
struct VerticalLine {
    x: usize,
    y1: usize,
    y2: usize,
}

#[derive(Debug, Copy, Clone)]
struct HorizontalLine {
    y: usize,
    x1: usize,
    x2: usize,
}

impl FromStr for ScanLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('x') {
            lazy_static! {
                static ref regex: Regex = Regex::new(r"^x=(\d+), y=(\d+)\.\.(\d+)$").unwrap();
            }
            let captures = match regex.captures(s) {
                Some(captures) => captures,
                None => {
                    return Err("input does not match expected format".to_string());
                }
            };
            let mut values = [0; 3];
            for (i, val) in values.iter_mut().enumerate() {
                *val = match captures.get(i + 1) {
                    Some(capture) => capture
                        .as_str()
                        .parse::<usize>()
                        .map_err(|e| format!("cannot parse number: {}", e))?,
                    None => {
                        return Err("input does not match expected format".to_string());
                    }
                };
            }
            if captures.get(4).is_some() {
                return Err("input does not match expected format".to_string());
            }
            Ok(ScanLine::VerticalLine(VerticalLine {
                x: values[0],
                y1: values[1],
                y2: values[2],
            }))
        } else if s.starts_with('y') {
            lazy_static! {
                static ref regex: Regex = Regex::new(r"^y=(\d+), x=(\d+)\.\.(\d+)$").unwrap();
            }
            let captures = match regex.captures(s) {
                Some(captures) => captures,
                None => {
                    return Err("input does not match expected format".to_string());
                }
            };
            let mut values = [0; 3];
            for (i, val) in values.iter_mut().enumerate() {
                *val = match captures.get(i + 1) {
                    Some(capture) => capture
                        .as_str()
                        .parse::<usize>()
                        .map_err(|e| format!("cannot parse number: {}", e))?,
                    None => {
                        return Err("input does not match expected format".to_string());
                    }
                };
            }
            if captures.get(4).is_some() {
                return Err("input does not match expected format".to_string());
            }
            Ok(ScanLine::HorizontalLine(HorizontalLine {
                y: values[0],
                x1: values[1],
                x2: values[2],
            }))
        } else {
            Err("input does not match expected format".to_string())
        }
    }
}
