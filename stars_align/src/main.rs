extern crate util;

use std::env;
use std::io::BufRead;
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

    let mut input: Vec<Point> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    for point in input.iter() {
        println!("{:?}", point);
    }

    for i in 1..30000 {
        for point in input.iter_mut() {
            point.propagate();
        }
        let dims = determine_dimensions(&input);
        if dims.x_min >= -250 && dims.x_max <= 250 && dims.y_min >= -300 && dims.y_max <= 300 {
            display_points(&input, dims);
            println!("Seconds passed: {}", i);
            let mut input_buffer = String::new();
            let _ = std::io::stdin().lock().read_line(&mut input_buffer);
        }
    }

    println!("Dimensions: {:?}", determine_dimensions(&input));
}

fn determine_dimensions(points: &[Point]) -> Dimensions {
    let mut x_min = i32::max_value();
    let mut x_max = i32::min_value();
    let mut y_min = i32::max_value();
    let mut y_max = i32::min_value();

    for point in points {
        if point.x_position < x_min {
            x_min = point.x_position;
        }
        if point.x_position > x_max {
            x_max = point.x_position;
        }
        if point.y_position < y_min {
            y_min = point.y_position;
        }
        if point.y_position > y_max {
            y_max = point.y_position;
        }
    }

    Dimensions {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

fn display_points(points: &[Point], dimensions: Dimensions) {
    let width = (dimensions.x_max - dimensions.x_min) as usize + 1;
    let height = (dimensions.y_max - dimensions.y_min) as usize + 1;
    let mut buffer = vec![b'.'; width * height];
    for point in points {
        buffer[(point.y_position - dimensions.y_min) as usize * width
            + (point.x_position - dimensions.x_min) as usize] = b'#';
    }

    for (i, &b) in buffer.iter().enumerate() {
        print!("{}", b as char);
        if (i + 1) % width == 0 {
            println!();
        }
    }
}

#[derive(Debug)]
struct Dimensions {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug)]
struct Point {
    x_position: i32,
    y_position: i32,
    x_velocity: i32,
    y_velocity: i32,
}

impl Point {
    fn propagate(&mut self) {
        self.x_position = self.x_position.saturating_add(self.x_velocity);
        self.y_position = self.y_position.saturating_add(self.y_velocity);
    }
}

#[derive(Debug)]
enum PointParseError {
    ParseIntError(std::num::ParseIntError),
    ParseError(String),
}

impl std::fmt::Display for PointParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PointParseError::ParseIntError(e) => write!(f, "Error parsing int: {}", e),
            PointParseError::ParseError(s) => write!(f, "Error parsing point: {}", s),
        }
    }
}

impl From<std::num::ParseIntError> for PointParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        PointParseError::ParseIntError(error)
    }
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let substrings: Vec<_> = s
            .split(|c| c == '<' || c == '>' || c == ',')
            .map(|s| s.trim())
            .filter(|&s| !s.is_empty())
            .filter(|&s| !s.starts_with("position") && !s.starts_with("velocity"))
            .collect();
        if substrings.len() != 4 {
            return Err(PointParseError::ParseError(String::from(
                "input does not match format",
            )));
        }
        Ok(Self {
            x_position: substrings[0].parse()?,
            y_position: substrings[1].parse()?,
            x_velocity: substrings[2].parse()?,
            y_velocity: substrings[3].parse()?,
        })
    }
}
