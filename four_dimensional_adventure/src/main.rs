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

    let input: Vec<Point4D> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    for point in input {
        println!("{:?}", point);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point4D {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
}

impl FromStr for Point4D {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut point = Point4D {
            x: 0,
            y: 0,
            z: 0,
            t: 0,
        };
        for (i, coord) in s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<isize>())
            .enumerate()
        {
            match i {
                0 => point.x = coord?,
                1 => point.y = coord?,
                2 => point.z = coord?,
                3 => point.t = coord?,
                _ => panic!("invalid input"),
            }
        }
        Ok(point)
    }
}
