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

    let input: Vec<Dependency> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    for dep in input {
        println!("{}", dep);
    }
}

#[derive(Debug, Clone)]
struct Dependency {
    before: u8,
    step: u8,
}

impl FromStr for Dependency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters: Vec<char> = s
            .split_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.trim())
            .filter(|&s| s.len() == 1)
            .map(|s| s.chars().next().unwrap())
            .collect();

        if letters.len() != 2 {
            return Err(String::from("input does not match format"));
        }

        Ok(Self {
            before: letters[0] as u8,
            step: letters[0] as u8,
        })
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} => {}", self.before as char, self.step as char)
    }
}
