extern crate util;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::str::FromStr;

use util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Dependency> = match FileReader::read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut unprocessed_fwd = Vec::new();
    let mut unprocessed_bwd = Vec::new();
    for i in b'A'..=b'Z' {
        unprocessed_fwd.push(Step::new(i));
        unprocessed_bwd.push(Step::new(i));
    }

    for dep in input {
        unprocessed_fwd[StepId::idx(dep.before)]
            .dependents
            .push(dep.step);
        println!("Dependent: {} => {}", dep.before as char, dep.step as char);
        unprocessed_bwd[StepId::idx(dep.step)]
            .dependencies
            .push(dep.before);
        println!(
            "Dependencies: {} => {}",
            dep.before as char, dep.step as char
        );
    }

    let mut ready_list: BinaryHeap<StepId> = BinaryHeap::new();

    // Find all elements without dependency
    for step in unprocessed_bwd
        .iter()
        .filter(|step| step.dependencies.is_empty())
        .filter(|step| !unprocessed_fwd[StepId::idx(step.id)].dependents.is_empty())
    {
        println!("Pushing {} to ready list.", step.id as char);
        ready_list.push(StepId(step.id));
    }

    // Process all elements in ready list
    let mut order = String::new();
    loop {
        let next = match ready_list.pop() {
            Some(id) => id,
            None => {
                println!("No more elements. Aborting...");
                break;
            }
        };

        println!("Processing: {}", next.char());

        // Push to result string
        order.push(next.char());

        // Remove next as dependency and insert dependents that are ready (no further dependencies) into ready list
        for step in unprocessed_fwd[next.to_idx()].dependents.iter() {
            let dependencies = &mut unprocessed_bwd[StepId::idx(*step)].dependencies;
            dependencies.retain(|&id| id != next.char() as u8);
            if dependencies.is_empty() {
                println!("Pushing {} to ready list.", *step as char);
                ready_list.push(StepId(*step));
            } else {
                println!("{} has more dependencies.", *step as char);
            }
        }
    }

    println!("Order: {}", order);
}

#[derive(Debug)]
struct StepId(u8);

impl Ord for StepId {
    fn cmp(&self, other: &StepId) -> Ordering {
        // We want to implement a Min Heap, so invert normal sort order
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for StepId {
    fn partial_cmp(&self, other: &StepId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StepId {
    fn eq(&self, other: &StepId) -> bool {
        self.0 == other.0
    }
}

impl Eq for StepId {}

impl StepId {
    fn idx(letter: u8) -> usize {
        (letter - b'A') as usize
    }

    fn to_idx(&self) -> usize {
        StepId::idx(self.0)
    }

    fn char(&self) -> char {
        self.0 as char
    }
}

#[derive(Debug)]
struct Step {
    id: u8,
    dependencies: Vec<u8>,
    dependents: Vec<u8>,
}

impl Step {
    fn new(id: u8) -> Self {
        Self {
            id,
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Dependency {
    before: u8,
    step: u8,
}

impl FromStr for Dependency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters: Vec<char> = s
            .split_whitespace()
            .map(|s| s.trim())
            .filter(|&s| s.len() == 1)
            .map(|s| s.chars().next().unwrap())
            .collect();

        if letters.len() != 2 {
            return Err(String::from("input does not match format"));
        }

        Ok(Self {
            before: letters[0] as u8,
            step: letters[1] as u8,
        })
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} => {}", self.before as char, self.step as char)
    }
}
