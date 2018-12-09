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

    let input: String = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let substrings: Vec<_> = input.split_whitespace().collect();
    let number_players = substrings[0];
    let last_marble = substrings[6];

    println!(
        "Number of players: {}; Last marble: {}",
        number_players, last_marble
    );
}
