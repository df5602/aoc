extern crate util;

use std::collections::HashMap;
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
    let number_players: usize = substrings[0].parse().unwrap();
    let last_marble: usize = substrings[6].parse().unwrap();

    println!(
        "Number of players: {}; Last marble: {}",
        number_players, last_marble
    );

    let high_score = play_game(number_players, last_marble);
    println!("High score: {}", high_score);

    let high_score = play_game(number_players, last_marble * 100);
    println!("High score if last marble were 100x larger: {}", high_score);
}

fn play_game(players: usize, last_marble: usize) -> usize {
    if last_marble <= 2 || players == 0 {
        return 0;
    }

    let mut circle: Vec<usize> = vec![0, 2, 1];
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut current_player = 3;
    let mut current_marble = 1;

    // Game is currently in the state:
    // [2]  0 (2) 1

    for marble in 3..=last_marble {
        if marble % 16384 == 0 {
            println!("Current marble: {}", marble);
        }
        if marble % 23 == 0 {
            let mut score = marble;
            let remove_position = (current_marble + circle.len() - 7) % circle.len();
            score += circle.remove(remove_position);
            current_marble = remove_position;
            *(scores.entry(current_player).or_insert(0)) += score;
        } else {
            let next_position = (current_marble + 2) % circle.len();
            circle.insert(next_position, marble);
            current_marble = next_position;
        }
        current_player = (current_player % players) + 1;
    }

    match scores.values().max() {
        Some(score) => *score,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        assert_eq!(32, play_game(9, 25));
        assert_eq!(8317, play_game(10, 1618));
        assert_eq!(146373, play_game(13, 7999));
        assert_eq!(2764, play_game(17, 1104));
        assert_eq!(54718, play_game(21, 6111));
        assert_eq!(37305, play_game(30, 5807));
    }
}
