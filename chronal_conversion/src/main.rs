extern crate util;

use std::collections::HashMap;
use std::env;

use util::input::{FileReader, FromFile};

type RegSize = u32;
const REG_NUMBER: usize = 6;

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

    let program = parse_input(&input);

    let mut regs = [0; REG_NUMBER];
    program.execute(&mut regs);
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
    assert_eq!(REG_NUMBER, regs.len());
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

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    args: [RegSize; 3],
}

#[derive(Debug)]
struct Program {
    ip: RegSize,
    instructions: Vec<Instruction>,
}

impl Program {
    fn execute(&self, mut regs: &mut [RegSize]) {
        assert_eq!(REG_NUMBER, regs.len());

        // From inspection of the input: R0 only occurs in one instruction, namely line 28 (in my input).
        // Let's verify that:
        assert_eq!(Opcode::Eqrr, self.instructions[28].opcode);

        // The instruction is eqrr x 0 1, i.e. compare Rx with R0
        // (This would be equivalent to eqrr 0 x 1)
        let args = self.instructions[28].args;
        let rx = if args[0] != 0 {
            args[0]
        } else {
            assert_ne!(0, args[1]);
            args[1]
        };
        assert!((rx as usize) < REG_NUMBER);

        // For part 2 we try to find the last unique value for Rx
        let mut counts = HashMap::new();
        let mut count = 0u64;
        let mut first_found = false;
        let mut last_unique = 0;
        loop {
            let ip = regs[self.ip as usize];
            if (ip as usize) >= self.instructions.len() {
                break;
            }

            count += 1;
            if ip == 28 {
                let reg_x = regs[rx as usize];
                if !first_found {
                    println!(
                        "Value of R0 for which program terminates in fewest steps: {}",
                        reg_x
                    );
                    first_found = true;
                }
                let entry = counts.entry(reg_x).or_insert(0u64);
                if *entry == 0 {
                    *entry = count;
                    last_unique = reg_x;
                } else {
                    println!(
                        "Value of R0 for which program terminates in most steps: {}",
                        last_unique
                    );
                    break;
                }
            }

            let instruction = &self.instructions[ip as usize];
            evaluate_instruction(&mut regs, instruction.opcode, &instruction.args);
            regs[self.ip as usize] += 1;
        }
    }
}

