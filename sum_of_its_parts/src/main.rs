use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::str::FromStr;

use util::input::{FileReader, FromFile};

const DEBUG: bool = false;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<Dependency> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let (unprocessed_fwd, mut unprocessed_bwd) = create_dependencies(&input);

    let order = part1(&unprocessed_fwd.clone(), &mut unprocessed_bwd.clone());

    println!("Order (alone): {}", order);
    println!();

    let (order, finish_time) = part2(&unprocessed_fwd, &mut unprocessed_bwd);

    println!("Order (with help): {}", order);
    println!("Finish Time: {}", finish_time);
}

fn create_dependencies(dependencies: &[Dependency]) -> (Vec<Step>, Vec<Step>) {
    let mut unprocessed_fwd = Vec::new();
    let mut unprocessed_bwd = Vec::new();
    for i in b'A'..=b'Z' {
        unprocessed_fwd.push(Step::new(i));
        unprocessed_bwd.push(Step::new(i));
    }

    for dep in dependencies.iter() {
        unprocessed_fwd[StepId::idx(dep.before)]
            .dependents
            .push(dep.step);
        unprocessed_bwd[StepId::idx(dep.step)]
            .dependencies
            .push(dep.before);
    }

    (unprocessed_fwd, unprocessed_bwd)
}

fn part1(unprocessed_fwd: &[Step], unprocessed_bwd: &mut Vec<Step>) -> String {
    let mut ready_list: BinaryHeap<StepId> = BinaryHeap::new();

    // Find all elements without dependency
    for step in unprocessed_bwd
        .iter()
        .filter(|step| step.dependencies.is_empty())
        .filter(|step| !unprocessed_fwd[StepId::idx(step.id)].dependents.is_empty())
    {
        if DEBUG {
            println!("Pushing {} to ready list.", step.id as char);
        }
        ready_list.push(StepId(step.id));
    }

    // Process all elements in ready list
    let mut order = String::new();
    loop {
        let next = match ready_list.pop() {
            Some(id) => id,
            None => {
                if DEBUG {
                    println!("No more elements. Aborting...");
                }
                break;
            }
        };

        if DEBUG {
            println!("Processing: {}", next.char());
        }

        // Push to result string
        order.push(next.char());

        // Remove next as dependency and insert dependents that are ready (no further dependencies) into ready list
        for step in unprocessed_fwd[next.to_idx()].dependents.iter() {
            let dependencies = &mut unprocessed_bwd[StepId::idx(*step)].dependencies;
            dependencies.retain(|&id| id != next.char() as u8);
            if dependencies.is_empty() {
                if DEBUG {
                    println!("Pushing {} to ready list.", *step as char);
                }
                ready_list.push(StepId(*step));
            } else if DEBUG {
                println!("{} has more dependencies.", *step as char);
            }
        }
    }

    order
}

fn part2(unprocessed_fwd: &[Step], unprocessed_bwd: &mut Vec<Step>) -> (String, usize) {
    let mut ready_list: BinaryHeap<Event> = BinaryHeap::new();

    // Find all elements without dependency
    for step in unprocessed_bwd
        .iter()
        .filter(|step| step.dependencies.is_empty())
        .filter(|step| !unprocessed_fwd[StepId::idx(step.id)].dependents.is_empty())
    {
        if DEBUG {
            println!("Pushing {} to ready list with T = 0.", step.id as char);
        }
        ready_list.push(Event::new(0, StepId(step.id)));
    }

    // Process all elements in ready list
    let mut order = String::new();
    let mut finish_time = 0;
    let mut workers_idle = 5;
    loop {
        let mut next = match ready_list.pop() {
            Some(id) => id,
            None => {
                if DEBUG {
                    println!("No more elements. Aborting...");
                }
                break;
            }
        };

        if workers_idle == 0 && !next.worker {
            if DEBUG {
                println!("[T = {}] All workers busy", next.time);
            }
            // TODO: maybe keep track of expected readyness of workers to have less useless events?
            next.time += 1;
            ready_list.push(next);
            continue;
        }

        let next = next; // immutable from now on

        if next.worker {
            if DEBUG {
                println!("[T = {}] Worker becomes ready", next.time);
            }
            workers_idle += 1;
            continue;
        }

        if DEBUG {
            println!("[T = {}] Processing: {}", next.time, next.id.char());
        }
        workers_idle -= 1;
        // TODO: maybe keep track of expected readyness of workers to have less useless events?
        ready_list.push(Event::worker(next.finish_time()));

        // Update finish time
        order.push(next.id.char());
        if next.finish_time() > finish_time {
            finish_time = next.finish_time();
        }

        // Remove next as dependency and insert dependents that are ready (no further dependencies) into ready list
        for step in unprocessed_fwd[next.id.to_idx()].dependents.iter() {
            let dependencies = &mut unprocessed_bwd[StepId::idx(*step)].dependencies;
            dependencies.retain(|&id| id != next.id.char() as u8);
            if dependencies.is_empty() {
                if DEBUG {
                    println!(
                        "Pushing {} to ready list with T = {}.",
                        *step as char,
                        next.finish_time()
                    );
                }
                ready_list.push(Event::new(next.finish_time(), StepId(*step)));
            } else if DEBUG {
                println!("{} has more dependencies.", *step as char);
            }
        }
    }

    (order, finish_time)
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
struct Event {
    time: usize,
    id: StepId,
    worker: bool,
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        // We want to implement a Min Heap, so invert normal sort order
        if self.time == other.time {
            if self.worker && !other.worker {
                Ordering::Greater
            } else if !self.worker && other.worker {
                Ordering::Less
            } else if self.worker && other.worker {
                Ordering::Equal
            } else {
                self.id.cmp(&other.id)
            }
        } else {
            other.time.cmp(&self.time)
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time && self.id == other.id
    }
}

impl Eq for Event {}

impl Event {
    fn new(time: usize, id: StepId) -> Self {
        Self {
            time,
            id,
            worker: false,
        }
    }

    fn worker(time: usize) -> Self {
        Self {
            time,
            id: StepId(0),
            worker: true,
        }
    }

    fn finish_time(&self) -> usize {
        self.time + 60 + self.id.to_idx() + 1
    }
}

#[derive(Debug, Clone)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.before as char, self.step as char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dependencies: Vec<Dependency> = FileReader::new().read_from_file("input.txt").unwrap();
        let (unprocessed_fwd, mut unprocessed_bwd) = create_dependencies(&dependencies);
        let order = part1(&unprocessed_fwd, &mut unprocessed_bwd);
        assert_eq!("CQSWKZFJONPBEUMXADLYIGVRHT", order);
    }

    #[test]
    fn test_part2() {
        let dependencies: Vec<Dependency> = FileReader::new().read_from_file("input.txt").unwrap();
        let (unprocessed_fwd, mut unprocessed_bwd) = create_dependencies(&dependencies);
        let (_, finish_time) = part2(&unprocessed_fwd, &mut unprocessed_bwd);
        assert_eq!(914, finish_time);
    }
}
