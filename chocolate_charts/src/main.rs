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

    let input: Vec<usize> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    if input.len() != 1 {
        println!("Expected single input!");
        std::process::exit(1);
    }
    let input = input[0];
    println!("Input: {}", input);

    let score = make_recipes([3, 7], input, 10);
    println!("Score: {}", score);
}

fn make_recipes(
    initial_state: [u8; 2],
    number_of_recipes: usize,
    recipes_in_score: usize,
) -> String {
    let mut positions = (0, 1);
    let mut state: Vec<u8> = vec![initial_state[0], initial_state[1]];

    while state.len() < number_of_recipes + recipes_in_score {
        let sum = state[positions.0] + state[positions.1];
        if sum > 9 {
            state.push(sum / 10);
        }
        state.push(sum - sum / 10 * 10);

        positions = (
            (positions.0 + state[positions.0] as usize + 1) % state.len(),
            (positions.1 + state[positions.1] as usize + 1) % state.len(),
        );
    }

    state
        .iter()
        .skip(number_of_recipes)
        .take(recipes_in_score)
        .map(|&int| (int + b'0') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        assert_eq!("37", make_recipes([3, 7], 0, 10));
    }

    #[test]
    fn test_recipes() {
        assert_eq!("5158916779", make_recipes([3, 7], 9, 10));
        assert_eq!("0124515891", make_recipes([3, 7], 5, 10));
        assert_eq!("9251071085", make_recipes([3, 7], 18, 10));
        assert_eq!("5941429882", make_recipes([3, 7], 2018, 10));
    }
}
