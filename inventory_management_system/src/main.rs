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

    let input: Vec<String> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    println!("Checksum: {}", checksum(&input));
}

fn checksum(inputs: &[String]) -> u64 {
    let counts = inputs
        .iter()
        .map(|input| count_exact_dual_and_triple_occurences(&input))
        .fold((0, 0), |mut sum, counts| {
            if counts.0 > 0 {
                sum.0 += 1;
            }
            if counts.1 > 0 {
                sum.1 += 1;
            }
            sum
        });

    counts.0 * counts.1
}

fn count_exact_dual_and_triple_occurences(input: &str) -> (usize, usize) {
    // Count number of occurrence of each letter
    let mut char_count = HashMap::new();
    for c in input.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    // Count number of times a character occurred exactly two or three times
    char_count.values().fold((0, 0), |mut sum, &v| {
        if v == 2 {
            sum.0 += 1;
        } else if v == 3 {
            sum.1 += 1;
        }
        sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_occurrences() {
        let inputs = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];
        assert_eq!((0, 0), count_exact_dual_and_triple_occurences(&inputs[0]));
        assert_eq!((1, 1), count_exact_dual_and_triple_occurences(&inputs[1]));
        assert_eq!((1, 0), count_exact_dual_and_triple_occurences(&inputs[2]));
        assert_eq!((0, 1), count_exact_dual_and_triple_occurences(&inputs[3]));
        assert_eq!((2, 0), count_exact_dual_and_triple_occurences(&inputs[4]));
        assert_eq!((1, 0), count_exact_dual_and_triple_occurences(&inputs[5]));
        assert_eq!((0, 2), count_exact_dual_and_triple_occurences(&inputs[6]));
    }

    #[test]
    fn test_checksum() {
        let inputs = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];
        assert_eq!(12, checksum(&inputs));
    }
}
