extern crate util;

use util::input::Input;

use std::collections::HashSet;
use std::env;

fn first_frequency_reached_twice(frequencies: &[isize]) -> isize {
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
    if let Some(input_file) = env::args().nth(1) {
        let input = Vec::<isize>::read_from_file(input_file);

        let sum: isize = input.iter().sum();

        println!("Resulting frequency: {}", sum);

        let first_freq = first_frequency_reached_twice(&input);

        println!("First frequency reached twice: {}", first_freq);
    } else {
        println!("Please supply input file!");
    }
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
