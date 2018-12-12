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

    let input: Vec<String> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut initial_state = Vec::new();
    let mut patterns = [0; 32];
    for line in input.iter().filter(|&s| !s.is_empty()) {
        if line.starts_with("initial state") {
            initial_state = line
                .split(':')
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
                .filter(|&s| s.starts_with(|c| c == '.' || c == '#'))
                .flat_map(|s| s.chars())
                .map(|c| {
                    if c == '.' {
                        0
                    } else if c == '#' {
                        1
                    } else {
                        println!("unexpected initial state!");
                        std::process::exit(1);
                    }
                })
                .collect();
        } else if line.starts_with(|c| c == '.' || c == '#') {
            let mut iter = line.split("=>").map(|s| s.trim()).filter(|s| !s.is_empty());

            let mut p: usize;
            if let Some(pattern) = iter.next() {
                p = 0;

                if pattern.len() != 5 {
                    println!("unexpected pattern length!");
                    std::process::exit(1);
                }

                for (i, c) in pattern
                    .chars()
                    .map(|c| {
                        if c == '.' {
                            0
                        } else if c == '#' {
                            1
                        } else {
                            println!("unexpected initial state!");
                            std::process::exit(1);
                        }
                    })
                    .enumerate()
                {
                    p += (1 << (4 - i)) * c;
                }
            } else {
                println!("Unexpected input!");
                std::process::exit(1);
            }

            if let Some(result) = iter.next() {
                if result.len() != 1 {
                    println!("unexpected result length!");
                    std::process::exit(1);
                }

                let result = result
                    .chars()
                    .map(|c| {
                        if c == '.' {
                            0
                        } else if c == '#' {
                            1
                        } else {
                            println!("unexpected initial state!");
                            std::process::exit(1);
                        }
                    })
                    .nth(0)
                    .unwrap();
                patterns[p] = result;
            } else {
                println!("Unexpected input!");
                std::process::exit(1);
            }
        }
    }

    println!("{:?}", initial_state);
    println!("{:?}", patterns);
}
