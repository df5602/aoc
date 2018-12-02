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

    match find_first_match(&input, 1) {
        Some(idx) => {
            println!("Boxes: {}, {}", input[idx.0], input[idx.1]);
            println!(
                "Common letters: {}",
                common_letters(&input[idx.0], &input[idx.1])
            );
        }
        None => println!("No matches found!"),
    }
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

fn find_first_match(inputs: &[String], distance: usize) -> Option<(usize, usize)> {
    for (i, a) in inputs.iter().enumerate() {
        for (j, b) in inputs[i + 1..].iter().enumerate() {
            if hamming_distance(a, b) == distance {
                return Some((i, i + j + 1));
            }
        }
    }

    None
}

fn hamming_distance(a: &str, b: &str) -> usize {
    let mut distance = 0;
    for (cha, chb) in a.chars().zip(b.chars()) {
        if cha != chb {
            distance += 1;
        }
    }
    if a.len() != b.len() {
        distance += std::cmp::max(a.len(), b.len()) - std::cmp::min(a.len(), b.len());
    }
    distance
}

fn common_letters(a: &str, b: &str) -> String {
    let mut result = String::new();
    for (cha, chb) in a.chars().zip(b.chars()) {
        if cha == chb {
            result.push(cha);
        }
    }
    result
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

    #[test]
    fn test_hamming_distance() {
        let inputs = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
            String::from("wvxy"),
        ];
        assert_eq!(0, hamming_distance(&inputs[0], &inputs[0]));
        assert_eq!(2, hamming_distance(&inputs[0], &inputs[5]));
        assert_eq!(1, hamming_distance(&inputs[1], &inputs[4]));
        assert_eq!(5, hamming_distance(&inputs[0], &inputs[1]));
        assert_eq!(1, hamming_distance(&inputs[6], &inputs[7]));
        assert_eq!(1, hamming_distance(&inputs[7], &inputs[6]));
    }

    #[test]
    fn test_find_first_close_match() {
        let inputs = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];
        assert_eq!(Some((1, 4)), find_first_match(&inputs, 1));
    }

    #[test]
    fn test_common_letters() {
        assert_eq!("", common_letters("abcde", "fghij"));
        assert_eq!("fgij", common_letters("fghij", "fguij"));
    }
}
