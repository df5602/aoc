use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use chrono::naive::NaiveDateTime;
use chrono::Timelike;

use util::input::{FileReader, FromFile};

use adhoc_derive::FromStr;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let mut records: Vec<Record> = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    records.sort_unstable_by_key(|r| r.timestamp);
    let records = records;

    let sleep_distributions = create_distributions(&records);

    let (guard_most_asleep, asleep_time) = find_guard_most_minutes_asleep(&sleep_distributions);
    let minute_asleep_most = sleep_distributions[&guard_most_asleep].minute_most_asleep();

    println!(
        "Guard most asleep: {} => was asleep for {} minutes (most at minute {}) => Result: {}",
        guard_most_asleep,
        asleep_time,
        minute_asleep_most,
        guard_most_asleep * minute_asleep_most
    );

    let (guard_most_asleep_at_same_minute, minute_asleep_most) =
        find_guard_most_asleep_at_same_minute(&sleep_distributions);

    println!(
        "Guard most asleep at single minute: {} @ minute {} => Result: {}",
        guard_most_asleep_at_same_minute,
        minute_asleep_most,
        guard_most_asleep_at_same_minute * minute_asleep_most
    );
}

fn create_distributions(records: &[Record]) -> HashMap<usize, SleepDistribution> {
    let mut distributions: HashMap<usize, SleepDistribution> = HashMap::new();

    let mut current_guard_id = 0;
    let mut fall_asleep_time = 0;
    for record in records.iter() {
        match record.entry {
            Entry::ShiftBegin(id) => current_guard_id = id,
            Entry::FallAsleep => fall_asleep_time = record.timestamp.time().minute(),
            Entry::WakeUp => {
                let wake_up = record.timestamp.time().minute();
                distributions
                    .entry(current_guard_id)
                    .or_insert_with(SleepDistribution::new)
                    .increment_asleep(fall_asleep_time as usize, wake_up as usize);
            }
        }
    }

    distributions
}

fn find_guard_most_minutes_asleep(
    sleep_distributions: &HashMap<usize, SleepDistribution>,
) -> (usize, u32) {
    let mut guard_most_asleep = usize::min_value();
    let mut asleep_time = u32::min_value();
    for (&id, sum) in sleep_distributions.iter().map(|(k, v)| (k, v.sum())) {
        if sum > asleep_time {
            guard_most_asleep = id;
            asleep_time = sum;
        }
    }
    (guard_most_asleep, asleep_time)
}

fn find_guard_most_asleep_at_same_minute(
    sleep_distributions: &HashMap<usize, SleepDistribution>,
) -> (usize, usize) {
    let mut guard_most_asleep = usize::min_value();
    let mut minute_asleep_most = usize::min_value();
    let mut maximum_minutes = u32::min_value();
    for (&id, (minute_most_asleep, max_minute)) in sleep_distributions.iter().map(|(k, v)| {
        let minute_most_asleep = v.minute_most_asleep();
        let max_minute = v.at(minute_most_asleep);
        (k, (minute_most_asleep, max_minute))
    }) {
        if max_minute > maximum_minutes {
            maximum_minutes = max_minute;
            minute_asleep_most = minute_most_asleep;
            guard_most_asleep = id;
        }
    }

    (guard_most_asleep, minute_asleep_most)
}

struct SleepDistribution {
    minutes: [u32; 60],
}

impl SleepDistribution {
    fn new() -> Self {
        Self { minutes: [0; 60] }
    }

    fn at(&self, minute: usize) -> u32 {
        assert!(minute < 60);
        self.minutes[minute]
    }

    fn increment_asleep(&mut self, from: usize, until: usize) {
        assert!(from <= until);
        assert!(until < 60);

        for min in from..until {
            self.minutes[min] += 1;
        }
    }

    fn sum(&self) -> u32 {
        self.minutes.iter().sum()
    }

    fn minute_most_asleep(&self) -> usize {
        let mut minute_most_asleep = usize::min_value();
        let mut minutes_asleep = u32::min_value();
        for (minute, &amount) in self.minutes.iter().enumerate() {
            if amount > minutes_asleep {
                minute_most_asleep = minute;
                minutes_asleep = amount;
            }
        }
        minute_most_asleep
    }
}

#[derive(Debug)]
enum Entry {
    ShiftBegin(usize),
    FallAsleep,
    WakeUp,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Guard") {
            match s
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.trim())
                .filter(|s| s.starts_with('#'))
                .map(|s| s[1..].parse().map_err(|e| format!("{}", e)))
                .nth(0)
            {
                Some(id) => Ok(Entry::ShiftBegin(id?)),
                None => Err("could not find guard id".to_string()),
            }
        } else if s.contains("asleep") {
            Ok(Entry::FallAsleep)
        } else if s.contains("wakes") {
            Ok(Entry::WakeUp)
        } else {
            Err("invalid entry".to_string())
        }
    }
}

#[derive(Debug, FromStr)]
#[adhoc(regex = r"^\[(?P<timestamp>.+)\] (?P<entry>.+)$")]
struct Record {
    #[adhoc(
        construct_with = r#"NaiveDateTime::parse_from_str(timestamp: &str, "%Y-%m-%d %H:%M")?"#
    )]
    timestamp: NaiveDateTime,
    entry: Entry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut records: Vec<Record> = FileReader::new().read_from_file("input.txt").unwrap();
        records.sort_unstable_by_key(|r| r.timestamp);
        let sleep_distributions = create_distributions(&records);
        let (guard_most_asleep, _) = find_guard_most_minutes_asleep(&sleep_distributions);
        let minute_asleep_most = sleep_distributions[&guard_most_asleep].minute_most_asleep();
        assert_eq!(36898, guard_most_asleep * minute_asleep_most);
    }

    #[test]
    fn test_part2() {
        let mut records: Vec<Record> = FileReader::new().read_from_file("input.txt").unwrap();
        records.sort_unstable_by_key(|r| r.timestamp);
        let sleep_distributions = create_distributions(&records);
        let (guard_most_asleep_at_same_minute, minute_asleep_most) =
            find_guard_most_asleep_at_same_minute(&sleep_distributions);
        assert_eq!(80711, guard_most_asleep_at_same_minute * minute_asleep_most);
    }
}
