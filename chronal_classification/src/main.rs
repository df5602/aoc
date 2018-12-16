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

    let instruction_samples = parse_input(&input);
    for sample in instruction_samples {
        println!("{:?}", sample);
    }
}

#[derive(Debug, Default)]
struct InstructionSample {
    regs_before: Vec<u8>,
    instruction: Vec<u8>,
    regs_after: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ParseState {
    Before,
    Instruction,
    After,
    Newline,
}

fn parse_input(input: &[String]) -> Vec<InstructionSample> {
    let mut state = ParseState::Before;
    let mut sample = InstructionSample::default();
    let mut samples: Vec<InstructionSample> = Vec::new();

    for line in input {
        if state == ParseState::Before && line.starts_with("Before") {
            sample.regs_before = parse_registers(line);
            assert_eq!(4, sample.regs_before.len());
            state = ParseState::Instruction;
        } else if state == ParseState::Instruction
            && !line.starts_with("Before")
            && !line.starts_with("After")
            && !line.is_empty()
        {
            sample.instruction = line
                .split_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            assert_eq!(4, sample.instruction.len());
            state = ParseState::After;
        } else if state == ParseState::After && line.starts_with("After") {
            sample.regs_after = parse_registers(line);
            assert_eq!(4, sample.regs_after.len());
            state = ParseState::Newline;
        } else if state == ParseState::Newline && line.is_empty() {
            let parsed_sample = std::mem::replace(&mut sample, InstructionSample::default());
            samples.push(parsed_sample);
            state = ParseState::Before;
        } else if state != ParseState::Newline && line.is_empty() {
            break;
        } else {
            panic!("Unexpected input!");
        }
    }

    samples
}

fn parse_registers(input: &str) -> Vec<u8> {
    input
        .split(|c| c == '[' || c == ']')
        .filter(|s| !s.starts_with("Before") && !s.starts_with("After"))
        .flat_map(|s| s.split(','))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.chars())
        .map(|c| c as u8 - b'0')
        .collect()
}