fn parse_input(input: &[String]) -> Program {
    let mut ip = 0;
    let mut got_ip = false;

    let mut instructions = Vec::new();

    for line in input {
        if line.starts_with('#') {
            if got_ip {
                panic!("Already got an IP!");
            }
            ip = line
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<RegSize>().unwrap())
                .nth(0)
                .unwrap();
            assert!((ip as usize) < REG_NUMBER);
            got_ip = true;
        } else if line.starts_with(';') {
            // skip comments
            continue;
        } else {
            let mut iter = line.split_whitespace();
            let opcode = match iter.next().unwrap() {
                "addr" => Opcode::Addr,
                "addi" => Opcode::Addi,
                "mulr" => Opcode::Mulr,
                "muli" => Opcode::Muli,
                "banr" => Opcode::Banr,
                "bani" => Opcode::Bani,
                "borr" => Opcode::Borr,
                "bori" => Opcode::Bori,
                "setr" => Opcode::Setr,
                "seti" => Opcode::Seti,
                "gtir" => Opcode::Gtir,
                "gtri" => Opcode::Gtri,
                "gtrr" => Opcode::Gtrr,
                "eqir" => Opcode::Eqir,
                "eqri" => Opcode::Eqri,
                "eqrr" => Opcode::Eqrr,
                s => panic!("Unknown opcode: {}", s),
            };
            let mut args: [RegSize; 3] = [0; 3];
            for arg in args.iter_mut() {
                *arg = iter.next().unwrap().parse::<RegSize>().unwrap();
            }
            assert!(iter.next().is_none());
            instructions.push(Instruction { opcode, args });
        }
    }

    Program { ip, instructions }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr() {
        let mut registers = [0, 1, 2, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Addr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 3, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Addr, &[2, 3, 1]);
        assert_eq!([0, 5, 2, 3, 0, 0], registers);
    }

    #[test]
    fn addi() {
        let mut registers = [0, 1, 2, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Addi, &[1, 5, 3]);
        assert_eq!([0, 1, 2, 6, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Addr, &[2, 2, 1]);
        assert_eq!([0, 4, 2, 6, 0, 0], registers);
    }

    #[test]
    fn mulr() {
        let mut registers = [0, 1, 2, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Mulr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 2, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Mulr, &[0, 3, 1]);
        assert_eq!([0, 0, 2, 2, 0, 0], registers);
    }

    #[test]
    fn muli() {
        let mut registers = [0, 1, 2, 10, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Muli, &[1, 2, 1]);
        assert_eq!([0, 2, 2, 10, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Muli, &[1, 10, 0]);
        assert_eq!([20, 2, 2, 10, 0, 0], registers);
    }

    #[test]
    fn banr() {
        let mut registers = [0, 1, 3, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Banr, &[1, 2, 3]);
        assert_eq!([0, 1, 3, 1, 0, 0], registers);
    }

    #[test]
    fn bani() {
        let mut registers = [0, 1, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Bani, &[1, 3, 2]);
        assert_eq!([0, 1, 1, 0, 0, 0], registers);
    }

    #[test]
    fn borr() {
        let mut registers = [0, 1, 2, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Borr, &[1, 2, 3]);
        assert_eq!([0, 1, 2, 3, 0, 0], registers);
    }

    #[test]
    fn bori() {
        let mut registers = [0, 2, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Bori, &[1, 4, 3]);
        assert_eq!([0, 2, 0, 6, 0, 0], registers);
    }

    #[test]
    fn setr() {
        let mut registers = [0, 2, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Setr, &[1, 2, 0]);
        assert_eq!([2, 2, 0, 0, 0, 0], registers);
    }

    #[test]
    fn seti() {
        let mut registers = [0, 2, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Seti, &[42, 2, 0]);
        assert_eq!([42, 2, 0, 0, 0, 0], registers);
    }

    #[test]
    fn gtir() {
        let mut registers = [0, 2, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Gtir, &[5, 1, 3]);
        assert_eq!([0, 2, 0, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Gtir, &[1, 1, 3]);
        assert_eq!([0, 2, 0, 0, 0, 0], registers);
    }

    #[test]
    fn gtri() {
        let mut registers = [0, 2, 0, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Gtri, &[1, 1, 3]);
        assert_eq!([0, 2, 0, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Gtri, &[1, 3, 3]);
        assert_eq!([0, 2, 0, 0, 0, 0], registers);
    }

    #[test]
    fn gtrr() {
        let mut registers = [0, 2, 1, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Gtrr, &[1, 2, 3]);
        assert_eq!([0, 2, 1, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Gtrr, &[2, 1, 3]);
        assert_eq!([0, 2, 1, 0, 0, 0], registers);
    }

    #[test]
    fn eqir() {
        let mut registers = [0, 0, 1, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Eqir, &[1, 2, 3]);
        assert_eq!([0, 0, 1, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Eqir, &[0, 2, 3]);
        assert_eq!([0, 0, 1, 0, 0, 0], registers);
    }

    #[test]
    fn eqri() {
        let mut registers = [0, 5, 1, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Eqri, &[1, 5, 3]);
        assert_eq!([0, 5, 1, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Eqri, &[1, 4, 3]);
        assert_eq!([0, 5, 1, 0, 0, 0], registers);
    }

    #[test]
    fn eqrr() {
        let mut registers = [0, 5, 5, 0, 0, 0];
        evaluate_instruction(&mut registers, Opcode::Eqrr, &[1, 2, 3]);
        assert_eq!([0, 5, 5, 1, 0, 0], registers);

        evaluate_instruction(&mut registers, Opcode::Eqrr, &[0, 1, 3]);
        assert_eq!([0, 5, 5, 0, 0, 0], registers);
    }
}
