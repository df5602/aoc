extern crate util;

use std::env;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<String> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let depth = input[0]
        .split("depth: ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .nth(0)
        .unwrap();

    let mut target = input[1]
        .split("target: ")
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap());
    let target = (target.next().unwrap(), target.next().unwrap());

    println!("depth = {}", depth);
    println!("target = ({},{})", target.0, target.1);
}
