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

    let input: Vec<String> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    if input.len() != 1 {
        println!("Expected exactly one String as input!");
        std::process::exit(1);
    }

    // Collect to Vec<usize>
    let input: Vec<usize> = if let Ok(v) = input[0]
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .collect()
    {
        v
    } else {
        println!("Could not parse input!");
        std::process::exit(1);
    };

    let (sum_of_metadata, root_value) = match sum_metadata(&input) {
        Ok((sum, root_value, _)) => (sum, root_value),
        Err(e) => {
            println!("Error summing metadata: {}", e);
            std::process::exit(1);
        }
    };
    println!("Sum of metadata: {}", sum_of_metadata);
    println!("Value of root node: {}", root_value);
}

fn sum_metadata(input: &[usize]) -> Result<(usize, usize, &[usize]), String> {
    if input.len() < 2 {
        return Err(String::from("Expected header, got end of data"));
    }

    let number_child_nodes = input[0];
    let number_metadata_entries = input[1];

    let mut input = &input[2..];
    let mut sum = 0;

    let mut values: Vec<usize> = Vec::new();
    for _ in 0..number_child_nodes {
        let res = sum_metadata(&input)?;
        sum += res.0;
        values.push(res.1);
        input = res.2;
    }

    let sum_of_metadata = input[..number_metadata_entries].iter().sum::<usize>();
    sum += sum_of_metadata;

    let node_value = if values.is_empty() {
        sum_of_metadata
    } else {
        input[..number_metadata_entries]
            .iter()
            .fold(0, |mut sum, &metadata| {
                if metadata != 0 && metadata <= values.len() {
                    sum += values[metadata - 1];
                }

                sum
            })
    };

    Ok((sum, node_value, &input[number_metadata_entries..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_metadata() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (sum, _, _) = sum_metadata(&input).unwrap();
        assert_eq!(138, sum);
    }

    #[test]
    fn test_node_value() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (_, root_value, _) = sum_metadata(&input).unwrap();
        assert_eq!(66, root_value);
    }
}
