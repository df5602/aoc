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

    let input: Vec<u64> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };
}

/*#[derive(Debug)]
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
}*/
