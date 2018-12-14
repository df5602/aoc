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

    let mut offset = -5;
    let mut initial_state: Vec<usize> = vec![0; -offset as usize];
    let mut patterns = [0; 32];
    for line in input.iter().filter(|&s| !s.is_empty()) {
        if line.starts_with("initial state") {
            line.split(':')
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
                .for_each(|v| initial_state.push(v));
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

    for _ in 0..4 {
        initial_state.push(0);
    }

    print_state(0, 0, &initial_state);

    let mut state = initial_state;
    let mut previous_sum = 0;
    let mut sum_20 = 0;
    let mut sum_199 = 0;
    let mut sum_200 = 0;
    for g in 1..=200 {
        let next_offset = calculate_next_gen(&mut state, offset, &patterns);
        offset = next_offset;
        print_state(g, offset, &state);
        let sum = calculate_sum_of_state(offset - 1, &state);
        println!("sum: {} ({})", sum, sum - previous_sum);
        previous_sum = sum;
        if g == 20 {
            sum_20 = sum;
        } else if g == 199 {
            sum_199 = sum;
        } else if g == 200 {
            sum_200 = sum;
        }
    }

    println!("sum (20 generations): {}", sum_20);
    println!(
        "sum (very many generations): {}",
        (50_000_000_000u64 - 200) * (sum_200 as u64 - sum_199 as u64) + sum_200 as u64
    );
}

fn calculate_next_gen(
    current_state: &mut Vec<usize>,
    offset: isize,
    patterns: &[usize; 32],
) -> isize {
    let mut current_pattern = 0usize;
    let mut next_offset = offset;

    if current_state[5] == 1 {
        next_offset -= 5;
        for _ in 0..=5 {
            current_state.insert(0, 0);
        }
    }

    for (i, elem) in current_state.iter().enumerate() {
        current_pattern = ((current_pattern << 1) & 0x1F) | *elem;
        let value = patterns[current_pattern];
        if i >= 2 {
            let ptr = &current_state[i - 2] as *const usize;
            unsafe {
                let p2 = ptr as *mut usize;
                *p2 = value;
            }
        }
    }

    let length = current_state.len();
    if current_state[length - 5] == 1 {
        for _ in 0..4 {
            current_state.push(0);
        }
    }

    next_offset
}

fn calculate_sum_of_state(offset: isize, state: &[usize]) -> isize {
    state
        .iter()
        .enumerate()
        .map(|(i, v)| *v as isize * (i as isize + offset))
        .sum()
}

fn print_state(generation: usize, zero_position: isize, state: &[usize]) {
    print!("{} ({}): ", generation, zero_position);
    for elem in state {
        if *elem == 0 {
            print!(".");
        } else if *elem == 1 {
            print!("#");
        } else {
            print!("?");
        }
    }
    println!();
}
