use std::collections::HashMap;
use std::env;

use util::input::{FileReader, FromFile};

type RegSize = u16;

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

    let (instruction_samples, test_program) = parse_input(&input);
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
    for sample in instruction_samples.iter() {
        let mut matching_opcodes = 0;
        for &opcode in opcodes.iter() {
            if matches_opcode(
                &sample.regs_before,
                opcode,
                &sample.instruction[1..],
                &sample.regs_after,
            ) {
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

    let mut possibilities: [Vec<Opcode>; 16] = Default::default();
    for p in &mut possibilities {
        *p = opcodes.to_vec();
    }

    let mut matches: HashMap<u8, Opcode> = HashMap::new();

    loop {
        let mut found_this_iteration = 0;

        for sample in instruction_samples.iter() {
            let opcode = sample.instruction[0];

            if !possibilities[opcode as usize].is_empty() {
                possibilities[opcode as usize].retain(|&opcode| {
                    matches_opcode(
                        &sample.regs_before,
                        opcode,
                        &sample.instruction[1..],
                        &sample.regs_after,
                    )
                });
                if possibilities[opcode as usize].len() == 1 {
                    let found_op = possibilities[opcode as usize][0];
                    if matches.insert(opcode as u8, found_op).is_some() {
                        panic!("Opcode is not unique!");
                    }

                    for p in &mut possibilities {
                        p.retain(|&opcode| opcode != found_op);
                    }

                    found_this_iteration += 1;
                }
            }
        }

        if found_this_iteration == 0 || matches.len() == 16 {
            break;
        }
    }

    println!("Matches:");
    for (k, v) in matches.iter() {
        println!("{} => {:?}", k, v);
    }

    let mut regs = [0; 4];
    for instruction in test_program {
        evaluate_instruction(
            &mut regs,
            matches[&(instruction[0] as u8)],
            &instruction[1..],
        );
    }
    println!("Value contained in register 0: {}", regs[0]);
}

fn matches_opcode(
    regs_before: &[RegSize],
    opcode: Opcode,
    args: &[RegSize],
    regs_after: &[RegSize],
) -> bool {
    assert_eq!(4, regs_before.len());
    let mut regs = [0; 4];
    regs[..regs_before.len()].clone_from_slice(&regs_before[..]);

    evaluate_instruction(&mut regs, opcode, &args);

    regs == regs_after
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

fn evaluate_instruction(regs: &mut [RegSize], opcode: Opcode, arguments: &[RegSize]) {
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
    regs_before: Vec<RegSize>,
    instruction: Vec<RegSize>,
    regs_after: Vec<RegSize>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ParseState {
    Before,
    Instruction,
    After,
    Newline,
    Program,
}

fn parse_input(input: &[String]) -> (Vec<InstructionSample>, Vec<Vec<RegSize>>) {
    let mut state = ParseState::Before;
    let mut sample = InstructionSample::default();
    let mut samples: Vec<InstructionSample> = Vec::new();
    let mut program: Vec<Vec<RegSize>> = Vec::new();

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
                .map(|s| s.parse::<RegSize>().unwrap())
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
            state = ParseState::Program;
        } else if state == ParseState::Program && !line.is_empty() {
            let instruction: Vec<RegSize> = line
                .split_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<RegSize>().unwrap())
                .collect();
            program.push(instruction);
        } else {
            panic!("Unexpected input!");
        }
    }

    (samples, program)
}

fn parse_registers(input: &str) -> Vec<RegSize> {
    input
        .split(|c| c == '[' || c == ']')
        .filter(|s| !s.starts_with("Before") && !s.starts_with("After"))
        .flat_map(|s| s.split(','))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<RegSize>().unwrap())
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
