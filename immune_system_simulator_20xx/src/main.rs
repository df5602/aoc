extern crate regex;
extern crate util;

use std::env;

use regex::Regex;

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

    let (immune_system, infection) = parse_input(&input);
    println!("Immune system:");
    for group in immune_system {
        println!("{:?}", group);
    }
    println!("Infection:");
    for group in infection {
        println!("{:?}", group);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ArmyType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum AttackType {
    Bludgeoning,
    Slashing,
    Radiation,
    Fire,
    Cold,
}

#[derive(Debug, Clone, PartialEq)]
struct Group {
    army_id: ArmyType,
    group_id: usize,
    units: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: AttackType,
    initiative: usize,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ParseState {
    Start,
    ImmuneSystem,
    Infection,
}

fn parse_input(input: &[String]) -> (Vec<Group>, Vec<Group>) {
    let mut state = ParseState::Start;
    let mut immune_system = Vec::new();
    let mut infection = Vec::new();
    let mut group_id = 0;

    let regex: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points (?:\((.+)\) )?with an attack that does (\d+) (.+) damage at initiative (\d+)$").unwrap();

    for line in input {
        if state == ParseState::Start && line.starts_with("Immune System") {
            state = ParseState::ImmuneSystem;
        } else if state == ParseState::Start && line.starts_with("Infection") {
            state = ParseState::Infection;
        } else if line.is_empty() {
            // skip
        } else if state == ParseState::ImmuneSystem && line.starts_with("Infection") {
            state = ParseState::Infection;
        } else if state == ParseState::Infection && line.starts_with("Immune System") {
            state = ParseState::ImmuneSystem;
        } else if state == ParseState::ImmuneSystem || state == ParseState::Infection {
            let captures = match regex.captures(line) {
                Some(captures) => captures,
                None => {
                    panic!("input does not match expected format");
                }
            };
            let mut values: Vec<&str> = Vec::new();
            for capture in captures.iter().skip(1) {
                match capture {
                    Some(capture) => values.push(capture.as_str()),
                    None => values.push(""),
                }
            }

            let attack_type: AttackType = match values[4] {
                "bludgeoning" => AttackType::Bludgeoning,
                "slashing" => AttackType::Slashing,
                "radiation" => AttackType::Radiation,
                "fire" => AttackType::Fire,
                "cold" => AttackType::Cold,
                at => panic!("unknown attack type: {}", at),
            };

            let mut weaknesses: Vec<AttackType> = Vec::new();
            let mut immunities: Vec<AttackType> = Vec::new();
            for s in values[2]
                .split(';')
                .map(|s| s.trim())
                .filter(|&s| !s.is_empty())
            {
                if s.starts_with("immune to") {
                    for s in s.split_whitespace().flat_map(|s| {
                        s.split(',')
                            .map(|s| s.trim())
                            .filter(|&s| !s.is_empty() && s != "immune" && s != "to")
                    }) {
                        match s {
                            "bludgeoning" => immunities.push(AttackType::Bludgeoning),
                            "slashing" => immunities.push(AttackType::Slashing),
                            "radiation" => immunities.push(AttackType::Radiation),
                            "fire" => immunities.push(AttackType::Fire),
                            "cold" => immunities.push(AttackType::Cold),
                            at => panic!("unknown attack type: {}", at),
                        }
                    }
                } else if s.starts_with("weak to") {
                    for s in s.split_whitespace().flat_map(|s| {
                        s.split(',')
                            .map(|s| s.trim())
                            .filter(|&s| !s.is_empty() && s != "weak" && s != "to")
                    }) {
                        match s {
                            "bludgeoning" => weaknesses.push(AttackType::Bludgeoning),
                            "slashing" => weaknesses.push(AttackType::Slashing),
                            "radiation" => weaknesses.push(AttackType::Radiation),
                            "fire" => weaknesses.push(AttackType::Fire),
                            "cold" => weaknesses.push(AttackType::Cold),
                            at => panic!("unknown attack type: {}", at),
                        }
                    }
                } else {
                    panic!("invalid input {}", s);
                }
            }

            let group = Group {
                army_id: if state == ParseState::ImmuneSystem {
                    ArmyType::ImmuneSystem
                } else {
                    ArmyType::Infection
                },
                group_id: {
                    group_id += 1;
                    group_id - 1
                },
                units: values[0].parse::<usize>().unwrap(),
                hit_points: values[1].parse::<usize>().unwrap(),
                attack_damage: values[3].parse::<usize>().unwrap(),
                attack_type,
                initiative: values[5].parse::<usize>().unwrap(),
                weaknesses,
                immunities,
            };

            match state {
                ParseState::ImmuneSystem => immune_system.push(group),
                ParseState::Infection => infection.push(group),
                _ => unreachable!(),
            }
        } else {
            panic!("invalid input");
        }
    }

    (immune_system, infection)
}
