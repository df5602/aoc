extern crate util;

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

    let input: Vec<Point> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    for point in input.iter() {
        println!("{:?}", point);
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
