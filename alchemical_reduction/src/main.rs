extern crate util;

use std::collections::VecDeque;
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

    let input: String = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    // Check ASCII
    if !input.is_ascii() {
        println!("Input is not ASCII!");
        std::process::exit(1);
    }

    println!("Remaining units: {}", react(&input, None));

    let mut shortest_polymer = usize::max_value();
    let mut problematic_unit = 0;

    for c in b'a'..=b'z' {
        let length = react(&input, Some(c));
        if length < shortest_polymer {
            shortest_polymer = length;
            problematic_unit = c;
        }
    }

    println!(
        "Shortest polymer: Remove {} => Resulting length: {}",
        problematic_unit as char, shortest_polymer
    );
}

fn react(input: &str, ignore: Option<u8>) -> usize {
    let mut stack = VecDeque::new();

    let ignore = match ignore {
        Some(ignore) => ignore & !32,
        None => 0,
    };

    for c in input.bytes() {
        if ignore > 0 && (c & !32) == ignore {
            continue;
        }

        // Cloned shouldn't hurt performance wise, since T is u8 which is copy
        match stack.back().cloned() {
            Some(last) => {
                if last ^ c == 32 {
                    stack.pop_back();
                } else {
                    stack.push_back(c);
                }
            }
            None => stack.push_back(c),
        }
    }

    stack.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction() {
        assert_eq!(0, react("aA", None));
        assert_eq!(0, react("abBA", None));
        assert_eq!(4, react("abAB", None));
        assert_eq!(6, react("aabAAB", None));
        assert_eq!(10, react("dabAcCaCBAcCcaDA", None));
    }

    #[test]
    fn test_reaction_with_ignore() {
        assert_eq!(6, react("dabAcCaCBAcCcaDA", Some(b'a')));
        assert_eq!(8, react("dabAcCaCBAcCcaDA", Some(b'b')));
        assert_eq!(4, react("dabAcCaCBAcCcaDA", Some(b'c')));
        assert_eq!(6, react("dabAcCaCBAcCcaDA", Some(b'd')));
    }
}
