extern crate regex;
extern crate util;

#[macro_use]
extern crate lazy_static;

use std::collections::VecDeque;
use std::env;
use std::io::BufRead;
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

    let mut grid = Grid::create(&input);
    grid.fill_with_water();

    println!("Filled with water: {}", grid.count_water());
    println!(
        "Retained after spring stops: {}",
        grid.count_only_water_retained()
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Clay,
    Sand,
    Spring,
    StillWater,
    DriedWater,
    OutOfBounds,
}

impl Cell {
    fn is_blocked(self) -> bool {
        match self {
            Cell::Clay | Cell::Spring | Cell::StillWater => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Clay => write!(f, "#"),
            Cell::Sand => write!(f, "."),
            Cell::Spring => write!(f, "+"),
            Cell::StillWater => write!(f, "~"),
            Cell::DriedWater => write!(f, "|"),
            Cell::OutOfBounds => write!(f, "="),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    offset_x: usize,
    lowest_y: usize,
    grid: Vec<Cell>,
    debug: bool,
}

impl Grid {
    fn create(scan_lines: &[ScanLine]) -> Self {
        let (x_min, x_max, y_min, y_max) = Grid::calculate_dimensions(&scan_lines);

        let width = x_max - x_min + 3;
        let height = y_max + 1;
        let mut grid = Self {
            width,
            height,
            offset_x: x_min - 1,
            lowest_y: y_min,
            grid: Vec::with_capacity(width * height),
            debug: false,
        };

        for _ in 0..width * height {
            grid.grid.push(Cell::Sand);
        }

        grid.fill_grid(&scan_lines);
        grid.set_at(500, 0, Cell::Spring);
        grid
    }

    fn count_water(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == Cell::DriedWater || c == Cell::StillWater)
            .filter(|(i, _)| *i >= self.lowest_y * self.width)
            .count()
    }

    fn count_only_water_retained(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == Cell::StillWater)
            .filter(|(i, _)| *i >= self.lowest_y * self.width)
            .count()
    }

    fn fill_with_water(&mut self) {
        let mut x = 500;
        let mut y = 1;

        let mut dir = 0isize;
        let mut ongoing = false;
        let mut need_next = false;
        let mut queue = VecDeque::new();

        let mut count = 0;

        loop {
            count += 1;

            if self.debug {
                println!("[{}] Current position: ({},{})", count, x, y);

                self.view(x, y, 25);
                let mut input_buffer = String::new();
                let _ = std::io::stdin().lock().read_line(&mut input_buffer);
            }

            if self.at(x, y) == Cell::StillWater || self.at(x, y) == Cell::OutOfBounds {
                if self.debug {
                    println!("Already been there / out of bounds..");
                }
                need_next = true;
            } else if self.at(x, y + 1) == Cell::Sand {
                if self.debug {
                    println!("Flow down");
                }
                self.set_at(x, y, Cell::DriedWater);
                y += 1;
            } else if self.at(x, y + 1) == Cell::DriedWater
                || self.at(x, y + 1) == Cell::OutOfBounds
            {
                if self.debug {
                    println!("Already been there / out of bounds...");
                }
                self.set_at(x, y, Cell::DriedWater);
                need_next = true;
            } else {
                // Check boundaries
                match self.find_boundary(x, y) {
                    (Some(x_min), Some(x_max)) => {
                        // Contained on both sides => fill with water
                        if self.debug {
                            println!("Fill container with water");
                        }
                        for x in x_min + 1..x_max {
                            self.set_at(x, y, Cell::StillWater);
                        }
                        y -= 1;
                    }
                    (Some(x_min), None) => {
                        // Bounded on left side => go right
                        if self.debug {
                            println!("Bounded on left side => go right");
                        }
                        for x in x_min + 1..=x {
                            self.set_at(x, y, Cell::DriedWater);
                        }
                        x += 1;
                    }
                    (None, Some(x_max)) => {
                        // Bounded on right side => go left
                        if self.debug {
                            println!("Bounded on right side => go left");
                        }
                        for x in x..x_max {
                            self.set_at(x, y, Cell::DriedWater);
                        }
                        x -= 1;
                    }
                    (None, None) => {
                        // Unbounded on both sides
                        if ongoing {
                            self.set_at(x, y, Cell::DriedWater);
                            x = (x as isize + dir) as usize;
                            if !self.at(x, y + 1).is_blocked() {
                                if self.debug {
                                    println!("Below is now free");
                                }
                                ongoing = false;
                            }
                        } else {
                            if self.debug {
                                println!("push x={}, y={}, dir=-1", x, y);
                                println!("push x={}, y={}, dir=1", x, y);
                            }
                            queue.push_back((x, y, -1));
                            queue.push_back((x, y, 1));
                            need_next = true;
                        }
                    }
                }
            }

            if need_next {
                match queue.pop_front() {
                    Some((x_new, y_new, dir_new)) => {
                        x = x_new;
                        y = y_new;
                        dir = dir_new;

                        ongoing = true;
                        need_next = false;
                    }
                    None => break,
                }
            }
        }
    }

    fn find_boundary(&self, x: usize, y: usize) -> (Option<usize>, Option<usize>) {
        let mut x_min = x;
        let mut x_max = x;
        while self.at(x_min - 1, y + 1).is_blocked() {
            x_min -= 1;
            if self.at(x_min, y) == Cell::Clay {
                break;
            }
        }
        while self.at(x_max + 1, y + 1).is_blocked() {
            x_max += 1;
            if self.at(x_max, y) == Cell::Clay {
                break;
            }
        }

        match (
            self.at(x_min, y) == Cell::Clay,
            self.at(x_max, y) == Cell::Clay,
        ) {
            (true, true) => (Some(x_min), Some(x_max)),
            (true, false) => (Some(x_min), None),
            (false, true) => (None, Some(x_max)),
            (false, false) => (None, None),
        }
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        if x < self.offset_x || (x - self.offset_x) >= self.width || y >= self.height {
            return Cell::OutOfBounds;
        }
        let idx = y * self.width + (x - self.offset_x);
        self.grid[idx]
    }

    fn set_at(&mut self, x: usize, y: usize, v: Cell) {
        if x < self.offset_x || (x - self.offset_x) >= self.width || y >= self.height {
            panic!("set_at: out of bounds: x={}, y={}", x, y);
        }
        let idx = y * self.width + (x - self.offset_x);
        self.grid[idx] = v;
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

    fn view(&self, x: usize, y: usize, size: usize) {
        let y_orig = y;
        let x_orig = x;

        let y_min = if y > size { y - size } else { 0 };
        let y_max = if y + size < self.height {
            y + size
        } else {
            self.height - 1
        };
        let x_min = if (x - self.offset_x) > size {
            x - size
        } else {
            self.offset_x
        };
        let x_max = if (x - self.offset_x) + size < self.width {
            x + size
        } else {
            self.width + self.offset_x - 1
        };

        println!("View: x={}..{}, y={}..{}", x_min, x_max, y_min, y_max);
        println!(
            "Dimensions: w={}, h={}, off={}",
            self.width, self.height, self.offset_x
        );

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if x == x_orig && y == y_orig {
                    print!("x");
                } else {
                    print!("{}", self.at(x, y));
                }
            }
            println!();
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
