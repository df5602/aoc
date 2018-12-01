extern crate util;

use std::collections::HashSet;
use std::env;

use util::input::{FileReader, FromFile};

fn first_frequency_reached_twice(frequencies: &[i64]) -> i64 {
    let mut intermediates = HashSet::new();
    intermediates.insert(0);
    let mut result = 0;
    let mut sum = 0;

    for f in frequencies.iter().cycle() {
        sum += f;
        if !intermediates.insert(sum) {
            result = sum;
            break;
        }
    }
    result
}

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<i64> = FileReader::read_from_file(input_file);

    let sum: i64 = input.iter().sum();
    println!("Resulting frequency: {}", sum);

    let first_freq = first_frequency_reached_twice(&input);
    println!("First frequency reached twice: {}", first_freq);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_frequency_reached_twice_1() {
        let f = [1, -1];
        assert_eq!(0, first_frequency_reached_twice(&f));
    }

    #[test]
    fn test_first_frequency_reached_twice_2() {
        let f = [3, 3, 4, -2, -4];
        assert_eq!(10, first_frequency_reached_twice(&f));
    }

    #[test]
    fn test_first_frequency_reached_twice_3() {
        let f = [-6, 3, 8, 5, -6];
        assert_eq!(5, first_frequency_reached_twice(&f));
    }

    #[test]
    fn test_first_frequency_reached_twice_4() {
        let f = [7, 7, -2, -7, -4];
        assert_eq!(14, first_frequency_reached_twice(&f));
    }
}
