use std::collections::{HashMap, VecDeque};
use std::env;

use util::input::{FileReader, FromFile};

use regex::Regex;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let (number_players, last_marble) = {
        let regex = Regex::new(r"^(\d+)\D+(\d+)\D*$").unwrap();
        match FileReader::new().parse(regex).read_from_file(input_file) {
            Ok(input) => input,
            Err(e) => {
                println!("Error reading input: {}", e);
                std::process::exit(1);
            }
        }
    };

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

    let mut circle: CircularTape<usize> = CircularTape::new();
    circle.push(0);
    circle.push(2);
    circle.push(1);
    circle.rotate(1); // move 2 into current position

    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut current_player = 3;

    // Game is currently in the state:
    // [2]  0 (2) 1

    for marble in 3..=last_marble {
        if marble % 23 == 0 {
            circle.rotate(-7);
            let score = marble + circle.remove().unwrap();

            *(scores.entry(current_player).or_insert(0)) += score;
        } else {
            circle.rotate(2);
            circle.insert(marble);
        }
        current_player = (current_player % players) + 1;
    }

    match scores.values().max() {
        Some(score) => *score,
        None => 0,
    }
}

struct CircularTape<T> {
    deque: VecDeque<T>,
}

impl<T> CircularTape<T> {
    fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    fn len(&self) -> usize {
        self.deque.len()
    }

    fn push(&mut self, value: T) {
        self.deque.push_back(value);
    }

    fn insert(&mut self, value: T) {
        self.deque.push_front(value);
    }

    fn remove(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    fn rotate(&mut self, mut amount: isize) {
        if self.len() < 2 {
            return;
        }

        // Amount > 0 <=> rotate counter-clockwise (or left-to-right)
        while amount > 0 {
            let front = self.deque.pop_front().unwrap();
            self.deque.push_back(front);
            amount -= 1;
        }

        // Amount < 0 <=> rotate clock-wise (or right-to-left)
        while amount < 0 {
            let back = self.deque.pop_back().unwrap();
            self.deque.push_front(back);
            amount += 1;
        }
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
