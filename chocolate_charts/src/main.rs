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

    let score = make_recipes_part1([3, 7], input, 10);
    println!("Score: {}", score);

    let first_appearance = make_recipes_part2([3, 7], input);
    println!("First appearance: {}", first_appearance);
}

fn make_recipes_part1(
    initial_state: [u8; 2],
    number_of_recipes: usize,
    recipes_in_score: usize,
) -> String {
    make_recipes(initial_state, number_of_recipes, recipes_in_score, false).0
}

fn make_recipes_part2(initial_state: [u8; 2], number_of_recipes: usize) -> usize {
    make_recipes(initial_state, number_of_recipes, 0, true).1
}

#[allow(clippy::mut_range_bound)]
fn make_recipes(
    initial_state: [u8; 2],
    number_of_recipes: usize,
    recipes_in_score: usize,
    part2: bool,
) -> (String, usize) {
    let mut positions = (0, 1);
    let mut state: Vec<u8> = vec![initial_state[0], initial_state[1]];
    let mut idx = 0;
    let mut idx_found = 0;
    let sequence = number_of_recipes.to_string();
    let sequence: Vec<u8> = sequence.chars().map(|c| c as u8 - b'0').collect();

    while state.len() < number_of_recipes + recipes_in_score || (part2 && idx_found == 0) {
        let sum = state[positions.0] + state[positions.1];
        if sum > 9 {
            state.push(sum / 10);
        }
        state.push(sum % 10);

        positions = (
            (positions.0 + state[positions.0] as usize + 1) % state.len(),
            (positions.1 + state[positions.1] as usize + 1) % state.len(),
        );

        if part2 && state.len() >= sequence.len() {
            for i in idx..=(state.len() - sequence.len()) {
                let recipes = &state[i..i + sequence.len()];
                if recipes == &sequence[..] {
                    idx_found = i;
                    break;
                }
                idx += 1;
            }
        }
    }

    (
        state
            .iter()
            .skip(number_of_recipes)
            .take(recipes_in_score)
            .map(|&int| (int + b'0') as char)
            .collect(),
        idx_found,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        assert_eq!("37", make_recipes_part1([3, 7], 0, 2));
    }

    #[test]
    fn test_recipes() {
        assert_eq!("5158916779", make_recipes_part1([3, 7], 9, 10));
        assert_eq!("0124515891", make_recipes_part1([3, 7], 5, 10));
        assert_eq!("9251071085", make_recipes_part1([3, 7], 18, 10));
        assert_eq!("5941429882", make_recipes_part1([3, 7], 2018, 10));
    }

    #[test]
    fn test_first_appearance() {
        assert_eq!(9, make_recipes_part2([3, 7], 51589));
        assert_eq!(18, make_recipes_part2([3, 7], 92510));
        assert_eq!(2018, make_recipes_part2([3, 7], 59414));
        //assert_eq!(20278122, make_recipes_part2([3, 7], 540391));
    }
}
