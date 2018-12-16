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
    let opcodes = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    let mut three_or_more = 0;
    for sample in instruction_samples {
        let mut matching_opcodes = 0;
        for &opcode in opcodes.iter() {
            let mut regs = sample.regs_before.clone();
            evaluate_instruction(&mut regs, opcode, &sample.instruction[1..]);

            if regs == sample.regs_after {
                matching_opcodes += 1;
            }
        }

        if matching_opcodes >= 3 {
            three_or_more += 1;
        }
    }

    println!(
        "Number of samples that behave like three or more opcodes: {}",
        three_or_more
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn evaluate_instruction(regs: &mut [u8], opcode: Opcode, arguments: &[u8]) {
    assert_eq!(4, regs.len());
    assert_eq!(3, arguments.len());

    let a = arguments[0] as usize;
    let b = arguments[1] as usize;
    let imma = arguments[0];
    let immb = arguments[1];
    let output = arguments[2] as usize;

    match opcode {
        Opcode::Addr => regs[output] = regs[a] + regs[b],
        Opcode::Addi => regs[output] = regs[a] + immb,
        Opcode::Mulr => regs[output] = regs[a] * regs[b],
        Opcode::Muli => regs[output] = regs[a] * immb,
        Opcode::Banr => regs[output] = regs[a] & regs[b],
        Opcode::Bani => regs[output] = regs[a] & immb,
        Opcode::Borr => regs[output] = regs[a] | regs[b],
        Opcode::Bori => regs[output] = regs[a] | immb,
        Opcode::Setr => regs[output] = regs[a],
        Opcode::Seti => regs[output] = imma,
        Opcode::Gtir => regs[output] = if imma > regs[b] { 1 } else { 0 },
        Opcode::Gtri => regs[output] = if regs[a] > immb { 1 } else { 0 },
        Opcode::Gtrr => regs[output] = if regs[a] > regs[b] { 1 } else { 0 },
        Opcode::Eqir => regs[output] = if imma == regs[b] { 1 } else { 0 },
        Opcode::Eqri => regs[output] = if regs[a] == immb { 1 } else { 0 },
        Opcode::Eqrr => regs[output] = if regs[a] == regs[b] { 1 } else { 0 },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr() {
        let mut registers = [0, 1, 2, 0];
        evaluate_instruction(&mut registers, Opcode::Addr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 3], registers);

        evaluate_instruction(&mut registers, Opcode::Addr, &[2, 3, 1]);
        assert_eq!([0, 5, 2, 3], registers);
    }

    #[test]
    fn addi() {
        let mut registers = [0, 1, 2, 0];
        evaluate_instruction(&mut registers, Opcode::Addi, &[1, 5, 3]);
        assert_eq!([0, 1, 2, 6], registers);

        evaluate_instruction(&mut registers, Opcode::Addr, &[2, 2, 1]);
        assert_eq!([0, 4, 2, 6], registers);
    }

    #[test]
    fn mulr() {
        let mut registers = [0, 1, 2, 0];
        evaluate_instruction(&mut registers, Opcode::Mulr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 2], registers);

        evaluate_instruction(&mut registers, Opcode::Mulr, &[0, 3, 1]);
        assert_eq!([0, 0, 2, 2], registers);
    }

    #[test]
    fn muli() {
        let mut registers = [0, 1, 2, 10];
        evaluate_instruction(&mut registers, Opcode::Muli, &[1, 2, 1]);
        assert_eq!([0, 2, 2, 10], registers);

        evaluate_instruction(&mut registers, Opcode::Muli, &[1, 10, 0]);
        assert_eq!([20, 2, 2, 10], registers);
    }

    #[test]
    fn banr() {
        let mut registers = [0, 1, 3, 0];
        evaluate_instruction(&mut registers, Opcode::Banr, &[1, 2, 3]);
        assert_eq!([0, 1, 3, 1], registers);
    }

    #[test]
    fn bani() {
        let mut registers = [0, 1, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Bani, &[1, 3, 2]);
        assert_eq!([0, 1, 1, 0], registers);
    }

    #[test]
    fn borr() {
        let mut registers = [0, 1, 2, 0];
        evaluate_instruction(&mut registers, Opcode::Borr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 3], registers);
    }

    #[test]
    fn bori() {
        let mut registers = [0, 2, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Bori, &[1, 4, 3]);
        assert_eq!([0, 2, 0, 6], registers);
    }

    #[test]
    fn setr() {
        let mut registers = [0, 2, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Setr, &[1, 2, 0]);
        assert_eq!([2, 2, 0, 0], registers);
    }

    #[test]
    fn seti() {
        let mut registers = [0, 2, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Seti, &[42, 2, 0]);
        assert_eq!([42, 2, 0, 0], registers);
    }

    #[test]
    fn gtir() {
        let mut registers = [0, 2, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Gtir, &[5, 1, 3]);
        assert_eq!([0, 2, 0, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Gtir, &[1, 1, 3]);
        assert_eq!([0, 2, 0, 0], registers);
    }

    #[test]
    fn gtri() {
        let mut registers = [0, 2, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Gtri, &[1, 1, 3]);
        assert_eq!([0, 2, 0, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Gtri, &[1, 3, 3]);
        assert_eq!([0, 2, 0, 0], registers);
    }

    #[test]
    fn gtrr() {
        let mut registers = [0, 2, 1, 0];
        evaluate_instruction(&mut registers, Opcode::Gtrr, &[1, 2, 3]);
        assert_eq!([0, 2, 1, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Gtrr, &[2, 1, 3]);
        assert_eq!([0, 2, 1, 0], registers);
    }

    #[test]
    fn eqir() {
        let mut registers = [0, 0, 1, 0];
        evaluate_instruction(&mut registers, Opcode::Eqir, &[1, 2, 3]);
        assert_eq!([0, 0, 1, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Eqir, &[0, 2, 3]);
        assert_eq!([0, 0, 1, 0], registers);
    }

    #[test]
    fn eqri() {
        let mut registers = [0, 5, 1, 0];
        evaluate_instruction(&mut registers, Opcode::Eqri, &[1, 5, 3]);
        assert_eq!([0, 5, 1, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Eqri, &[1, 4, 3]);
        assert_eq!([0, 5, 1, 0], registers);
    }

    #[test]
    fn eqrr() {
        let mut registers = [0, 5, 5, 0];
        evaluate_instruction(&mut registers, Opcode::Eqrr, &[1, 2, 3]);
        assert_eq!([0, 5, 5, 1], registers);

        evaluate_instruction(&mut registers, Opcode::Eqrr, &[0, 1, 3]);
        assert_eq!([0, 5, 5, 0], registers);
    }
}
