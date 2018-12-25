extern crate regex;
extern crate util;

use std::collections::HashMap;
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

    let (mut immune_system, mut infection) = parse_input(&input);

    simulate_fight(&mut immune_system, &mut infection);
}

fn simulate_fight(immune_system: &mut Vec<Group>, infection: &mut Vec<Group>) {
    while !immune_system.is_empty() && !infection.is_empty() {
        fight_round(immune_system, infection);
    }

    println!("Immune system:");
    println!("Remaining units: {}", get_remaining_units(&immune_system));
    println!("Infection:");
    println!("Remaining units: {}", get_remaining_units(&infection));
}

fn get_remaining_units(groups: &[Group]) -> usize {
    groups.iter().map(|group| group.units).sum()
}

fn fight_round(immune_system: &mut Vec<Group>, infection: &mut Vec<Group>) {
    let targets = target_selection(immune_system, infection);
    let attack_order = get_attack_order(immune_system, infection);
    let mut dead_groups: Vec<Group> = Vec::new();

    for attacker in attack_order {
        // Check whether still alive
        if dead_groups
            .iter()
            .any(|group| group.group_id == attacker.group_id)
        {
            continue;
        }

        // Update attacker
        let attacker = match attacker.army_id {
            ArmyType::ImmuneSystem => {
                let pos = immune_system
                    .iter()
                    .position(|group| group.group_id == attacker.group_id)
                    .unwrap();
                immune_system[pos].clone()
            }
            ArmyType::Infection => {
                let pos = infection
                    .iter()
                    .position(|group| group.group_id == attacker.group_id)
                    .unwrap();
                infection[pos].clone()
            }
        };

        // Get victim
        let (mut victim, pos) = if let Some(Some(victim)) = targets.get(&attacker.group_id) {
            match attacker.army_id {
                ArmyType::ImmuneSystem => {
                    let pos = infection
                        .iter()
                        .position(|group| group.group_id == *victim)
                        .unwrap();
                    (&mut infection[pos], pos)
                }
                ArmyType::Infection => {
                    let pos = immune_system
                        .iter()
                        .position(|group| group.group_id == *victim)
                        .unwrap();
                    (&mut immune_system[pos], pos)
                }
            }
        } else {
            continue;
        };

        // Deal damage
        if deal_damage(&attacker, &mut victim) {
            let dead_victim = match attacker.army_id {
                ArmyType::ImmuneSystem => infection.remove(pos),
                ArmyType::Infection => immune_system.remove(pos),
            };
            dead_groups.push(dead_victim);
        }
    }
}

fn target_selection(
    immune_system: &mut Vec<Group>,
    infection: &mut Vec<Group>,
) -> HashMap<usize, Option<usize>> {
    let mut targets: HashMap<usize, Option<usize>> = HashMap::new();

    immune_system.sort_by(|a, b| {
        if a.effective_power() == b.effective_power() {
            b.initiative.cmp(&a.initiative)
        } else {
            b.effective_power().cmp(&a.effective_power())
        }
    });
    infection.sort_by(|a, b| {
        if a.effective_power() == b.effective_power() {
            b.initiative.cmp(&a.initiative)
        } else {
            b.effective_power().cmp(&a.effective_power())
        }
    });

    let mut immune_system_targets = immune_system.clone();
    let mut infection_targets = infection.clone();

    for group in immune_system {
        if let Some(target) = group.select_target(&infection_targets) {
            let target_id = target.group_id;
            infection_targets.retain(|group| group.group_id != target_id);
            targets.insert(group.group_id, Some(target_id));
        } else {
            targets.insert(group.group_id, None);
        }
    }

    for group in infection {
        if let Some(target) = group.select_target(&immune_system_targets) {
            let target_id = target.group_id;
            immune_system_targets.retain(|group| group.group_id != target_id);
            targets.insert(group.group_id, Some(target_id));
        } else {
            targets.insert(group.group_id, None);
        }
    }

    targets
}

fn get_attack_order(immune_system: &[Group], infection: &[Group]) -> Vec<Group> {
    let mut attack_order = Vec::new();
    attack_order.extend_from_slice(immune_system);
    attack_order.extend_from_slice(infection);

    attack_order.sort_by(|a, b| b.initiative.cmp(&a.initiative));

    attack_order
}

fn deal_damage(attacker: &Group, victim: &mut Group) -> bool {
    let damage = attacker.calculate_damage(victim);
    let victim_units = victim.units;
    let victim_hp = victim.hit_points;
    let killed = damage / victim_hp;

    if killed >= victim_units {
        victim.units = 0;
        true
    } else {
        victim.units -= killed;
        false
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

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }

    fn calculate_damage(&self, other: &Group) -> usize {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn select_target<'a>(&self, enemies: &'a [Group]) -> Option<&'a Group> {
        let mut chosen = None;
        let mut max_damage = usize::min_value();
        let mut max_power = usize::min_value();
        let mut max_initiative = usize::min_value();

        for enemy in enemies.iter() {
            let damage = self.calculate_damage(enemy);
            if damage >= max_damage {
                let power = enemy.effective_power();
                let initiative = enemy.initiative;
                if damage > max_damage {
                    max_power = power;
                    max_initiative = initiative;
                    chosen = Some(enemy);
                } else {
                    // damage is equal
                    if power >= max_power {
                        if power > max_power {
                            max_initiative = initiative;
                            chosen = Some(enemy);
                        } else {
                            // damage + power is equal
                            if initiative > max_initiative {
                                max_initiative = initiative;
                                chosen = Some(enemy);
                            }
                        }
                        max_power = power;
                    }
                }
                max_damage = damage;
            }
        }

        if max_damage == 0 {
            chosen = None;
        }
        chosen
    }
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
