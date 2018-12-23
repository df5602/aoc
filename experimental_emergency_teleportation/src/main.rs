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

    let input: Vec<Nanobot> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let strongest_nanobot = find_strongest_nanobot(&input);
    println!("Strongest nanobot: {:?}", strongest_nanobot);

    let number_in_range = number_of_nanobots_in_range(strongest_nanobot, &input);
    println!("Number of nanobots in range: {}", number_in_range);
}

fn find_strongest_nanobot(nanobots: &[Nanobot]) -> Nanobot {
    let mut strongest = Nanobot {
        position: Point3D { x: 0, y: 0, z: 0 },
        signal_radius: usize::min_value(),
    };

    for nanobot in nanobots {
        if nanobot.signal_radius > strongest.signal_radius {
            strongest = *nanobot;
        }
    }

    strongest
}

fn number_of_nanobots_in_range(nanobot: Nanobot, others: &[Nanobot]) -> usize {
    let mut number_in_range = 0;
    for other_bot in others {
        if nanobot.position.manhattan_distance_to(other_bot.position) <= nanobot.signal_radius {
            number_in_range += 1;
        }
    }

    number_in_range
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn manhattan_distance_to(self, other: Point3D) -> usize {
        ((other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()) as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Nanobot {
    position: Point3D,
    signal_radius: usize,
}

impl FromStr for Nanobot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref regex: Regex =
                Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$").unwrap();
        }
        let captures = match regex.captures(s) {
            Some(captures) => captures,
            None => {
                return Err("input does not match expected format".to_string());
            }
        };
        let mut values = [0isize; 4];
        for (i, val) in values.iter_mut().enumerate() {
            *val = match captures.get(i + 1) {
                Some(capture) => capture
                    .as_str()
                    .parse::<isize>()
                    .map_err(|e| format!("cannot parse number: {}", e))?,
                None => {
                    return Err("input does not match expected format".to_string());
                }
            };
        }
        if captures.get(5).is_some() {
            return Err("input does not match expected format".to_string());
        }
        Ok(Nanobot {
            position: Point3D {
                x: values[0],
                y: values[1],
                z: values[2],
            },
            signal_radius: values[3] as usize,
        })
    }
}
